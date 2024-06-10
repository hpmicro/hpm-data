//! handle pinmux matching

use std::{
    collections::{HashMap, HashSet},
    path::Path,
};

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct Pin {
    name: String,
    r#type: String,
    bank: String,
    alts: HashMap<String, AltDef>,
    // specials: { ANALOGS: { ... }}
    specials: HashMap<String, HashMap<String, AltDef>>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct AltDef {
    index: String, // integer as string
    module: String,
    instance: String,
    group: String,
    func: String,
}

impl AltDef {
    fn alt_num(&self) -> u32 {
        self.index.parse().unwrap()
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct PinmuxRaw {
    #[serde(rename = "statusCode")]
    status_code: u32,
    message: String,
    data: Vec<Pin>,
}

// The following peripherals are supported now.
const PERIPHERAL_LIST: &[&str] = &[
    "GPTMR", "I2C", "SPI", "UART", "MCAN", "USB", "I2S", "PWM", "ACMP", "CAM",
];

fn normalize_func(module: &str, func: &str) -> String {
    if module == "SYSCTL" {
        func.replace("CLK_", "").replace("[", "").replace("]", "")
    } else {
        func.replace("[", "").replace("]", "")
    }
}

fn get_pmic_periph_and_func(func: &str) -> Option<(String, String)> {
    // PUART, PTMR,
    if let Some((periph, f)) = func.split_once(".") {
        match periph {
            "PUART" => Some(("PUART".to_string(), f.to_string())),
            "PTMR" => Some(("PTMR".to_string(), f.replace("[", "").replace("]", ""))),
            _ => None,
        }
    } else {
        None
    }
}

pub fn handle_pinmux<P: AsRef<Path>>(
    path: P,
    chip: &mut hpm_data_serde::Chip,
) -> anyhow::Result<()> {
    let data = std::fs::read_to_string(path)?;
    let pinmux: PinmuxRaw = serde_json::from_str(&data)?;

    let pins = pinmux.data;

    // println!("Found {} pins", pins.len());

    // peripheral_name, signal_name, pin_name, alt_num
    let mut pinmux_alt_defs: HashSet<(String, String, String, u32)> = HashSet::new();

    for pin in pins {
        for (_alt_name, alt_def) in &pin.alts {
            if PERIPHERAL_LIST.contains(&&*alt_def.module) {
                let signal_name = normalize_func(&alt_def.module, &alt_def.func);
                pinmux_alt_defs.insert((
                    alt_def.instance.clone(),
                    signal_name,
                    pin.name.clone(),
                    alt_def.alt_num(),
                ));
            }
        }
        // TODO: handle ANALOGS
        if pin.specials.contains_key("PMIC") {
            for (_alt_name, alt_def) in &pin.specials["PMIC"] {
                if let Some((periph, signal_name)) = get_pmic_periph_and_func(&alt_def.func) {
                    pinmux_alt_defs.insert((
                        periph,
                        signal_name,
                        pin.name.clone(),
                        alt_def.alt_num(),
                    ));
                }
            }
        }
    }

    // println!("Found {:#?} pinmux alt defs", pinmux_alt_defs);

    let mut periph_pins: HashMap<String, Vec<(String, String, u32)>> = HashMap::new();

    for (peripheral_name, signal_name, pin_name, alt_num) in pinmux_alt_defs {
        periph_pins
            .entry(peripheral_name)
            .or_insert_with(Vec::new)
            .push((signal_name, pin_name, alt_num));
    }

    // fill pins
    for core in &mut chip.cores {
        for peripheral in &mut core.peripherals {
            if !peripheral.pins.is_empty() {
                println!(
                    "Skipping peripheral {} as it already has pins",
                    peripheral.name
                );
                continue;
            }

            peripheral.pins = periph_pins
                .get(&peripheral.name)
                .map(|pins| {
                    pins.iter()
                        .map(|(signal_name, pin_name, alt_num)| {
                            hpm_data_serde::chip::core::peripheral::Pin {
                                signal: signal_name.clone(),
                                pin: hpm_data_serde::chip::core::peripheral::pin::Pin::parse(
                                    pin_name,
                                )
                                .unwrap(),
                                alt: Some(*alt_num as _),
                            }
                        })
                        .collect()
                })
                .unwrap_or_default();
        }
    }

    Ok(())
}
