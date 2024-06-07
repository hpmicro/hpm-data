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
