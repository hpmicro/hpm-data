//! parse sysctl registers from sdk_code

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

// SDK name is 4 char name, like MCT0, CAN0, TMR0, etc.
// Here, we need to convert it to the peripheral name used in hpm-data.
fn match_peripheral_name(chip_name: &str, sdk_name: &str, periph_name: &str) -> bool {
    let mut pname = sdk_name
        .replace("TMR", "GPTMR")
        .replace("URT", "UART")
        .replace("OPA", "OPAMP")
        .replace("CRC0", "CRC")
        .replace("KMAN", "KEYM")
        .replace("SDP0", "SDP");

    if chip_name.starts_with("HPM53") || chip_name.starts_with("HPM68") {
        pname = pname.replace("CAN", "MCAN")
    }

    pname == periph_name
}

pub fn add_sysctl_from_sdk<P: AsRef<Path>>(
    data_dir: P,
    chip: &mut hpm_data_serde::Chip,
) -> anyhow::Result<()> {
    let sdk_path = std::env::var("HPM_SDK_BASE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| data_dir.as_ref().parent().unwrap().join("hpm_sdk"));

    let chip_name = &chip.name;

    // Defined in hpm_sysctl_drv.h
    // This is the relation between the resource and the group link number.
    // The conversion logic is `sysctl_enable_group_resource`.
    const SYSCTL_RESOURCE_LINKABLE_START: u32 = 256;

    let header_file = match chip_name {
        n if n.starts_with("HPM5301") => sdk_path.join("soc/HPM5301/hpm_sysctl_regs.h"),
        n if n.starts_with("HPM53") => sdk_path.join("soc/HPM5361/hpm_sysctl_regs.h"),
        n if n.starts_with("HPM62") => sdk_path.join("soc/HPM6280/hpm_sysctl_regs.h"),
        n if n.starts_with("HPM63") => sdk_path.join("soc/HPM6360/hpm_sysctl_regs.h"),
        n if n.starts_with("HPM67") || n.starts_with("HPM64") => {
            sdk_path.join("soc/HPM6750/hpm_sysctl_regs.h")
        }
        n if n.starts_with("HPM6830") => sdk_path.join("soc/HPM6830/hpm_sysctl_regs.h"),
        n if n.starts_with("HPM6850") => sdk_path.join("soc/HPM6850/hpm_sysctl_regs.h"),
        n if n.starts_with("HPM6880") => sdk_path.join("soc/HPM6880/hpm_sysctl_regs.h"),
        n if n.starts_with("HPM6E") => {
            eprintln!("HPM6E Series is not supported yet");
            return Ok(());
        }
        _ => anyhow::bail!("Unknown chip: {}", chip_name),
    };

    let content = std::fs::read_to_string(&header_file)?;

    // #define SYSCTL_RESOURCE_MCT0 (258UL)
    let resource_pattern =
        regex::Regex::new(r"#define\s+SYSCTL_RESOURCE_(\w+)\s+\((\d+)UL\)").expect("Invalid regex");
    let resources: HashMap<String, u32> = resource_pattern
        .captures_iter(&content)
        .map(|cap| {
            (
                cap.get(1).unwrap().as_str().to_string(),
                cap.get(2).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect();

    // println!("resources: {:#?}", resources);

    // #define SYSCTL_CLOCK_CLK_TOP_TMR2 (11UL)
    let clock_top_pattern =
        regex::Regex::new(r"#define\s+SYSCTL_CLOCK_CLK_TOP_(\w+)\s+\((\d+)UL\)")
            .expect("Invalid regex");
    let clocks: HashMap<String, u32> = clock_top_pattern
        .captures_iter(&content)
        .map(|cap| {
            (
                cap.get(1).unwrap().as_str().to_string(),
                cap.get(2).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect();

    // println!("clocks: {:#?}", clocks);

    // build Systick info
    for core in &mut chip.cores {
        core.resources = resources
            .iter()
            .map(|(name, idx)| hpm_data_serde::chip::core::Resource {
                name: name.clone(),
                index: *idx as _,
            })
            .collect();
        core.clocks = clocks
            .iter()
            .map(|(name, idx)| hpm_data_serde::chip::core::Clock {
                name: name.clone(),
                index: *idx as _,
            })
            .collect();

        for periph in &mut core.peripherals {
            let resource = resources
                .iter()
                .find(|(name, _)| match_peripheral_name(&chip_name, &name, &periph.name));

            let Some(resource_info) = resource else {
                continue;
            };
            let resource = *resource_info.1;

            let clock = clocks
                .iter()
                .find(|(name, _)| match_peripheral_name(&chip_name, &name, &periph.name))
                .map(|(_, no)| *no as usize);

            let sdk_clk_top_name = format!("CLK_TOP_{}", resource_info.0);

            let clock_top_resource = resources
                .iter()
                .find(|(name, _)| &**name == &sdk_clk_top_name)
                .map(|(_, no)| *no as usize);

            if resource < SYSCTL_RESOURCE_LINKABLE_START {
                continue; // skip non-linkable resources
            }

            let link_index = (resource - SYSCTL_RESOURCE_LINKABLE_START) / 32;
            let link_offset = (resource - SYSCTL_RESOURCE_LINKABLE_START) % 32;

            let sysclk = hpm_data_serde::chip::core::peripheral::Sysctl {
                group_link: link_index as _,
                group_bit_offset: link_offset as _,
                resource: resource as _,
                resource_clock_top: clock_top_resource,
                clock_node: clock,
            };

            periph.sysctl = Some(sysclk);
        }
    }

    Ok(())
}
