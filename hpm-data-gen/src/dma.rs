use std::{collections::HashMap, path::Path};

fn parse_signal(signal_name: &str, periph_name: &str) -> String {
    if signal_name.contains("_") {
        let suffix = signal_name.split("_").last().unwrap();

        if signal_name.starts_with("GPTMR") || signal_name.starts_with("NTMR") {
            format!("CH{}", suffix)
        } else {
            suffix.to_string()
        }
    } else if signal_name.starts_with("I2C") {
        "GLOBAL".to_string()
    } else {
        periph_name.to_string()
    }
}

pub fn handle_chip_dmamux_include<P: AsRef<Path>>(
    path: P,
    chip: &mut hpm_data_serde::Chip,
) -> anyhow::Result<()> {
    let meta_yaml_path = path.as_ref();

    for core in &mut chip.cores {
        if let Some(include_path) = core.include_dmamux.take() {
            let dma_yaml_path = meta_yaml_path.parent().unwrap().join(&include_path);
            let content = std::fs::read_to_string(&dma_yaml_path)?;
            let dmamux: HashMap<String, usize> = serde_yaml::from_str(&content)?;
            // println!("dma_channels: {:#?}", dmamux);

            for (signal_name, request_no) in dmamux {
                for periph in core.peripherals.iter_mut() {
                    if signal_name.starts_with(&periph.name) {
                        // println!("matches signal_name: {:#?}", signal_name);

                        let signal = parse_signal(&signal_name, &periph.name);

                        periph.dma_channels.push(
                            hpm_data_serde::chip::core::peripheral::DmaChannel {
                                signal: signal.clone(),
                                dmamux: Some("DMAMUX".to_string()),
                                request: request_no as u8,
                            },
                        );
                    }
                }
            }
        }
    }

    Ok(())
}
