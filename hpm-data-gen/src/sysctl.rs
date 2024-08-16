//! parse sysctl registers from sdk_code

use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    sync::LazyLock,
};

// Defined in hpm_sysctl_drv.h
// This is the relation between the resource and the group link number.
// The conversion logic is `sysctl_enable_group_resource`.
const SYSCTL_RESOURCE_LINKABLE_START: u32 = 256;

static HPM_SDK_BASE: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::var("HPM_SDK_BASE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| std::env::current_dir().unwrap().join("./hpm_sdk"))
});

static HPM5300_SYSCTL: LazyLock<SysctlInfo> = LazyLock::new(|| {
    load_sysctl_info_from_header(
        "HPM5300",
        HPM_SDK_BASE.join("soc/HPM5300/ip/hpm_sysctl_regs.h"),
    )
    .expect("Failed to load HPM5300 sysctl info")
});
static HPM6200_SYSCTL: LazyLock<SysctlInfo> = LazyLock::new(|| {
    load_sysctl_info_from_header(
        "HPM6200",
        HPM_SDK_BASE.join("soc/HPM6200/ip/hpm_sysctl_regs.h"),
    )
    .expect("Failed to load HPM6200 sysctl info")
});
static HPM6300_SYSCTL: LazyLock<SysctlInfo> = LazyLock::new(|| {
    load_sysctl_info_from_header(
        "HPM6300",
        HPM_SDK_BASE.join("soc/HPM6300/ip/hpm_sysctl_regs.h"),
    )
    .expect("Failed to load HPM6300 sysctl info")
});
static HPM6700_SYSCTL: LazyLock<SysctlInfo> = LazyLock::new(|| {
    load_sysctl_info_from_header(
        "HPM6700",
        HPM_SDK_BASE.join("soc/HPM6700/ip/hpm_sysctl_regs.h"),
    )
    .expect("Failed to load HPM6700 sysctl info")
});
static HPM6800_SYSCTL: LazyLock<SysctlInfo> = LazyLock::new(|| {
    load_sysctl_info_from_header(
        "HPM6800",
        HPM_SDK_BASE.join("soc/HPM6800/ip/hpm_sysctl_regs.h"),
    )
    .expect("Failed to load HPM6800 sysctl info")
});
static HPM6E00_SYSCTL: LazyLock<SysctlInfo> = LazyLock::new(|| {
    load_sysctl_info_from_header(
        "HPM6E00",
        HPM_SDK_BASE.join("soc/HPM6E00/ip/hpm_sysctl_regs.h"),
    )
    .expect("Failed to load HPM6E00 sysctl info")
});

#[derive(Debug, Clone)]
pub struct SysctlInfo {
    pub chip_family: String,
    // for resources register
    pub resources: HashMap<String, u32>,
    // For clocks register
    pub clocks: HashMap<String, u32>,
}

impl SysctlInfo {
    // SDK name is 4 char name, like MCT0, CAN0, TMR0, etc.
    // Here, we need to convert it to the peripheral name used in hpm-data.
    fn peripheral_name_to_sdk_name(&self, name: &str) -> String {
        // convert peripheral name to SDK name
        let mut trans: HashMap<_, _> = [
            ("MCHTMR", "MCT"), // how to handle dual-core?
            ("GPTMR", "TMR"),
            ("OPAMP", "OPA"),
            ("UART", "URT"),
            ("KEYM", "KMAN"),
            ("CRC", "CRC0"),
            ("FFA", "FFA0"),
            ("MBX0A", "MBX0"),
            ("MBX0B", "MBX0"),
            ("MBX1A", "MBX1"),
            ("MBX1B", "MBX1"),
            ("SDP", "SDP0"),
            ("NTMR", "NTM"),
            ("SDXC", "SDC"),
            ("PPI", "PPI0"),
            ("SEI", "SEI0"),
            ("RNG", "RNG0"),
            ("TSW", "TSW0"),
            ("PLB", "PLB0"),
        ]
        .into_iter()
        .collect();
        if self.chip_family.starts_with("HPM53")
            || self.chip_family.starts_with("HPM62")
            || self.chip_family.starts_with("HPM68")
            || self.chip_family.starts_with("HPM6E")
        {
            trans.insert("MCAN", "CAN");
        }
        if self.chip_family.starts_with("HPM63") {
            trans.insert("HDMA", "DMA0");
            trans.insert("XDMA", "DMA1");
        }

        let mut pname = name.to_string();
        for (k, v) in &trans {
            if pname.starts_with(*k) {
                pname = pname.replace(k, v);
                break;
            }
        }

        pname
    }
    fn get_resource(&self, name: &str) -> Option<u32> {
        // applies to HPM6700 and HPM6400
        if self.chip_family == "HPM6700" {
            return self.resources.get(name).copied();
        }

        let pname = self.peripheral_name_to_sdk_name(name);

        self.resources.get(&pname).copied()

        /*
            let mot_pname = pname
                .replace("PWM", "MOT")
                .replace("HALL", "MOT")
                .replace("QEI", "MOT")
                .replace("TRGM", "MOT")
                .replace("MOT", "PWM");
            self.resources.get(&mot_pname).copied()
        })
        */
    }

    fn get_clock(&self, name: &str) -> Option<u32> {
        // applies to HPM6700 and HPM6400
        if self.chip_family == "HPM6700" {
            return self.clocks.get(name).copied();
        }

        let pname = self.peripheral_name_to_sdk_name(name);

        self.clocks.get(&pname).copied()
    }

    fn get_clock_top_resource(&self, name: &str) -> Option<u32> {
        let pname = self.peripheral_name_to_sdk_name(name);
        let sdk_clk_top_name = format!("CLK_TOP_{}", pname);
        self.resources.get(&sdk_clk_top_name).copied()
    }
}

fn load_sysctl_info_from_header<P: AsRef<Path>>(
    chip_family: &str,
    header_path: P,
) -> anyhow::Result<SysctlInfo> {
    let content = std::fs::read_to_string(&header_path)
        .expect(format!("Failed to read file: {:?}", header_path.as_ref().display()).as_str());

    // #define SYSCTL_RESOURCE_MCT0 (258UL)
    // => MCT0: 258
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

    // #define SYSCTL_CLOCK_CLK_TOP_TMR2 (11UL)
    // => TMR2: 11
    let clock_top_pattern =
        regex::Regex::new(r"#define\s+SYSCTL_CLOCK_CLK_TOP_(\w+)\s+\((\d+)UL\)")
            .expect("Invalid regex");
    let mut clocks: HashMap<String, u32> = clock_top_pattern
        .captures_iter(&content)
        .map(|cap| {
            (
                cap.get(1).unwrap().as_str().to_string(),
                cap.get(2).unwrap().as_str().parse().unwrap(),
            )
        })
        .collect();

    // Fix: `#define SYSCTL_CLOCK_CLK_TOP_MCHTMR (3UL)`
    if chip_family.starts_with("HPM67") || chip_family.starts_with("HPM64") {
        if let Some(i) = clocks.remove("MCHTMR") {
            clocks.insert("MCHTMR1".to_string(), i);
        }
    }
    // println!("resources: {:#?}", resources);
    // println!("clocks: {:#?}", clocks);

    println!(
        "    Load SYSCTL for {}: {} resources, {} clocks",
        chip_family,
        resources.len(),
        clocks.len()
    );

    Ok(SysctlInfo {
        chip_family: chip_family.to_string(),
        resources,
        clocks,
    })
}

pub fn add_sysctl_from_sdk<P: AsRef<Path>>(
    _data_dir: P,
    chip: &mut hpm_data_serde::Chip,
) -> anyhow::Result<()> {
    let chip_name = &chip.name;

    let info = match chip_name {
        n if n.starts_with("HPM53") => &HPM5300_SYSCTL,
        n if n.starts_with("HPM62") => &HPM6200_SYSCTL,
        n if n.starts_with("HPM63") => &HPM6300_SYSCTL,
        n if n.starts_with("HPM67") || n.starts_with("HPM64") => &HPM6700_SYSCTL,
        n if n.starts_with("HPM68") => &HPM6800_SYSCTL,
        n if n.starts_with("HPM6E") => &HPM6E00_SYSCTL,
        _ => anyhow::bail!("Unknown chip: {}", chip_name),
    };

    // build Systick info
    // only one core variant
    let core = &mut chip.cores[0];

    core.resources = info
        .resources
        .iter()
        .map(|(name, idx)| hpm_data_serde::chip::core::Resource {
            name: name.clone(),
            index: *idx as _,
        })
        .collect();
    core.resources.sort_by_key(|r| r.index);
    core.clocks = info
        .clocks
        .iter()
        .map(|(name, idx)| hpm_data_serde::chip::core::Clock {
            name: name.clone(),
            index: *idx as _,
        })
        .collect();
    core.clocks.sort_by_key(|r| r.index);

    // match clocks and resources to peripherals
    for periph in &mut core.peripherals {
        if periph.sysctl.is_some() {
            continue; // already set
        }

        let res = info.get_resource(&periph.name);

        let Some(res_no) = res else {
            continue; // skip peripherals without sysctl
        };

        let clock = info.get_clock(&periph.name);

        let clock_top_res_no = info.get_clock_top_resource(&periph.name);

        if res_no < SYSCTL_RESOURCE_LINKABLE_START {
            continue; // skip non-linkable resources
        }

        let link_index = (res_no - SYSCTL_RESOURCE_LINKABLE_START) / 32;
        let link_offset = (res_no - SYSCTL_RESOURCE_LINKABLE_START) % 32;

        let sysclk = hpm_data_serde::chip::core::peripheral::Sysctl {
            group_link: link_index as _,
            group_bit_offset: link_offset as _,
            resource: res_no as _,
            resource_clock_top: clock_top_res_no.map(|c| c as usize),
            clock_node: clock.map(|c| c as usize),
        };

        periph.sysctl = Some(sysclk);
    }

    Ok(())
}
