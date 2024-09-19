# hpm-data & hpm-metapac

[![CI Status][badge-actions]][actions-build]
[![Crates.io][badge-crates-io]][crates-io]
[![Docs.rs][badge-docs-rs]][docs-rs]

[badge-actions]: https://img.shields.io/github/actions/workflow/status/hpmicro/hpm-data/build.yml?style=for-the-badge&label=CI&20Tests
[actions-build]: https://github.com/hpmicro/hpm-data/actions/workflows/build.yml
[badge-crates-io]: https://img.shields.io/crates/v/hpm-metapac.svg?style=for-the-badge
[crates-io]: https://crates.io/crates/hpm-metapac
[badge-docs-rs]: https://img.shields.io/docsrs/hpm-metapac?style=for-the-badge
[docs-rs]: https://docs.rs/hpm-metapac

The structured MCU DB of HPM MCUs. The home of [hpm-metapac][docs-rs].

All PRs and Issues are handled in [hpmicro/hpm-data](https://github.com/hpmicro/hpm-data).

`hpm-metapac` is generated from this repo. For each commit(or push) of hpm-data, it's pushed to <https://github.com/hpmicro-rs/hpm-metapac>,
with a tag of `hpm-data-<commit-hash>`.

## hpm-metapac

- The `hpm-metapac` crate has a `metadata` feature, when enabled, it will provide the basic metadata of the currrent MCU
- Patch vectored interrupt mode, add `CORE_LOCAL` for Non-External Interrupts
- To best fit for HPM RISC-V's clustered register desigin, the following is added:
  - All clocks, for `SYSCTL.CLOCK`, under `hpm_metapac::clocks::`
  - All SYSCTL resources, under `hpm_metapac::resources::`
  - All GPIOs and it's PADs, for `IOC`, under `hpm_metapac::pins::`
  - All IOMUX settings (`FUNC_CTL`), under `hpm_metapac::iomux::`
  - All TRGM const definitions, under `hpm_metapac::trgmmux::`
- The version on crates.io is not updated frequently, please use the git repo directly

### Usage

```toml
[dependencies]
hpm-metapac = { version = "0.0.4", git = "https://github.com/hpmicro-rs/hpm-metapac.git", tag = "hpm-data-d8c87c6a676818ff6abd3b7ae54a1a7612cc8534", features = ["hpm5361"] }

# If you want to use the metadata feature in build.rs
[build-dependencies]
hpm-metapac = { version = "0.0.4", git = "https://github.com/hpmicro-rs/hpm-metapac.git", tag = "hpm-data-d8c87c6a676818ff6abd3b7ae54a1a7612cc8534", default-features = false, features = [
    "metadata",
    "hpm5361",
] }
```

A simple example to configure pin PA25 for PWM1_P1:

```rust
use hpm_metapac as pac;
use pac::{iomux, pins};

pac::IOC
    .pad(pins::PA25)
    .func_ctl()
    .modify(|w| w.set_alt_select(iomux::IOC_PA25_FUNC_CTL_PWM1_P_1));
```

### Development

To get a local build of `hpm-metapac`, you can use the following commands:

```sh
./d download-all
./d gen
```

Now you have a local build of `hpm-metapac` in the `build/hpm-metapac` directory.

```toml
[dependencies]
hpm-metapac = { path = "path/to/hpm-data/build/hpm-metapac", features = ["hpm5361"] }
```

## Support Status

- All peripherals are supported
- All MCU families are supported
- Peripherals that have an HAL driver or raw PAC demo in [hpm-hal](https://github.com/hpmicro/hpm-hal) are reviewed and tested

### MCU Family

(in order of release date)

- HPM6700/HPM6400 - High performance
- HPM6300 - General purpose
- HPM6200 - High performance, real-time, mixed signal
- HPM5300 - General purpose, motion control
- HPM6800 - Display dirver, user interface
- HPM6E00 - EtherCAT

## Data Source

- <https://www.hpmicro.com/>
- <https://github.com/hpmicro/hpm_pinmux_tool>
- <https://github.com/hpmicro/hpm_sdk>
- <https://tools.hpmicro.com/pinmux>

## Project History

As of 2024-09-19, this project is transferred from [andelf](https://github.com/andelf) to [hpmicro](https://github.com/hpmicro).
