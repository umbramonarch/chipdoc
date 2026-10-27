# Datasheet - STM32G030x6/x8 - Arm® Cortex®-M0+ 32-bit MCU, up to 64 KB Flash, 8 KB RAM, 2x USART, timers, ADC, comm. I/Fs, 2.0-3.6 V

> **Agent Navigation Guide**:
> - Prefer 1-turn CLI queries: `chipdoc reg <name|hex>`, `chipdoc table <query> --show`, `chipdoc xref <symbol>`, `chipdoc page <N>`.
> - For fast 1-line-per-bitfield register lookups, grep `index/registers.tsv`.
> - For cross-chapter hardware relationships (pins, interrupts, memory map, registers), grep `index/xref.tsv`.
> - For electrical specification or summary tables, grep `index/tables.tsv`.
> - For chapter/peripheral details, grep `sections/` or open the corresponding file below.

## Document Summary

- **Source File**: `stm32g030k6_datasheet.pdf`
- **Author**: STMICROELECTRONICS
- **Pages Converted**: 100 / 100
- **Section Files**: 10 (`sections/*.md`)
- **Tables Extracted**: 75 (20 stitched across pages) -> `index/tables.tsv`
- **Registers / Bitfields Indexed**: 0 registers / 0 bitfields -> `index/registers.tsv`
- **Figures Detected**: 34 (34 rendered to `assets/figures/`) -> `index/figures.tsv`
- **Cross-References Indexed**: 1033 symbols -> `index/xref.tsv`
- **Sanity Score**: 100/100 (~44477 tokens)

## Section Shards (`sections/*.md`)

| ID | Section Title | File | Pages | Lines | Tables | Registers | Figures |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: |
| `00` | Frontmatter and Overview | [sections/00-frontmatter-and-overview.md](sections/00-frontmatter-and-overview.md) | 1-7 | 152 | 2 | 0 | 0 |
| `01` | 1 Introduction | [sections/01-1-introduction.md](sections/01-1-introduction.md) | 8-8 | 21 | 0 | 0 | 0 |
| `02` | 2 Description | [sections/02-2-description.md](sections/02-2-description.md) | 9-11 | 58 | 1 | 0 | 1 |
| `03` | 3 Functional overview | [sections/03-3-functional-overview.md](sections/03-3-functional-overview.md) | 12-28 | 551 | 8 | 0 | 1 |
| `04` | 4 Pinouts, pin description and alternate functions | [sections/04-4-pinouts-pin-description-and-alternate-functions.md](sections/04-4-pinouts-pin-description-and-alternate-functions.md) | 29-37 | 120 | 4 | 0 | 4 |
| `05` | 5 Electrical characteristics | [sections/05-5-electrical-characteristics.md](sections/05-5-electrical-characteristics.md) | 38-82 | 1270 | 53 | 0 | 20 |
| `06` | 6 Package information | [sections/06-6-package-information.md](sections/06-6-package-information.md) | 83-95 | 232 | 5 | 0 | 8 |
| `07` | 7 Ordering information | [sections/07-7-ordering-information.md](sections/07-7-ordering-information.md) | 96-96 | 31 | 0 | 0 | 0 |
| `08` | 8 Important security notice | [sections/08-8-important-security-notice.md](sections/08-8-important-security-notice.md) | 97-97 | 23 | 0 | 0 | 0 |
| `09` | 9 Revision history | [sections/09-9-revision-history.md](sections/09-9-revision-history.md) | 98-100 | 52 | 2 | 0 | 0 |

## Detailed Outline (Table of Contents)

- **1 Introduction** — p. 8 (`sections/01-1-introduction.md`)
- **2 Description** — p. 9 (`sections/02-2-description.md`)
- **3 Functional overview** — p. 12 (`sections/03-3-functional-overview.md`)
  - **3.1 Arm® Cortex®-M0+ core with MPU** — p. 12 (`sections/03-3-functional-overview.md`)
  - **3.2 Memory protection unit** — p. 12 (`sections/03-3-functional-overview.md`)
  - **3.3 Embedded flash memory** — p. 12 (`sections/03-3-functional-overview.md`)
  - **3.4 Embedded SRAM** — p. 13 (`sections/03-3-functional-overview.md`)
  - **3.5 Boot modes** — p. 14 (`sections/03-3-functional-overview.md`)
  - **3.6 Cyclic redundancy check calculation unit (CRC)** — p. 14 (`sections/03-3-functional-overview.md`)
  - **3.7 Power supply management** — p. 14 (`sections/03-3-functional-overview.md`)
    - **3.7.1 Power supply schemes** — p. 14 (`sections/03-3-functional-overview.md`)
    - **3.7.2 Power supply supervisor** — p. 15 (`sections/03-3-functional-overview.md`)
    - **3.7.3 Voltage regulator** — p. 15 (`sections/03-3-functional-overview.md`)
    - **3.7.4 Low-power modes** — p. 16 (`sections/03-3-functional-overview.md`)
    - **3.7.5 Reset mode** — p. 17 (`sections/03-3-functional-overview.md`)
    - **3.7.6 VBAT operation** — p. 17 (`sections/03-3-functional-overview.md`)
  - **3.8 Interconnect of peripherals** — p. 17 (`sections/03-3-functional-overview.md`)
  - **3.9 Clocks and startup** — p. 18 (`sections/03-3-functional-overview.md`)
  - **3.10 General-purpose inputs/outputs (GPIOs)** — p. 19 (`sections/03-3-functional-overview.md`)
  - **3.11 Direct memory access controller (DMA)** — p. 19 (`sections/03-3-functional-overview.md`)
  - **3.12 DMA request multiplexer (DMAMUX)** — p. 20 (`sections/03-3-functional-overview.md`)
  - **3.13 Interrupts and events** — p. 20 (`sections/03-3-functional-overview.md`)
    - **3.13.1 Nested vectored interrupt controller (NVIC)** — p. 21 (`sections/03-3-functional-overview.md`)
    - **3.13.2 Extended interrupt/event controller (EXTI)** — p. 21 (`sections/03-3-functional-overview.md`)
  - **3.14 Analog-to-digital converter (ADC)** — p. 21 (`sections/03-3-functional-overview.md`)
    - **3.14.1 Temperature sensor** — p. 22 (`sections/03-3-functional-overview.md`)
    - **3.14.2 Internal voltage reference (VREFINT)** — p. 22 (`sections/03-3-functional-overview.md`)
    - **3.14.3 VBAT battery voltage monitoring** — p. 23 (`sections/03-3-functional-overview.md`)
  - **3.15 Timers and watchdogs** — p. 23 (`sections/03-3-functional-overview.md`)
    - **3.15.1 Advanced-control timer (TIM1)** — p. 23 (`sections/03-3-functional-overview.md`)
    - **3.15.2 General-purpose timers (TIM3, 14, 16, 17)** — p. 24 (`sections/03-3-functional-overview.md`)
    - **3.15.3 Independent watchdog (IWDG)** — p. 24 (`sections/03-3-functional-overview.md`)
    - **3.15.4 System window watchdog (WWDG)** — p. 24 (`sections/03-3-functional-overview.md`)
    - **3.15.5 SysTick timer** — p. 24 (`sections/03-3-functional-overview.md`)
  - **3.16 Real-time clock (RTC), tamper (TAMP) and backup registers** — p. 25 (`sections/03-3-functional-overview.md`)
  - **3.17 Inter-integrated circuit interface (I2C)** — p. 26 (`sections/03-3-functional-overview.md`)
  - **3.18 Universal synchronous/asynchronous receiver transmitter (USART)** — p. 27 (`sections/03-3-functional-overview.md`)
  - **3.19 Serial peripheral interface (SPI)** — p. 27 (`sections/03-3-functional-overview.md`)
  - **3.20 Development support** — p. 28 (`sections/03-3-functional-overview.md`)
    - **3.20.1 Serial wire debug port (SW-DP)** — p. 28 (`sections/03-3-functional-overview.md`)
- **4 Pinouts, pin description and alternate functions** — p. 29 (`sections/04-4-pinouts-pin-description-and-alternate-functions.md`)
- **5 Electrical characteristics** — p. 38 (`sections/05-5-electrical-characteristics.md`)
  - **5.1 Parameter conditions** — p. 38 (`sections/05-5-electrical-characteristics.md`)
    - **5.1.1 Minimum and maximum values** — p. 38 (`sections/05-5-electrical-characteristics.md`)
    - **5.1.2 Typical values** — p. 38 (`sections/05-5-electrical-characteristics.md`)
    - **5.1.3 Typical curves** — p. 38 (`sections/05-5-electrical-characteristics.md`)
    - **5.1.4 Loading capacitor** — p. 38 (`sections/05-5-electrical-characteristics.md`)
    - **5.1.5 Pin input voltage** — p. 38 (`sections/05-5-electrical-characteristics.md`)
    - **5.1.6 Power supply scheme** — p. 39 (`sections/05-5-electrical-characteristics.md`)
    - **5.1.7 Current consumption measurement** — p. 40 (`sections/05-5-electrical-characteristics.md`)
  - **5.2 Absolute maximum ratings** — p. 40 (`sections/05-5-electrical-characteristics.md`)
  - **5.3 Operating conditions** — p. 41 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.1 General operating conditions** — p. 41 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.2 Operating conditions at power-up / power-down** — p. 42 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.3 Embedded reset and power control block characteristics** — p. 42 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.4 Embedded voltage reference** — p. 42 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.5 Supply current characteristics** — p. 43 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.6 Wakeup time from low-power modes and voltage scaling transition times** — p. 49 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.7 External clock source characteristics** — p. 51 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.8 Internal clock source characteristics** — p. 55 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.9 PLL characteristics** — p. 56 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.10 Flash memory characteristics** — p. 57 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.11 EMC characteristics** — p. 58 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.12 Electrical sensitivity characteristics** — p. 59 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.13 I/O current injection characteristics** — p. 60 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.14 I/O port characteristics** — p. 61 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.15 NRST input characteristics** — p. 66 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.16 Extended interrupt and event controller input (EXTI) characteristics** — p. 67 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.17 Analog switch booster** — p. 67 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.18 Analog-to-digital converter characteristics** — p. 68 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.19 Temperature sensor characteristics** — p. 73 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.20 VBAT monitoring characteristics** — p. 73 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.21 Timer characteristics** — p. 73 (`sections/05-5-electrical-characteristics.md`)
    - **5.3.22 Characteristics of communication interfaces** — p. 74 (`sections/05-5-electrical-characteristics.md`)
- **6 Package information** — p. 83 (`sections/06-6-package-information.md`)
  - **6.1 Device marking** — p. 83 (`sections/06-6-package-information.md`)
  - **6.2 SO8N package information (O7)** — p. 83 (`sections/06-6-package-information.md`)
  - **6.3 TSSOP20 package information (YA)** — p. 86 (`sections/06-6-package-information.md`)
  - **6.4 LQFP32 package information (5V)** — p. 88 (`sections/06-6-package-information.md`)
  - **6.5 LQFP48 package information (5B)** — p. 92 (`sections/06-6-package-information.md`)
  - **6.6 Thermal characteristics** — p. 95 (`sections/06-6-package-information.md`)
    - **6.6.1 Reference document** — p. 95 (`sections/06-6-package-information.md`)
- **7 Ordering information** — p. 96 (`sections/07-7-ordering-information.md`)
- **8 Important security notice** — p. 97 (`sections/08-8-important-security-notice.md`)
- **9 Revision history** — p. 98 (`sections/09-9-revision-history.md`)
