//! handle pinmux matching

use std::path::Path;

pub fn handle_pinmux<P: AsRef<Path>>(
    path: P,
    chip: &mut hpm_data_serde::Chip,
) -> anyhow::Result<()> {
    Ok(())
}
