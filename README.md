# hpm-data & hpm-metapac

[![CI Status][badge-actions]][actions-build]
[![Crates.io][badge-crates-io]][crates-io]
[![Docs.rs][badge-docs-rs]][docs-rs]

[badge-actions]: https://img.shields.io/github/actions/workflow/status/andelf/hpm-data/build.yml?style=for-the-badge&label=CI&20Tests
[actions-build]: https://github.com/andelf/hpm-data/actions/workflows/build.yml
[badge-crates-io]: https://img.shields.io/crates/v/hpm-metapac.svg?style=for-the-badge
[crates-io]: https://crates.io/crates/hpm-metapac
[badge-docs-rs]: https://img.shields.io/docsrs/hpm-metapac?style=for-the-badge
[docs-rs]: https://docs.rs/hpm-metapac

The structured MCU DB of HPM MCUs. The home of [hpm-metapac][docs-rs].

All PRs and Issues are handled in [andelf/hpm-data](https://github.com/andelf/hpm-data).

## MCU Family

- HPM5300
- HPM6200
- HPM6300
- HPM6700/HPM6400
- HPM6800
- HPM6E00 (announced)

### Support status

- [x] HPM5300
- [x] HPM6700/HPM6400
- [x] HPM6300
- [x] HPM6800
- [ ] HPM6200

### Metadata patch

The `hpm-metapac` crate has a `metadata` feature, when enabled, it will provide the basic metadata of the currrent MCU:

- Core name, basic info
- All resources, for `SYSCTL`
- All clocks, for `SYSCTL.CLOCK`
- All GPIOs and it's PADs, for `IOC`
- Patch vectored interrupt mode, add `CORE_LOCAL` for Non-External Interrupts

## Data Source

- <https://www.hpmicro.com/>
- <https://github.com/hpmicro/hpm_pinmux_tool>
- <https://github.com/hpmicro/hpm_sdk>
- <https://tools.hpmicro.com/pinmux>
