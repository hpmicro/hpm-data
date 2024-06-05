# hpm-data

The structured MCU DB of HPM MCUs. The home of hpm-metapac.

| Product No. | Core | SRAM(KB) | Audio Interface | Ethernet | CAN Bus | USB | Cryptographic | ADC | Packaging | Temperature Range | Extra Note |
|-----------------------------------------------|-------------------------------------|------|-------------------------------|---------|--------|------------------|-----------------------|----------------------|--------------------------------------|---------------------------|--------|
| HPM6750 | Dual-core 32-bit | 2088 | 4×I2S, 1×digital audio output | Gigabit | CAN FD | USB HS w/ PHY ×2 | AES128/256, SHA-1/256 | 3×12 bits, 1×16 bits | 14×14 289BGA 0.8P,10×10 196BGA 0.65P | -40~105                   |     |
| HPM64A0 | Single-core 32-bit | 2088 | 4×I2S, 1×digital audio output | Gigabit | CAN FD | USB HS w/ PHY ×2 | AES128/256, SHA-1/256 | 3×12 bits, 1×16 bits | 14×14 289BGA 0.8P,10×10 196BGA 0.65P | AEC-Q100 G2: -40℃~105℃ Ta | Automotive Grade High Performance MCU |
| HPM6730 | Dual-core 32-bit | 2088 | 4×I2S, 1×digital audio output | Gigabit | CAN    | USB HS w/ PHY ×2 | AES128/256, SHA-1/256 | 3×12 bits, 1×16 bits | 14×14 289BGA 0.8P,10×10 196BGA 0.65P |                           |     |
| HPM6450 | Single-core 32-bit | 2088 | 4×I2S, 1×digital audio output | Gigabit | CAN FD | USB HS w/ PHY ×2 | AES128/256, SHA-1/256 | 3×12 bits, 1×16 bits | 14×14 289BGA 0.8P,10×10 196BGA 0.65P |                           |     |
| HPM6430 | Single-core 32-bit | 2088 | 4×I2S, 1×digital audio output | Gigabit | CAN    | USB HS w/ PHY ×2 | AES128/256, SHA-1/256 | 3×12 bits, 1×16 bits | 14×14 289BGA 0.8P,10×10 196BGA 0.65P |                           |     |
| HPM6360 | Single-core 32-bit | 800 | 2×I2S, 1×digital audio output | 100M | CAN FD | USB HS w/ PHY ×1 | AES128/256, SHA-1/256 | 3×16 bits | 20×20 144eLQFP P0.5,7×7 116BGA P0.5 | -40∼125 °C Tj，-40∼105 °C Ta |     |
| HPM6350 | Single-core 32-bit | 800 | 2×I2S, 1×digital audio output | 100M | CAN    | USB HS w/ PHY ×1 | AES128/256, SHA-1/256 | 3×16 bits | 20×20 144eLQFP P0.5,7×7 116BGA P0.5 | -40∼125 °C Tj，-40∼105 °C Ta |     |
| HPM6340 | Single-core 32-bit | 800 | 2×I2S, 1×digital audio output |    | CAN FD |    | AES128/256, SHA-1/256 | 3×16 bits | 20×20 144eLQFP P0.5,7×7 116BGA P0.5 | -40∼125 °C Tj，-40∼105 °C Ta |     |
| HPM6320 | Single-core 32-bit | 800 | 2×I2S, 1×digital audio output | 100M |    | USB HS w/ PHY ×1 | AES128/256, SHA-1/256 | 1×16 bit  | 20×20 144eLQFP P0.5,7×7 116BGA P0.5 | -40∼125 °C Tj，-40∼105 °C Ta |     |
| HPM6364 | Single-core 32-bit | 800 | 2×I2S, 1×digital audio output | 100M | CAN FD | USB HS w/ PHY ×1 | AES128/256, SHA-1/256 | 3×16 bits | 20×20 144eLQFP P0.5,7×7 116BGA P0.5 | -40∼125 °C Tj，-40∼105 °C Ta |     |
| HPM6280 | Dual-core 32-bit | 800 |    |    | CAN FD | USB HS w/ PHY ×1 | AES128/256, SHA-1/256 | 3×16 bits | 20×20 144eLQFP P0.5,7×7 116BGA P0.5 | -40∼125 °C Tj，-40∼105 °C Ta |     |
| HPM6260 | Single-core 32-bit | 800 |    |    | CAN FD | USB HS w/ PHY ×1 | AES128/256, SHA-1/256 | 3×16 bits | 20×20 144eLQFP P0.5,7×7 116BGA P0.5 | -40∼125 °C Tj，-40∼105 °C Ta |     |
| HPM6240 | Single-core 32-bit | 800 |    |    | CAN FD | USB HS w/ PHY ×1 | AES128/256, SHA-1/256 | 3×16 bits | 20×20 144eLQFP P0.5,7×7 116BGA P0.5 | -40∼125 °C Tj，-40∼105 °C Ta |     |
| HPM6284 | Dual-core 32-bit | 800 |    |    | CAN FD | USB HS w/ PHY ×1 | AES128/256, SHA-1/256 | 3×16 bits | 20×20 144eLQFP P0.5,7×7 116BGA P0.5 | -40∼125 °C Tj，-40∼105 °C Ta |     |
| HPM6264 | Single-core 32-bit | 800 |    |    | CAN FD | USB HS w/ PHY ×1 | AES128/256, SHA-1/256 | 3×16 bits | 20×20 144eLQFP P0.5,7×7 116BGA P0.5 | -40∼125 °C Tj，-40∼105 °C Ta |     |
| HPM6220 | Single-core 32-bit | 800 |    |    |    | USB HS w/ PHY ×1 | AES128/256, SHA-1/256 | 1×16 bit  |                                     | -40∼125 °C Tj，-40∼105 °C Ta |     |
| HPM5361 | Single-core 32-bit | 288 |    |    | CAN FD | USB HS w/ PHY ×1 | AES128/256, SHA-1/256 | 2*16bit | 14×14 100LQFP P0.5，10×10 64LQFP P0.5，6×6 48QFN P0.4 | −40 ∼ 125◦C Tj    −40 ∼ 105◦C Ta |     |
| HPM5331 | Single-core 32-bit | 288 |    |    |    | USB HS w/ PHY ×1 | AES128/256, SHA-1/256 | 2*16bit | 14×14 100LQFP P0.5，10×10 64LQFP P0.5，6×6 48QFN P0.4 | −40 ∼ 125◦C Tj    −40 ∼ 105◦C Ta |     |
| HPM5321 | Single-core 32-bit | 288 |    |    | CAN FD | USB HS w/ PHY ×1 | AES128/256, SHA-1/256 | 2*16bit | 14×14 100LQFP P0.5 10×10 64LQFP P0.5 6×6 48QFN P0.4 | −40 ∼ 125◦C Tj    −40 ∼ 105◦C Ta |     |
| HPM6880 | Single-core 32-bit | 1064 | 4 | Gigabit | CAN FD | USB HS w/ PHY ×1 | AES128/256, SHA-1/256 | 1×16 bit | 17×17 417BGA P0.8 | −40 ∼ 105 | 2.5D OpenVG GPU |
| HPM6850 | Single-core 32-bit | 1064 | 4 | Gigabit | CAN FD | USB HS w/ PHY ×1 | AES128/256, SHA-1/256 | 1×16 bit | 17×17 417BGA P0.8 | −40 ∼ 105 | 2.5D OpenVG GPU |
| HPM6830 | Single-core 32-bit | 1064 |    | Gigabit | CAN FD | USB HS w/ PHY ×1 | AES128/256, SHA-1/256 | 1×16 bit | 17×17 417BGA P0.8 | −40 ∼ 105 |     |


## Data Source

- <https://www.hpmicro.com/>
- <https://github.com/hpmicro/hpm_pinmux_tool>
- <https://github.com/hpmicro/hpm_sdk>
