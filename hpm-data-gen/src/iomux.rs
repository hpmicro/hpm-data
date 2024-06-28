//! parse iomux definitions from sdk_code

use std::path::{Path, PathBuf};

pub fn add_iomux_from_sdk<P: AsRef<Path>>(
    data_dir: P,
    chip: &mut hpm_data_serde::Chip,
) -> anyhow::Result<()> {
    let sdk_path = std::env::var("HPM_SDK_BASE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| data_dir.as_ref().parent().unwrap().join("hpm_sdk"));

    let chip_name = &chip.name;

    let mut all_iomux: Vec<hpm_data_serde::chip::core::IoMux> = vec![];

    let chip_inc_path = match chip_name {
        n if n.starts_with("HPM5301") => sdk_path.join("soc/HPM5300/HPM5301/"),
        n if n.starts_with("HPM53") => sdk_path.join("soc/HPM5300/HPM5361/"),
        n if n.starts_with("HPM62") => sdk_path.join("soc/HPM6200/HPM6280/"),
        n if n.starts_with("HPM63") => sdk_path.join("soc/HPM6300/HPM6360/"),
        n if n.starts_with("HPM67") || n.starts_with("HPM64") => {
            sdk_path.join("soc/HPM6700/HPM6750/")
        }
        n if n.starts_with("HPM68") => sdk_path.join("soc/HPM6800/HPM6880/"),
        n if n.starts_with("HPM6E") => sdk_path.join("soc/HPM6E00/HPM6E80/"),
        _ => anyhow::bail!("Unknown chip: {}", chip_name),
    };

    let iomux_path = chip_inc_path.join("hpm_iomux.h");

    let content = std::fs::read_to_string(&iomux_path)
        .expect(format!("Failed to read file: {:?}", &iomux_path).as_str());

    // #define IOC_PA16_FUNC_CTL_MCAN4_TXD            IOC_PAD_FUNC_CTL_ALT_SELECT_SET(7)
    let iomux_pattern = regex::Regex::new(
        r"#define\s+(IOC_(\w+)_FUNC_CTL_(\w+))\s+IOC_PAD_FUNC_CTL_ALT_SELECT_SET\((\d+)\)",
    )
    .expect("Invalid regex");

    for mux in iomux_pattern.captures_iter(&content).map(|cap| {
        (
            cap.get(1).unwrap().as_str().to_string(),
            cap.get(4).unwrap().as_str().parse().unwrap(),
        )
    }) {
        all_iomux.push(hpm_data_serde::chip::core::IoMux {
            name: mux.0,
            value: mux.1,
        });
    }

    // PMIC domain

    let pmic_iomux = chip_inc_path.join("hpm_pmic_iomux.h");

    let content = std::fs::read_to_string(&pmic_iomux)
        .expect(format!("Failed to read file: {:?}", &pmic_iomux).as_str());

    // #define PIOC_PY01_FUNC_CTL_PGPIO_Y_01          IOC_PAD_FUNC_CTL_ALT_SELECT_SET(0)
    let pmic_iomux_pattern = regex::Regex::new(
        r"#define\s+(PIOC_(\w+)_FUNC_CTL_(\w+))\s+IOC_PAD_FUNC_CTL_ALT_SELECT_SET\((\d+)\)",
    )
    .expect("Invalid regex");

    for mux in pmic_iomux_pattern.captures_iter(&content).map(|cap| {
        (
            cap.get(1).unwrap().as_str().to_string(),
            cap.get(4).unwrap().as_str().parse().unwrap(),
        )
    }) {
        all_iomux.push(hpm_data_serde::chip::core::IoMux {
            name: mux.0,
            value: mux.1,
        });
    }

    // BATT domain

    let batt_iomux = chip_inc_path.join("hpm_batt_iomux.h");

    if batt_iomux.exists() {
        let content = std::fs::read_to_string(&batt_iomux)
            .expect(format!("Failed to read file: {:?}", &batt_iomux).as_str());

        // #define BIOC_PZ00_FUNC_CTL_BGPIO_Z_00          IOC_PAD_FUNC_CTL_ALT_SELECT_SET(0)
        let batt_iomux_pattern = regex::Regex::new(
            r"#define\s+(BIOC_(\w+)_FUNC_CTL_(\w+))\s+IOC_PAD_FUNC_CTL_ALT_SELECT_SET\((\d+)\)",
        )
        .expect("Invalid regex");

        for mux in batt_iomux_pattern
            .captures_iter(&content)
            .map(|cap| {
                (
                    cap.get(1).unwrap().as_str().to_string(),
                    cap.get(4).unwrap().as_str().parse().unwrap(),
                )
            })
            .filter(|(name, _)| !name.contains("_GPIO"))
        {
            // filter out old style GPIO

            all_iomux.push(hpm_data_serde::chip::core::IoMux {
                name: mux.0,
                value: mux.1,
            });
        }
    }

    all_iomux.sort_by_key(|p| p.name.to_string());

    all_iomux.dedup();

    println!("    {} load iomux {:#?}", chip_name, all_iomux.len());

    chip.cores[0].iomuxes = all_iomux;

    Ok(())
}
