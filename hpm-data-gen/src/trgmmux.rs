//! parse trgm mux defines from sdk_code

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

pub fn add_trgmmux_from_sdk<P: AsRef<Path>>(
    data_dir: P,
    chip: &mut hpm_data_serde::Chip,
) -> anyhow::Result<()> {
    let sdk_path = std::env::var("HPM_SDK_BASE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| data_dir.as_ref().parent().unwrap().join("hpm_sdk"));

    let chip_name = &chip.name;

    if chip.cores[0]
        .peripherals
        .iter()
        .find(|p| p.name.starts_with("TRGM"))
        .is_none()
    {
        return Ok(()); // No TRGM peripheral
    }

    let header_file = match chip_name {
        n if n.starts_with("HPM53") => sdk_path.join("soc/HPM5300/HPM5361/hpm_trgmmux_src.h"),
        n if n.starts_with("HPM62") => sdk_path.join("soc/HPM6200/HPM6280/hpm_trgmmux_src.h"),
        n if n.starts_with("HPM63") => sdk_path.join("soc/HPM6300/HPM6360/hpm_trgmmux_src.h"),
        n if n.starts_with("HPM67") || n.starts_with("HPM64") => {
            sdk_path.join("soc/HPM6700/HPM6750/hpm_trgmmux_src.h")
        }
        n if n.starts_with("HPM6E") => sdk_path.join("soc/HPM6E00/HPM6E80/hpm_trgmmux_src.h"),
        _ => anyhow::bail!("Unknown chip: {}", chip_name),
    };

    let content = std::fs::read_to_string(&header_file)
        .expect(format!("Failed to read file: {:?}", &header_file).as_str());

    // #define HPM_TRGM0_FILTER_SRC_PWM0_IN0                      (0x0UL)
    let resource_pattern =
        regex::Regex::new(r"#define\s+HPM_(TRGM\d_\w+)\s+\(0x([0-9A-Fa-f]+)UL\)")
            .expect("Invalid regex");
    let defines: HashMap<String, u32> = resource_pattern
        .captures_iter(&content)
        .map(|cap| {
            (
                cap.get(1).unwrap().as_str().to_string(),
                u32::from_str_radix(cap.get(2).unwrap().as_str(), 16).unwrap(),
            )
        })
        .collect();

    println!("    Chip: {} TRGM consts: {}", chip_name, defines.len());

    for core in &mut chip.cores {
        core.trgmmuxes = defines
            .iter()
            .map(|(name, val)| hpm_data_serde::chip::core::TrgmMux {
                name: name.clone(),
                value: *val as _,
            })
            .collect();
        core.trgmmuxes
            .sort_by(|a, b| trgm_cmp_key(a).cmp(&trgm_cmp_key(b)));
    }

    Ok(())
}

// TRGM0_FILTER_SRC_PWM0_IN0 => ("TRGM0_FILTER", 0)
fn trgm_cmp_key(val: &hpm_data_serde::chip::core::TrgmMux) -> (&str, u32) {
    let i: usize = val.name.find('_').unwrap();
    val.name[i + 1..]
        .find('_')
        .map_or((&val.name, val.value as u32), |j| {
            (&val.name[..i + j + 1], val.value as u32)
        })
}
