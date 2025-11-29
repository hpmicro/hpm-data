use regex::Regex;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

/// Parse interrupts from header file (hpm_soc_irq.h)
fn parse_interrupts_from_header(header_path: &Path) -> anyhow::Result<HashMap<String, u8>> {
    let content = std::fs::read_to_string(header_path).map_err(|e| {
        anyhow::anyhow!("Failed to read interrupts header {:?}: {}", header_path, e)
    })?;

    // Match: #define IRQn_HDMA    34    /* HDMA IRQ */
    let pattern =
        Regex::new(r"#define\s+IRQn_(\w+)\s+(\d+)\s+/\*.*\*/").expect("Invalid interrupts regex");

    let mut interrupts = HashMap::new();

    for cap in pattern.captures_iter(&content) {
        let irq_name = cap.get(1).unwrap().as_str().to_string();
        let irq_number_str = cap.get(2).unwrap().as_str();
        let irq_number = irq_number_str.parse::<u8>().map_err(|_| {
            anyhow::anyhow!(
                "Failed to parse interrupt number '{}' in {:?}",
                irq_number_str,
                header_path
            )
        })?;

        // Apply naming fixes for consistency
        let fixed_name = fix_interrupt_naming(&irq_name);
        interrupts.insert(fixed_name, irq_number);
    }

    if interrupts.is_empty() {
        anyhow::bail!("No interrupt definitions found in {:?}", header_path);
    }

    println!(
        "    Loaded {} interrupts from header: {:?}",
        interrupts.len(),
        header_path.file_name().unwrap()
    );

    Ok(interrupts)
}

/// Fix naming inconsistencies between header and expected naming  
fn fix_interrupt_naming(name: &str) -> String {
    // Mapping table for naming fixes based on verification results
    // Note: We prioritize header file naming as it's the official source
    let name_fixes: HashMap<&str, &str> = [
        ("DAC", "DAC0"), // HPM6360: header DAC -> expected DAC0 consistency
                         // For HPM5301: header has PEWDG, TRGMUX0, TRGMUX1 (these are correct)
                         // For other inconsistencies, we keep header names as authoritative
    ]
    .into_iter()
    .collect();

    name_fixes.get(name).unwrap_or(&name).to_string()
}

/// Get interrupts header path for chip
fn get_interrupts_header_path(chip_name: &str) -> Option<PathBuf> {
    let sdk_path = std::env::var("HPM_SDK_BASE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap().join("./hpm_sdk"));

    let header_path = match chip_name {
        n if n.starts_with("HPM5301") => sdk_path.join("soc/HPM5300/HPM5301/hpm_soc_irq.h"),
        n if n.starts_with("HPM53") => sdk_path.join("soc/HPM5300/HPM5361/hpm_soc_irq.h"),
        n if n.starts_with("HPM5E") => sdk_path.join("soc/HPM5E00/HPM5E31/hpm_soc_irq.h"),
        n if n.starts_with("HPM62") => sdk_path.join("soc/HPM6200/HPM6280/hpm_soc_irq.h"),
        n if n.starts_with("HPM63") => sdk_path.join("soc/HPM6300/HPM6360/hpm_soc_irq.h"),
        n if n.starts_with("HPM67") || n.starts_with("HPM64") => {
            sdk_path.join("soc/HPM6700/HPM6750/hpm_soc_irq.h")
        }
        n if n.starts_with("HPM68") => sdk_path.join("soc/HPM6800/HPM6880/hpm_soc_irq.h"),
        n if n.starts_with("HPM6E") => sdk_path.join("soc/HPM6E00/HPM6E80/hpm_soc_irq.h"),
        n if n.starts_with("HPM6P") => sdk_path.join("soc/HPM6P00/HPM6P81/hpm_soc_irq.h"),
        _ => return None,
    };

    if header_path.exists() {
        Some(header_path)
    } else {
        None
    }
}

/// Load interrupts from header file for the given chip
pub fn load_interrupts_from_header(chip_name: &str) -> anyhow::Result<Option<HashMap<String, u8>>> {
    if let Some(header_path) = get_interrupts_header_path(chip_name) {
        let interrupts = parse_interrupts_from_header(&header_path)?;
        Ok(Some(interrupts))
    } else {
        Ok(None)
    }
}

fn parse_interrupt_signal(irq_name: &str) -> String {
    if irq_name.contains("_") {
        let suffix = irq_name.split("_").last().unwrap();

        if irq_name.starts_with("GPIO") {
            format!("P{}", suffix)
        } else if irq_name.starts_with("ACMP") {
            format!("CH{}", suffix)
        } else {
            suffix.to_string()
        }
    } else {
        "GLOBAL".to_string()
    }
}

pub fn fill_peripheral_interrupts(chip: &mut hpm_data_serde::Chip) -> anyhow::Result<()> {
    for core in chip.cores.iter_mut() {
        let interrupts = core.interrupts.clone();

        for interrupt in &interrupts {
            for periph in core.peripherals.iter_mut() {
                if !interrupt.name.starts_with(&periph.name) {
                    continue;
                }
                // special handling for UART10+
                if periph.name.starts_with("UART") && periph.name != interrupt.name {
                    continue;
                }
                // println!("matches interrupt: {:#?}", interrupt);

                let signal = parse_interrupt_signal(&interrupt.name);

                let mut periph_ints = periph.interrupts.take().unwrap_or_default();

                periph_ints.push(hpm_data_serde::chip::core::peripheral::Interrupt {
                    signal: signal.clone(),
                    interrupt: interrupt.name.clone(),
                });

                periph.interrupts = Some(periph_ints);
            }
        }
    }

    Ok(())
}
