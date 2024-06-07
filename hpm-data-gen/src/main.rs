use std::{collections::HashMap, path::Path};

mod registers;

#[macro_export]
macro_rules! regex {
    ($re:literal) => {{
        ::ref_thread_local::ref_thread_local! {
            static managed REGEX: ::regex::Regex = ::regex::Regex::new($re).unwrap();
        }
        <REGEX as ::ref_thread_local::RefThreadLocal<::regex::Regex>>::borrow(&REGEX)
    }};
}

struct Stopwatch {
    start: std::time::Instant,
    section_start: Option<std::time::Instant>,
}

impl Stopwatch {
    fn new() -> Self {
        eprintln!("Starting timer");
        let start = std::time::Instant::now();
        Self {
            start,
            section_start: None,
        }
    }

    fn section(&mut self, status: &str) {
        let now = std::time::Instant::now();
        self.print_done(now);
        eprintln!("  {status}");
        self.section_start = Some(now);
    }

    fn stop(self) {
        let now = std::time::Instant::now();
        self.print_done(now);
        let total_elapsed = now - self.start;
        eprintln!("Total time: {:.2} seconds", total_elapsed.as_secs_f32());
    }

    fn print_done(&self, now: std::time::Instant) {
        if let Some(section_start) = self.section_start {
            let elapsed = now - section_start;
            eprintln!("    done in {:.2} seconds", elapsed.as_secs_f32());
        }
    }
}

fn main() -> anyhow::Result<()> {
    pretty_env_logger::init();

    let mut stopwatch = Stopwatch::new();

    //stopwatch.section("Parsing headers");
    //let headers = header::Headers::parse()?;

    //stopwatch.section("Parsing other stuff");

    stopwatch.section("Parsing registers");
    let registers = registers::Registers::parse()?;
    registers.write()?;

    stopwatch.section("Parsing chips");

    let data_dir = Path::new("./data");
    let mut chip_meta_files: Vec<_> = std::fs::read_dir(data_dir.join("chips"))
        .unwrap()
        .filter_map(|res| res.unwrap().file_name().to_str().map(|s| s.to_string()))
        .filter(|s| s.ends_with(".yaml"))
        .filter(|s| s.starts_with("HPM"))
        .map(|s| s.strip_suffix(".yaml").unwrap().to_string())
        .collect();
    chip_meta_files.sort();

    println!("chips: {:?}", chip_meta_files);

    std::fs::create_dir_all("build/data/chips")?;

    for name in &chip_meta_files {
        let meta_yaml_path = data_dir.join(&format!("chips/{}.yaml", name));
        let content = std::fs::read_to_string(&meta_yaml_path)?;
        let mut chip: hpm_data_serde::Chip = serde_yaml::from_str(&content)?;

        // handle include_x
        for core in &mut chip.cores {
            if let Some(inc_path) = core.include_interrupts.take() {
                let interrupts_yaml_path = meta_yaml_path.parent().unwrap().join(&inc_path);
                let content = std::fs::read_to_string(&interrupts_yaml_path)?;
                let interrupts: HashMap<String, u8> = serde_yaml::from_str(&content)?;
                let mut interrupts: Vec<(String, u8)> = interrupts.into_iter().collect();
                interrupts.sort_by_key(|(_, number)| *number);

                // println!("interrupts: {:#?}", interrupts);
                for (name, number) in interrupts {
                    core.interrupts
                        .push(hpm_data_serde::chip::core::Interrupt { name, number });
                }
                // core.interrupts.extend(interrupts.interrupts);
            }

            // append peripherals from includes
            if let Some(inc_paths) = &mut core.include_peripherals.take() {
                for inc_path in inc_paths {
                    let peripheral_yaml_path = meta_yaml_path.parent().unwrap().join(&inc_path);
                    let content = std::fs::read_to_string(&peripheral_yaml_path)?;
                    let peripherals: Vec<hpm_data_serde::chip::core::Peripheral> =
                        serde_yaml::from_str(&content)?;
                    core.peripherals.extend(peripherals);
                }
            }

            // generate dma channels
            if let Some(gen) = &mut core.gen_dma_channels.take() {
                let &hdma_chs = gen.get("HDMA").expect("HDMA not found");
                for ch in 0..hdma_chs {
                    core.dma_channels
                        .push(hpm_data_serde::chip::core::DmaChannels {
                            name: format!("HDMA_MUX{}", ch),
                            dma: "HDMA".to_string(),
                            channel: ch as _,
                        });
                }

                if let Some(&xdma_ch) = gen.get("XDMA") {
                    for ch in 0..xdma_ch {
                        core.dma_channels
                            .push(hpm_data_serde::chip::core::DmaChannels {
                                name: format!("XDMA_MUX{}", ch),
                                dma: "XDMA".to_string(),
                                channel: (hdma_chs + ch) as _, // xdma starts after hdma
                            });
                    }
                }
            }

            // append dmamux from includes
            if let Some(inc_path) = &mut core.include_dmamux.take() {
                let dma_yaml_path = meta_yaml_path.parent().unwrap().join(&inc_path);
                let content = std::fs::read_to_string(&dma_yaml_path)?;
                let dmamux: HashMap<String, usize> = serde_yaml::from_str(&content)?;
                let mut dma_channels: Vec<(String, usize)> = dmamux.into_iter().collect();
                dma_channels.sort_by_key(|(_, number)| *number);
                println!("dma_channels: {:#?}", dma_channels);

                //           core.dma_channels.extend(dma_channels);
            }
        }

        // DMA includes and dma_channels
        // dma::handle_chip_dma_include(&meta_yaml_path, &mut chip)?;

        println!(
            "chip: {}, peripherals: {}",
            chip.name,
            chip.cores[0].peripherals.len()
        );
        let dump = serde_json::to_string_pretty(&chip)?;
        std::fs::write(format!("build/data/chips/{name}.json"), dump)?;
    }

    // stopwatch.section("Parsing memories");
    //let memories = memory::Memories::parse()?;

    // stopwatch.section("Parsing interrupts");
    //let chip_interrupts = interrupts::ChipInterrupts::parse()?;

    // stopwatch.section("Parsing RCC registers");
    //let peripheral_to_clock = rcc::ParsedRccs::parse(&registers)?;

    // stopwatch.section("Parsing docs");
    // let docs = docs::Docs::parse()?;

    // stopwatch.section("Parsing DMA");
    //let dma_channels = dma::DmaChannels::parse()?;

    // stopwatch.section("Parsing GPIO AF");
    // let af = gpio_af::Af::parse()?;

    //let (chips, chip_groups) = chips::parse_groups()?;

    //stopwatch.section("Processing chips");
    /* chips::dump_all_chips(
        chip_groups,
        //  headers,
        // af,
        chip_interrupts,
        peripheral_to_clock,
        //  dma_channels,
        chips,
        memories,
        docs,
    )?;
    */

    stopwatch.stop();

    Ok(())
}
