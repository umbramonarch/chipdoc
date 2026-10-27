# MSPM0C1105, MSPM0C1106 Mixed-Signal Microcontrollers datasheet (Rev. B)

> **Agent Navigation Guide**:
> - For fast 1-line-per-bitfield register lookups, grep `index/registers.tsv`.
> - For electrical specification or summary tables, grep `index/tables.tsv`.
> - For chapter/peripheral details, grep `sections/` or open the corresponding file below.

## Document Summary

- **Source File**: `mspm0c1105_datasheet.pdf`
- **Author**: Texas Instruments, Incorporated [SLASFJ6,B
]
- **Document ID / Keywords**: `SLASFJ6B`
- **Pages Converted**: 138 / 138
- **Section Files**: 12 (`sections/*.md`)
- **Tables Extracted**: 96 (29 stitched across pages) -> `index/tables.tsv`
- **Registers / Bitfields Indexed**: 0 registers / 0 bitfields -> `index/registers.tsv`
- **Figures Detected**: 9 (9 rendered to `assets/figures/`) -> `index/figures.tsv`
- **Sanity Score**: 100/100 (~61657 tokens)

## Section Shards (`sections/*.md`)

| ID | Section Title | File | Pages | Lines | Tables | Registers | Figures |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: |
| `01` | 1 Features / 2 Applications | [sections/01-1-features-2-applications.md](sections/01-1-features-2-applications.md) | 1-1 | 47 | 0 | 0 | 0 |
| `02` | 3 Description | [sections/02-3-description.md](sections/02-3-description.md) | 2-3 | 56 | 2 | 0 | 0 |
| `03` | 4 Functional Block Diagram | [sections/03-4-functional-block-diagram.md](sections/03-4-functional-block-diagram.md) | 4-4 | 16 | 0 | 0 | 1 |
| `04` | Table of Contents | [sections/04-table-of-contents.md](sections/04-table-of-contents.md) | 5-5 | 51 | 1 | 0 | 0 |
| `05` | 5 Device Comparison | [sections/05-5-device-comparison.md](sections/05-5-device-comparison.md) | 6-6 | 44 | 1 | 0 | 0 |
| `06` | 6 Pin Configuration and Functions | [sections/06-6-pin-configuration-and-functions.md](sections/06-6-pin-configuration-and-functions.md) | 7-40 | 904 | 16 | 0 | 5 |
| `07` | 7 Specifications | [sections/07-7-specifications.md](sections/07-7-specifications.md) | 41-57 | 630 | 28 | 0 | 0 |
| `08` | 8 Detailed Description | [sections/08-8-detailed-description.md](sections/08-8-detailed-description.md) | 58-78 | 885 | 18 | 0 | 2 |
| `09` | 9 Applications, Implementation, and Layout | [sections/09-9-applications-implementation-and-layout.md](sections/09-9-applications-implementation-and-layout.md) | 79-79 | 28 | 0 | 0 | 1 |
| `10` | 10 Device and Documentation Support | [sections/10-10-device-and-documentation-support.md](sections/10-10-device-and-documentation-support.md) | 80-81 | 71 | 1 | 0 | 0 |
| `11` | 11 Revision History | [sections/11-11-revision-history.md](sections/11-11-revision-history.md) | 82-82 | 43 | 1 | 0 | 0 |
| `12` | 12 Mechanical, Packaging, and Orderable Information | [sections/12-12-mechanical-packaging-and-orderable-information.md](sections/12-12-mechanical-packaging-and-orderable-information.md) | 83-138 | 492 | 28 | 0 | 0 |

## Detailed Outline (Table of Contents)

- **1 Features** — p. 1 (`sections/01-1-features-2-applications.md`)
- **2 Applications** — p. 1 (`sections/01-1-features-2-applications.md`)
- **3 Description** — p. 2 (`sections/02-3-description.md`)
- **4 Functional Block Diagram** — p. 4 (`sections/03-4-functional-block-diagram.md`)
- **Table of Contents** — p. 5 (`sections/04-table-of-contents.md`)
- **5 Device Comparison** — p. 6 (`sections/05-5-device-comparison.md`)
- **6 Pin Configuration and Functions** — p. 7 (`sections/06-6-pin-configuration-and-functions.md`)
  - **6.1 Pin Diagrams** — p. 7 (`sections/06-6-pin-configuration-and-functions.md`)
  - **6.2 Pin Attributes** — p. 14 (`sections/06-6-pin-configuration-and-functions.md`)
  - **6.3 Signal Descriptions** — p. 28 (`sections/06-6-pin-configuration-and-functions.md`)
  - **6.4 Connections for Unused Pins** — p. 40 (`sections/06-6-pin-configuration-and-functions.md`)
- **7 Specifications** — p. 41 (`sections/07-7-specifications.md`)
  - **7.1 Absolute Maximum Ratings** — p. 41 (`sections/07-7-specifications.md`)
  - **7.2 ESD Ratings** — p. 41 (`sections/07-7-specifications.md`)
  - **7.3 Recommended Operating Conditions** — p. 41 (`sections/07-7-specifications.md`)
  - **7.4 Thermal Information** — p. 42 (`sections/07-7-specifications.md`)
  - **7.5 Supply Current Characteristics** — p. 44 (`sections/07-7-specifications.md`)
    - **7.5.1 RUN/SLEEP Modes** — p. 44 (`sections/07-7-specifications.md`)
    - **7.5.2 STOP/STANDBY Modes** — p. 44 (`sections/07-7-specifications.md`)
    - **7.5.3 SHUTDOWN Mode** — p. 44 (`sections/07-7-specifications.md`)
  - **7.6 Power Supply Sequencing** — p. 44 (`sections/07-7-specifications.md`)
    - **7.6.1 Power Supply Ramp** — p. 44 (`sections/07-7-specifications.md`)
    - **7.6.2 POR and BOR** — p. 45 (`sections/07-7-specifications.md`)
  - **7.7 Flash Memory Characteristics** — p. 46 (`sections/07-7-specifications.md`)
  - **7.8 Timing Characteristics** — p. 46 (`sections/07-7-specifications.md`)
  - **7.9 Clock Specifications** — p. 47 (`sections/07-7-specifications.md`)
    - **7.9.1 System Oscillator (SYSOSC)** — p. 47 (`sections/07-7-specifications.md`)
    - **7.9.2 Low Frequency Oscillator (LFOSC)** — p. 48 (`sections/07-7-specifications.md`)
    - **7.9.3 Low Frequency Crystal/Clock** — p. 48 (`sections/07-7-specifications.md`)
    - **7.9.4 High Frequency Crystal/Clock** — p. 48 (`sections/07-7-specifications.md`)
  - **7.10 Digital IO** — p. 49 (`sections/07-7-specifications.md`)
    - **7.10.1 Electrical Characteristics** — p. 49 (`sections/07-7-specifications.md`)
    - **7.10.2 Switching Characteristics** — p. 50 (`sections/07-7-specifications.md`)
  - **7.11 Analog Mux VBOOST** — p. 50 (`sections/07-7-specifications.md`)
  - **7.12 ADC** — p. 50 (`sections/07-7-specifications.md`)
    - **7.12.1 Electrical Characteristics** — p. 50 (`sections/07-7-specifications.md`)
    - **7.12.2 Switching Characteristics** — p. 51 (`sections/07-7-specifications.md`)
    - **7.12.3 Linearity Parameters** — p. 51 (`sections/07-7-specifications.md`)
    - **7.12.4 Typical Connection Diagram** — p. 52 (`sections/07-7-specifications.md`)
  - **7.13 Temperature Sensor** — p. 52 (`sections/07-7-specifications.md`)
  - **7.14 VREF** — p. 53 (`sections/07-7-specifications.md`)
    - **7.14.1 Electrical Characteristics** — p. 53 (`sections/07-7-specifications.md`)
    - **7.14.2 Voltage Characteristics** — p. 53 (`sections/07-7-specifications.md`)
  - **7.15 Comparator (COMP)** — p. 53 (`sections/07-7-specifications.md`)
    - **7.15.1 Comparator Electrical Characteristics** — p. 53 (`sections/07-7-specifications.md`)
  - **7.16 I2C** — p. 54 (`sections/07-7-specifications.md`)
    - **7.16.1 I2C Characteristics** — p. 54 (`sections/07-7-specifications.md`)
    - **7.16.2 I2C Filter** — p. 55 (`sections/07-7-specifications.md`)
    - **7.16.3 I2C Timing Diagram** — p. 55 (`sections/07-7-specifications.md`)
  - **7.17 SPI** — p. 55 (`sections/07-7-specifications.md`)
    - **7.17.1 SPI** — p. 55 (`sections/07-7-specifications.md`)
    - **7.17.2 SPI Timing Diagram** — p. 56 (`sections/07-7-specifications.md`)
  - **7.18 UART** — p. 57 (`sections/07-7-specifications.md`)
  - **7.19 TIMx** — p. 57 (`sections/07-7-specifications.md`)
  - **7.20 Emulation and Debug** — p. 57 (`sections/07-7-specifications.md`)
    - **7.20.1 SWD Timing** — p. 57 (`sections/07-7-specifications.md`)
- **8 Detailed Description** — p. 58 (`sections/08-8-detailed-description.md`)
  - **8.1 Overview** — p. 59 (`sections/08-8-detailed-description.md`)
  - **8.2 CPU** — p. 59 (`sections/08-8-detailed-description.md`)
  - **8.3 Operating Modes** — p. 59 (`sections/08-8-detailed-description.md`)
    - **8.3.1 Functionality by Operating Mode** — p. 59 (`sections/08-8-detailed-description.md`)
  - **8.4 Power Management Unit (PMU)** — p. 61 (`sections/08-8-detailed-description.md`)
  - **8.5 Clock Module (CKM)** — p. 61 (`sections/08-8-detailed-description.md`)
  - **8.6 DMA_B** — p. 62 (`sections/08-8-detailed-description.md`)
  - **8.7 Events** — p. 63 (`sections/08-8-detailed-description.md`)
  - **8.8 Memory** — p. 63 (`sections/08-8-detailed-description.md`)
    - **8.8.1 Memory Organization** — p. 63 (`sections/08-8-detailed-description.md`)
    - **8.8.2 Peripheral File Map** — p. 64 (`sections/08-8-detailed-description.md`)
    - **8.8.3 Peripheral Interrupt Vector** — p. 66 (`sections/08-8-detailed-description.md`)
  - **8.9 Flash Memory** — p. 66 (`sections/08-8-detailed-description.md`)
  - **8.10 SRAM** — p. 66 (`sections/08-8-detailed-description.md`)
  - **8.11 GPIO** — p. 67 (`sections/08-8-detailed-description.md`)
  - **8.12 IOMUX** — p. 67 (`sections/08-8-detailed-description.md`)
  - **8.13 ADC** — p. 67 (`sections/08-8-detailed-description.md`)
  - **8.14 Temperature Sensor** — p. 68 (`sections/08-8-detailed-description.md`)
  - **8.15 Low-Frequency Sub System (LFSS)** — p. 68 (`sections/08-8-detailed-description.md`)
  - **8.16 VREF** — p. 68 (`sections/08-8-detailed-description.md`)
  - **8.17 COMP** — p. 69 (`sections/08-8-detailed-description.md`)
  - **8.18 Security** — p. 70 (`sections/08-8-detailed-description.md`)
  - **8.19 CRC** — p. 70 (`sections/08-8-detailed-description.md`)
  - **8.20 UART** — p. 70 (`sections/08-8-detailed-description.md`)
  - **8.21 I2C** — p. 71 (`sections/08-8-detailed-description.md`)
  - **8.22 SPI** — p. 71 (`sections/08-8-detailed-description.md`)
  - **8.23 IWDT** — p. 71 (`sections/08-8-detailed-description.md`)
  - **8.24 WWDT** — p. 72 (`sections/08-8-detailed-description.md`)
  - **8.25 RTC_B** — p. 72 (`sections/08-8-detailed-description.md`)
  - **8.26 Timers (TIMx)** — p. 73 (`sections/08-8-detailed-description.md`)
  - **8.27 Device Analog Connections** — p. 75 (`sections/08-8-detailed-description.md`)
  - **8.28 Input/Output Diagrams** — p. 76 (`sections/08-8-detailed-description.md`)
  - **8.29 Serial Wire Debug Interface** — p. 76 (`sections/08-8-detailed-description.md`)
  - **8.30 DEBUGSS** — p. 77 (`sections/08-8-detailed-description.md`)
  - **8.31 Device Factory Constants** — p. 77 (`sections/08-8-detailed-description.md`)
  - **8.32 Identification** — p. 78 (`sections/08-8-detailed-description.md`)
- **9 Applications, Implementation, and Layout** — p. 79 (`sections/09-9-applications-implementation-and-layout.md`)
  - **9.1 Typical Application** — p. 79 (`sections/09-9-applications-implementation-and-layout.md`)
    - **9.1.1 Schematic** — p. 79 (`sections/09-9-applications-implementation-and-layout.md`)
- **10 Device and Documentation Support** — p. 80 (`sections/10-10-device-and-documentation-support.md`)
  - **10.1 Getting Started and Next Steps** — p. 80 (`sections/10-10-device-and-documentation-support.md`)
  - **10.2 Device Nomenclature** — p. 80 (`sections/10-10-device-and-documentation-support.md`)
  - **10.3 Tools and Software** — p. 81 (`sections/10-10-device-and-documentation-support.md`)
  - **10.4 Documentation Support** — p. 81 (`sections/10-10-device-and-documentation-support.md`)
  - **10.5 Support Resources** — p. 82 (`sections/11-11-revision-history.md`)
  - **10.6 Trademarks** — p. 82 (`sections/11-11-revision-history.md`)
  - **10.7 Electrostatic Discharge Caution** — p. 82 (`sections/11-11-revision-history.md`)
  - **10.8 Glossary** — p. 82 (`sections/11-11-revision-history.md`)
- **11 Revision History** — p. 82 (`sections/11-11-revision-history.md`)
- **12 Mechanical, Packaging, and Orderable Information** — p. 83 (`sections/12-12-mechanical-packaging-and-orderable-information.md`)

---

---
document: "MSPM0C1105, MSPM0C1106 Mixed-Signal Microcontrollers datasheet (Rev. B)"
section_id: "01"
section_title: "1 Features / 2 Applications"
pages: "1-1"
---

# 1 Features / 2 Applications (pp. 1-1)

,

## MSPM0C1105, MSPM0C1106 Mixed-Signal Microcontrollers (p. 1)

1 Features – Internal 32MHz oscillator with -2.1% to 1.6% accuracy (SYSOSC)

- Core – Internal 32kHz oscillator (LFOSC) with ±3% – Arm®32-bit Cortex®-M0+ CPU with memory accuracy protection unit, frequency up to 32MHz – External 4MHz to 32MHz crystal oscillator

- Operating characteristics (HFXT) – Extended temperature: -40°C up to 125°C – External 32kHz crystal oscillator (LFXT) – Wide supply voltage range: 1.62V to 3.6V – External Low Frequency (LF) and High

- Memories Frequency (HF) digital clock inputs – Up to 64KB of flash memory – Digital clock output – 8KB of SRAM • Data integrity and encryption

- High-performance analog peripherals – Cyclic redundancy checker (CRC-16)

- Flexible I/O features – 12-bit 1.6 Msps analog-to-digital converter (ADC), up to 27 external channels – Up to 45 total GPIOs – Configurable 1.4V or 2.5V internal shared – Two 5V-tolerant open-drain IOs voltage reference (VREF) • Development support – Comparator (COMP) with 8-bit reference DAC – 2-pin serial wire debug (SWD) 1 – Integrated temperature sensor • Package options

- Optimized low-power modes – 48-pin LQFP (PT), VQFN (RGZ) , NFBGA – RUN: 91µA/MHz (CoreMark) (ZCM) – STANDBY 2µA and SRAM and registers fully – 32-pin LQFP (VFC), VQFN (RHB) retained – 28-pin VSSOP (DGS28) – SHUTDOWN: 68nA with I/O wake-up – 24-pin VQFN (RGE)

- Intelligent digital peripherals – 20-pin WQFN (RUK), VSSOP (DGS20) – 3-channel DMA controller • Family members (also see Device Comparison) – 7-channel event fabric signaling system – MSPM0C1106: 64KB of flash, 8KB of RAM – Five timers supporting up to 18 PWM outputs, – MSPM0C1105: 32KB of flash, 8KB of RAM all operational down to STANDBY mode – MSP32G031C8: 64KB of flash, 8KB of RAM

- One 16-bit advanced timer with deadband – MSP32G031C6: 32KB of flash, 8KB of RAM and the timer frequency up to 64Mhz – MSP32C031C6: 32KB of flash, 8KB of RAM

- One 16-bit general purpose timer with 4 • Development kits and software (also see Tools capture/compares and Software)

- Three 16-bit general-purpose timers with 2 – LP-MSPM0C1106 LaunchPad ™ development capture/compares kit – Window-watchdog timer (WWDT) – MSP Software Development Kit (SDK) – Independent watchdog timer (IWDT)

## 2 Applications (p. 1)

– RTC with alarm and calendar mode – BEEPER generating 1/2/4/8kHz square wave to • Battery charging and management drive an external beeper • Power supplies and power delivery

- Communication interfaces • Personal electronics – Three UART modules, with one supporting LIN, • Building security and fire safety IrDA, DALI, smart card, Manchester • Connected peripherals and printers – Two I 2 C modules supporting SMBus/PMBus • Grid infrastructure and wakeup from STOP mode, supporting up • Smart metering to FM+ (1Mbps) • Communication modules – One SPI module supporting up to 16Mbps • Medical and healthcare

- Clock system • Lighting

32-pin LQFP (VFC) is preview

An IMPORTANT NOTICE at the end of this data sheet addresses availability, warranty, changes, use in safety-critical applications, intellectual property matters and other important disclaimers. PRODUCTION DATA.


---

---
document: "MSPM0C1105, MSPM0C1106 Mixed-Signal Microcontrollers datasheet (Rev. B)"
section_id: "02"
section_title: "3 Description"
pages: "2-3"
---

# 3 Description (pp. 2-3)

## 3 Description (p. 2)

MSPM0C1105/6 microcontrollers (MCUs) are part of MSP's highly integrated, ultra-low-power 32-bit MSPM0 MCU family based on the Arm® Cortex®-M0+ 32-bit core platform, operating at up to 32MHz frequency. These cost-optimized MCUs offer high-performance analog peripheral integration, support extended temperature ranges from -40°C to 125°C, and operate with supply voltages from 1.62V to 3.6V.

The MSPM0C1105/6 devices provide up to 64KB embedded flash program memory with 8KB SRAM. These MCUs incorporate a high-speed on-chip oscillator with an accuracy from -2.1% to +1.6%, eliminating the need for an external crystal. Additional features include a 3-channel DMA, CRC-16 accelerator, and a variety of high-performance analog peripherals such as one 12-bit 1.6 Msps ADC with VDD as the voltage reference, a comparator with 8-bit reference DAC and an on-chip temperature sensor. These devices also offer intelligent digital peripherals such as one 16-bit advanced timer with deadband and timer frequency up to 64MHz, four 16-bit general purpose timer, one windowed watchdog timer, and a variety of communication peripherals including three UART, one SPI, and two I2C. These communication peripherals offer protocol support for LIN, IrDA, DALI, Manchester, smart card, SMBus, and PMBus.

The TI MSPM0 family of low-power MCUs consists of devices with varying degrees of analog and digital integration allowing for customers to find the MCU that meets the project's needs. The MSPM0 MCU platform combines the Arm Cortex-M0+ platform with a holistic ultra-low-power system architecture, allowing system designers to increase performance while reducing energy consumption.

MSPM0C1105/6 MCUs are supported by an extensive hardware and software ecosystem with reference designs and code examples to get the design started quickly. Development kits include a LaunchPad available for purchase. TI also provides a free MSP Software Development Kit (SDK), which is available as a component of Code Composer Studio ™ IDE desktop and cloud version within the TI Resource Explorer. MSPM0 MCUs are also supported by extensive online collateral, training with MSP Academy, and online support through the TI E2E ™ support forums.

For complete module descriptions, see the MSPM0 C-Series Microcontrollers Technical Reference Manual.

**Table 3-1. Package Information (p. 2)**

| PART NUMBER (1) (2) | PACKAGE | PACKAGE SIZE (3) (4) |
| --- | --- | --- |
| MSPM0C1106SPTR | PT (LQFP, 48) (6) | 9mm × 9mm |
| MSPM0C1105SPTR | PT (LQFP, 48) (6) | 9mm × 9mm |
| MSPM0C1106SRGZR | RGZ (VQFN, 48) | 7mm × 7mm |
| MSPM0C1105SRGZR | RGZ (VQFN, 48) | 7mm × 7mm |
| MSPM0C1106SZCMR | ZCM (NFBGA, 48) | 3.5mm × 3.5mm |
| MSPM0C1105SZCMR | ZCM (NFBGA, 48) | 3.5mm × 3.5mm |
| MSPM0C1106SRHBR | RHB (VQFN, 32) | 5mm × 5mm |
| MSPM0C1105SRHBR | RHB (VQFN, 32) | 5mm × 5mm |
| MSPM0C1106SDGS28R | DGS28 (VSSOP, 28) | 7.1mm × 4.9mm |
| MSPM0C1105SDGS28R | DGS28 (VSSOP, 28) | 7.1mm × 4.9mm |
| MSPM0C1106SRGER | RGE (VQFN, 24) | 4mm × 4mm |
| MSPM0C1105SRGER | RGE (VQFN, 24) | 4mm × 4mm |
| MSPM0C1106SDGS20R | DGS20 (VSSOP, 20) | 5.1mm × 4.9mm |
| MSPM0C1105SDGS20R | DGS20 (VSSOP, 20) | 5.1mm × 4.9mm |
| MSPM0C1106SRUKR | RUK (WQFN, 20) | 3mm × 3mm |
| MSPM0C1105SRUKR | RUK (WQFN, 20) | 3mm × 3mm |
| MSP32C031C6SPTR | PT (LQFP, 48) (6) | 9mm × 9mm |
| MSP32G031C6SPTR | PT (LQFP, 48) (6) | 9mm × 9mm |
| MSP32G031C8SPTR | PT (LQFP, 48) (6) | 9mm × 9mm |
| MSP32G031K6SVFCR | VFC (LQFP, 32) (5) | 9mm × 9mm |

**Table 3-1. Package Information (continued) (p. 3)**

| Col 1 | PART NUMBER (1) (2) | PACKAGE | PACKAGE SIZE | (3) (4) |
| --- | --- | --- | --- | --- |
|  | MSP32C031K6SVFCR | VFC (LQFP, 32) (5) | 9mm × 9mm |  |
|  | MSP32G031K8SVFCR | VFC (LQFP, 32) (5) | 9mm × 9mm |  |
| (1) (2) (3) (4) (5) (6) | For the most current part, package, and ordering information for all available devices, see the TI website. Please find more infromation about the device name in the For more information, see Mechanical, Packaging, and Orderable Information The package size (length × width) is a nominal value and includes pins, where applicable. 32-pin LQFP (VFC) is preview. There are two pin configurations for the PT (LQFP, 48) package. Refer to | Package Option Addendum Device Nomenclature section. . Section 6.2 for more information. | , or see the |  |
|  |  | CAUTION |  |  |
|  | to prevent electrical overstress or disturbing of data or code memory. See ESD Considerations MSPM0 MCUs. | System-level ESD protection must be applied in compliance with the device-level ESD specification MSP430 ™ for more information. The principles in this application note are applicable to | System-Level |  |


---

---
document: "MSPM0C1105, MSPM0C1106 Mixed-Signal Microcontrollers datasheet (Rev. B)"
section_id: "03"
section_title: "4 Functional Block Diagram"
pages: "4-4"
---

# 4 Functional Block Diagram (pp. 4-4)

## 4 Functional Block Diagram (p. 4)

Figure 4-1 shows the functional block diagram.

![Figure 4-1. MSPM0C1105/6 Functional Block Diagram](../assets/figures/p004-figure-4-1-mspm0c1105-6-functional-block-diagram.png)
*Figure 4-1. MSPM0C1105/6 Functional Block Diagram (p. 4)*


---

---
document: "MSPM0C1105, MSPM0C1106 Mixed-Signal Microcontrollers datasheet (Rev. B)"
section_id: "04"
section_title: "Table of Contents"
pages: "5-5"
---

# Table of Contents (pp. 5-5)

| Table of Contents | Col 2 |
| --- | --- |
| 1 Features............................................................................ 1 | 8.8 Memory..................................................................... 63 |
| 2 Applications..................................................................... 1 | 8.9 Flash Memory........................................................... 66 |
| 3 Description....................................................................... 2 | 8.10 SRAM...................................................................... 66 |
| 4 Functional Block Diagram.............................................. 4 | 8.11 GPIO....................................................................... 67 |
| 5 Device Comparison......................................................... 6 | 8.12 IOMUX.................................................................... 67 |
| 6 Pin Configuration and Functions................................... 7 | 8.13 ADC........................................................................ 67 |
| 6.1 Pin Diagrams.............................................................. 7 | 8.14 Temperature Sensor............................................... 68 |
| 6.2 Pin Attributes............................................................. 14 | 8.15 Low-Frequency Sub System (LFSS)...................... 68 |
| 6.3 Signal Descriptions................................................... 28 | 8.16 VREF...................................................................... 68 |
| 6.4 Connections for Unused Pins................................... 40 | 8.17 COMP..................................................................... 69 |
| 7 Specifications................................................................ 41 | 8.18 Security................................................................... 70 |
| 7.1 Absolute Maximum Ratings...................................... 41 | 8.19 CRC........................................................................ 70 |
| 7.2 ESD Ratings............................................................. 41 | 8.20 UART...................................................................... 70 |
| 7.3 Recommended Operating Conditions....................... 41 | 8.21 I2C.......................................................................... 71 |
| 7.4 Thermal Information.................................................. 42 | 8.22 SPI.......................................................................... 71 |
| 7.5 Supply Current Characteristics................................. 44 | 8.23 IWDT....................................................................... 71 |
| 7.6 Power Supply Sequencing........................................ 44 | 8.24 WWDT.................................................................... 72 |
| 7.7 Flash Memory Characteristics.................................. 46 | 8.25 RTC_B.................................................................... 72 |
| 7.8 Timing Characteristics...............................................46 | 8.26 Timers (TIMx).......................................................... 73 |
| 7.9 Clock Specifications.................................................. 47 | 8.27 Device Analog Connections.................................... 75 |
| 7.10 Digital IO................................................................ 49 | 8.28 Input/Output Diagrams............................................ 76 |
| 7.11 Analog Mux VBOOST............................................. 50 | 8.29 Serial Wire Debug Interface.................................... 76 |
| 7.12 ADC....................................................................... 50 | 8.30 DEBUGSS.............................................................. 77 |
| 7.13 Temperature Sensor............................................... 52 | 8.31 Device Factory Constants...................................... 77 |
| 7.14 VREF...................................................................... 53 | 8.32 Identification............................................................ 78 |
| 7.15 Comparator (COMP).............................................. 53 | 9 Applications, Implementation, and Layout................. 79 |
| 7.16 I2C......................................................................... 54 | 9.1 Typical Application.................................................... 79 |
| 7.17 SPI......................................................................... 55 | 10 Device and Documentation Support.......................... 80 |
| 7.18 UART...................................................................... 57 | 10.1 Getting Started and Next Steps.............................. 80 |
| 7.19 TIMx........................................................................ 57 | 10.2 Device Nomenclature.............................................. 80 |
| 7.20 Emulation and Debug............................................. 57 | 10.3 Tools and Software................................................. 81 |
| 8 Detailed Description...................................................... 58 | 10.4 Documentation Support.......................................... 81 |
| 8.1 Overview................................................................... 59 | 10.5 Support Resources................................................. 82 |
| 8.2 CPU.......................................................................... 59 | 10.6 Trademarks............................................................. 82 |
| 8.3 Operating Modes...................................................... 59 | 10.7 Electrostatic Discharge Caution.............................. 82 |
| 8.4 Power Management Unit (PMU)............................... 61 | 10.8 Glossary.................................................................. 82 |
| 8.5 Clock Module (CKM)................................................. 61 | 11 Revision History.......................................................... 82 |
| 8.6 DMA_B...................................................................... 62 | 12 Mechanical, Packaging, and Orderable |
| 8.7 Events....................................................................... 63 | Information.................................................................... 83 |


---

---
document: "MSPM0C1105, MSPM0C1106 Mixed-Signal Microcontrollers datasheet (Rev. B)"
section_id: "05"
section_title: "5 Device Comparison"
pages: "6-6"
---

# 5 Device Comparison (pp. 6-6)

## 5 Device Comparison (p. 6)

**Table 5-1. Device Comparison Table (p. 6)**

| DEVICE NAME (1) | FLASH / SRAM (KB) | ADC CHANNEL | UART / I2C / SPI | TIMG | TIMA | GPIO | COMP | PACKAGE (2) |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| MSPM0C1106SPTR | 64 / 8 | 27 | 3 /2 / 1 | 4 | 1 | 45 | 1 | 48 LQFP |
| MSPM0C1105SPTR | 32 / 8 | 27 | 3 /2 / 1 | 4 | 1 | 45 | 1 | (9mm × 9mm) |
| MSPM0C1106SRGZR | 64 / 8 | 27 | 3 /2 / 1 | 4 | 1 | 45 | 1 | 48 VQFN |
| MSPM0C1105SRGZR (3) | 32 / 8 | 27 | 3 /2 / 1 | 4 | 1 | 45 | 1 | (7mm × 7mm) |
| MSPM0C1106SZCMR (3) | 64 / 8 | 27 | 3 /2 / 1 | 4 | 1 | 45 | 1 | 48 NFBGA |
| MSPM0C1105SZCMR (3) | 32 / 8 | 27 | 3 /2 / 1 | 4 | 1 | 45 | 1 | (3.5mm × 3.5mm) |
| MSPM0C1106SRHBR | 64 / 8 | 18 | 3 /2 / 1 | 4 | 1 | 29 | 1 | 32 VQFN |
| MSPM0C1105SRHBR | 32 / 8 | 18 | 3 /2 / 1 | 4 | 1 | 29 | 1 | (5mm × 5mm) |
| MSPM0C1106SDGS28R | 64 / 8 | 15 | 3 /2 / 1 | 4 | 1 | 25 | 1 | 28 VSSOP |
| MSPM0C1105SDGS28R | 32 / 8 | 15 | 3 /2 / 1 | 4 | 1 | 25 | 1 | (7.1mm × 4.9mm) |
| MSPM0C1106SRGER | 64 / 8 | 13 | 3 /2 / 1 | 4 | 1 | 21 | 1 | 24 VQFN |
| MSPM0C1105SRGER | 32 / 8 | 13 | 3 /2 / 1 | 4 | 1 | 21 | 1 | (4mm × 4mm) |
| MSPM0C1106SDGS20R | 64 / 8 | 12 | 3 /2 / 1 | 4 | 1 | 17 | 1 | 20 VSSOP |
| MSPM0C1105SDGS20R | 32 / 8 | 12 | 3 /2 / 1 | 4 | 1 | 17 | 1 | (5.1mm × 4.9mm) |
| MSPM0C1106SRUKR | 64 / 8 | 12 | 3 /2 / 1 | 4 | 1 | 17 | 1 | 20 WQFN |
| MSPM0C1105SRUKR | 32 / 8 | 12 | 3 /2 / 1 | 4 | 1 | 17 | 1 | (3mm × 3mm) |
| MSP32C031C6SPTR | 32 / 8 |  |  |  |  |  |  |  |
| MSP32G031C6SPTR | 32 / 8 | 27 | 3 /2 / 1 | 4 | 1 | 45 | 1 | 48 LQFP (9mm × 9mm) |
| MSP32G031C8SPTR | 64 / 8 |  |  |  |  |  |  |  |
| MSP32G031K6SVFCR (3) | 32 / 8 |  |  |  |  |  |  |  |
| MSP32C031K6SVFCR (3) | 32 / 8 | 18 | 3 /2 / 1 | 4 | 1 | 29 | 1 | 32 LQFP (9mm × 9mm) |
| MSP32G031K8SVFCR (3) | 64 / 8 |  |  |  |  |  |  |  |

(1) For the most current part, package, and ordering information for all available devices, see the Package Option Addendum, or see the TI website.

(2) The sizes shown here are approximations. For the package dimensions with tolerances, see the Mechanical Data.

(3) 32-pin LQFP (VFC) and 48-pin NFBGA (ZCM) are preview


---

---
document: "MSPM0C1105, MSPM0C1106 Mixed-Signal Microcontrollers datasheet (Rev. B)"
section_id: "06"
section_title: "6 Pin Configuration and Functions"
pages: "7-40"
---

# 6 Pin Configuration and Functions (pp. 7-40)

6 Pin Configuration and Functions

The System Configuration tool provides a graphical interface to enable, configurable, and generate initialization code for pin multiplexing and simplifying pin settings. The pin diagrams shown in the data sheet show the primary peripheral functions, some of the integrated device features, and available clock signals to simplify the device pinout.

For full descriptions of the pin functions, see the Pin Attributes and Signal Descriptions sections.

6.1 Pin Diagrams

![Figure 6-1. 48-pin RGZ (VQFN) Package](../assets/figures/p007-figure-6-1-48-pin-rgz-vqfn-package.png)
*Figure 6-1. 48-pin RGZ (VQFN) Package (p. 7)*

Pin Attributes and Signal

![Figure 6-2. 48-pin PT (A) 48-pin PT (LQFP) Package](../assets/figures/p008-figure-6-2-48-pin-pt-a-48-pin-pt-lqfp-package.png)
*Figure 6-2. 48-pin PT (A) 48-pin PT (LQFP) Package (p. 8)*

Figure 6-3. 48-pin PT (B) (LQFP) Package

Figure 6-4. 32-pin RHB (VQFN) Package

![Figure 6-5. 32-pin VFC (LQFP)](../assets/figures/p010-figure-6-5-32-pin-vfc-lqfp.png)
*Figure 6-5. 32-pin VFC (LQFP) (p. 10)*

Figure 6-6. 28-pin DGS28 (VSSOP)

![Figure 6-7. 24-pin RGE (VQFN) Package](../assets/figures/p011-figure-6-7-24-pin-rge-vqfn-package.png)
*Figure 6-7. 24-pin RGE (VQFN) Package (p. 11)*

Figure 6-8. 20-pin DGS20 (VSSOP)

![Figure 6-9. 20-pin RUK (WQFN)](../assets/figures/p012-figure-6-9-20-pin-ruk-wqfn.png)
*Figure 6-9. 20-pin RUK (WQFN) (p. 12)*

Figure 6-10. 48-pin ZCM (NFBGA) Package (Top view)

6.2 Pin Attributes

The following table describes the functions available on every pin for each device package.

Note Each digital I/O on a device is mapped to a specific Pin Control Management Register (PINCMx) that lets users configure the desired Pin Function using the PINCM.PF control bits.

Each digital I/O on a device is mapped to a specific Pin Control Management Register (PINCMx) which allows users to configure the desired Pin Function using the PINCM.PF control bits. The IOMUX only supports connecting one IOMUX-managed digital function to the pin at the same time. The PINCM.PF and PINCM.PC in IOMUX are recommended to be set to 0 when non-IOMUX managed functions (such as analog connections) are intended to be used on a pin. However, non-IOMUX managed signals (such as analog inputs and WAKE inputs) can be enabled on a pin at the same time that an IOMUX managed digital function is enabled on the pin, provided there is no contention between the functions. In this case, the designer must verify that no contention exists between the functions enabled on each pin.

**Table 6-1. Digital IO Features by IO Type (p. 14)**

| Col 1 | BUFFER TYPE | INVERSION CONTROL | DRIVE STRENGTH CONTROL | HYSTERESIS CONTROL | PULLUP RESISTOR | PULLDOWN RESISTOR | WAKEUP LOGIC | Col 9 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | SDIO (standard drive) | Y |  |  | Y | Y |  |  |
|  | SDIO (standard drive) with wake 1 | Y |  |  | Y | Y | Y |  |
|  | ODIO (5V-tolerant open drain) | Y |  | Y |  | Y | Y |  |
| 1. | the | All I/O can be configured to wakeup the MCU from higher low-power modes. See section MSPM0 C-Series Microcontrollers Technical Reference Manual | Standard with Wake allows the I/O to wake up the device from the lowest low-power mode of SHUTDOWN. |  | for details. |  | GPIO FastWake | in |
|  | following devices correspond to these respective configurations: • PT (A): MSPM0C1105SPTR, & MSPM0C1106SPTR • | PT (B): MSP32C031C6SPTR, MSP32G031C6SPTR, & MSP32G031C8SPTR | Note There are two pin configurations for the 48 LQFP (PT) package denoted as PT (A) and PT (B). The |  |  |  |  |  |

**Table 6-2. Pin Attributes (PT (A), ZCM, VFC, PT (B), RGZ, RHB, DGS28, RGE, DGS20, RUK Packages) (pp. 14-20)**

| PT (A) PIN | ZCM PIN | VFC PIN | PT (B) PIN | RGZ PIN | RHB PIN | DGS2 8 PIN | RGE PIN | DGS2 0 PIN | RUK PIN | PIN NAME/ IOMUX REG/ IOMUX ADDR | SIGNAL NAME | IOMUX PF | SIGNAL TYPE | BUFFER TYPE |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 4 | A1 | 6 | 10 | 4 | 3 | 6 | 2 | 5 | 3 | NRST | NRST | (Non-IOMUX 1) 0 | RESET | RESET |
|  |  |  |  |  |  |  |  |  |  |  | PA0 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_TX | 2 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SDA | 3 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C0 | 4 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PA0 | TIMA_FAL1 | 5 | I | ODIO |
| 1 | D4 |  | 1 | 1 | 1 | 4 | 24 | 4 | 2 | PA0 PINCM1 | FCC_IN | 6 | I | (5V- |
|  |  |  |  |  |  |  |  |  |  | PINCM1 0x40428000 | TIMG8_C1 | 7 | IO | tol)with wake |
|  |  |  |  |  |  |  |  |  |  |  | BEEP | 8 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG14_C0 | 9 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS1_MISO1 | 10 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | RTC_OUT | 12 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | PA1 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_RX | 2 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SCL | 3 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C1 | 4 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA_FAL2 | 5 | I |  |
|  |  |  |  |  |  |  |  |  |  | PA1 | TIMG8_IDX | 6 | I | ODIO |
| 2 | C5 | 2 | 2 | 2 | 2 | 5 | 1 | 5 | 3 | PA1 PINCM2 | TIMG8_C0 | 7 | IO | (5V- |
|  |  |  |  |  |  |  |  |  |  | PINCM2 0x40428004 | TIMG14_C1 | 9 | IO | tol)with wake |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS3_CD _MISO3 | 10 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | HFCLKIN | 11 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_TX | 12 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RTS | 13 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SDA | 14 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | PA2 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C1 | 2 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS0 | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG2_C1 | 4 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PA2 | TIMG8_IDX | 5 | I | SDIO |
| 8 | A4 |  |  | 8 | 6 | 9 | 5 | 8 | 6 | PINCM5 | TIMA0_C3N | 6 | O | (standar |
|  |  |  |  |  |  |  |  |  |  | 0x40428010 | TIMA0_C2N | 7 | O | d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMA_FAL0 | 8 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA_FAL1 | 9 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C0 | 11 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SCL | 12 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | PA3 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C0 | 2 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS1_MISO1 | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C1_SDA | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C1 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PA3 | TIMG2_C0 | 7 | IO |  |
| 9 | A3 |  | 8 | 9 | 7 | 10 | 6 |  |  | PA3 PINCM6 | TIMA0_C2 | 8 | IO | SDIO (standar |
|  |  |  |  |  |  |  |  |  |  | PINCM6 0x40428014 | UART2_CTS | 9 | I | d) |
|  |  |  |  |  |  |  |  |  |  |  | UART1_TX | 10 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS3_CD _MISO3 | 11 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SDA | 12 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | COMP0_OUT | 14 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | LFXIN | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA4 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C1 | 2 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_POCI | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C1_SCL | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C1N | 5 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | LFCLKIN | 6 | I |  |
|  |  |  |  |  |  |  |  |  |  | PA4 | TIMG2_C1 | 7 | IO | SDIO |
| 10 | A2 |  | 9 | 10 | 8 | 11 | 7 | 9 | 7 | PINCM7 0x40428018 | TIMA0_C3 | 8 | IO | (standar d) |
|  |  |  |  |  |  |  |  |  |  | 0x40428018 | UART2_RTS | 9 | O | d) |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RX | 10 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS0 | 11 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C0N | 12 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | HFCLKIN | 13 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | LFXOUT | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA5 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C0 | 2 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_PICO | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C1_SDA | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG14_C0 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PA5 | FCC_IN | 6 | I | SDIO |
| 11 | B1 | 7 | 11 | 11 | 9 | 12 |  |  |  | PINCM8 0x4042801c | TIMG1_C0 | 7 | IO | (standar d) |
|  |  |  |  |  |  |  |  |  |  | 0x4042801c | TIMA_FAL1 | 8 | I | d) |
|  |  |  |  |  |  |  |  |  |  |  | UART0_CTS | 9 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_TX | 11 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C1 | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | HFXIN | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA6 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C1 | 2 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_SCLK | 3 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C1_SCL | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG14_C1 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | HFCLKIN | 6 | I |  |
|  |  |  |  |  |  |  |  |  |  | PA6 | TIMG1_C1 | 7 | IO |  |
| 12 | C1 | 8 | 12 | 12 | 10 | 13 |  | 10 |  | PA6 PINCM9 | TIMA_FAL0 | 8 | I | SDIO (standar |
|  |  |  |  |  |  |  |  |  |  | PINCM9 0x40428020 | UART0_RTS | 9 | O | d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C2N | 10 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RX | 11 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C2 | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SDA | 13 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | BEEP | 14 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | HFXOUT | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA7 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | CLK_OUT | 2 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C0 | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C2 | 4 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_IDX | 5 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG2_C1 | 6 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PA7 | TIMA0_C1 | 7 | IO | SDIO |
| 13 | B2 | 9 | 13 | 13 | 11 |  |  |  |  | PINCM10 0x40428024 | SPI0_CS2_MISO2 | 8 | IO | (standar d) |
|  |  |  |  |  |  |  |  |  |  |  | FCC_IN | 9 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_POCI | 10 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_PICO | 11 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_TX | 12 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG1_C0 | 13 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | COMP0_OUT | 14 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | PA8 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_TX | 2 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS0 | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SDA | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C0 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA_FAL2 | 6 | I |  |
|  |  |  |  |  |  |  |  |  |  | PA8 | TIMA_FAL0 | 7 | I | SDIO |
| 16 | C4 | 12 | 16 | 16 | 12 |  |  |  |  | PINCM13 0x40428030 | SPI0_CS3_CD _MISO3 | 8 | IO | (standar d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMG2_C1 | 9 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | HFCLKIN | 10 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_RTS | 11 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_SCLK | 12 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RX | 13 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C3N | 14 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | PA9 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RX | 2 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_PICO | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SCL | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C0N | 5 | O |  |
|  |  |  |  |  |  |  |  |  |  | PA9 | CLK_OUT | 6 | O | SDIO |
| 17 | D3 | 13 | 17 | 17 | 13 | 14 | 8 |  |  | PINCM14 | TIMA0_C1 | 7 | IO | (standar |
|  |  |  |  |  |  |  |  |  |  | 0x40428034 | RTC_OUT | 8 | O | d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMG2_C0 | 9 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_POCI | 10 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_CTS | 11 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA_FAL1 | 12 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG1_C1 | 13 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | PA10 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_TX | 2 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_POCI | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SDA | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C2 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PA10 | CLK_OUT | 6 | O | SDIO |
| 18 | D2 | 14 | 18 | 18 | 14 | 15 | 9 |  |  | PINCM15 | TIMG14_C0 | 7 | IO | (standar d)with |
|  |  |  |  |  |  |  |  |  |  | 0x40428038 | I2C1_SDA | 8 | IOD | wake |
|  |  |  |  |  |  |  |  |  |  |  | TIMA_FAL1 | 10 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG2_C1 | 11 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C1N | 12 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C1 | 13 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_PICO | 14 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | PA11 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_RX | 2 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_SCLK | 3 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SCL | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C2N | 5 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RX | 6 | I |  |
|  | E4 |  |  |  |  |  |  | 11 |  | PA11 | TIMG14_C1 | 7 | IO | SDIO (standar |
| 19 | E4 | 15 | 19 | 19 | 15 | 16 | 10 | 11 | 9 | PINCM16 0x4042803c | I2C1_SCL | 8 | IOD | d)with wake |
|  |  |  |  |  |  |  |  |  |  | 0x4042803c | TIMA_FAL0 | 10 | I | wake |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS0 | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | COMP0_OUT | 14 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_25 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | COMP0_DAC_OUT | (Non-IOMUX 2) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA12 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_SCLK | 2 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C3 | 4 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | FCC_IN | 5 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG14_C0 | 6 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PA12 | SPI0_CS1_MISO1 | 8 | IO | SDIO |
| 27 | G2 |  | 27 | 27 | 16 |  |  |  |  | PINCM24 | UART2_CTS | 9 | I | (standar |
|  |  |  |  |  |  |  |  |  |  | 0x4042805c | UART1_CTS | 10 | I | d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C3N | 11 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C1_SCL | 12 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG2_C1 | 13 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | COMP0_OUT | 14 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_18 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA13 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_RX | 2 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_POCI | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C2N | 4 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C3N | 5 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | RTC_OUT | 6 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG14_C1 | 7 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG14_C3 | 8 | IO |  |
| 28 | G3 | 18 | 28 | 28 | 17 |  |  |  |  | PA13 PINCM25 0x40428060 | SPI0_CS3_CD _MISO3 | 9 | IO | SDIO (standar d) |
|  |  |  |  |  |  |  |  |  |  | 0x40428060 | UART2_TX | 10 | O | d) |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RTS | 11 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS0 | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C1 | 13 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C1 | 14 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_17 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | COMP0_IN2- | (Non-IOMUX 2) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA14 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_CTS | 2 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_PICO | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG1_C0 | 4 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | CLK_OUT | 6 | O |  |
|  |  |  |  |  |  |  |  |  |  | PA14 | SPI0_CS2_MISO2 | 9 | IO | SDIO |
| 29 | F3 | 19 | 29 | 29 | 18 | 17 |  |  |  | PINCM26 0x40428064 | UART2_RX | 10 | I | (standar d) |
|  |  |  |  |  |  |  |  |  |  | 0x40428064 | I2C0_SCL | 12 | IOD | d) |
|  |  |  |  |  |  |  |  |  |  |  | UART0_TX | 13 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C2 | 14 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_16 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | COMP0_IN2+ | (Non-IOMUX 2) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA15 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_RTS | 2 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS2_MISO2 | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C1_SCL | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  | PA15 | TIMA0_C2 | 5 | IO | SDIO |
| 30 | G4 | 20 | 30 | 30 | 19 | 18 | 11 |  |  | PINCM27 0x40428068 | TIMG8_IDX | 7 | I | (standar d) |
|  |  |  |  |  |  |  |  |  |  | 0x40428068 | UART2_RTS | 10 | O | d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMG14_C1 | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_15 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | COMP0_IN3+ | (Non-IOMUX 2) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA16 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_POCI | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C1_SDA | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  | PA16 | TIMA0_C2N | 5 | O |  |
| 31 | F4 |  | 31 | 31 | 20 | 19 | 12 | 12 | 10 | PA16 PINCM28 | FCC_IN | 7 | I | SDIO (standar |
|  |  |  |  |  |  |  |  |  |  | PINCM28 0x4042806c | UART2_CTS | 10 | I | d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMG14_C2 | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | COMP0_OUT | 14 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_14 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA17 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_TX | 2 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C2 | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C1_SCL | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C3 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG2_C0 | 6 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C0 | 7 | IO |  |
| 32 | G5 | 21 | 32 | 32 | 21 | 20 | 13 | 13 | 11 | PA17 PINCM29 | TIMA0_C0N | 8 | O | SDIO (standar |
| 32 | G5 | 21 | 32 | 32 | 21 | 20 | 13 | 13 | 11 | PINCM29 0x40428070 | SPI0_CS1_MISO1 | 9 | IO | (standar d) |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_SCLK | 10 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SDA | 11 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_RX | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_13 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | COMP0_IN1- | (Non-IOMUX 2) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA18 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RX | 2 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RTS | 3 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C1_SDA | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C3N | 5 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG2_C1 | 6 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C1 | 7 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_PICO | 8 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PA18 | SPI0_CS0 | 9 | IO | SDIO |
| 33 | F5 | 22 | 33 | 33 | 22 | 21 | 14 | 14 | 12 | PINCM30 | TIMA0_C1N | 10 | O | (standar |
|  |  |  |  |  |  |  |  |  |  | 0x40428074 | TIMA0_C0 | 11 | IO | d) |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_POCI | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA_FAL2 | 13 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | CLK_OUT | 14 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_12 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | COMP0_IN1+ | (Non-IOMUX 2) 0 | A |  |

Table 6-4 (Non-IOMUX 2) I 0

**Table 6-2. Pin Attributes (PT (A), ZCM, VFC, PT (B), RGZ, RHB, DGS28, RGE, DGS20, RUK Packages) (pp. 21-28)**

| PT (A) PIN | ZCM PIN | VFC PIN | PT (B) PIN | RGZ PIN | RHB PIN | DGS2 8 PIN | RGE PIN | DGS2 0 PIN | RUK PIN | PIN NAME/ IOMUX REG/ IOMUX ADDR | SIGNAL NAME | IOMUX PF | SIGNAL TYPE | BUFFER TYPE |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  |  |  |  |  |  | PA19 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SWDIO | 2 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_SCLK | 3 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C1_SDA | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  | PA19 | TIMA0_C2 | 5 | IO |  |
| 34 | B6 | 24 | 35 | 34 | 23 | 22 | 15 | 15 | 13 | PA19 PINCM32 | TIMG14_C0 | 6 | IO | SDIO (standar |
|  |  |  |  |  |  |  |  |  |  | PINCM32 0x4042807c | SPI0_POCI | 7 | IO | d) |
|  |  |  |  |  |  |  |  |  |  |  | UART0_CTS | 8 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RX | 11 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_PICO | 13 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_22 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA20 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SWCLK | 2 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA_FAL1 | 3 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C1_SCL | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C2N | 5 | O |  |
|  |  |  |  |  |  |  |  |  |  | PA20 | TIMG14_C1 | 6 | IO |  |
| 35 | G6 | 25 | 36 | 35 | 24 | 23 | 16 | 16 | 14 | PA20 PINCM33 | SPI0_PICO | 7 | IO | SDIO (standar |
|  |  |  |  |  |  |  |  |  |  | PINCM33 0x40428080 | TIMA0_C0 | 8 | IO | d) |
|  |  |  |  |  |  |  |  |  |  |  | UART0_RTS | 10 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_TX | 11 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS0 | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RX | 13 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_4 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA21 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART2_TX | 2 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS3_CD _MISO3 | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_CTS | 4 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C0 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PA21 | TIMG1_C0 | 6 | IO | SDIO |
| 39 | E6 |  | 39 | 39 | 25 | 24 | 17 |  |  | PINCM37 0x40428090 | UART2_CTS | 8 | I | (standar d) |
|  |  |  |  |  |  |  |  |  |  | 0x40428090 | TIMG8_C0 | 10 | IO | d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C0N | 12 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | UART2_RX | 13 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_8 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_VREF- | (Non-IOMUX 2) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA22 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART2_RX | 2 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS2_MISO2 | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RTS | 4 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C0N | 5 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG1_C1 | 6 | IO |  |
| 40 | F7 |  | 40 | 40 | 26 | 25 | 18 | 17 | 15 | PA22 | TIMA0_C1 | 7 | IO | SDIO (standar |
| 40 | F7 |  | 40 | 40 | 26 | 25 | 18 | 17 | 15 | PINCM38 0x40428094 | CLK_OUT | 8 | O | (standar d) |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SCL | 9 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C1 | 10 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RX | 11 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_POCI | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART2_TX | 13 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_7 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA23 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART2_TX | 2 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS3_CD _MISO3 | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C3 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C0 | 6 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PA23 | TIMG2_C0 | 7 | IO | SDIO |
| 43 | D7 | 28 | 43 | 43 | 27 | 26 | 19 | 18 | 16 | PINCM41 0x404280a0 | UART0_TX | 8 | O | (standar d) |
|  |  |  |  |  |  |  |  |  |  | 0x404280a0 | TIMG14_C0 | 9 | IO | d) |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_POCI | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_CTS | 13 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_26 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_VREF+ | (Non-IOMUX 2) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA24 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART2_RX | 2 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS2_MISO2 | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_RTS | 4 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C3N | 5 | O |  |
| 44 | C7 | 29 | 44 | 44 | 28 | 27 | 20 | 19 | 17 | PA24 | TIMG8_C1 | 6 | IO | SDIO (standar |
| 44 | C7 | 29 | 44 | 44 | 28 | 27 | 20 | 19 | 17 | PINCM42 0x404280a4 | TIMG2_C1 | 7 | IO | (standar d) |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RX | 8 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG14_C1 | 9 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_PICO | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SDA | 13 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_3 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA25 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_PICO | 2 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_POCI | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_SCLK | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C3 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C1N | 6 | O |  |
|  |  |  |  |  |  |  |  |  |  | PA25 | TIMA0_C2 | 7 | IO |  |
| 45 | C6 | 30 | 45 | 45 | 29 | 28 | 21 | 20 | 18 | PA25 PINCM43 | UART2_CTS | 8 | I | SDIO (standar |
|  |  |  |  |  |  |  |  |  |  | PINCM43 0x404280a8 | TIMG14_C0 | 9 | IO | d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMG1_C0 | 10 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SDA | 11 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_TX | 12 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA_FAL2 | 13 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SCL | 14 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_2 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA26 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | BEEP | 2 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_POCI | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C0 | 4 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA_FAL0 | 5 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C3N | 6 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG2_C0 | 7 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PA26 | UART2_RTS | 8 | O | SDIO |
| 46 | B7 | 31 | 46 | 46 | 30 | 1 | 22 | 1 | 19 | PINCM44 | I2C0_SCL | 9 | IOD | SDIO (standar |
|  |  |  |  |  |  |  |  |  |  | 0x404280ac | TIMG1_C1 | 10 | IO | d) |
|  |  |  |  |  |  |  |  |  |  |  | UART0_RX | 11 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C0 | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SDA | 13 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_CTS | 14 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_1 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | COMP0_IN0+ | (Non-IOMUX 2) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA27 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS3_CD _MISO3 | 2 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C0N | 3 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C1 | 4 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA_FAL2 | 5 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | CLK_OUT | 6 | O |  |
|  |  |  |  |  |  |  |  |  |  | PA27 | TIMG2_C1 | 7 | IO |  |
| 47 | A7 | 32 | 47 | 47 | 31 | 2 |  | 2 | 20 | PA27 PINCM45 | RTC_OUT | 8 | O | SDIO (standar |
|  |  |  |  |  |  |  |  |  |  | PINCM45 0x404280b0 | UART1_CTS | 9 | I | d) |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SCL | 10 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_TX | 11 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_POCI | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | COMP0_OUT | 14 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_0 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | COMP0_IN0- | (Non-IOMUX 2) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PA28 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_TX | 2 | O |  |
|  |  |  |  |  |  |  |  |  |  | PA28 | I2C0_SDA | 3 | IOD | SDIO |
| 3 | B5 | 3 | 3 | 3 |  |  |  |  |  | PINCM3 | TIMA0_C3 | 4 | IO | (standar |
|  |  |  |  |  |  |  |  |  |  | 0x40428008 | TIMA_FAL0 | 5 | I | d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMG2_C0 | 6 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C1 | 7 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PA29 | PA29 | 1 | IO | SDIO |
|  |  | 23 | 34 |  |  |  |  |  |  | PINCM31 | UART0_RTS | 2 | O | (standar |
|  |  |  |  |  |  |  |  |  |  | 0x40428078 | SPI0_PICO | 3 | IO | d) |
|  |  |  |  |  |  |  |  |  |  |  | PA30 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_RX | 4 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_IDX | 5 | I |  |
| 48 | D5 | 1 | 48 | 48 | 32 | 3 | 23 | 3 | 1 | PA30 | TIMA0_C0 | 6 | IO | SDIO (standar |
| 48 | D5 | 1 | 48 | 48 | 32 | 3 | 23 | 3 | 1 | PINCM46 0x404280b4 | UART1_RTS | 9 | O | (standar d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMG2_C1 | 10 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG14_C2 | 11 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SDA | 12 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | PA31 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PA31 | UART0_RX | 2 | IO | SDIO |
| 5 | B4 |  | 4 | 5 |  |  |  |  |  | PINCM4 | I2C0_SCL | 3 | IOD | (standar |
|  |  |  |  |  |  |  |  |  |  | 0x4042800c | TIMA0_C3N | 4 | O | d) |
|  |  |  |  |  |  |  |  |  |  |  | CLK_OUT | 6 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | PB2 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART2_CTS | 3 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C1_SCL | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C3 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PB2 | UART1_CTS | 6 | I | SDIO |
| 14 | B3 | 10 | 14 | 14 |  |  |  |  |  | PINCM11 | TIMG1_C0 | 7 | IO | (standar |
|  |  |  |  |  |  |  |  |  |  | 0x40428028 | UART2_TX | 8 | O | d) |
|  |  |  |  |  |  |  |  |  |  |  | HFCLKIN | 10 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_PICO | 11 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RX | 12 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C1N | 13 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | PB3 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA_FAL0 | 2 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | UART2_RTS | 3 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C1_SDA | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C3N | 5 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RTS | 6 | O |  |
| 15 | C2 | 11 | 15 | 15 |  |  |  |  | 8 | PB3 | TIMG1_C1 | 7 | IO | SDIO (standar |
| 15 | C2 | 11 | 15 | 15 |  |  |  |  | 8 | PINCM12 0x4042802c | UART2_RX | 8 | I | (standar d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMG2_C1 | 9 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C0 | 10 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_SCLK | 11 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS0 | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_TX | 13 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | RTC_OUT | 14 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | PB6 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_TX | 2 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C0 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART2_CTS | 6 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG1_C0 | 7 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PB6 | TIMA_FAL2 | 8 | I | SDIO |
| 20 | D1 | 16 | 20 | 20 |  |  |  |  |  | PINCM17 0x40428040 | SPI0_CS1_MISO1 | 9 | IO | SDIO (standar d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C3N | 11 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C1 | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C2N | 13 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_TX | 14 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_24 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PB7 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RX | 2 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C1 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART2_RTS | 6 | O |  |
|  |  |  |  |  |  |  |  |  |  | PB7 | TIMG1_C1 | 7 | IO | SDIO |
| 21 | E1 | 17 | 21 | 21 |  |  |  |  |  | PINCM18 0x40428044 | SPI0_CS2_MISO2 | 9 | IO | SDIO (standar d) |
|  |  |  |  |  |  |  |  |  |  |  | BEEP | 12 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_SCLK | 13 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_RX | 14 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_23 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PB8 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_CTS | 2 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C0 | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG1_C0 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PB8 | SPI0_SCLK | 7 | IOD | SDIO |
| 22 | F1 |  | 22 | 22 |  |  |  |  |  | PINCM19 | BEEP | 8 | O | (standar |
|  |  |  |  |  |  |  |  |  |  | 0x40428048 | TIMG8_C0 | 9 | IO | d) |
|  |  |  |  |  |  |  |  |  |  |  | UART0_RX | 10 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_POCI | 11 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SCL | 12 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | COMP0_OUT | 14 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | PB9 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART1_RTS | 2 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C0N | 5 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C1 | 6 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PB9 | TIMG1_C1 | 7 | IO | SDIO |
| 23 | E3 |  | 23 | 23 |  |  |  |  |  | PINCM20 | TIMG2_C0 | 8 | IO | (standar |
|  |  |  |  |  |  |  |  |  |  | 0x4042804c | SPI0_POCI | 10 | IO | d) |
|  |  |  |  |  |  |  |  |  |  |  | UART0_RX | 11 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SCL | 12 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_TX | 13 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SDA | 14 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | PB14 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C0 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_IDX | 6 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS3_CD _MISO3 | 7 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG2_C1 | 8 | IO |  |
| 24 | G1 |  | 24 | 24 |  |  |  |  |  | PB14 | I2C0_SDA | 9 | IOD | SDIO (standar |
| 24 | G1 |  | 24 | 24 |  |  |  |  |  | PINCM21 0x40428050 | SPI0_PICO | 10 | IO | (standar d) |
|  |  |  |  |  |  |  |  |  |  |  | UART0_TX | 11 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA_FAL2 | 12 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA_FAL0 | 13 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG14_C2 | 14 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_21 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PB15 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART2_TX | 2 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C0 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PB15 | TIMG2_C0 | 6 | IO | SDIO |
| 25 | E2 |  | 25 | 25 |  |  |  |  |  | PINCM22 0x40428054 | TIMA0_C1N | 12 | O | (standar d) |
|  |  |  |  |  |  |  |  |  |  | 0x40428054 | UART1_TX | 13 | O | d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMG2_C1 | 14 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_20 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PB16 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART2_RX | 2 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C1 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PB16 | TIMG2_C1 | 6 | IO | SDIO |
| 26 | F2 |  | 26 | 26 |  |  |  |  |  | PINCM23 0x40428058 | TIMA0_C2N | 12 | O | (standar d) |
|  |  |  |  |  |  |  |  |  |  | 0x40428058 | UART1_RX | 13 | I | d) |
|  |  |  |  |  |  |  |  |  |  |  | I2C1_SDA | 14 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_19 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PB17 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART2_TX | 2 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_PICO | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PB17 | I2C0_SCL | 4 | IOD |  |
| 36 | G7 |  |  | 36 |  |  |  |  |  | PB17 PINCM34 | TIMA0_C2 | 5 | IO | SDIO (standar |
|  |  |  |  |  |  |  |  |  |  | PINCM34 0x40428084 | TIMG14_C0 | 6 | IO | d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMG1_C0 | 9 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS0 | 10 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_11 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PB18 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART2_RX | 2 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_SCLK | 3 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | I2C0_SDA | 4 | IOD |  |
|  |  |  |  |  |  |  |  |  |  | PB18 | TIMA0_C2N | 5 | O |  |
| 37 | E5 | 26 | 37 | 37 |  |  |  |  |  | PB18 PINCM35 | TIMG14_C1 | 6 | IO | SDIO (standar |
|  |  |  |  |  |  |  |  |  |  | PINCM35 0x40428088 | SPI0_CS0 | 7 | IO | d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMG1_C1 | 9 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C1 | 12 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_RTS | 13 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_10 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PB19 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_POCI | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C1 | 4 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_CTS | 5 | I |  |
|  |  |  |  |  |  |  |  |  |  | PB19 | TIMG2_C1 | 6 | IO |  |
| 38 | F6 |  | 38 | 38 |  |  |  |  |  | PB19 PINCM36 | TIMG8_IDX | 7 | I | SDIO (standar |
|  |  |  |  |  |  |  |  |  |  | PINCM36 0x4042808c | UART2_CTS | 8 | I | d) |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C1N | 12 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | UART2_RX | 13 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | COMP0_OUT | 14 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_9 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PB20 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS2_MISO2 | 2 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C2 | 5 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA_FAL1 | 6 | I |  |
|  |  |  |  |  |  |  |  |  |  | PB20 | TIMA0_C1 | 7 | IO | SDIO |
| 41 | E7 |  | 41 | 41 |  |  |  |  |  | PINCM39 | UART2_RTS | 8 | O | (standar |
|  |  |  |  |  |  |  |  |  |  | 0x40428098 | I2C0_SDA | 9 | IOD | d) |
|  |  |  |  |  |  |  |  |  |  |  | UART1_CTS | 12 | I |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMA0_C2N | 13 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG8_C1 | 14 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_6 | (Non-IOMUX 1) 0 | A |  |
|  |  |  |  |  |  |  |  |  |  |  | PB24 | 1 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS3_CD _MISO3 | 2 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_CS1_MISO1 | 3 | IO |  |
|  |  |  |  |  |  |  |  |  |  | PB24 | TIMA0_C3 | 5 | IO | SDIO |
| 42 | D6 | 27 | 42 | 42 |  |  |  |  |  | PINCM40 | TIMA0_C1N | 6 | O | SDIO (standar |
|  |  |  |  |  |  |  |  |  |  | 0x4042809c | UART2_RTS | 8 | O | d) |
|  |  |  |  |  |  |  |  |  |  |  | SPI0_SCLK | 12 | IOD |  |
|  |  |  |  |  |  |  |  |  |  |  | TIMG14_C2 | 13 | IO |  |
|  |  |  |  |  |  |  |  |  |  |  | UART0_RTS | 14 | O |  |
|  |  |  |  |  |  |  |  |  |  |  | ADC0_5 | (Non-IOMUX 1) 0 | A |  |
| 6 | A6 | 4 | 6 | 6 | 4 | 7 | 3 | 6 | 4 | VDD | VDD | (Non-IOMUX 1) 0 | PWR | PWR |
| 7 | A5 | 5 | 7 | 7 | 5 | 8 | 4 | 7 | 5 | VSS | VSS | (Non-IOMUX 1) 0 | PWR | PWR |

### 6.3 Signal Descriptions (p. 28)

**Table 6-3. Analog to Digital Converter (ADC) Signal Descriptions (pp. 28-29)**

| SIGNAL NAME | PIN TYPE | DESCRIPTION | PT (A) PIN | ZCM PIN | VFC PIN | PT (B) PIN | RGZ PIN | RHB PIN | RGE PIN | DGS2 0 PIN | RUK PIN |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| ADC0_VREF+ | A | ADC0 voltage reference (VREF) power supply | 43 | D7 | 28 | 43 | 43 | 27 | 19 | 18 | 16 |
| ADC0_VREF- | A | ADC0 voltage reference (VREF) ground supply | 39 | E6 |  | 39 | 39 | 25 | 17 |  |  |
| ADC0_0 | A | ADC0 analog input channel 0 | 47 | A7 | 32 | 47 | 47 | 31 |  | 2 | 20 |
| ADC0_1 | A | ADC0 analog input channel 1 | 46 | B7 | 31 | 46 | 46 | 30 | 22 | 1 | 19 |
| ADC0_2 | A | ADC0 analog input channel 2 | 45 | C6 | 30 | 45 | 45 | 29 | 21 | 20 | 18 |
| ADC0_3 | A | ADC0 analog input channel 3 | 44 | C7 | 29 | 44 | 44 | 28 | 20 | 19 | 17 |
| ADC0_4 | A | ADC0 analog input channel 4 | 35 | G6 | 25 | 36 | 35 | 24 | 16 | 16 | 14 |
| ADC0_5 | A | ADC0 analog input channel 5 | 42 | D6 | 27 | 42 | 42 |  |  |  |  |
| ADC0_6 | A | ADC0 analog input channel 6 | 41 | E7 |  | 41 | 41 |  |  |  |  |
| ADC0_7 | A | ADC0 analog input channel 7 | 40 | F7 |  | 40 | 40 | 26 | 18 | 17 | 15 |
| ADC0_8 | A | ADC0 analog input channel 8 | 39 | E6 |  | 39 | 39 | 25 | 17 |  |  |
| ADC0_9 | A | ADC0 analog input channel 9 | 38 | F6 |  | 38 | 38 |  |  |  |  |
| ADC0_10 | A | ADC0 analog input channel 10 | 37 | E5 | 26 | 37 | 37 |  |  |  |  |
| ADC0_11 | A | ADC0 analog input channel 11 | 36 | G7 |  |  | 36 |  |  |  |  |
| ADC0_12 | A | ADC0 analog input channel 12 | 33 | F5 | 22 | 33 | 33 | 22 | 14 | 14 | 12 |
| ADC0_13 | A | ADC0 analog input channel 13 | 32 | G5 | 21 | 32 | 32 | 21 | 13 | 13 | 11 |
| ADC0_14 | A | ADC0 analog input channel 14 | 31 | F4 |  | 31 | 31 | 20 | 12 | 12 | 10 |
| ADC0_15 | A | ADC0 analog input channel 15 | 30 | G4 | 20 | 30 | 30 | 19 | 11 |  |  |
| ADC0_16 | A | ADC0 analog input channel 16 | 29 | F3 | 19 | 29 | 29 | 18 |  |  |  |
| ADC0_17 | A | ADC0 analog input channel 17 | 28 | G3 | 18 | 28 | 28 | 17 |  |  |  |
| ADC0_18 | A | ADC0 analog input channel 18 | 27 | G2 |  | 27 | 27 | 16 |  |  |  |
| ADC0_19 | A | ADC0 analog input channel 19 | 26 | F2 |  | 26 | 26 |  |  |  |  |
| ADC0_20 | A | ADC0 analog input channel 20 | 25 | E2 |  | 25 | 25 |  |  |  |  |
| ADC0_21 | A | ADC0 analog input channel 21 | 24 | G1 |  | 24 | 24 |  |  |  |  |
| ADC0_22 | A | ADC0 analog input channel 22 | 34 | B6 | 24 | 35 | 34 | 23 | 15 | 15 | 13 |
| ADC0_23 | A | ADC0 analog input channel 23 | 21 | E1 | 17 | 21 | 21 |  |  |  |  |
| ADC0_24 | A | ADC0 analog input channel 24 | 20 | D1 | 16 | 20 | 20 |  |  |  |  |
| ADC0_25 | A | ADC0 analog input channel 25 | 19 | E4 | 15 | 19 | 19 | 15 | 10 | 11 | 9 |
| ADC0_26 | A | ADC0 analog input channel 26 | 43 | D7 | 28 | 43 | 43 | 27 | 19 | 18 | 16 |

**Table 6-4. Flash Bootstrap Loader (BSL) Signal Descriptions (p. 29)**

| SIGNAL NAME | PIN TYPE | DESCRIPTION | PT (A) PIN | ZCM PIN | VFC PIN | PT (B) PIN | RGZ PIN | RHB PIN | DGS28 PIN | RGE PIN | DGS20 PIN | RUK PIN |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| BSL_invoke (Flash) | I | Default Flash BSL invoke signal | 33 | F5 | 22 | 33 | 33 | 22 | 21 | 14 | 14 | 12 |

Table 6-5. Clock Module (CKM) Signal Descriptions PT PT SIGNAL PIN ZCM VFC RGZ RHB DGS2 RGE DGS2 RUK DESCRIPTION (A) (B) NAME TYPE PIN PIN PIN PIN 8 PIN PIN 0 PIN PIN PIN PIN

13 A7 13 13 13 11 14 14 14 12 ,,,,,,,,,, 17 B2 14 17 17 13 15 18 8 17 2 15 20 ,,,,,,,,,,, 18 B4 19 18 18 14 17 2 9 ,,,,,,,, O CLK_OUT digital clock output 29, D2, 22, 29, 29, 18, 21, 25 CLK_OUT from the PMCU 33 D3 32 9 33 4 33 22 ,,,,,,, 40 F3 40 47 40 26 31 ,,,,, 47 5 F5 47 5 ,,, F7

**Table 6-5. Clock Module (CKM) Signal Descriptions (continued) (p. 30)**

| SIGNAL NAME | PIN TYPE | DESCRIPTION | PT (A) PIN | ZCM PIN | VFC PIN | PT (B) PIN | RGZ PIN | RHB PIN | DGS2 8 PIN | RGE PIN | DGS2 0 PIN | RUK PIN |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| FCC_IN | I | Frequency clock counter (FCC) input signal | 1 11 ,, 13 , 27 31 , | B1 , B2 , D4 , F4 , G2 | 7 9 , | 1 11 ,, 13 , 27 31 , | 1 11 ,, 13 , 27 31 , | 1 11 ,, 16 , 20 9 , | 12 , 19 4 , | 12 24 , | 12 4 , | 10 2 , |
| HFCLKIN | I | High frequency clock digital clock input signal | 10 , 12 , 14 , 16 2 , | A2 , B3 , C1 , C4 , C5 | 10 , 12 2 ,, 8 | 12 , 14 , 16 2 ,, 9 | 10 , 12 , 14 , 16 2 , | 10 , 12 2 ,, 8 | 11 , 13 5 , | 1 7 , | 10 5 ,, 9 | 3 7 , |
| HFXIN | A | High frequency crystal oscillator (HFXT) signal | 11 | B1 | 7 | 11 | 11 | 9 | 12 |  |  |  |
| HFXOUT | A | High frequency crystal oscillator (HFXT) signal | 12 | C1 | 8 | 12 | 12 | 10 | 13 |  | 10 |  |
| LFCLKIN | I | Low frequency clock digital clock input signal | 10 | A2 |  | 9 | 10 | 8 | 11 | 7 | 9 | 7 |
| LFXIN | A | Low frequency crystal oscillator (LFXT) signal | 9 | A3 |  | 8 | 9 | 7 | 10 | 6 |  |  |
| LFXOUT | A | Low frequency crystal oscillator (LFXT) signal | 10 | A2 |  | 9 | 10 | 8 | 11 | 7 | 9 | 7 |

**Table 6-6. Comparator (COMP) Signal Descriptions (p. 30)**

| SIGNAL NAME | PIN TYPE | DESCRIPTION | PT (A) PIN | ZCM PIN | VFC PIN | PT (B) PIN | RGZ PIN | RHB PIN | DGS2 8 PIN | RGE PIN | DGS2 0 PIN | RUK PIN |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| COMP0_DAC_OUT | A | COMP0 DAC output | 19 | E4 | 15 | 19 | 19 | 15 | 16 | 10 | 11 | 9 |
| COMP0_OUT | O | COMP0 output | 13 , 19 , 22 , 27 , 31 , 38 , 47 9 , | A3 , A7 , B2 , E4 , F1 , F4 , F6 , G2 | 15 , 32 9 , | 13 , 19 , 22 , 27 , 31 , 38 , 47 8 , | 13 , 19 , 22 , 27 , 31 , 38 , 47 9 , | 11 , 15 , 16 , 20 , 31 7 , | 10 , 16 , 19 2 , | 10 , 12 6 , | 11 , 12 2 , | 10 , 20 9 , |
| COMP0_IN0+ | A | COMP0 non-inverting input 0 | 46 | B7 | 31 | 46 | 46 | 30 | 1 | 22 | 1 | 19 |
| COMP0_IN0- | A | COMP0 inverting input 0 | 47 | A7 | 32 | 47 | 47 | 31 | 2 |  | 2 | 20 |
| COMP0_IN1+ | A | COMP0 non-inverting input 1 | 33 | F5 | 22 | 33 | 33 | 22 | 21 | 14 | 14 | 12 |
| COMP0_IN1- | A | COMP0 inverting input 1 | 32 | G5 | 21 | 32 | 32 | 21 | 20 | 13 | 13 | 11 |
| COMP0_IN2+ | A | COMP0 non-inverting input 2 | 29 | F3 | 19 | 29 | 29 | 18 | 17 |  |  |  |
| COMP0_IN2- | A | COMP0 inverting input 2 | 28 | G3 | 18 | 28 | 28 | 17 |  |  |  |  |
| COMP0_IN3+ | A | COMP0 non-inverting input 3 | 30 | G4 | 20 | 30 | 30 | 19 | 18 | 11 |  |  |

**Table 6-7. General Purpose Input Output Module Signal Descriptions (pp. 30-31)**

| SIGNAL NAME | PIN TYPE | DESCRIPTION | PT (A) PIN | ZCM PIN | VFC PIN | PT (B) PIN | RGZ PIN | RHB PIN | DGS2 8 PIN | RGE PIN | DGS2 0 PIN | RUK PIN |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| PA0 | IO | GPIO port A input/output 0 | 1 | D4 |  | 1 | 1 | 1 | 4 | 24 | 4 | 2 |
| PA1 | IO | GPIO port A input/output 1 | 2 | C5 | 2 | 2 | 2 | 2 | 5 | 1 | 5 | 3 |
| PA2 | IO | GPIO port A input/output 2 | 8 | A4 |  |  | 8 | 6 | 9 | 5 | 8 | 6 |
| PA3 | IO | GPIO port A input/output 3 | 9 | A3 |  | 8 | 9 | 7 | 10 | 6 |  |  |
| PA4 | IO | GPIO port A input/output 4 | 10 | A2 |  | 9 | 10 | 8 | 11 | 7 | 9 | 7 |
| PA5 | IO | GPIO port A input/output 5 | 11 | B1 | 7 | 11 | 11 | 9 | 12 |  |  |  |
| PA6 | IO | GPIO port A input/output 6 | 12 | C1 | 8 | 12 | 12 | 10 | 13 |  | 10 |  |
| PA7 | IO | GPIO port A input/output 7 | 13 | B2 | 9 | 13 | 13 | 11 |  |  |  |  |
| PA8 | IO | GPIO port A input/output 8 | 16 | C4 | 12 | 16 | 16 | 12 |  |  |  |  |
| PA9 | IO | GPIO port A input/output 9 | 17 | D3 | 13 | 17 | 17 | 13 | 14 | 8 |  |  |
| PA10 | IO | GPIO port A input/output 10 | 18 | D2 | 14 | 18 | 18 | 14 | 15 | 9 |  |  |
| PA11 | IO | GPIO port A input/output 11 | 19 | E4 | 15 | 19 | 19 | 15 | 16 | 10 | 11 | 9 |
| PA12 | IO | GPIO port A input/output 12 | 27 | G2 |  | 27 | 27 | 16 |  |  |  |  |
| PA13 | IO | GPIO port A input/output 13 | 28 | G3 | 18 | 28 | 28 | 17 |  |  |  |  |
| PA14 | IO | GPIO port A input/output 14 | 29 | F3 | 19 | 29 | 29 | 18 | 17 |  |  |  |
| PA15 | IO | GPIO port A input/output 15 | 30 | G4 | 20 | 30 | 30 | 19 | 18 | 11 |  |  |
| PA16 | IO | GPIO port A input/output 16 | 31 | F4 |  | 31 | 31 | 20 | 19 | 12 | 12 | 10 |
| PA17 | IO | GPIO port A input/output 17 | 32 | G5 | 21 | 32 | 32 | 21 | 20 | 13 | 13 | 11 |
| PA18 | IO | GPIO port A input/output 18 | 33 | F5 | 22 | 33 | 33 | 22 | 21 | 14 | 14 | 12 |
| PA19 | IO | GPIO port A input/output 19 | 34 | B6 | 24 | 35 | 34 | 23 | 22 | 15 | 15 | 13 |
| PA20 | IO | GPIO port A input/output 20 | 35 | G6 | 25 | 36 | 35 | 24 | 23 | 16 | 16 | 14 |
| PA21 | IO | GPIO port A input/output 21 | 39 | E6 |  | 39 | 39 | 25 | 24 | 17 |  |  |
| PA22 | IO | GPIO port A input/output 22 | 40 | F7 |  | 40 | 40 | 26 | 25 | 18 | 17 | 15 |
| PA23 | IO | GPIO port A input/output 23 | 43 | D7 | 28 | 43 | 43 | 27 | 26 | 19 | 18 | 16 |
| PA24 | IO | GPIO port A input/output 24 | 44 | C7 | 29 | 44 | 44 | 28 | 27 | 20 | 19 | 17 |
| PA25 | IO | GPIO port A input/output 25 | 45 | C6 | 30 | 45 | 45 | 29 | 28 | 21 | 20 | 18 |
| PA26 | IO | GPIO port A input/output 26 | 46 | B7 | 31 | 46 | 46 | 30 | 1 | 22 | 1 | 19 |
| PA27 | IO | GPIO port A input/output 27 | 47 | A7 | 32 | 47 | 47 | 31 | 2 |  | 2 | 20 |
| PA28 | IO | GPIO port A input/output 28 | 3 | B5 | 3 | 3 | 3 |  |  |  |  |  |
| PA29 | IO | GPIO port A input/output 29 |  |  | 23 | 34 |  |  |  |  |  |  |
| PA30 | IO | GPIO port A input/output 30 | 48 | D5 | 1 | 48 | 48 | 32 | 3 | 23 | 3 | 1 |
| PA31 | IO | GPIO port A input/output 31 | 5 | B4 |  | 4 | 5 |  |  |  |  |  |
| PB2 | IO | GPIO port B input/output 2 | 14 | B3 | 10 | 14 | 14 |  |  |  |  |  |
| PB3 | IO | GPIO port B input/output 3 | 15 | C2 | 11 | 15 | 15 |  |  |  |  | 8 |
| PB6 | IO | GPIO port B input/output 6 | 20 | D1 | 16 | 20 | 20 |  |  |  |  |  |
| PB7 | IO | GPIO port B input/output 7 | 21 | E1 | 17 | 21 | 21 |  |  |  |  |  |
| PB8 | IO | GPIO port B input/output 8 | 22 | F1 |  | 22 | 22 |  |  |  |  |  |
| PB9 | IO | GPIO port B input/output 9 | 23 | E3 |  | 23 | 23 |  |  |  |  |  |
| PB14 | IO | GPIO port B input/output 14 | 24 | G1 |  | 24 | 24 |  |  |  |  |  |
| PB15 | IO | GPIO port B input/output 15 | 25 | E2 |  | 25 | 25 |  |  |  |  |  |
| PB16 | IO | GPIO port B input/output 16 | 26 | F2 |  | 26 | 26 |  |  |  |  |  |
| PB17 | IO | GPIO port B input/output 17 | 36 | G7 |  |  | 36 |  |  |  |  |  |
| PB18 | IO | GPIO port B input/output 18 | 37 | E5 | 26 | 37 | 37 |  |  |  |  |  |
| PB19 | IO | GPIO port B input/output 19 | 38 | F6 |  | 38 | 38 |  |  |  |  |  |
| PB20 | IO | GPIO port B input/output 20 | 41 | E7 |  | 41 | 41 |  |  |  |  |  |
| PB24 | IO | GPIO port B input/output 24 | 42 | D6 | 27 | 42 | 42 |  |  |  |  |  |

**Table 6-8. I2C Signal Descriptions (p. 32)**

| SIGNAL NAME | PIN TYPE | DESCRIPTION | PT (A) PIN | ZCM PIN | VFC PIN | PT (B) PIN | RGZ PIN | RHB PIN | DGS2 8 PIN | RGE PIN | DGS2 0 PIN | RUK PIN |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| I2C0_SCL | IOD | I2C0 serial clock signal (SCL) | 17 , 19 2 ,, 22 , 23 , 29 , 36 , 40 , 45 , 46 , 47 5 ,, 8 | A4 , A7 , B4 , B7 , C5 , C6 , D3 , E3 , E4 , F1 , F3 , F7 , G7 | 13 , 15 , 19 2 ,, 30 , 31 32 , | 17 , 19 2 ,, 22 , 23 , 29 4 ,, 40 , 45 , 46 47 , | 17 , 19 2 ,, 22 , 23 , 29 , 36 , 40 , 45 , 46 , 47 5 ,, 8 | 13 , 15 , 18 2 ,, 26 , 29 , 30 , 31 6 , | 1 14 ,, 16 , 17 2 ,, 25 , 28 5 ,, 9 | 1 10 ,, 18 , 21 , 22 5 ,, 8 | 1 11 ,, 17 2 ,, 20 5 ,, 8 | 15 , 18 , 19 , 20 3 ,, 6 9 , |
| I2C0_SDA | IOD | I2C0 serial data signal (SDA) | 1 12 ,, 16 , 18 2 ,, 23 , 24 3 ,, 32 , 37 , 41 , 44 , 45 , 46 , 48 9 , | A3 , B5 , B7 , C1 , C4 , C5 , C6 , C7 , D2 , D4 , D5 , E3 , E5 , E7 , G1 , G5 | 1 12 ,, 14 2 ,, 21 , 26 , 29 3 ,, 30 , 31 8 , | 1 12 ,, 16 , 18 2 ,, 23 , 24 3 ,, 32 , 37 , 41 , 44 , 45 , 46 , 48 8 , | 1 12 ,, 16 , 18 2 ,, 23 , 24 3 ,, 32 , 37 , 41 , 44 , 45 , 46 , 48 9 , | 1 10 ,, 12 , 14 2 ,, 21 , 28 , 29 , 30 , 32 7 , | 1 10 ,, 13 , 15 , 20 , 27 , 28 3 ,, 4 5 , | 1 13 ,, 20 , 21 , 22 , 23 , 24 6 ,, 9 | 1 10 ,, 13 , 19 , 20 3 ,, 4 5 , | 1 11 ,, 17 , 18 , 19 2 ,, 3 |
| I2C1_SCL | IOD | I2C1 serial clock signal (SCL) | 10 , 12 , 14 , 19 , 27 , 30 , 32 35 , | A2 , B3 , C1 , E4 , G2 , G4 , G5 , G6 | 10 , 15 , 20 , 21 , 25 8 , | 12 , 14 , 19 , 27 , 30 , 32 , 36 9 , | 10 , 12 , 14 , 19 , 27 , 30 , 32 35 , | 10 , 15 , 16 , 19 , 21 , 24 8 , | 11 , 13 , 16 , 18 , 20 23 , | 10 , 11 , 13 , 16 7 , | 10 , 11 , 13 , 16 9 , | 11 , 14 7 ,, 9 |
| I2C1_SDA | IOD | I2C1 serial data signal (SDA) | 11 , 15 , 18 , 26 , 31 , 33 , 34 9 , | A3 , B1 , B6 , C2 , D2 , F2 , F4 , F5 | 11 , 14 , 22 , 24 7 , | 11 , 15 , 18 , 26 , 31 , 33 , 35 8 , | 11 , 15 , 18 , 26 , 31 , 33 , 34 9 , | 14 , 20 , 22 , 23 7 ,, 9 | 10 , 12 , 15 , 19 , 21 22 , | 12 , 14 , 15 6 ,, 9 | 12 , 14 15 , | 10 , 12 , 13 8 , |

**Table 6-9. Real-time Clock (RTC) Signal Descriptions (p. 32)**

| SIGNAL NAME | PIN TYPE | DESCRIPTION | PT (A) PIN | ZCM PIN | VFC PIN | PT (B) PIN | RGZ PIN | RHB PIN | DGS2 8 PIN | RGE PIN | DGS2 0 PIN | RUK PIN |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  | 1 15 ,, | A7 , | 11 , | 1 15 ,, | 1 15 ,, | 1 13 ,, | 14 2 ,, | 24 8 , | 2 4 , | 2 20 ,, |
|  |  |  | ,, | , | , | ,, | ,, | ,, | ,, | , | , | ,, |
|  |  |  | 17 , | C2 , | 13 , | 17 , | 17 , | 17 31 , | 4 |  |  | 8 |
|  |  |  | , | , | , | , | , | , |  |  |  |  |
| RTC_OUT | O | Real-time clock output signal | 28 47 , | D3 , | 18 32 , | 28 47 , | 28 47 , |  |  |  |  |  |
|  |  |  |  | D4 , |  |  |  |  |  |  |  |  |
|  |  |  |  | , |  |  |  |  |  |  |  |  |
|  |  |  |  | G3 |  |  |  |  |  |  |  |  |

**Table 6-10. Serial Peripheral Interface (SPI) Signal Descriptions (pp. 33-34)**

| SIGNAL NAME | PIN TYPE | DESCRIPTION | PT (A) PIN | ZCM PIN | VFC PIN | PT (B) PIN | RGZ PIN | RHB PIN | DGS2 8 PIN | RGE PIN | DGS2 0 PIN | RUK PIN |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| SPI0_PICO | IO | SPI0 peripheral in controller out signal | 11 , 13 , 14 , 17 , 18 , 24 , 29 , 33 , 34 , 35 , 36 , 44 45 , | B1 , B2 , B3 , B6 , C6 , C7 , D2 , D3 , F3 , F5 , G1 , G6 , G7 | 10 , 13 , 14 , 19 , 22 , 23 , 24 , 25 , 29 , 30 7 ,, 9 | 11 , 13 , 14 , 17 , 18 , 24 , 29 , 33 , 34 , 35 , 36 , 44 45 , | 11 , 13 , 14 , 17 , 18 , 24 , 29 , 33 , 34 , 35 , 36 , 44 45 , | 11 , 13 , 14 , 18 , 22 , 23 , 24 , 28 , 29 9 , | 12 , 14 , 15 , 17 , 21 , 22 , 23 , 27 28 , | 14 , 15 , 16 , 20 , 21 8 ,, 9 | 14 , 15 , 16 , 19 20 , | 12 , 13 , 14 , 17 18 , |
| SPI0_POCI | IO | SPI0 peripheral out controller in signal | 10 , 13 , 17 , 18 , 22 , 23 , 28 , 31 , 33 , 34 , 38 , 40 , 43 , 45 , 46 47 , | A2 , A7 , B2 , B6 , B7 , C6 , D2 , D3 , D7 , E3 , F1 , F4 , F5 , F6 , F7 , G3 | 13 , 14 , 18 , 22 , 24 , 28 , 30 , 31 , 32 9 , | 13 , 17 , 18 , 22 , 23 , 28 , 31 , 33 , 35 , 38 , 40 , 43 , 45 , 46 , 47 9 , | 10 , 13 , 17 , 18 , 22 , 23 , 28 , 31 , 33 , 34 , 38 , 40 , 43 , 45 , 46 47 , | 11 , 13 , 14 , 17 , 20 , 22 , 23 , 26 , 27 , 29 , 30 , 31 8 , | 1 11 ,, 14 , 15 , 19 2 ,, 21 , 22 , 25 , 26 28 , | 12 , 14 , 15 , 18 , 19 , 21 , 22 7 ,, 8 9 , | 1 12 ,, 14 , 15 , 17 , 18 2 ,, 20 9 , | 10 , 12 , 13 , 15 , 16 , 18 , 19 , 20 7 , |
| SPI0_SCLK | IOD | SPI0 serial clock | 12 , 15 , 16 , 19 , 21 , 22 , 27 , 32 , 34 , 37 , 42 45 , | B6 , C1 , C2 , C4 , C6 , D6 , E1 , E4 , E5 , F1 , G2 , G5 | 11 , 12 , 15 , 17 , 21 , 24 , 26 , 27 , 30 8 , | 12 , 15 , 16 , 19 , 21 , 22 , 27 , 32 , 35 , 37 , 42 45 , | 12 , 15 , 16 , 19 , 21 , 22 , 27 , 32 , 34 , 37 , 42 45 , | 10 , 12 , 15 , 16 , 21 , 23 29 , | 13 , 16 , 20 , 22 28 , | 10 , 13 , 15 21 , | 10 , 11 , 13 , 15 20 , | 11 , 13 , 18 8 ,, 9 |
| SPI0_CS0 | IO | SPI0 chip-select 0 signal | 10 , 15 , 16 , 19 , 28 , 33 , 35 , 36 , 37 8 , | A2 , A4 , C2 , C4 , E4 , E5 , F5 , G3 , G6 , G7 | 11 , 12 , 15 , 18 , 22 , 25 26 , | 15 , 16 , 19 , 28 , 33 , 36 , 37 9 , | 10 , 15 , 16 , 19 , 28 , 33 , 35 , 36 , 37 8 , | 12 , 15 , 17 , 22 , 24 6 ,, 8 | 11 , 16 , 21 , 23 9 , | 10 , 14 , 16 5 ,, 7 | 11 , 14 , 16 8 ,, 9 | 12 , 14 6 ,, 7 8 9 ,, |
| SPI0_CS1_MISO1 | IO |  | 1 20 ,, 27 , 32 , 42 9 , | A3 , D1 , D4 , D6 , G2 , G5 | 16 , 21 27 , | 1 20 ,, 27 , 32 , 42 8 , | 1 20 ,, 27 , 32 , 42 9 , | 1 16 ,, 21 7 , | 10 , 20 4 , | 13 , 24 6 , | 13 4 , | 11 2 , |
| SPI0_CS2_MISO2 | IO |  | 13 , 21 , 29 , 30 , 40 , 41 44 , | B2 , C7 , E1 , E7 , F3 , F7 , G4 | 17 , 19 , 20 , 29 9 , | 13 , 21 , 29 , 30 , 40 , 41 44 , | 13 , 21 , 29 , 30 , 40 , 41 44 , | 11 , 18 , 19 , 26 28 , | 17 , 18 , 25 27 , | 11 , 18 20 , | 17 19 , | 15 17 , |
| SPI0_CS3_CD_MISO3 | IO |  | 16 2 ,, 24 , 28 , 39 , 42 , 43 , 47 9 , | A3 , A7 , C4 , C5 , D6 , D7 , E6 , G1 , G3 | 12 , 18 2 ,, 27 , 28 32 , | 16 2 ,, 24 , 28 , 39 , 42 , 43 , 47 8 , | 16 2 ,, 24 , 28 , 39 , 42 , 43 , 47 9 , | 12 , 17 2 ,, 25 , 27 , 31 7 , | 10 2 ,, 24 , 26 5 , | 1 17 ,, 19 6 , | 18 2 ,, 5 | 16 , 20 3 , |

**Table 6-11. Serial Wire Debug (SWD) Signal Descriptions (p. 34)**

| SIGNAL NAME | PIN TYPE | DESCRIPTION | PT (A) PIN | ZCM PIN | VFC PIN | PT (B) PIN | RGZ PIN | RHB PIN | DGS2 8 PIN | RGE PIN | DGS2 0 PIN | RUK PIN |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| SWCLK | I | Serial wire debug interface clock input signal | 35 | G6 | 25 | 36 | 35 | 24 | 23 | 16 | 16 | 14 |
| SWDIO | IO | Serial wire debug interface data input/output signal | 34 | B6 | 24 | 35 | 34 | 23 | 22 | 15 | 15 | 13 |

**Table 6-12. System Controller (SYSCTL) Signal Descriptions (p. 34)**

| SIGNAL NAME | PIN TYPE | DESCRIPTION | PT (A) PIN | ZCM PIN | VFC PIN | PT (B) PIN | RGZ PIN | RHB PIN | DGS2 8 PIN | RGE PIN | DGS2 0 PIN | RUK PIN |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| BEEP | O | Beep output | 1 12 ,, 21 , 22 46 , | B7 , C1 , D4 , E1 , F1 | 17 , 31 8 , | 1 12 ,, 21 , 22 46 , | 1 12 ,, 21 , 22 46 , | 1 10 ,, 30 | 1 13 ,, 4 | 22 24 , | 1 10 ,, 4 | 19 2 , |
| NRST | RESET | Active-low reset signal (must be logic high for the device to start) | 4 | A1 | 6 | 10 | 4 | 3 | 6 | 2 | 5 | 3 |
| VDD | PWR | VDD supply | 6 | A6 | 4 | 6 | 6 | 4 | 7 | 3 | 6 | 4 |
| VSS | PWR | VSS (ground) | 7 | A5 | 5 | 7 | 7 | 5 | 8 | 4 | 7 | 5 |

**Table 6-13. Timer (TIMx) Signal Descriptions (pp. 34-38)**

| SIGNAL NAME | PIN TYPE | DESCRIPTION | PT (A) PIN | ZCM PIN | VFC PIN | PT (B) PIN | RGZ PIN | RHB PIN | DGS2 8 PIN | RGE PIN | DGS2 0 PIN | RUK PIN |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  | 1 15 ,, | A4 , | 1 11 ,, | 1 15 ,, | 1 15 ,, | 1 12 ,, | 1 21 ,, | 14 , | 1 14 ,, | 1 12 ,, |
|  |  |  | ,, | , | ,, | ,, | ,, | ,, | ,, | , | ,, | ,, |
|  |  |  | 16 , | B7 , | 12 , | 16 , | 16 , | 22 , | 23 , | 16 , | 16 3 ,, | 14 , |
|  |  |  | , | , | , | , | , | , | , | , | ,, | , |
|  |  |  | 22 , | C2 , | 22 , | 22 , | 22 , | 24 , | 24 3 ,, | 17 , | 4 8 , | 19 2 ,, |
|  |  |  | , | , | , | , | , | , | ,, | , | , | ,, |
|  |  |  | 24 , | C4 , | 25 31 , | 24 , | 24 , | 25 , | 4 9 , | 22 , |  | 6 8 , |
|  |  |  | , | , | , | , | , | , | , | , |  | , |
|  |  |  | 33 , | D4 , |  | 33 , | 33 , | 30 , |  | 23 , |  |  |
|  |  | TIMA0 capture/compare 0 | , | , |  | , | , | , |  | , |  |  |
| TIMA0_C0 | IO |  | 35 , | D5 , |  | 36 , | 35 , | 32 6 , |  | 24 5 , |  |  |
|  |  | signal | , | , |  | , | , | , |  | , |  |  |
|  |  |  | 39 , | E6 , |  | 39 , | 39 , |  |  |  |  |  |
|  |  |  | , | , |  | , | , |  |  |  |  |  |
|  |  |  | 46 , | F1 , |  | 46 48 , | 46 , |  |  |  |  |  |
|  |  |  | , | , |  | , | , |  |  |  |  |  |
|  |  |  | 48 8 , | F5 , |  |  | 48 8 , |  |  |  |  |  |
|  |  |  | , | , |  |  | , |  |  |  |  |  |
|  |  |  |  | G1 , |  |  |  |  |  |  |  |  |
|  |  |  |  | , |  |  |  |  |  |  |  |  |
|  |  |  |  | G6 |  |  |  |  |  |  |  |  |
| TIMA0_C1 | IO | TIMA0 capture/compare 1 signal | 11 , 13 , 17 2 ,, 23 , 28 3 ,, 37 , 40 , 41 9 , | A3 , B1 , B2 , B5 , C5 , D3 , E3 , E5 , E7 , F7 , G3 | 13 , 18 2 ,, 26 3 ,, 7 9 , | 11 , 13 , 17 2 ,, 23 , 28 3 ,, 37 , 40 , 41 8 , | 11 , 13 , 17 2 ,, 23 , 28 3 ,, 37 , 40 , 41 9 , | 11 , 13 , 17 2 ,, 26 7 ,, 9 | 10 , 12 , 14 , 25 5 , | 1 18 ,, 6 8 , | 17 5 , | 15 3 , |
| TIMA0_C2 | IO | TIMA0 capture/compare 2 signal | 12 , 13 , 18 , 29 , 30 , 32 , 34 , 36 , 41 , 45 9 , | A3 , B2 , B6 , C1 , C6 , D2 , E7 , F3 , G4 , G5 , G7 | 14 , 19 , 20 , 21 , 24 , 30 8 ,, 9 | 12 , 13 , 18 , 29 , 30 , 32 , 35 , 41 , 45 8 , | 12 , 13 , 18 , 29 , 30 , 32 , 34 , 36 , 41 , 45 9 , | 10 , 11 , 14 , 18 , 19 , 21 , 23 , 29 7 , | 10 , 13 , 15 , 17 , 18 , 20 , 22 28 , | 11 , 13 , 15 , 21 6 ,, 9 | 10 , 13 , 15 20 , | 11 , 13 18 , |
| TIMA0_C3 | IO | TIMA0 capture/compare 3 signal | 10 , 14 , 27 3 ,, 32 , 42 , 43 45 , | A2 , B3 , B5 , C6 , D6 , D7 , G2 , G5 | 10 , 21 , 27 , 28 3 ,, 30 | 14 , 27 3 ,, 32 , 42 , 43 , 45 9 , | 10 , 14 , 27 3 ,, 32 , 42 , 43 45 , | 16 , 21 , 27 , 29 8 , | 11 , 20 , 26 28 , | 13 , 19 , 21 7 , | 13 , 18 , 20 9 , | 11 , 16 , 18 7 , |
| TIMA0_C0N | O | TIMA0 capture/compare 0 complementary output | 10 , 17 , 23 , 32 , 39 , 40 47 , | A2 , A7 , D3 , E3 , E6 , F7 , G5 | 13 , 21 32 , | 17 , 23 , 32 , 39 , 40 , 47 9 , | 10 , 17 , 23 , 32 , 39 , 40 47 , | 13 , 21 , 25 , 26 , 31 8 , | 11 , 14 2 ,, 20 , 24 25 , | 13 , 17 , 18 7 ,, 8 | 13 , 17 2 ,, 9 | 11 , 15 , 20 7 , |
| TIMA0_C1N | O | TIMA0 capture/compare 1 complementary output | 10 , 14 , 18 , 25 , 33 , 38 , 42 45 , | A2 , B3 , C6 , D2 , D6 , E2 , F5 , F6 | 10 , 14 , 22 , 27 30 , | 14 , 18 , 25 , 33 , 38 , 42 , 45 9 , | 10 , 14 , 18 , 25 , 33 , 38 , 42 45 , | 14 , 22 , 29 8 , | 11 , 15 , 21 28 , | 14 , 21 7 ,, 9 | 14 , 20 9 , | 12 , 18 7 , |
| TIMA0_C2N | O | TIMA0 capture/compare 2 complementary output | 12 , 19 , 20 , 26 , 28 , 31 , 35 , 37 , 41 8 , | A4 , C1 , D1 , E4 , E5 , E7 , F2 , F4 , G3 , G6 | 15 , 16 , 18 , 25 , 26 8 , | 12 , 19 , 20 , 26 , 28 , 31 , 36 , 37 41 , | 12 , 19 , 20 , 26 , 28 , 31 , 35 , 37 , 41 8 , | 10 , 15 , 17 , 20 , 24 6 , | 13 , 16 , 19 , 23 9 , | 10 , 12 , 16 5 , | 10 , 11 , 12 , 16 8 , | 10 , 14 6 ,, 9 |
| TIMA0_C3N | O | TIMA0 capture/compare 3 complementary output | 15 , 16 , 20 , 27 , 28 , 33 , 44 , 46 5 ,, 8 | A4 , B4 , B7 , C2 , C4 , C7 , D1 , F5 , G2 , G3 | 11 , 12 , 16 , 18 , 22 , 29 31 , | 15 , 16 , 20 , 27 , 28 , 33 4 ,, 44 46 , | 15 , 16 , 20 , 27 , 28 , 33 , 44 , 46 5 ,, 8 | 12 , 16 , 17 , 22 , 28 , 30 6 , | 1 21 ,, 27 9 , | 14 , 20 , 22 5 , | 1 14 ,, 19 8 , | 12 , 17 , 19 6 ,, 8 |
| TIMA_FAL0 | I | TIMA fault input 0 | 12 , 15 , 16 , 19 , 24 3 ,, 46 8 , | A4 , B5 , B7 , C1 , C2 , C4 , E4 , G1 | 11 , 12 , 15 3 ,, 31 8 , | 12 , 15 , 16 , 19 , 24 3 ,, 46 | 12 , 15 , 16 , 19 , 24 3 ,, 46 8 , | 10 , 12 , 15 , 30 6 , | 1 13 ,, 16 9 , | 10 , 22 5 , | 1 10 ,, 11 8 , | 19 6 ,, 8 9 , |
| TIMA_FAL1 | I | TIMA fault input 1 | 1 11 ,, 17 , 18 , 35 , 41 8 , | A4 , B1 , D2 , D3 , D4 , E7 , G6 | 13 , 14 , 25 7 , | 1 11 ,, 17 , 18 , 36 41 , | 1 11 ,, 17 , 18 , 35 , 41 8 , | 1 13 ,, 14 , 24 6 ,, 9 | 12 , 14 , 15 , 23 4 ,, 9 | 16 , 24 5 ,, 8 9 , | 16 4 ,, 8 | 14 2 ,, 6 |
| TIMA_FAL2 | I | TIMA fault input 2 | 16 2 ,, 20 , 24 , 33 , 45 47 , | A7 , C4 , C5 , C6 , D1 , F5 , G1 | 12 , 16 2 ,, 22 , 30 32 , | 16 2 ,, 20 , 24 , 33 , 45 47 , | 16 2 ,, 20 , 24 , 33 , 45 47 , | 12 2 ,, 22 , 29 31 , | 2 21 ,, 28 5 , | 1 14 ,, 21 | 14 2 ,, 20 5 , | 12 , 18 , 20 3 , |
| TIMG8_IDX | I | TIMG8 quadrature encoder index pulse signal | 13 2 ,, 24 , 30 , 38 , 48 8 , | A4 , B2 , C5 , D5 , F6 , G1 , G4 | 1 2 ,, 20 9 , | 13 2 ,, 24 , 30 , 38 48 , | 13 2 ,, 24 , 30 , 38 , 48 8 , | 11 , 19 2 ,, 32 6 , | 18 3 ,, 5 9 , | 1 11 ,, 23 5 , | 3 5 8 ,, | 1 3 6 ,, |
| TIMG14_C0 | IO | TIMG14 capture/compare 0 signal | 1 11 ,, 18 , 27 , 34 , 36 , 43 45 , | B1 , B6 , C6 , D2 , D4 , D7 , G2 , G7 | 14 , 24 , 28 , 30 7 , | 1 11 ,, 18 , 27 , 35 , 43 45 , | 1 11 ,, 18 , 27 , 34 , 36 , 43 45 , | 1 14 ,, 16 , 23 , 27 , 29 9 , | 12 , 15 , 22 , 26 , 28 4 , | 15 , 19 , 21 , 24 9 , | 15 , 18 , 20 4 , | 13 , 16 , 18 2 , |
| TIMG14_C1 | IO | TIMG14 capture/compare 1 signal | 12 , 19 2 ,, 28 , 30 , 35 , 37 44 , | C1 , C5 , C7 , E4 , E5 , G3 , G4 , G6 | 15 , 18 2 ,, 20 , 25 , 26 , 29 8 , | 12 , 19 2 ,, 28 , 30 , 36 , 37 44 , | 12 , 19 2 ,, 28 , 30 , 35 , 37 44 , | 10 , 15 , 17 , 19 2 ,, 24 28 , | 13 , 16 , 18 , 23 , 27 5 , | 1 10 ,, 11 , 16 20 , | 10 , 11 , 16 , 19 5 , | 14 , 17 3 ,, 9 |
| TIMG14_C2 | IO | TIMG14 capture/compare 2 signal | 24 , 31 , 42 48 , | D5 , D6 , F4 , G1 | 1 27 , | 24 , 31 , 42 48 , | 24 , 31 , 42 48 , | 20 32 , | 19 3 , | 12 23 , | 12 3 , | 1 10 , |
| TIMG14_C3 | IO | TIMG14 capture/compare 3 signal | 28 | G3 | 18 | 28 | 28 | 17 |  |  |  |  |
| TIMG1_C0 | IO | TIMG1 capture/compare 0 signal | 11 , 13 , 14 , 20 , 22 , 29 , 36 , 39 45 , | B1 , B2 , B3 , C6 , D1 , E6 , F1 , F3 , G7 | 10 , 16 , 19 , 30 7 ,, 9 | 11 , 13 , 14 , 20 , 22 , 29 , 39 45 , | 11 , 13 , 14 , 20 , 22 , 29 , 36 , 39 45 , | 11 , 18 , 25 , 29 9 , | 12 , 17 , 24 28 , | 17 21 , | 20 | 18 |
| TIMG1_C1 | IO | TIMG1 capture/compare 1 signal | 12 , 15 , 17 , 21 , 23 , 37 , 40 46 , | B7 , C1 , C2 , D3 , E1 , E3 , E5 , F7 | 11 , 13 , 17 , 26 , 31 8 , | 12 , 15 , 17 , 21 , 23 , 37 , 40 46 , | 12 , 15 , 17 , 21 , 23 , 37 , 40 46 , | 10 , 13 , 26 30 , | 1 13 ,, 14 25 , | 18 , 22 8 , | 1 10 ,, 17 | 15 , 19 8 , |
| TIMG2_C0 | IO | TIMG2 capture/compare 0 signal | 17 , 23 , 25 3 ,, 32 , 43 , 46 9 , | A3 , B5 , B7 , D3 , D7 , E2 , E3 , G5 | 13 , 21 , 28 3 ,, 31 | 17 , 23 , 25 3 ,, 32 , 43 , 46 8 , | 17 , 23 , 25 3 ,, 32 , 43 , 46 9 , | 13 , 21 , 27 , 30 7 , | 1 10 ,, 14 , 20 26 , | 13 , 19 , 22 6 ,, 8 | 1 13 ,, 18 | 11 , 16 19 , |
| TIMG2_C1 | IO | TIMG2 capture/compare 1 signal | 10 , 13 , 15 , 16 , 18 , 24 , 25 , 26 , 27 , 33 , 38 , 44 , 47 , 48 8 , | A2 , A4 , A7 , B2 , C2 , C4 , C7 , D2 , D5 , E2 , F2 , F5 , F6 , G1 , G2 | 1 11 ,, 12 , 14 , 22 , 29 , 32 9 , | 13 , 15 , 16 , 18 , 24 , 25 , 26 , 27 , 33 , 38 , 44 , 47 , 48 9 , | 10 , 13 , 15 , 16 , 18 , 24 , 25 , 26 , 27 , 33 , 38 , 44 , 47 , 48 8 , | 11 , 12 , 14 , 16 , 22 , 28 , 31 , 32 6 ,, 8 | 11 , 15 2 ,, 21 , 27 3 ,, 9 | 14 , 20 , 23 5 ,, 7 9 , | 14 , 19 2 ,, 3 8 9 ,, | 1 12 ,, 17 , 20 6 ,, 7 8 , |
| TIMG8_C0 | IO | TIMG8 capture/compare 0 signal | 11 , 13 2 ,, 20 , 22 , 25 , 32 , 39 , 43 , 46 9 , | A3 , B1 , B2 , B7 , C5 , D1 , D7 , E2 , E6 , F1 , G5 | 16 2 ,, 21 , 28 , 31 7 ,, 9 | 11 , 13 2 ,, 20 , 22 , 25 , 32 , 39 , 43 , 46 8 , | 11 , 13 2 ,, 20 , 22 , 25 , 32 , 39 , 43 , 46 9 , | 11 2 ,, 21 , 25 , 27 , 30 7 ,, 9 | 1 10 ,, 12 , 20 , 24 , 26 5 , | 1 13 ,, 17 , 19 , 22 6 , | 1 13 ,, 18 5 , | 11 , 16 , 19 3 , |
|  |  |  | 1 10 ,, | A2 , | 14 , | 1 12 ,, | 1 10 ,, | 1 10 ,, | 11 , | 14 , | 10 , | 12 , |
|  |  |  | ,, | , | , | ,, | ,, | ,, | , | , | , | , |
|  |  |  | 12 , | A4 , | 16 , | 18 , | 12 , | 14 , | 13 , | 18 , | 14 , | 15 , |
|  |  |  | , | , | , | , | , | , | , | , | , | , |
|  |  |  | 18 , | A7 , | 17 , | 20 , | 18 , | 17 , | 15 2 ,, | 20 , | 17 , | 17 2 ,, |
|  |  |  | , | , | , | , | , | , | ,, | , | , | ,, |
|  |  |  | 20 , | C1 , | 18 , | 21 , | 20 , | 22 , | 21 , | 24 5 ,, | 19 2 ,, | 20 6 ,, |
|  |  |  | , | , | , | , | , | , | , | ,, | ,, | ,, |
|  |  |  | 21 , | C7 , | 22 , | 26 , | 21 , | 26 , | 25 , | 7 9 , | 4 8 9 ,, | 7 |
|  |  |  | , | , | , | , | , | , | , | , | ,, |  |
|  |  |  | 26 , | D1 , | 29 , | 28 , | 26 , | 28 , | 27 4 ,, |  |  |  |
|  |  |  | , | , | , | , | , | , | ,, |  |  |  |
|  |  |  | 28 , | D2 , | 32 8 , | 33 , | 28 , | 31 6 ,, | 9 |  |  |  |
|  |  | TIMG8 capture/compare 1 | , | , | , | , | , | ,, |  |  |  |  |
| TIMG8_C1 | IO |  | 33 , | D4 , |  | 38 , | 33 , | 8 |  |  |  |  |
|  |  | signal | , | , |  | , | , |  |  |  |  |  |
|  |  |  | 38 , | E1 , |  | 40 , | 38 , |  |  |  |  |  |
|  |  |  | , | , |  | , | , |  |  |  |  |  |
|  |  |  | 40 , | E7 , |  | 41 , | 40 , |  |  |  |  |  |
|  |  |  | , | , |  | , | , |  |  |  |  |  |
|  |  |  | 41 , | F2 , |  | 44 , | 41 , |  |  |  |  |  |
|  |  |  | , | , |  | , | , |  |  |  |  |  |
|  |  |  | 44 , | F5 , |  | 47 9 , | 44 , |  |  |  |  |  |
|  |  |  | , | , |  | , | , |  |  |  |  |  |
|  |  |  | 47 8 , | F6 , |  |  | 47 8 , |  |  |  |  |  |
|  |  |  | , | , |  |  | , |  |  |  |  |  |
|  |  |  |  | F7 , |  |  |  |  |  |  |  |  |
|  |  |  |  | , |  |  |  |  |  |  |  |  |
|  |  |  |  | G3 |  |  |  |  |  |  |  |  |

**Table 6-14. Universal Asynchronous Receiver Transmitter (UART) Signal Descriptions (pp. 38-40)**

| SIGNAL NAME | PIN TYPE | DESCRIPTION | PT (A) PIN | ZCM PIN | VFC PIN | PT (B) PIN | RGZ PIN | RHB PIN | DGS2 8 PIN | RGE PIN | DGS2 0 PIN | RUK PIN |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| UART0_CTS | I | UART0 clear to send signal | 11 , 17 , 29 , 34 , 38 43 , | B1 , B6 , D3 , D7 , F3 , F6 | 13 , 19 , 24 , 28 7 , | 11 , 17 , 29 , 35 , 38 43 , | 11 , 17 , 29 , 34 , 38 43 , | 13 , 18 , 23 , 27 9 , | 12 , 14 , 17 , 22 26 , | 15 , 19 8 , | 15 18 , | 13 16 , |
| UART0_RTS | O | UART0 ready to send signal | 12 , 16 , 30 , 35 , 37 , 42 44 , | C1 , C4 , C7 , D6 , E5 , G4 , G6 | 12 , 20 , 23 , 25 , 26 , 27 , 29 8 , | 12 , 16 , 30 , 34 , 36 , 37 , 42 44 , | 12 , 16 , 30 , 35 , 37 , 42 44 , | 10 , 12 , 19 , 24 28 , | 13 , 18 , 23 27 , | 11 , 16 20 , | 10 , 16 19 , | 14 17 , |
| UART0_RX | IO | UART0 receive signal (RXD) | 19 2 ,, 21 , 22 , 23 , 28 , 32 , 46 , 48 5 , | B4 , B7 , C5 , D5 , E1 , E3 , E4 , F1 , G3 , G5 | 1 15 ,, 17 , 18 2 ,, 21 31 , | 19 2 ,, 21 , 22 , 23 , 28 , 32 4 ,, 46 48 , | 19 2 ,, 21 , 22 , 23 , 28 , 32 , 46 , 48 5 , | 15 , 17 2 ,, 21 , 30 32 , | 1 16 ,, 20 3 ,, 5 | 1 10 ,, 13 , 22 23 , | 1 11 ,, 13 3 ,, 5 | 1 11 ,, 19 3 ,, 9 |
| UART0_TX | O | UART0 transmit signal (TXD) | 1 18 ,, 2 20 ,, 23 , 24 , 29 3 ,, 43 , 45 47 , | A7 , B5 , C5 , C6 , D1 , D2 , D4 , D7 , E3 , F3 , G1 | 14 , 16 , 19 2 ,, 28 3 ,, 30 32 , | 1 18 ,, 2 20 ,, 23 , 24 , 29 3 ,, 43 , 45 47 , | 1 18 ,, 2 20 ,, 23 , 24 , 29 3 ,, 43 , 45 47 , | 1 14 ,, 18 2 ,, 27 , 29 31 , | 15 , 17 2 ,, 26 , 28 4 ,, 5 | 1 19 ,, 21 , 24 9 , | 18 2 ,, 20 4 ,, 5 | 16 , 18 2 ,, 20 3 , |
| UART1_CTS | I | UART1 clear to send signal | 14 , 22 , 27 , 39 , 41 , 46 47 , | A7 , B3 , B7 , E6 , E7 , F1 , G2 | 10 , 31 32 , | 14 , 22 , 27 , 39 , 41 , 46 47 , | 14 , 22 , 27 , 39 , 41 , 46 47 , | 16 , 25 , 30 31 , | 1 2 ,, 24 | 17 22 , | 1 2 , | 19 20 , |
| UART1_RTS | O | UART1 ready to send signal | 15 2 ,, 23 , 28 , 33 , 40 48 , | C2 , C5 , D5 , E3 , F5 , F7 , G3 | 1 11 ,, 18 2 ,, 22 | 15 2 ,, 23 , 28 , 33 , 40 48 , | 15 2 ,, 23 , 28 , 33 , 40 48 , | 17 2 ,, 22 , 26 32 , | 21 , 25 3 ,, 5 | 1 14 ,, 18 23 , | 14 , 17 3 ,, 5 | 1 12 ,, 15 3 ,, 8 |
| UART1_RX | I | UART1 receive signal (RXD) | 10 , 12 , 14 , 16 , 17 , 19 , 21 , 26 , 33 , 34 , 35 , 40 44 , | A2 , B3 , B6 , C1 , C4 , C7 , D3 , E1 , E4 , F2 , F5 , F7 , G6 | 10 , 12 , 13 , 15 , 17 , 22 , 24 , 25 , 29 8 , | 12 , 14 , 16 , 17 , 19 , 21 , 26 , 33 , 35 , 36 , 40 , 44 9 , | 10 , 12 , 14 , 16 , 17 , 19 , 21 , 26 , 33 , 34 , 35 , 40 44 , | 10 , 12 , 13 , 15 , 22 , 23 , 24 , 26 , 28 8 , | 11 , 13 , 14 , 16 , 21 , 22 , 23 , 25 27 , | 10 , 14 , 15 , 16 , 18 , 20 7 ,, 8 | 10 , 11 , 14 , 15 , 16 , 17 , 19 9 , | 12 , 13 , 14 , 15 , 17 7 ,, 9 |
| UART1_TX | O | UART1 transmit signal (TXD) | 11 , 13 , 15 , 16 , 20 , 25 , 32 , 35 9 , | A3 , B1 , B2 , C2 , C4 , D1 , E2 , G5 , G6 | 11 , 12 , 16 , 21 , 25 7 ,, 9 | 11 , 13 , 15 , 16 , 20 , 25 , 32 , 36 8 , | 11 , 13 , 15 , 16 , 20 , 25 , 32 , 35 9 , | 11 , 12 , 21 , 24 7 ,, 9 | 10 , 12 , 20 23 , | 13 , 16 6 , | 13 16 , | 11 , 14 8 , |
| UART2_CTS | I | UART2 clear to send signal | 14 , 20 , 27 , 31 , 38 , 39 , 45 9 , | A3 , B3 , C6 , D1 , E6 , F4 , F6 , G2 | 10 , 16 30 , | 14 , 20 , 27 , 31 , 38 , 39 , 45 8 , | 14 , 20 , 27 , 31 , 38 , 39 , 45 9 , | 16 , 20 , 25 , 29 7 , | 10 , 19 , 24 28 , | 12 , 17 , 21 6 , | 12 20 , | 10 18 , |
| UART2_RTS | O | UART2 ready to send signal | 10 , 15 , 21 , 30 , 41 , 42 46 , | A2 , B7 , C2 , D6 , E1 , E7 , G4 | 11 , 17 , 20 , 27 31 , | 15 , 21 , 30 , 41 , 42 , 46 9 , | 10 , 15 , 21 , 30 , 41 , 42 46 , | 19 , 30 8 , | 1 11 ,, 18 | 11 , 22 7 , | 1 9 , | 19 7 ,, 8 |
| UART2_RX | I | UART2 receive signal (RXD) | 15 , 26 , 29 , 37 , 38 , 39 , 40 44 , | C2 , C7 , E5 , E6 , F2 , F3 , F6 , F7 | 11 , 19 , 26 29 , | 15 , 26 , 29 , 37 , 38 , 39 , 40 44 , | 15 , 26 , 29 , 37 , 38 , 39 , 40 44 , | 18 , 25 , 26 28 , | 17 , 24 , 25 27 , | 17 , 18 20 , | 17 19 , | 15 , 17 8 , |
|  |  |  | 14 , | B3 , | 10 , | 14 , | 14 , | 17 , | 24 , | 17 , | 17 18 , | 15 16 , |
|  |  |  | , | , | , | , | , | , | , | , | , | , |
|  |  |  | 25 , | D7 , | 18 28 , | 25 , | 25 , | 25 , | 25 26 , | 18 19 , |  |  |
|  |  |  | , | , | , | , | , | , | , | , |  |  |
|  |  |  | 28 , | E2 , |  | 28 , | 28 , | 26 27 , |  |  |  |  |
|  |  |  | , | , |  | , | , | , |  |  |  |  |
| UART2_TX | O | UART2 transmit signal (TXD) | 36 , | E6 , |  | 39 , | 36 , |  |  |  |  |  |
|  |  |  | 39 , | F7 , |  | 40 43 , | 39 , |  |  |  |  |  |
|  |  |  | , | , |  | , | , |  |  |  |  |  |
|  |  |  | 40 43 , | G3 , |  |  | 40 43 , |  |  |  |  |  |
|  |  |  | , | , |  |  | , |  |  |  |  |  |
|  |  |  |  | G7 |  |  |  |  |  |  |  |  |

### 6.4 Connections for Unused Pins (p. 40)

Table 6-15 lists the correct termination of unused pins.

**Table 6-15. Connection of Unused Pins (p. 40)**

| PIN (1) | POTENTIAL | COMMENT |
| --- | --- | --- |
| PAx and PBx | Open | Set corresponding pin functions to GPIO (PINCMx.PF = 0x1) and configure unused pins to output low or input with internal pullup/ pulldown resistor. |
| NRST | VCC | NRST is an active-low reset signal; it must be pulled high to VCC or the device will not start, for more information refer to Section 9.1 |

(1) Any unused pin with a function that is shared with general-purpose I/O should follow the "PAx and PBx" unused pin connection guidelines.


---

---
document: "MSPM0C1105, MSPM0C1106 Mixed-Signal Microcontrollers datasheet (Rev. B)"
section_id: "07"
section_title: "7 Specifications"
pages: "41-57"
---

# 7 Specifications (pp. 41-57)

## 7 Specifications (p. 41)

### 7.1 Absolute Maximum Ratings (p. 41)

over operating free-air temperature range (unless otherwise noted) (1)

| Col 1 | Col 2 | Col 3 | MIN | MAX | UNIT |
| --- | --- | --- | --- | --- | --- |
| VDD | Supply voltage | At VDD pin, with respect to VSS | -0.3 | 4.1 | V |
| VI | Input voltage | Applied to any 5-V tolerant open-drain pins | -0.3 | 5.5 | V |
| VI | Input voltage | Applied to any common tolerance pins | -0.3 | VDD+ 0.3 (4.1 MAX) | V |
| I (3) | Current into VDD pin | -40°C≤ Tj ≤ 130°C |  | 80 | mA |
| IVDD | (source) | -40°C≤ Tj ≤ 85°C |  | 100 | mA |
| I (3) | Current out of VSS pin | -40°C≤ Tj ≤ 130°C |  | 80 | mA |
| IVSS | (sink) | -40°C≤ Tj ≤ 85°C |  | 100 | mA |
| I | Current for SDIO pin | Current sunk or sourced by SDIO pin |  | 6 | mA |
| I IO | Current for ODIO pin | Current sunk by ODIO pin |  | 20 | mA |
| I D | Supported diode current | Diode current at any device pin |  | ±2 | mA |
| Tj | Junction temperature |  | -40 | 130 | °C |
| T stg | Storage temperature (2) |  | -40 | 150 | °C |

(1) Stresses beyond those listed under Absolute Maximum Rating may cause permanent damage to the device. These are stress ratings only, which do not imply functional operation of the device at these or any other conditions beyond those indicated under Recommended Operating Condition. Exposure to absolute-maximum-rated conditions for extended periods may affect device reliability.

(2) Higher temperatures may be applied during board soldering according to the current JEDEC J-STD-020 specification with peak reflow temperatures not higher than classified on the device label on the shipping boxes or reels.

(3) For applications operating at VDD=1.62V, I_VDD/I_VSS<=20mA is required to ensure device functionality

### 7.2 ESD Ratings (p. 41)

| Col 1 | Col 2 | Col 3 | VALUE | UNIT |
| --- | --- | --- | --- | --- |
| V | Electrostatic discharge | Human body model (HBM), per ANSI/ESDA/ JEDEC JS-001, all pins (1) | ±2000 | V |
| V (ESD) | Electrostatic discharge | Charged device model (CDM), per JEDEC specification JESD22-C101, all pins (2) | ±500 | V |

(1) JEDEC document JEP155 states that 500-V HBM allows safe manufacturing with a standard ESD control process.

(2) JEDEC document JEP157 states that 250-V CDM allows safe manufacturing with a standard ESD control process.

### 7.3 Recommended Operating Conditions (p. 41)

over operating free-air temperature range (unless otherwise noted)

| Col 1 | Col 2 | MIN | NOM | MAX | UNIT |
| --- | --- | --- | --- | --- | --- |
| VDD | Supply voltage (3) | 1.62(4) |  | 3.6 | V |
| CVDD | Capacitor placed between VDD and VSS (1) |  | 10 |  | uF |
| TA | Ambient temperature | -40 |  | 125 | °C |
| TJ | Max junction temperature |  |  | 130 | °C |
| f | MCLK, CPUCLK, ULPCLK frequency with 1 flash wait state (2) |  |  | 32 | MHz |
| fMCLK | MCLK, CPUCLK, ULPCLK frequency with 0 flash wait states (2) |  |  | 24 | MHz |

(1) Connect C VDD between VDD/VSS as close to the device pins as possible. A low-ESR capacitor with at least the specified value and tolerance of ±20% or better is required for C VDD.

(2) Wait states are managed automatically by the system controller (SYSCTL) and do not need to be configured by application software.

(3) There is no dependency on MCLK frequency with respect to VDD recommended operating range.

(4) Functionality is guaranteed down to V BOR0-(max).

### 7.4 Thermal Information (p. 42)

| Col 1 | THERMAL METRIC (1) | PACKAGE | VALUE | UNIT |
| --- | --- | --- | --- | --- |
| RθJA | Junction-to-ambient thermal resistance |  | 78.8 | °C/W |
| R θJC(top) | Junction-to-case (top) thermal resistance |  | 35.1 | °C/W |
| RθJB | Junction-to-board thermal resistance | LQFP-48 (PT) | 50.7 | °C/W |
| ΨJT | Junction-to-top characterization parameter | LQFP-48 (PT) | 3.5 | °C/W |
| ΨJB | Junction-to-board characterization parameter |  | 50.1 | °C/W |
| R θJC(bot) | Junction-to-case (bottom) thermal resistance |  | N/A | °C/W |
| RθJA | Junction-to-ambient thermal resistance |  | 34.6 | °C/W |
| R θJC(top) | Junction-to-case (top) thermal resistance |  | 25.4 | °C/W |
| RθJB | Junction-to-board thermal resistance | VQFN-48 (RGZ) | 17.5 | °C/W |
| ΨJT | Junction-to-top characterization parameter | VQFN-48 (RGZ) | 1.6 | °C/W |
| ΨJB | Junction-to-board characterization parameter |  | 17.5 | °C/W |
| R θJC(bot) | Junction-to-case (bottom) thermal resistance |  | 9.0 | °C/W |
| RθJA | Junction-to-ambient thermal resistance |  | 101.1 | °C/W |
| R θJC(top) | Junction-to-case (top) thermal resistance |  | 42.1 | °C/W |
| RθJB | Junction-to-board thermal resistance | NFBGA-48 (ZCM) | 62.2 | °C/W |
| ΨJT | Junction-to-top characterization parameter | NFBGA-48 (ZCM) | 1.8 | °C/W |
| ΨJB | Junction-to-board characterization parameter |  | 62.0 | °C/W |
| R θJC(bot) | Junction-to-case (bottom) thermal resistance |  | N/A | °C/W |
| RθJA | Junction-to-ambient thermal resistance |  | 38.7 | °C/W |
| R θJC(top) | Junction-to-case (top) thermal resistance |  | 31.1 | °C/W |
| RθJB | Junction-to-board thermal resistance | VQFN-32 (RHB) | 18.7 | °C/W |
| ΨJT | Junction-to-top characterization parameter | VQFN-32 (RHB) | 1.6 | °C/W |
| ΨJB | Junction-to-board characterization parameter |  | 18.7 | °C/W |
| R θJC(bot) | Junction-to-case (bottom) thermal resistance |  | 9.1 | °C/W |
| RθJA | Junction-to-ambient thermal resistance |  | 80.6 | °C/W |
| R θJC(top) | Junction-to-case (top) thermal resistance |  | 39.9 | °C/W |
| RθJB | Junction-to-board thermal resistance | VSSOP-28 (DGS28) | 42.6 | °C/W |
| ΨJT | Junction-to-top characterization parameter | VSSOP-28 (DGS28) | 3.5 | °C/W |
| ΨJB | Junction-to-board characterization parameter |  | 42.2 | °C/W |
| R θJC(bot) | Junction-to-case (bottom) thermal resistance |  | N/A | °C/W |
| RθJA | Junction-to-ambient thermal resistance |  | 47.1 | °C/W |
| R θJC(top) | Junction-to-case (top) thermal resistance |  | 40.4 | °C/W |
| RθJB | Junction-to-board thermal resistance | VQFN-24 (RGE) | 23.7 | °C/W |
| ΨJT | Junction-to-top characterization parameter | VQFN-24 (RGE) | 2.4 | °C/W |
| ΨJB | Junction-to-board characterization parameter |  | 23.6 | °C/W |
| R θJC(bot) | Junction-to-case (bottom) thermal resistance |  | 9.3 | °C/W |
| RθJA | Junction-to-ambient thermal resistance |  | 92.8 | °C/W |
| R θJC(top) | Junction-to-case (top) thermal resistance |  | 35.5 | °C/W |
| RθJB | Junction-to-board thermal resistance | VSSOP-20 (DGS20) | 49.6 | °C/W |
| ΨJT | Junction-to-top characterization parameter | VSSOP-20 (DGS20) | 1.3 | °C/W |
| ΨJB | Junction-to-board characterization parameter |  | 49.1 | °C/W |
| R θJC(bot) | Junction-to-case (bottom) thermal resistance |  | N/A | °C/W |
| RθJA | Junction-to-ambient thermal resistance |  | 49.6 | °C/W |
| R θJC(top) | Junction-to-case (top) thermal resistance |  | 48.9 | °C/W |
| RθJB | Junction-to-board thermal resistance | WQFN-20 (RUK) | 23.6 | °C/W |
| ΨJT | Junction-to-top characterization parameter | WQFN-20 (RUK) | 1.4 | °C/W |
| ΨJB | Junction-to-board characterization parameter |  | 23.6 | °C/W |
| R θJC(bot) | Junction-to-case (bottom) thermal resistance |  | 9.2 | °C/W |

(1) For more information about traditional and new thermal metrics, see the Semiconductor and IC Package Thermal Metrics application report.

### 7.5 Supply Current Characteristics (p. 44)

#### 7.5.1 RUN/SLEEP Modes (p. 44)

VDD=3.3V. All inputs tied to 0V or VDD. Outputs do not source or sink any current. All peripherals are disabled.

| Col 1 | PARAMETER PARAMETER | Col 3 | MCLK MCLK | -40°C TYP | -40°C MAX | 25°C TYP | 25°C MAX | 85°C TYP | 85°C MAX | 85°C TYP | 105°C TYP | 105°C MAX | 105°C TYP | 125°C MAX | UNIT UNIT |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| RUN Mode |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| IDD RUN | MCLK=SYSOSC, CoreMark, execute from flash | 32MHz |  | 2.9 |  | 2.9 |  | 2.9 |  |  | 2.9 |  | 2.9 |  | mA |
| IDD RUN, | MCLK=SYSOSC, While(1), execute from flash | 32MHz |  | 52 | 58 | 53 | 59 | 53 | 59 |  | 54 |  | 54 | 60 | uA/Mhz |
| per MHz | MCLK=SYSOSC, CoreMark, execute from flash | 32MHz |  | 90 |  | 91 |  | 91 |  |  | 91 |  | 91 |  | uA/Mhz |
| SLEEP Mode |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| IDDSLEEP | MCLK=SYSOSC, CPU is halted | 32MHz |  | 1332 | 1455 | 1346 | 1468 | 1358 | 1479 | 1362 | 1362 | 1486 | 1370 | 1490 | uA |
| IDDSLEEP | MCLK=LFCLK, CPU is halted | 32kHz |  | 524 | 591 | 529 | 595 | 542 | 643 |  | 549 | 650 | 558 | 660 | uA |
| 7.5.2 noted are disabled. | STOP/STANDBY Modes |  |  | VDD=3.3V unless otherwise noted. All inputs tied to 0V or VDD. Outputs do not source or sink any current. All peripherals not | VDD=3.3V unless otherwise noted. All inputs tied to 0V or VDD. Outputs do not source or sink any current. All peripherals not |  |  |  |  |  |  |  |  |  |  |
|  | PARAMETER |  | ULPCLK | -40°C | -40°C | 25°C |  | 85°C |  |  | 105°C | 105°C |  | 125°C | UNIT |
|  | PARAMETER |  | ULPCLK | TYP | MAX | TYP | MAX | TYP | MAX | TYP | TYP | MAX | TYP | MAX | UNIT |
| STOP Mode |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
| IDDSTOP0 | SYSOSC=32MHz, DISABLESTOP=0 | 4MHz |  | 415 | 456 | 422 | 459 | 431 | 473 |  | 434 | 476 | 436 | 481 | uA |
| IDDSTOP2 | SYSOSC off, DISABLESTOP=1, ULPCLK=LFCLK | 32kHz |  | 81 | 91 | 84 | 93 | 93 | 118 |  | 96 | 121 | 99 | 127 | uA |
| STANDBY Mode |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  | LFXT and RTC enabled |  |  | 2.6 | 4.1 | 2.7 | 4.1 | 7.9 | 27 |  | 11 |  | 16 | 34 | uA |
|  | LFOSC and IWDT enabled |  |  | 2.4 | 3.8 | 2.5 | 3.8 | 7.6 | 27 |  | 10 |  | 13 | 35 | uA |
| IDDSTBY0 | LFXT and RTC enabled, IWDT enabled |  |  | 2.6 | 4.1 | 2.7 | 4.1 | 8.0 | 27 |  | 11 |  | 16 | 34 | uA |
|  | STOPCLKSTBY=0, TIMG0 enabled |  |  | 2.3 | 3.9 | 2.4 | 3.9 | 7.6 | 27 |  | 10 |  | 15 | 35 |  |
| IDD | STOPCLKSTBY=1, TIMG0 enabled | 32kHz |  | 2.1 | 3.4 | 2.2 | 3.4 | 7.1 | 26 |  | 10 |  | 15 | 34 | uA |
| IDDSTBY1 | STOPCLKSTBY=1, GPIOA enabled |  |  | 2.0 | 2.6 | 2.1 | 2.6 | 7.1 | 26 |  | 10 |  | 13 | 34 |  |
| 7.5.3 powered down. | SHUTDOWN Mode |  |  | VDD=3.3V unless otherwise noted. All inputs tied to 0V or VDD. Outputs do not source or sink any current. Core regulator is |  |  |  |  |  |  |  |  |  |  |  |
|  | PARAMETER |  | -40°C |  | 25°C | 25°C |  | 85°C |  | 105°C | 105°C |  | 125°C |  | UNIT |
|  | PARAMETER |  | TYP | MAX | TYP | MAX | TYP | MAX | TYP | TYP |  | MAX | TYP | MAX | UNIT |
| IDDSHDN | Supply current in SHUTDOWN mode |  | 54 |  | 68 |  | 255 |  |  | 524 |  |  | 1362 |  | nA |

### 7.6 Power Supply Sequencing (p. 44)

#### 7.6.1 Power Supply Ramp (p. 44)

Figure 7-1 gives the relationship of POR- POR+, BOR0-, and BOR0+ during power-up and power-down.

Figure 7-1. Power Cycle POR/BOR Conditions - VDD

#### 7.6.2 POR and BOR (p. 45)

over operating free-air temperature range (unless otherwise noted)

| Col 2 | Col 3 | Col 4 | PARAMETER | Col 6 | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  | Rising |  |  | 0.1 | V/us |
| dVDD/dt |  |  | VDD (supply voltage) slew rate |  | Falling (1) Falling, STANDBY |  |  | 0.01 0.1 | V/us V/ms |
| VPOR+ |  |  | Power-on reset voltage level |  | Rising | 0.95 | 1.3 | 1.59 | V |
| VPOR- |  |  | Power-on reset voltage level |  | Falling | 0.9 | 1.25 | 1.54 | V |
| V HYS, POR |  | POR hysteresis |  |  |  | 30 | 58 | 74 | mV |
| V BOR0+, |  |  |  |  | -40 °C ≤ Ta ≤ 25 °C, Cold start, rising | 1.50 | 1.56 | 1.63 |  |
| COLD |  |  | Brown-out reset voltage level 0 (default level) | Brown-out reset voltage level 0 (default level) | 25 °C ≤ Ta ≤ 125 °C, Cold start, rising | 1.51 | 1.58 | 1.65 | V |
| VBOR0+ |  |  |  |  | Rising (1) | 1.56 | 1.59 | 1.62 |  |
| VBOR0- |  |  |  |  | Falling (1) | 1.55 | 1.58 | 1.61 |  |
| V BOR0, STBY | BOR0, STBY |  |  |  | STANDBY mode | 1.51 | 1.56 | 1.61 |  |
| VBOR1+ |  |  |  |  | Rising (1) | 2.13 | 2.17 | 2.21 |  |
| VBOR1- |  |  | Brown-out-reset voltage level 1 |  | Falling (1) | 2.10 | 2.14 | 2.18 | V |
| V BOR1, STBY | BOR1, STBY |  |  |  | STANDBY mode | 2.06 | 2.13 | 2.20 |  |
| VBOR2+ |  |  |  |  | Rising (1) | 2.73 | 2.77 | 2.82 |  |
| VBOR2- |  |  | Brown-out-reset voltage level 2 |  | Falling (1) | 2.7 | 2.74 | 2.79 | V |
| V BOR2, STBY | BOR2, STBY |  |  |  | STANDBY mode | 2.62 | 2.71 | 2.8 |  |
| VBOR3+ |  |  |  |  | Rising (1) | 2.88 | 2.96 | 3.04 |  |
| VBOR3- |  |  | Brown-out-reset voltage level 3 |  | Falling (1) | 2.85 | 2.93 | 3.01 | V |
| V BOR3, STBY | BOR3, STBY |  |  |  | STANDBY mode | 2.82 | 2.92 | 3.02 |  |
| V |  |  | Brown-out reset hysteresis |  | Level 0 |  | 15 | 21 | mV |
| V HYS,BOR |  |  | Brown-out reset hysteresis |  | Levels 1-3 |  | 34 | 40 | mV |
| T |  | BOR propagation delay |  |  | RUN/SLEEP/STOP mode |  |  | 5 | us |
| T PD, BOR |  | BOR propagation delay |  |  |  |  |  |  |  |
|  |  |  |  |  | STANDBY mode |  |  | 100 | us |
| (1) |  |  | Device operating in RUN, SLEEP, or STOP mode. |  |  |  |  |  |  |

### 7.7 Flash Memory Characteristics (p. 46)

over operating free-air temperature range (unless otherwise noted)

| Col 1 | PARAMETER | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
| --- | --- | --- | --- | --- | --- | --- |
| Supply |  |  |  |  |  |  |
| VDD PGM/ERASE | Program and erase supply voltage |  | 1.62 |  | 3.6 | V |
| IDD ERASE | Supply current from VDD during erase operation | Supply current delta |  | 2 |  | mA |
| IDD PGM | Supply current from VDD during program operation | Supply current delta |  | 2.5 |  | mA |
| Endurance |  |  |  |  |  |  |
| NWEC (LOWER) | Erase/program cycle endurance (lower 32kB flash) (1) |  | 100 |  |  | k cycles |
| NWEC (UPPER) | Erase/program cycle endurance (remaining flash) (1) |  | 10 |  |  | k cycles |
| NE (MAX) | Total erase operations before failure (2) |  | 802 |  |  | k erase operations |
| NW (MAX) | Write operations per word line before sector erase (3) |  |  |  | 83 | write operations |
| Retention |  |  |  |  |  |  |
| t RET_85 | Flash memory data retention | -40°C ≤T ≤ 85°C j | 60 |  |  | years |
| t RET_105 | Flash memory data retention | -40°C ≤T ≤ 105°C j | 11.4 |  |  | years |
| Program and Erase Timing |  |  |  |  |  |  |
| t PROG (WORD, 64) | Program time for flash word (4) (6) |  |  | 50 | 275 | µs |
| t PROG (SEC, 64) | Program time for 1kB sector (5) (6) |  |  | 6.4 |  | ms |
| t ERASE (SEC) | Sector erase time | ≤2k erase/program cycles, T ≥25°C j |  | 4 | 20 | ms |
| t ERASE (SEC) | Sector erase time | ≤10k erase/program cycles, T ≥25°C j |  | 20 | 150 | ms |
| t ERASE (SEC) | Sector erase time | ≤10k erase/program cycles |  | 20 | 200 | ms |
| t ERASE (BANK) | Bank erase time | ≤10k erase/program cycles |  | 22 | 220 | ms |

(1) The lower 32kB flash address space supports higher erase/program endurance to enable EEPROM emulation applications. On devices with <=32kB flash memory, the entire flash memory supports NWEC (LOWER) erase/program cycles.

(2) Total number of cumulative erase operations supported by the flash before failure. A sector erase or bank erase operation is considered to be one erase operation.

(3) Maximum number of write operations allowed per word line before the word line must be erased. If additional writes to the same word line are required, a sector erase is required once the maximum number of write operations per word line is reached.

(4) Program time is defined as the time from when the program command is triggered until the command completion interrupt flag is set in the flash controller.

(5) Sector program time is defined as the time from when the first word program command is triggered until the final word program command completes and the interrupt flag is set in the flash controller. This time includes the time needed for software to load each flash word (after the first flash word) into the flash controller during programming of the sector.

(6) Flash word size is 64 data bits (8 bytes). On devices with ECC, the total flash word size is 72 bits (64 data bits plus 8 ECC bits).

### 7.8 Timing Characteristics (p. 46)

VDD=3.3V, T a=25 °C (unless otherwise noted)

| Col 1 | PARAMETER | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
| --- | --- | --- | --- | --- | --- | --- |
| Wakeup Timing |  |  |  |  |  |  |
| t WAKE, SLEEP | Wakeup time from SLEEP to RUN (1) |  |  | 2 |  | cycles |
| t WAKE, STOP0 | Wakeup time from STOP0 to RUN (SYSOSC enabled) (1) |  |  | 14 |  | us |
| t WAKE, STOP2 | Wakeup time from STOP2 to RUN (SYSOSC disabled) (1) |  |  | 13 |  | us |

| VDD=3.3V, T | a=25 °C (unless otherwise noted) PARAMETER | a=25 °C (unless otherwise noted) TEST CONDITIONS | a=25 °C (unless otherwise noted) MIN | a=25 °C (unless otherwise noted) TYP | a=25 °C (unless otherwise noted) MAX | a=25 °C (unless otherwise noted) UNIT |
| --- | --- | --- | --- | --- | --- | --- |
| t WAKE, STBY0 | Wakeup time from STANDBY to RUN (1) |  |  | 15 |  | us |
| t WAKE, SHDN | Wakeup time from SHUTDOWN to RUN | Fast boot enabled |  | 255 |  | us |
| t WAKE, SHDN | Wakeup time from SHUTDOWN to RUN | Fast boot disabled |  | 265 |  | us |
|  | Asynchronous Fast Clock Request Timing |  |  |  |  |  |
| tDELAY | Delay time from edge of asynchronous request to first 32MHz MCLK edge | Mode is SLEEP2 |  | 0.9 |  | us |
| tDELAY | Delay time from edge of asynchronous request to first 32MHz MCLK edge | Mode is STOP0 |  | 2.4 |  | us |
| tDELAY | Delay time from edge of asynchronous request to first 32MHz MCLK edge | Mode is STOP2 |  | 0.9 |  | us |
| tDELAY | Delay time from edge of asynchronous request to first 32MHz MCLK edge | Mode is STANDBY1 |  | 3.2 |  | us |
| Startup Timing |  |  |  |  |  |  |
| t START, | Device cold start-up time from reset/ | Fast boot enabled |  | 241 |  | us |
| RESET | (2) power-up | Fast boot disabled |  | 284 |  | us |
| NRST Timing |  |  |  |  |  |  |
| t RST, | Minimum pulse length on NRST pin to | ULPCLK≥4MHz |  | 2 |  | us |
| BOOTRST | generate BOOTRST | ULPCLK=32kHz |  | 100 |  | us |
| t RST, POR | Minimum pulse length on NRST pin to generate POR |  |  | 1 |  | s |

(1) The wake-up time is measured from the edge of an external signal (GPIO wake-up event) to the time that the first CPU instruction is executed, with the GPIO glitch filter disabled (FILTEREN=0x0) and fast wake enabled (FASTWAKEONLY=1)

(2) The start-up time is measured from the time that VDD crosses VBOR0+ (cold start-up) to the time that the first instruction of the user program is executed.

### 7.9 Clock Specifications (p. 47)

#### 7.9.1 System Oscillator (SYSOSC) (p. 47)

over operating free-air temperature range (unless otherwise noted)

| Col 1 | PARAMETER | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
| --- | --- | --- | --- | --- | --- | --- |
| fSYSOSC | Factory trimmed SYSOSC frequency | SYSOSCCFG.FREQ=00 (BASE) |  | 32 |  | MHz |
|  | SYSOSC frequency accuracy when | SETUSEFCL=1, T a= 25 °C | 0 |  | 1.0 |  |
| fSYSOSC | frequency correction loop (FCL) is enabled (1) (2) | SETUSEFCL=1, -40 °C≤ T a≤ 125 °C | -2.1 |  | 1.6 | % |
| fSYSOSC | SYSOSC accuracy when frequency correction loop (FCL) is disabled, 32MHz (1) (2) | SETUSEFCL=0, SYSOSCCFG.FREQ=00, -40 °C≤ T a≤ 125 °C | -2.6 |  | 1.8 | % |
| t settle, SYSOSC | Settling time to target accuracy (3) | SETUSEFCL=1 |  |  | 30 | us |

(1) SYSOSC accuracy is measured in the default power-up state, with MCLK = SYSOSC and the CPU running a while(1) loop.

(2) SYSOSC is measured with the internal FCC counter using an external 1ms pulse as the measurement trigger.

(3) When SYSOSC is waking up (for example, when exiting a low power mode) and FCL is enabled, the SYSOSC will initially undershoot the target frequency f SYSOSC by an additional error of up to f settle,SYSOSC for the time t settle,SYSOSC, after which the target accuracy is achieved.

| 7.9.2 | Col 2 | Low Frequency Oscillator (LFOSC) over operating free-air temperature range (unless otherwise noted) PARAMETER | Col 4 | Low Frequency Oscillator (LFOSC) over operating free-air temperature range (unless otherwise noted) TEST CONDITIONS | Col 6 | Col 7 | Low Frequency Oscillator (LFOSC) over operating free-air temperature range (unless otherwise noted) MIN | Col 9 | Low Frequency Oscillator (LFOSC) over operating free-air temperature range (unless otherwise noted) TYP | Col 11 | Low Frequency Oscillator (LFOSC) over operating free-air temperature range (unless otherwise noted) MAX | Low Frequency Oscillator (LFOSC) over operating free-air temperature range (unless otherwise noted) UNIT |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | LFOSC frequency |  |  |  |  |  |  | 32768 |  |  | Hz |
| fLFOSC |  | LFOSC accuracy | -40 °C≤ T ≤ 125 °C a |  |  |  | -5 |  |  |  | 5 | % |
|  |  | LFOSC accuracy | -40 °C≤ T ≤ 85 °C a |  |  |  | -3 |  |  |  | 3 | % |
| I LFOSC |  | LFOSC current consumption |  |  |  |  |  |  | 300 |  |  | nA |
| t start, LFOSC |  | LFOSC start-up time |  |  |  |  |  |  | 1 |  |  | ms |
| 7.9.3 |  | Low Frequency Crystal/Clock over operating free-air temperature range (unless otherwise noted) |  |  |  |  |  |  |  |  |  |  |
|  |  | PARAMETER |  | TEST CONDITIONS |  | MIN |  | TYP | MAX | MAX |  | UNIT |
|  |  | Low frequency crystal oscillator (LFXT) |  |  |  |  |  |  |  |  |  |  |
| fLFXT |  | LFXT frequency |  |  |  |  |  | 32768 |  |  |  | Hz |
| DCLFXT |  | LFXT duty cycle |  |  |  | 30 |  |  |  | 70 |  | % |
| OALFXT |  | LFXT crystal oscillation allowance |  |  |  |  |  | 419 |  |  |  | kΩ |
| C L, eff |  | Integrated effective load capacitance | (1) |  |  |  |  | 1 |  |  |  | pF |
| t start, LFXT |  | LFXT start-up time |  |  |  |  |  | 1000 |  |  |  | ms |
| I LFXT |  | LFXT current consumption |  | XT1DRIVE=0, LOWCAP=1 |  |  |  | 300 |  |  |  | nA |
|  |  | Low frequency digital clock input (LFCLK_IN) |  |  |  |  |  |  |  |  |  |  |
| fLFIN |  | LFCLK_IN frequency (2) |  | SETUSEEXLF=1 |  | 29491 |  | 32768 | 36045 |  |  | Hz |
| DCLFIN |  | LFCLK_IN duty cycle (2) |  | SETUSEEXLF=1 |  | 40 |  |  |  | 60 |  | % |
| LFCLK Monitor |  |  |  |  |  |  |  |  |  |  |  |  |
| fFAULTLF |  | LFCLK monitor fault frequency (3) |  | MONITOR=1 |  | 2800 |  | 4200 | 8400 | 8400 |  | Hz |
| (1) and C (2) (3) 7.9.4 | LFXOUT | This includes parasitic bond and package capacitance (≈2pF per pin), calculated as C are the total capacitance at LFXIN and LFXOUT, respectively. The digital clock input (LFCLK_IN) accepts a logic level square wave clock. fault above the MAX fault frequency. High Frequency Crystal/Clock over operating free-air temperature range (unless otherwise noted) | The LFCLK monitor may be used to monitor the LFXT or LFCLK_IN. It will always fault below the MIN fault frequency, and will never | LFXIN | LFXIN ×C | ×CLFXOUT | LFXOUT/(C | LFXIN | +C | LFXOUT | ), where C | LFXIN |
|  |  | PARAMETER |  | TEST CONDITIONS |  | MIN |  | TYP | MAX | MAX |  | UNIT |
|  |  | High frequency crystal oscillator (HFXT) |  |  |  |  |  |  |  |  |  |  |
| fHFXT |  | HFXT frequency |  | HFXTRSEL=00 |  | 4 |  |  |  | 8 |  | MHz |
| fHFXT |  | HFXT frequency |  | HFXTRSEL=01 |  | 8.01 |  |  |  | 16 |  | MHz |
| fHFXT |  | HFXT frequency |  | HFXTRSEL=10 |  | 16.01 |  |  |  | 32 |  | MHz |
| DCHFXT |  | HFXT duty cycle |  | HFXTRSEL=00 |  | 40 |  |  |  | 65 |  | % |
| DCHFXT |  | HFXT duty cycle |  | HFXTRSEL=01 |  | 40 |  |  |  | 60 |  | % |
| DCHFXT |  | HFXT duty cycle |  | HFXTRSEL=10 |  | 40 |  |  |  | 60 |  | % |
| OAHFXT |  | HFXT crystal oscillation allowance |  | HFXTRSEL=00 (4 to 8MHz range) |  |  |  | 2 |  |  |  | kΩ |
| C L, eff |  | Integrated effective load capacitance | (1) |  |  |  |  | 1 |  |  |  | pF |
| t start, HFXT |  | HFXT start-up time (2) |  | HFXTRSEL=11, 32MHz crystal |  |  |  | 0.5 |  |  |  | ms |

| Col 1 | over operating free-air temperature range (unless otherwise noted) PARAMETER | over operating free-air temperature range (unless otherwise noted) TEST CONDITIONS | over operating free-air temperature range (unless otherwise noted) MIN | over operating free-air temperature range (unless otherwise noted) TYP | over operating free-air temperature range (unless otherwise noted) MAX | over operating free-air temperature range (unless otherwise noted) UNIT |
| --- | --- | --- | --- | --- | --- | --- |
| I HFXT | HFXT current consumption (2) | fHFXT=4MHz, Rm =300Ω, CL=12pF |  | 100 |  | uA |
| I HFXT | HFXT current consumption (2) | fHFXT=32MHz, Rm =30Ω, CL=12pF, Cm =6.26fF, L =1.76mH m |  | 600 |  | uA |
|  | High frequency digital clock input (HFCLK_IN) |  |  |  |  |  |
| fHFIN | HFCLK_IN frequency (3) | USEEXTHFCLK =1 | 4 |  | 32 | MHz |
| DCHFIN | HFCLK_IN duty cycle (3) | USEEXTHFCLK =1 | 40 |  | 60 | % |

(1) This includes parasitic bond and package capacitance (≈2pF per pin), calculated as C HFXIN ×CHFXOUT/(CHFXIN +CHFXOUT), where C HFXIN and C HFXOUT are the total capacitance at HFXIN and HFXOUT, respectively.

(2) The HFXT startup time (t start, HFXT) is measured from the time the HFXT is enabled until stable oscillation for a typical crystal. Start-up time is dependent upon crystal frequency and crystal specifications. Current consumption increases with higher RSEL and start up time is decreases with higher RSEL.

(3) The digital clock input (HFCLK_IN) accepts a logic level square wave clock.

### 7.10 Digital IO (p. 49)

#### 7.10.1 Electrical Characteristics (p. 49)

over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted)

| Col 1 | PARAMETER | Col 3 | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | ODIO (1) | VDD≥1.62V | 0.7*VDD |  | 5.5 | V |
| VIH | High level input voltage | ODIO | VDD≥2.7V | 2 |  | 5.5 | V |
| IH |  | All I/O except ODIO & Reset | VDD≥1.62V | 0.7*VDD |  | VDD+0.3 | V |
|  |  | ODIO | VDD≥1.62V | -0.3 |  | 0.3*VDD | V |
| VIL | Low level input voltage | ODIO | VDD≥2.7V | -0.3 |  | 0.8 | V |
| IL |  | All I/O except ODIO & Reset | VDD≥1.62V | -0.3 |  | 0.3*VDD | V |
|  |  | ODIO |  | 0.05*VDD |  |  | V |
| VHYS | Hysteresis | All I/O except ODIO |  | 0.1*VDD |  |  | V |
| I lkg | High-Z leakage current | SDIO(2) (3) |  |  |  | 50(4) | nA |
| RPU | Pull up resistance | All I/O except ODIO |  |  | 40 |  | kΩ |
| RPD | Pull down resistance |  |  |  | 40 |  | kΩ |
| C I | Input capacitance |  |  |  | 5 |  | pF |
| V | Low level output voltage | SDIO | VDD≥2.7V, \|I IO \|,max=6mA VDD≥1.71V, \|I IO \|,max=2mA |  |  | 0.4 | V |
| VOL | Low level output voltage | ODIO | VDD≥2.7V, I OL,max=8mA VDD≥1.71V, I OL,max=4mA |  |  | 0.5 | V |
| V | High level output voltage | SDIO | VDD ≥ 2.7V, I OH,max = 6mA | VDD-0.5 |  |  | V |
| VOH | High level output voltage | SDIO | VDD ≥ 1.71V, I OH,max = 2mA | VDD-0.4 |  |  | V |

(1) I/O Types: ODIO = 5V Tolerant Open-Drain , SDIO = Standard-Drive

(2) The leakage current is measured with VSS or VDD applied to the corresponding pin(s), unless otherwise noted.

(3) The leakage of the digital port pins is measured individually. The port pin is selected for input and the pullup/pulldown resistor is disabled.

| (4) 7.10.2 | 100nA. Switching Characteristics PARAMETER | Col 3 | This value is for SDIO not muxed with any analog inputs. If the SDIO is muxed with analog inputs then the leakage can be as high as over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted) TEST CONDITIONS | This value is for SDIO not muxed with any analog inputs. If the SDIO is muxed with analog inputs then the leakage can be as high as over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted) MIN | This value is for SDIO not muxed with any analog inputs. If the SDIO is muxed with analog inputs then the leakage can be as high as over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted) TYP | This value is for SDIO not muxed with any analog inputs. If the SDIO is muxed with analog inputs then the leakage can be as high as over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted) MAX | This value is for SDIO not muxed with any analog inputs. If the SDIO is muxed with analog inputs then the leakage can be as high as over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted) UNIT |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | SDIO (1) | VDD ≥ 1.71V, C L= 20pF |  |  | 16 |  |
| f max | Port output frequency | SDIO | VDD ≥ 2.7V, CL= 20pF |  |  | 32 | MHz |
|  |  | ODIO | VDD ≥ 1.71V, FM +, CL= 20pF - 100pF |  |  | 1 |  |
| tr,tf | Output rise/fall time | SDIO | VDD ≥ 1.71V, C L= 20pF |  |  | 3.5 | ns |
| tr,tf | Output rise/fall time | SDIO | VDD ≥ 2.7V, C L= 20pF |  |  | 6.6 | ns |
| tf | Output fall time | ODIO | VDD ≥ 1.71V, FM +, CL= 20pF-100pF | 20*VDD/5.5 |  | 120 | ns |

(1) I/O Types: ODIO = 5V Tolerant Open-Drain , SDIO = Standard-Drive

### 7.11 Analog Mux VBOOST (p. 50)

over operating free-air temperature range (unless otherwise noted)

| Col 1 | PARAMETER | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
| --- | --- | --- | --- | --- | --- | --- |
|  |  | MCLK/ULPCLK is LFCLK |  | 0.8 |  |  |
| IVBST | VBOOST current adder | MCLK/ULPCLK is not LFCLK, SYSOSC frequency is 32MHz |  | 20 |  | uA |
| t START,VBST | VBOOST startup time |  |  | 12 |  | us |

### 7.12 ADC (p. 50)

#### 7.12.1 Electrical Characteristics (p. 50)

over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted), all TYP values are measured at 25°C and all accuracy parameters are measured using 12-bit resolution mode (unless otherwise noted)

| Col 1 | PARAMETER | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
| --- | --- | --- | --- | --- | --- | --- |
| Vin (ADC) | Analog input voltage range (1) | Applies to all ADC analog input pins | 0 |  | VDD | V |
|  |  | VR+sourced from VDD |  | VDD |  | V |
| VR+ | Positive ADC reference voltage | VR+sourced from external reference pin (VREF+) | 1.4 |  | VDD | V |
|  |  | VR+sourced from internal reference (VREF) |  | VREF |  | V |
| VR- | Negative ADC reference voltage |  |  | 0 |  | V |
| FS | ADC sampling frequency | RES = 0x0 (12-bit mode), External Reference |  |  | 1.6 | Msps |
| FS | ADC sampling frequency | RES = 0x1 (10-bit mode), External Reference |  |  | 1.7 | Msps |
| FS | ADC sampling frequency | RES = 0x2 (8-bit mode), External Reference |  |  | 1.9 | Msps |
| I | Operating supply current | FS= 1.6MSPS, External reference, V R+= VDD |  | 350 |  | μA |
| I (ADC) | into VDD terminal | FS= 500ksps, Internal reference, V R+ = VREF = 2.5V |  | 300 |  | μA |
| CS/H | ADC sample-and-hold capacitance |  |  | 0.22 |  | pF |
| Rin | ADC sampling switch resistance |  |  | 15 |  | kΩ |
| ENOB | Effective number of bits | Internal reference, V R+ = VREF = 2.5V, F in= 10KHz | 9.4 | 10.2 |  | bit |
| ENOB | Effective number of bits | External reference, F in= 10KHz (2) | 10 | 10.6 |  | bit |
| ENOB | Effective number of bits | External reference, hardware averaging enabled (16 samples), F in= 10KHz (2) |  | 11.8 |  | bit |
| SNR | Signal-to-noise ratio | External reference (2) |  | 67 |  | dB |
| SNR | Signal-to-noise ratio | Internal reference, V R+ = VREF = 2.5V |  | 64 |  | dB |
| SNR | Signal-to-noise ratio | External reference (2), hardware averaging enabled (16 samples) |  | 75 |  | dB |

| measured at 25 | Col 2 | °C PARAMETER | Col 4 | Col 5 | over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted), all TYP values are and all accuracy parameters are measured using 12-bit resolution mode (unless otherwise noted) TEST CONDITIONS | Col 7 | Col 8 | over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted), all TYP values are and all accuracy parameters are measured using 12-bit resolution mode (unless otherwise noted) MIN | Col 10 | over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted), all TYP values are and all accuracy parameters are measured using 12-bit resolution mode (unless otherwise noted) TYP | Col 12 | over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted), all TYP values are and all accuracy parameters are measured using 12-bit resolution mode (unless otherwise noted) MAX | over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted), all TYP values are and all accuracy parameters are measured using 12-bit resolution mode (unless otherwise noted) UNIT |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  | External reference (2) | , VDD = VDD (min) to VDD (max) |  |  |  |  | 68 |  |  |  |
| PSRRDC |  | Power supply rejection ratio, DC | VDD = VDD | to VDD (min) (max) Internal reference, V R+ | (max) = VREF = 2.5V |  |  |  |  | 61 |  |  | dB |
|  |  |  |  | External reference (2) | , ΔVDD = 0.1 V at 1 kHz |  |  |  |  | 61 |  |  |  |
| PSRRAC |  | Power supply rejection ratio, AC |  | ΔVDD = 0.1 V at 1 kHz Internal reference, V R+ | = VREF = 2.5V |  |  |  |  | 49 |  |  | dB |
| T wakeup |  | ADC Wakeup Time |  | Assumes internal reference is active |  |  |  |  |  |  |  | 5 | us |
| V SupplyMon |  | Supply Monitor voltage divider (VDD/3) accuracy |  | ADC input channel: Supply Monitor | (3) |  |  | -1.5 |  |  |  | +1.5 | % |
| I SupplyMon |  | Supply Monitor voltage divider current consumption |  | ADC input channel: Supply Monitor |  |  |  |  |  | 10 |  |  | uA |
| (1) (2) (3) VDD/3. 7.12.2 | cap on VREF+ pin | All external reference specifications are measured with V Switching Characteristics | The analog input voltage range must be within the selected ADC reference voltage range V | R+ over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted) | R+ = VREF+ = VDD = 3.3V and V R-Analog power supply monitor. Analog input on channel 15 is disconnected and is internally connected to the voltage divider which is | to V | R– |  | = VREF- = VSS = 0V and external 1uF | for valid conversion results. = VREF- = VSS = 0V and external 1uF |  |  |  |
|  |  | PARAMETER |  |  | TEST CONDITIONS |  | MIN |  | TYP |  | MAX |  | UNIT |
| fADCCLK |  | ADC clock frequency |  |  |  |  | 4 |  |  |  | 32 |  | MHz |
| t ADC trigger |  | Software trigger minimum width |  |  |  |  | 3 |  |  |  |  |  | ADCCLK cycles |
| t Sample_step |  | Sampling time for step input |  | 12-bit mode, R S | = 50Ω, C pext= 10pF |  | 0.188 |  |  |  |  |  | µs |
| t Sample_VREF |  | Sample time with internal VREF input |  |  | ADC CHANNEL=29,12-bit mode, VDD as reference |  | 10 |  |  |  |  |  | µs |
| t Sample_SupplyMon |  | Sample time with Supply Monitor (VDD/3) |  | 12-bit mode |  |  | 5 |  |  |  |  |  | µs |
| 7.12.3 measured at 25 |  | Linearity Parameters °C |  |  | over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted), all TYP values are and all linearity parameters are measured using 12-bit resolution mode (unless otherwise noted) |  |  |  |  |  |  |  |  |
|  |  | PARAMETER |  |  | TEST CONDITIONS |  |  | MIN |  | TYP |  | MAX | UNIT |
| E I |  | Integral linearity error (INL) | External reference | (1) |  |  |  | -2.0 |  |  |  | +2.0 | LSB |
| ED |  | Differential linearity error (DNL) Guaranteed no missing codes |  | External reference, 12-bit (1) |  |  |  | -1.0 |  |  |  | +1.0 | LSB |
| EO | Offset error |  | External reference | (1) |  |  |  | -5 |  |  |  | 5 | mV |
| EG | Gain error |  | External reference | (1) |  |  |  | -6 |  |  |  | 6 | LSB |

(1) All external reference specifications are measured with VR+ = VREF+ = VDD = 3.3V and VR- = VREF- = VSS = 0V and external 1uF cap on VREF+ pin

#### 7.12.4 Typical Connection Diagram (p. 52)

Figure 7-2. ADC Input Network

1. Refer to Electrical Characteristics for the values of R in and C S/H 2. Refer to Electrical Characteristics for the value of C I 3. Cpar and R par represent the parasitic capacitance and resistance of the external ADC input circuitry

Use the following equations to solve for the minimum sampling time (T) required for an ADC conversion: 1. Tau = (R par+ R in)* C S/H+ R par*(Cpar+ C I) 2. K= ln(2 n/Settling error) – ln((C par+ C I)/CS/H) 3. T (Min sampling time) = K*Tau

### 7.13 Temperature Sensor (p. 52)

over operating free-air temperature range (unless otherwise noted)

| Col 1 | Col 2 | PARAMETER | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
| --- | --- | --- | --- | --- | --- | --- | --- |
| TSTRIM | TRIM | Factory trim temperature (1) | ADC and VREF configuration: RES=0 (12-bit mode), VRSEL=2h (internal VREF), BUFCONFIG=1h (1.4V VREF), ADC t Sample=10µs | 27 | 30 | 33 | °C |
| TS c |  | Temperature coefficient |  | -2.05 | -1.89 | -1.75 | mV/°C |
| t SET, TS |  | Temperature sensor settling time (2) |  |  | 2.5 | 10 | us |
| (1) (2) |  | Higher absolute accuracy may be achieved through user calibration. minimum ADC sample time when measuring the temperature sensor. | This is the maximum time required for the temperature sensor to settle when measured by the ADC. It may be used to specify the |  |  |  |  |

### 7.14 VREF (p. 53)

#### 7.14.1 Electrical Characteristics (p. 53)

over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted)

| Col 1 | PARAMETER | TEST CONDITIONS | Col 5 | Col 6 | MIN | TYP | TYP | MAX | UNIT |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| IVREF | VREF operating supply current | BUFCONFIG = {0, 1}, No load |  |  |  |  | 80 | 100 | µA |
| TCVREF | Temperature coefficient of VREF (1) | BUFCONFIG = {0, 1} |  |  |  |  |  | 75 | ppm/°C |
| TCdrift | Long term VREF drift | Time = 1000 hours, BUFCONFIG = {0, 1}, T = 25 | °C |  |  |  |  | 300 | ppm |
| PSRR | VREF Power supply rejection ratio, | VDD = 1.7 V to VD Dmax, BUFCONFIG = 1 |  |  | 60 |  | 70 |  | dB |
| PSRRDC | DC | VDD = 2.7 V to VD Dmax, BUFCONFIG = 0 |  |  | 49 |  | 60 |  | dB |
| V | RMS noise at VREF output (0.1 Hz | BUFFCONFIG = 1 |  |  |  |  | 500 |  | µVrms |
| Vnoise | to 100 MHz) | BUFFCONFIG = 0 |  |  |  |  | 900 |  | µVrms |
| ADC F S | Max supported ADC sampling frequency | Using VREF as ADC reference |  |  |  |  |  | 515 | ksps |
| T startup | VREF startup time | BUFCONFIG = {0, 1} , VDD = 2.8 V |  |  |  |  |  | 30 | us |
| (1) reference. 7.14.2 | The temperature coefficient of the VREF output is the sum of TC reference. Voltage Characteristics | VRBUF over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted) | and the temperature coefficient of the internal bandgap | and the temperature coefficient of the internal bandgap |  |  |  |  |  |
|  | PARAMETER | TEST CONDITIONS |  | MIN |  | TYP |  | MAX | UNIT |
| VDD | Minimum supply voltage needed for | BUFCONFIG = 1 |  | 1.62 |  |  |  |  | V |
| VDD min | VREF operation | BUFCONFIG = 0 |  | 2.7 |  |  |  |  | V |
| VREF | Voltage reference output voltage | BUFCONFIG = 1 |  | 1.38 |  | 1.4 |  | 1.42 | V |
| VREF | Voltage reference output voltage | BUFCONFIG = 0 |  | 2.46 |  | 2.5 |  | 2.54 | V |

### 7.15 Comparator (COMP) (p. 53)

#### 7.15.1 Comparator Electrical Characteristics (p. 53)

over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted)

| Col 1 | PARAMETER | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
| --- | --- | --- | --- | --- | --- | --- |
|  | Comparator Electrical Characteristics |  |  |  |  |  |
| Vcm | Common mode input range |  | 0 |  | VDD | V |
| Voffset | Input offset voltage |  | -20 |  | 20 | mV |
|  |  | HYST=00h |  | 0.4 |  |  |
| V | DC input hysteresis | HYST=01h |  | 10 |  | mV |
| V hys | DC input hysteresis | HYST=02h |  | 20 |  | mV |
|  |  | HYST=03h |  | 30 |  |  |
| t | Propagation delay, response | Output Filter off, Overdrive = 100 mV, High Speed Mode |  | 32 | 50 | ns |
| t PD_ls | time | Output Filter off, Overdrive = 100 mV, Low Power Mode |  | 1.2 | 4 | µs |
| t | Comparator enable time | Startup time to reach propagation delay specification, High Speed Mode |  |  | 5 | µs |
| t en | Comparator enable time | Startup time to reach propagation delay specification, Low Power Mode |  |  | 10 | µs |

| Col 1 | PARAMETER | over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted) TEST CONDITIONS | over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted) MIN | over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted) TYP | over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted) MAX | over recommended ranges of supply voltage and operating free-air temperature (unless otherwise noted) UNIT |
| --- | --- | --- | --- | --- | --- | --- |
|  |  | Vcm = VDD/2, 100mV overdrive, DAC output as a voltage reference, VDD is reference for DAC, High Speed Mode |  | 130 | 200 | µA |
| I comp | Comparator current consumption. | Vcm = VDD/2, 100mV overdrive, DAC output as a voltage reference, VDD is reference for DAC, Low Power Mode |  | 0.85 | 2.7 | µA |
|  |  | Vcm = VDD/2, 100mV overdrive, comparator only. High Speed Mode |  | 120 | 180 | µA |
|  |  | Vcm = VDD/2, 100mV overdrive, comparator only, Low Power Mode |  | 0.7 | 2.1 | µA |
|  | 8-bit DAC Electrical Characteristics |  |  |  |  |  |
| Vdac | DAC output range |  | 0 |  | VDD | V |
| Vdac-code | 8-bit DAC output voltage for a given code | VIN = reference voltage into 8-bit DAC, code n = 0 to 255 |  | VIN × (n+1) / 256 |  | V |
| INL | Integral nonlinearity of 8-bit DAC |  | -1 |  | 1 | LSB |
| DNL | Differential nonlinearity of 8-bit DAC |  | -1 |  | 1 | LSB |
| Gain error | Gain error of 8-bit DAC | Reference voltage = VDD | -2 |  | 2 | % of FSR |
| Offset error | Offset error of 8-bit DAC |  | -5 |  | 5 | mV |
| Output Impedance | 8-bit DAC output impedance |  |  | 50 |  | kΩ |
| t dac_settle | 8-bit DAC settling time in static mode | DACCODE0 = 0 → 255, DAC output accurate to 1 LSB, DAC output on pin PA11, Cload = 15pF |  | 6 |  | µs |
| t dac_settle | 8-bit DAC settling time in static mode | DACCODE0 = 0 → 255, DAC output accurate to 1 LSB |  | 1.5 |  | µs |

### 7.16 I2C (p. 54)

#### 7.16.1 I2C Characteristics (p. 54)

over operating free-air temperature range (unless otherwise noted)

| Col 1 | PARAMETERS PARAMETERS | TEST CONDITIONS TEST CONDITIONS | Standard mode MIN | Standard mode MAX | Fast mode MIN | Fast mode MAX | Fast mode plus MIN | Fast mode plus MAX | UNIT UNIT |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| fI2C | I2C input clock frequency | I2C in Power Domain0 | 2 | 32 | 8 | 32 | 20 | 32 | MHz |
| fSCL | SCL clock frequency |  |  | 0.1 |  | 0.4 |  | 1 | MHz |
| t HD,STA | Hold time (repeated) START |  | 4 |  | 0.6 |  | 0.26 |  | us |
| tLOW | Low period of the SCL clock |  | 4.7 |  | 1.3 |  | 0.5 |  | us |
| tHIGH | High period of the SCL clock |  | 4 |  | 0.6 |  | 0.26 |  | us |
| t SU,STA | Setup time for a repeated START |  | 4.7 |  | 0.6 |  | 0.26 |  | us |
| t HD,DAT | Data hold time |  | 0 |  | 0 |  | 0 |  | ns |
| t SU,DAT | Data setup time |  | 250 |  | 100 |  | 50 |  | ns |
| t SU,STO | Setup time for STOP |  | 4 |  | 0.6 |  | 0.26 |  | us |
| tBUF | Bus free time between a STOP and START condition |  | 4.7 |  | 1.3 |  | 0.5 |  | us |
| t VD;DAT | Data valid time |  |  | 3.45 |  | 0.9 |  | 0.45 | us |
| t VD;ACK | Data valid acknowledge time |  |  | 3.45 |  | 0.9 |  | 0.45 | us |

| 7.16.2 | I2C Filter over operating free-air temperature range (unless otherwise noted) PARAMETERS | I2C Filter over operating free-air temperature range (unless otherwise noted) TEST CONDITIONS | I2C Filter over operating free-air temperature range (unless otherwise noted) MIN | I2C Filter over operating free-air temperature range (unless otherwise noted) TYP | I2C Filter over operating free-air temperature range (unless otherwise noted) MAX | I2C Filter over operating free-air temperature range (unless otherwise noted) UNIT |
| --- | --- | --- | --- | --- | --- | --- |
|  |  | AGFSELx = 0 |  | 6 |  | ns |
| f | Pulse duration of spikes suppressed by | AGFSELx = 1 |  | 14 | 35 | ns |
| fSP | input filter | AGFSELx = 2 |  | 22 | 60 | ns |
|  |  | AGFSELx = 3 |  | 35 | 90 | ns |

#### 7.16.3 I2 C Timing Diagram (p. 55)

Figure 7-3. I2C Timing Diagram

### 7.17 SPI (p. 55)

#### 7.17.1 SPI (p. 55)

over operating free-air temperature range (unless otherwise noted)

| Col 1 | Col 3 | PARAMETERS | Col 5 | Col 6 | Col 7 | Col 8 | Col 9 | MIN | TYP | MAX | UNIT |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| SPI |  |  |  |  |  |  |  |  |  |  |  |
| fSPI |  | SPI clock frequency |  |  |  | Clock max speed = 32MHz 1.62 < VDD < 3.6V Controller mode |  |  |  | 16 | MHz |
| fSPI |  | SPI clock frequency |  |  |  | Clock max speed = 32MHz 1.62 < VDD < 3.6V Peripheral mode |  |  |  | 16 | MHz |
| DCSCK |  | SCK Duty Cycle |  |  |  |  |  | 40 | 50 | 60 | % |
| Controller |  |  |  |  |  |  |  |  |  |  |  |
| t SCLK_H/L |  | SCLK High or Low time | SCLK High or Low time |  |  |  |  | (tSPI/2) -1 | tSPI / 2 | (tSPI/2) + 1 | ns |
| tCS.LEAD |  |  | CS lead-time, CS active to clock |  |  |  |  | 1 SPI Clock |  |  | ns |
| tCS.LAG | inactive | inactive | CS lag time, Last clock to CS |  |  |  |  | 1 SPI Clock |  |  | ns |
| tCS.ACC | data out | data out | CS access time, CS active to PICO |  |  |  |  |  |  | 1/2 SPI Clock | ns |
| tCS.DIS |  | PICO high inpedance | CS disable time, CS inactive to |  |  |  |  |  |  | 1 SPI Clock | ns |
| tSU.CI |  |  | POCI input data setup time | (1) | enabled enabled |  | 2.7 < VDD < 3.6V, delayed sampling 1.62 < VDD < 2.7V, delayed sampling | 1 8 |  |  | ns |
| tSU.CI |  |  | POCI input data setup time | (1) |  |  | 2.7 < VDD < 3.6V, no delayed sampling 1.62 < VDD < 2.7V, no delayed sampling | 30 39 |  |  | ns |

| Col 1 | Col 2 | over operating free-air temperature range (unless otherwise noted) PARAMETERS | Col 4 | Col 5 | over operating free-air temperature range (unless otherwise noted) TEST CONDITIONS | over operating free-air temperature range (unless otherwise noted) MIN | over operating free-air temperature range (unless otherwise noted) TYP | over operating free-air temperature range (unless otherwise noted) MAX | over operating free-air temperature range (unless otherwise noted) UNIT |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| tHD.CI |  | POCI input data hold time | No delayed sampling | No delayed sampling |  | 0 |  |  | ns |
| t |  | PICO output data valid time (2) |  |  |  |  |  | 16 | ns |
| tHD.CO |  | PICO output data hold time (3) |  |  |  | 1 |  |  | ns |
|  | Peripheral |  |  |  |  |  |  |  |  |
| t |  | CS lead-time, CS active to clock |  |  |  | 13.5 |  |  | ns |
| tCS.LAG |  | CS lag time, Last clock to CS inactive |  |  |  | 1 |  |  | ns |
| tCS.ACC |  | CS access time, CS active to POCI data out |  |  |  |  |  | 40 | ns |
| tCS.DIS |  | CS disable time, CS inactive to POCI high impedance |  |  |  |  |  | 40 | ns |
| tSU.PI |  | PICO input data setup time |  |  |  | 15 |  |  | ns |
| tHD.PI |  | PICO input data hold time |  |  |  | 2.5 |  |  | ns |
| t |  | POCI output data valid time (2) | 2.7 < VDD < 3.6V |  |  |  |  | 31 | ns |
| t |  | POCI output data valid time (2) | 1.62 < VDD < 2.7V |  |  |  |  | 40 | ns |
| tHD.PO |  | POCI output data hold time (3) |  |  |  | 5.5 |  |  | ns |
| (1) (2) (3) 7.17.2 |  | Specifies how long data on the output is valid after the output changing SCLK clock edge SPI Timing Diagram | The POCI input data setup time can be fully compensated when delayed sampling feature is enabled. Specifies the time to drive the next valid data to the output after the output changing SCLK clock edge Specifies how long data on the output is valid after the output changing SCLK clock edge |  |  |  |  |  |  |

Figure 7-4. SPI Timing Diagram - Controller Mode

Figure 7-5. SPI Timing Diagram - Peripheral Mode

### 7.18 UART (p. 57)

over operating free-air temperature range (unless otherwise noted)

| Col 1 | PARAMETERS | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
| --- | --- | --- | --- | --- | --- | --- |
| fUART | UART input clock frequency |  |  |  | 32 | MHz |
| fBITCLK | BITCLK clock frequency(equals baud rate in MBaud) |  |  |  | 4 | MHz |
|  |  | AGFSELx = 0 |  | 6 |  | ns |
| t | Pulse duration of spikes | AGFSELx = 1 |  | 14 | 35 | ns |
| tSP | suppressed by input filter | AGFSELx = 2 |  | 22 | 60 | ns |
|  |  | AGFSELx = 3 |  | 35 | 90 | ns |

### 7.19 TIMx (p. 57)

over operating free-air temperature range (unless otherwise noted)

| Col 1 | PARAMETERS | Col 5 | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | fTIMxCLK= 64MHz |  | 15.625 |  |  | ns |
| t res | Timer resolution time | fTIMxCLK= 32MHz |  | 31.25 |  |  | ns |
|  |  |  |  | 1 |  |  | t |
|  |  |  |  | 1 |  |  | tTIMxCLK |
|  |  | fTIMxCLK= 64MHz |  |  |  | 1024 | us |
| tCOUNTER | 16-bit counter clock period | fTIMxCLK= 32MHz |  |  |  | 2048 | us |
|  |  |  |  | 1 |  | 65536 | tTIMxCLK |

### 7.20 Emulation and Debug (p. 57)

#### 7.20.1 SWD Timing (p. 57)

over operating free-air temperature range (unless otherwise noted)

| Col 1 | Col 3 | PARAMETER | TEST CONDITIONS | MIN | TYP | MAX | UNIT |
| --- | --- | --- | --- | --- | --- | --- | --- |
| fSWD | SWD frequency |  |  |  |  | 10 | MHz |


---

---
document: "MSPM0C1105, MSPM0C1106 Mixed-Signal Microcontrollers datasheet (Rev. B)"
section_id: "08"
section_title: "8 Detailed Description"
pages: "58-78"
---

# 8 Detailed Description (pp. 58-78)

## 8 Detailed Description (p. 58)

The following sections describe all of the components that make up the devices in this data sheet. The peripherals integrated into these devices are configured by software through Memory Mapped Registers (MMRs). For more details, see the corresponding chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.1 Overview (p. 59)

MSPM0C1105/6 microcontrollers (MCUs) are part of MSP's highly integrated, ultra-low-power 32-bit MSPM0 MCU family based on the Arm® Cortex®-M0+ 32-bit core platform, operating at up to 32MHz frequency. These costoptimized MCUs offer high-performance analog peripheral integration, support extended temperature ranges from -40°C to 125°C, and operate with supply voltages from 1.62V to 3.6V.

The MSPM0C1105/6 devices provide up to 64KB embedded flash program memory with 8KB SRAM. These MCUs incorporate a high-speed on-chip oscillator with an accuracy from -2.1% to +1.6%, eliminating the need for an external crystal. Additional features include a 3-channel DMA, CRC-16 accelerator, and a variety of high-performance analog peripherals such as one 12-bit 1.6Msps ADC with VDD as the voltage reference, a comparator with 8-bit reference DAC and an on-chip temperature sensor. These devices also offer intelligent digital peripherals such as one 16-bit advanced timer with deadband and timer frequency up to 64MHz, four 16-bit general purpose timer, one windowed watchdog timer, and a variety of communication peripherals including three UART, one SPI, and two I2C. These communication peripherals offer protocol support for LIN, IrDA, DALI, Manchester, smart card, SMBus, and PMBus.

The TI MSPM0 family of low-power MCUs consists of devices with varying degrees of analog and digital integration allowing for customers find the MCU that meets their project's needs. The MSPM0 MCU platform combines the Arm Cortex-M0+ platform with a holistic ultra-low-power system architecture, allowing system designers to increase performance while reducing energy consumption.

For complete module descriptions, see the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.2 CPU (p. 59)

The CPU subsystem (MCPUSS) implements an Arm Cortex-M0+ CPU, an instruction pre-fetch/cache, a system timer, and interrupt management features. The Arm Cortex-M0+ is a cost-optimized, 32-bit CPU which delivers high performance and low power to embedded applications. Key features of the CPU Sub System include:

- Arm Cortex-M0+ CPU supporting clock frequencies up to 32MHz – ARMv6-M Thumb instruction set (little endian) with 32-cycle 32x32 multiply instruction

- Pre-fetch logic to improve sequential code execution, and I-cache with two 64-bit cache lines

- System timer (SysTick) with 24-bit down counter and automatic reload

- Nested vectored interrupt controller (NVIC) with 4 programmable priority levels and tail-chaining

### 8.3 Operating Modes (p. 59)

MSPM0 MCUs provide five main operating modes (power modes) to allow for optimization of the device power consumption based on application requirements. In order of decreasing power, the modes are: RUN, SLEEP, STOP, STANDBY, and SHUTDOWN. The CPU is active executing code in RUN mode. Peripheral interrupt events can wake the device from SLEEP, STOP, or STANDBY mode to the RUN mode. SHUTDOWN mode completely disables the internal core regulator to minimize power consumption, and wake is only possible via NRST, SWD, or a logic level match on certain IOs. RUN, SLEEP, STOP, and STANDBY modes also include several configurable policy options (for example, RUN.x) for balancing performance with power consumption.

To further balance performance and power consumption, MSPM0 devices implement two power domains: PD1 (for the CPU, memories, and high performance peripherals), and PD0 (for low speed, low power peripherals).

- PD1 is always powered in RUN and SLEEP modes, but is disabled in all other modes.

- PD0 is always powered in RUN, SLEEP, STOP, and STANDBY modes.

- PD1 and PD0 are both disabled in SHUTDOWN mode.

#### 8.3.1 Functionality by Operating Mode (p. 59)

Supported functionality in each operating mode is given in Supported Functionality by Operating Mode.

Functional key:

- EN: The function is enabled in the specified mode.

- DIS: The function is disabled (either clock or power gated) in the specified mode, but the function's configuration is retained.

- OPT: The function is optional in the specified mode, and remains enabled if configured to be enabled.

- NS: The function is not automatically disabled in the specified mode but is not supported.

- OFF: The function is fully powered off in the specified mode, and no configuration information is retained. When waking up from an OFF state, all module registers must be re-configured to the desired settings by application software.

**Table 8-1. Supported Functionality by Operating Mode (pp. 60-61)**

| Col 1 | Col 2 | Col 3 | RUN | Col 5 | Col 6 | SLEEP | Col 8 | STOP | STOP | STANDBY | STANDBY | SHUTD |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Operating Mode | Operating Mode | RUN0 | RUN1 | RUN2 | SLEEP0 | SLEEP1 | SLEEP2 | STOP0 | STOP2 | STANDBY0 | STANDBY1 | OWN |
|  | SYSOSC | EN | EN | DIS | EN | EN | DIS | OPT(1) | DIS | DIS | DIS | OFF |
| Oscillators | LFOSC or LFXT |  |  |  |  | EN | EN |  |  |  |  | OFF |
|  | HFXT | EN | DIS | DIS | OPT |  |  | DIS | DIS |  |  | OFF |
|  | CPUCLK | 32M | 32k | 32k |  |  |  | DIS |  |  |  | OFF |
|  | MCLK to PD1 | 32M | 32k | 32k | 32M | 32k | 32k |  | DIS | DIS |  | OFF |
|  | ULPCLK to PD0 | 32M | 32k | 32k | 32M | 32k | 32k | 4M (1) | 32k | 32k | DIS | OFF |
|  | ULPCLK to TIMG14/8 | 32M | 32k | 32k | 32M | 32k | 32k | 4M (1) |  | 32k |  | OFF |
| Clocks | RTCCLK |  |  |  |  | 32k | 32k |  |  |  |  | OFF |
| Clocks | MFCLK | OPT | DIS | DIS | OPT | DIS | DIS | OPT |  | DIS |  | OFF |
|  | LFCLK |  |  |  |  | 32k |  |  |  |  | DIS | OFF |
|  | LFCLK to TIMG14/8 |  |  |  |  | 32k | 32k |  |  |  |  | OFF |
|  | MCLK Monitor |  |  |  |  | OPT |  |  |  |  | DIS | OFF |
|  | LFCLK Monitor |  |  |  |  | OPT | OPT |  |  |  |  | OFF |
|  | POR Monitor |  |  |  |  |  | EN |  |  |  |  |  |
| PMU | BOR Monitor |  |  |  |  | EN | EN |  |  |  |  | OFF |
|  | Core Regulator |  |  | FULL DRIVE | FULL DRIVE |  |  | REDUCED DRIVE | REDUCED DRIVE | LOW DRIVE | LOW DRIVE | OFF |
|  | CPU |  | EN |  |  |  |  | DIS |  |  |  | OFF |
| Core Functions | DMA |  |  | OPT | OPT |  |  |  | NS (triggers supported) | NS (triggers supported) |  | OFF |
| Core Functions | Flash |  |  | EN | EN |  |  | OPT | OPT | DIS | DIS | OFF |
|  | SRAM |  |  | EN | EN |  |  | OPT | OPT | DIS | DIS | OFF |
| PD1 Peripherals | SPI0 |  |  | OPT | OPT |  |  |  | DIS | DIS |  | OFF |
| PD1 Peripherals | CRC |  |  | OPT | OPT |  |  |  | DIS | DIS |  | OFF |
|  | TIMG14/8 |  |  |  |  | OPT | OPT |  |  |  |  | OFF |
|  | TIMG1/2 |  |  |  |  | OPT |  |  |  |  | DIS | OFF |
|  | TIMA0 |  |  |  |  | OPT |  |  |  |  | DIS | OFF |
|  | UART0/1/2 |  |  |  |  | OPT |  |  |  |  | DIS | OFF |
| PD0 Peripherals | I2C0/1 |  |  |  |  | OPT |  |  |  |  | DIS | OFF |
|  | GPIOA/B |  |  |  |  | OPT |  |  |  |  | OPT2 | OFF |
|  | WWDT0 |  |  |  |  | OPT |  |  |  |  | OPT(2) | OFF |
|  | IWDT |  |  |  |  | OPT | OPT |  |  |  |  | OFF |
|  | RTC_B |  |  |  |  | OPT | OPT |  |  |  |  | OFF |
| Operating Mode | Operating Mode | RUN0 | RUN1 | RUN2 | SLEEP0 | SLEEP1 | SLEEP2 | STOP0 | STOP2 | STANDBY0 | STANDBY1 | OWN |
|  | ADC0 |  |  |  | OPT |  |  |  |  | NS (triggers supported) |  | OFF |
|  | COMP |  |  |  |  | OPT | OPT |  |  |  |  | OFF |
| Analog | VREF |  |  |  | OPT |  |  |  |  | NS |  | OFF |
|  | Temperature Sensor |  |  |  | OPT | OPT |  |  |  | OFF | OFF | OFF |
| IOMUX and IO Wakeup |  |  |  |  |  | EN | EN |  |  |  |  | DIS w/ WAKE |
| Wake Sources |  |  | N/A |  |  | ANY IRQ |  |  | PD0 IRQ | PD0 IRQ |  | IOMUX, NRST, SWD |

(1) If STOP0 is entered from RUN1 (SYSOSC enabled but MCLK sourced from LFCLK), SYSOSC remains enabled as it was in RUN1, and ULPCLK remains at 32kHz as it was in RUN1. If STOP0 is entered from RUN2 (SYSOSC was disabled and MCLK was sourced from LFCLK), SYSOSC remains disabled as it was in RUN2, and ULPCLK remains at 32kHz as it was in RUN2.

(2) When using the STANDBY1 policy for STANDBY, only specific peripherals (TIMG14, TIMG8, IWDT, and RTC) are clocked. Other PD0 peripherals can generate an asynchronous fast clock request upon external activity but are not actively clocked.

### 8.4 Power Management Unit (PMU) (p. 61)

The power management unit (PMU) generates the internally regulated core supplies for the device and provides supervision of the external supply (VDD). The PMU also contains the bandgap voltage reference used by the PMU itself as well as analog peripherals. Key features of the PMU include:

- Power-on reset (POR) supply monitor

- Brown-out reset (BOR) supply monitor with early warning capability using three programmable thresholds

- Core regulator with support for RUN, SLEEP, STOP, and STANDBY mode to dynamically balance performance with power consumption

- Parity-protected trim to immediately generate a power-on reset (POR) in the event that a power management trim is corrupted

- 4 bytes of shutdown memory

For more details, see the PMU chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.5 Clock Module (CKM) (p. 61)

The clock module provides the following oscillators:

- LFOSC: Internal low-frequency oscillator (32kHz)

- SYSOSC: Internal high-frequency oscillator (32MHz with factory trim)

- LFXT/LFCKIN: low-frequency external crystal oscillator or digital clock input (32kHz)

- HFXT/HFCKIN: high-frequency external crystal oscillator or digital clock input (4MHz to 32MHz)

The following clocks are distributed by the clock module for use by the processor, bus, and peripherals:

- MCLK: Main system clock for PD1 peripherals, derived from SYSOSC, LFCLK, or HSCLK, active in RUN and SLEEP modes

- CPUCLK: Clock for the processor (derived from MCLK), active in RUN mode

- ULPCLK: Ultra-low power clock for PD0 peripherals, active in RUN, SLEEP, STOP, and STANDBY modes

- MFCLK: 4MHz fixed mid-frequency clock for peripherals, available in RUN, SLEEP, and STOP modes

- LFCLK: 32kHz fixed low-frequency clock for peripherals or MCLK, active in RUN, SLEEP, STOP, and STANDBY modes

- ADCCLK: ADC clock, available in RUN, SLEEP and STOP modes

- RTCCLK: Fixed 32kHz clock direct to RTC

- CLK_OUT: Used to output a clock externally, available in RUN, SLEEP, STOP, and STANDBY modes

- HFCLK: High frequency clock derived from HFXT or HFCLK_IN, available in RUN and SLEEP mode

- HSCLK: High speed clock derived from HFCLK, available in RUN and SLEEP mode

For more details, see the CKM chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.6 DMA_B (p. 62)

The direct memory access (DMA) controller allows movement of data from one memory address to another without CPU intervention. For example, the DMA can be used to move data from ADC conversion memory to SRAM. The DMA reduces system power consumption by allowing the CPU to remain in low power mode, without having to awaken to move data to or from a peripheral.

The DMA_B in these devices support the following key features:

- 3 DMA transfer channel – 2 full-feature channels, supporting repeated transfer modes – 1 basic channel, supporting single transfer mode

- Configurable DMA channel priorities

- Direct peripheral to DMA trigger is supported from ADC, UART, SPI or timer triggers.

- Byte (8-bit), short word (16-bit) and word (32-bit) or mixed byte and word transfer capability

- Transfer counter block size supports up to 64k transfers of any data type

- Configurable DMA transfer trigger selection

- Active channel interruption to service other channels

- Early interrupt generation for ping-pong buffer architecture

- Cascading channels upon completion of activity on another channel

- Stride mode to support data re-organization, such as 3-phase metering applications

- Gather mode

Table 8-2 shows the DMA features that are supported and the corresponding DMA channel numbers.

**Table 8-2. DMA_B Channel Features (p. 62)**

| DMA Feature | DMA_B | DMA_B |
| --- | --- | --- |
| DMA Feature | Full-Feature Channel | Basic Channel |
| Channel Number | 0, 1 | 2 |
| Repeated mode | ✓ | – |
| Table & fill mode | ✓ | – |
| Gather mode | ✓ | – |
| Early IRQ notification | ✓ | – |
| Auto enable | ✓ | ✓ |
| Long long (128-bit) transfer | ✓ | ✓ |
| Stride mode | ✓ | ✓ |
| Cascading channel support | ✓ | ✓ |

Table 8-3 lists the available triggers for the DMA which are configured using the DMATCTL.DMATSEL control bits in the DMA memory mapped registers.

**Table 8-3. DMA Trigger Mapping (p. 62)**

| DMACTL.DMATSEL | TRIGGER SOURCE |
| --- | --- |
| 0 | Software |
| 1 | Generic Subscriber 0 (FSUB_0) |
| 2 | Generic Subscriber 0 (FSUB_1) |
| 3 | I2C0 PUBLISHER 1 |
| 4 | I2C0 PUBLISHER 2 |

5 I2C1 PUBLISHER 1

**Table 8-3. DMA Trigger Mapping (continued) (p. 63)**

| DMACTL.DMATSEL | TRIGGER SOURCE |
| --- | --- |
| 6 | I2C1 PUBLISHER 2 |
| 7 | SPI0 PUBLISHER 1 |
| 8 | SPI0 PUBLISHER 2 |
| 9 | UART0 PUBLISHER 1 |
| 10 | UART0 PUBLISHER 2 |
| 11 | UART1 PUBLISHER 1 |
| 12 | UART1 PUBLISHER 2 |
| 13 | UART2 PUBLISHER 1 |
| 14 | UART2 PUBLISHER 2 |
| 15 | ADC0 DMA Trigger |

### 8.7 Events (p. 63)

The event manager transfers digital events from one entity (for example, a peripheral) to another (for example, a second peripheral, the DMA, or the CPU). The event manager implements event transfer through a defined set of event publishers (generators) and subscribers (receivers) which are interconnected through an event fabric containing a combination of static and programmable routes.

Events which are transferred by the event manager include:

- Peripheral event transferred to the CPU as an interrupt request (IRQ) (Static Event) – Example: RTC interrupt is sent to the CPU

- Peripheral event transferred to the DMA as a DMA trigger (DMA Event) – Example: UART data receive trigger to DMA to request a DMA transfer

- Peripheral event transferred to another peripheral to directly trigger an action in hardware (Generic Event) – Example: TIMx timer peripheral publishes a periodic event to the ADC subscriber port, and the ADC uses the event to trigger start-of-sampling

For more details, see the EVENT chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

Table 8-4. Generic Event Channels

A generic route is either a point-to-point (1:1) route or a point-to-two (1:2) splitter route in which the peripheral publishing the event is configured to use one of several available generic route channels to publish its event to another entity (or entities, in the case of a splitter route), where an entity may be another peripheral, a generic DMA trigger event, or a generic CPU event.

| CHANID | Generic Route Channel Selection | Channel Type |
| --- | --- | --- |
| 0 | No generic event channel selected | N/A |
| 1 | Generic event channel 1 selected | 1 : 1 |
| 2 | Generic event channel 2 selected | 1 : 1 |
| 3 | Generic event channel 3 selected | 1 : 1 |
| 4 | Generic event channel 4 selected | 1 : 1 |
| 6 | Generic event channel 5 selected | 1:1 |
| 7 | Generic event channel 5 selected | 1 : 2 (splitter) |
| 8 | Generic event channel 6 selected | 1 : 2 (splitter) |

### 8.8 Memory (p. 63)

#### 8.8.1 Memory Organization (p. 63)

Table 8-5 summarizes the memory map of the devices. For more information about the memory region detail, see the Platform Memory Map section in the MSPM0 C-Series Microcontrollers Technical Reference Manual.

**Table 8-5. Memory Organization (p. 64)**

| Memory Region | Subregion | MSPM0C1105 | MSPM0C1106 |
| --- | --- | --- | --- |
| Code (Flash) | Flash | 32KB(1) 0x0000.0000 to 0x0000.7FFF | 64KB(1) 0x0000.0000 to 0x0000.FFFF |
| SRAM (SRAM) | SRAM | 8KB 0x2000.0000 to 0x2000.1FFF | 8KB 0x2000.0000 to 0x2000.1FFF |
|  | Peripherals | 0x4000.0000 to 0x40FF.FFFF | 0x4000.0000 to 0x40FF.FFFF |
|  | Flash | 0x0040.0000 to 0x0040.7FFF | 0x0040.0000 to 0x0040.9FFF |
| Peripheral | Configuration NVM | 512 bytes 0x41C0.0000 to 0x41C0.0200 | 512 bytes 0x41C0.0000 to 0x41C0.0200 |
|  | FACTORY | 0x41C4.0000 to 0x41C4.0080 | 0x41C4.0000 to 0x41C4.0080 |
|  | Subsystem | 0x6000.0000 to 0x7FFF.FFFF | 0x6000.0000 to 0x7FFF.FFFF |
|  | System PPB | 0xE000.0000 to 0xE00F.FFFF | 0xE000.0000 to 0xE00F.FFFF |

(1) First 32KB flash memory (address 0x0000.0000 to 0x0000.8000) has up to 100000 program/erase cycles.

#### 8.8.2 Peripheral File Map (p. 64)

Table 8-6 lists the available peripherals and the register base address for each.

**Table 8-6. Peripherals Summary (pp. 64-65)**

| PERIPHERAL NAME | BASE ADDRESS | SIZE |
| --- | --- | --- |
| COMP0 | 0x40008000 | 0x00001F0 |
| VREF | 0x40030000 | 0x00001F0 |
| WWDT0 | 0x40080000 | 0x0000150 |
| TIMG14 | 0x40084000 | 0x00001F0 |
| TIMG1 | 0x40086000 | 0x00001F0 |
| TIMG2 | 0x40088000 | 0x00001F0 |
| TIMG8 | 0x40090000 | 0x00001F0 |
| LFSS | 0x40094000 | 0x0000160 |
| RTC_B | 0x40094000 | 0x0000160 |
| IWDT | 0x40094000 | 0x0000160 |
| GPIOA | 0x400A0000 | 0x00001F0 |
| GPIOB | 0x400A2000 | 0x00001F0 |
| SYSCTL | 0x400AF000 | 0x0000310 |
| DEBUGSS | 0x400C7000 | 0x00001F0 |
| EVENTLP | 0x400C9000 | 0x0000300 |
| FLASHCTL | 0x4042A000 | 0x00001F0 |
| I2C0 | 0x40440000 | 0x0000200 |
| I2C1 | 0x400CD000 | 0x0000200 |
| UART1 | 0x400F0000 | 0x00001F0 |
| UART2 | 0x400F2000 | 0x00001F0 |
| UART0 | 0x40100000 | 0x00001F0 |
| CPUSS | 0x40102000 | 0x00001F0 |
| WUC | 0x40108000 | 0x00001F0 |
| IOMUX | 0x40400000 | 0x00001F0 |
| DMA | 0x40424000 | 0x0000050 |
| CRC | 0x40428000 | 0x0000200 |
| SPI0 | 0x40468000 | 0x00001F0 |
| ADC0_SVT | 0x4055A000 | 0x0000100 |
| TIMA0 | 0x40860000 | 0x00001F0 |

#### 8.8.3 Peripheral Interrupt Vector (p. 66)

Table 8-7 shows the IRQ number and the interrupt group number for each peripheral in this device.

**Table 8-7. Interrupt Vector Number (p. 66)**

| PERIPHERAL NAME | NVIC IRQ |
| --- | --- |
| SYSCTL | 0 |
| DEBUGSS | 1 |
| TIMG8 | 2 |
| UART1 | 3 |
| ADC0 | 4 |
| COMP0 | 7 |
| UART2 | 8 |
| SPI0 | 9 |
| UART0 | 15 |
| TIMG14 | 16 |
| TIMG2 | 17 |
| TIMA0 | 18 |
| TIMG1 | 19 |
| GPIOA | 22 |
| GPIOB | 23 |
| I2C0 | 24 |
| I2C1 | 25 |
| FLASHCTL | 27 |
| WWDT0 | 29 |
| LFSS | 30 |
| RTC_B | 30 |
| IWDT | 30 |
| DMA | 31 |

### 8.9 Flash Memory (p. 66)

A single bank of nonvolatile flash memory is provided for storing executable program code and application data.

Key features of the flash include:

- In-circuit program and erase operations supported across the entire recommended supply range

- Small 1KB sector sizes (minimum erase resolution of 1KB)

- Up to 100,000 program/erase cycles on 32 selected sectors of the flash memory, with up to 10,000 program/ erase cycles on the remaining flash memory (devices with 32kB support 100,000 cycles on the entire flash memory)

For more details, see the NVM chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.10 SRAM (p. 66)

MSPM0Cxx MCUs include a low-power high-performance SRAM memory with zero wait state access across the supported CPU frequency range of the device. SRAM memory can be used for storing volatile information such as the call stack, heap, global data, and code. The SRAM memory content is fully retained in RUN, SLEEP, STOP, and STANDBY operating modes and is lost in SHUTDOWN mode. A write protection mechanism is provided to allow the application to dynamically write protect the SRAM memory with 1KB resolution. Write protection is useful when placing executable code into SRAM to provide a level of protection against unintentional overwrites of code by either the CPU or DMA. Placing code in SRAM can improve performance of critical loops by enabling zero wait state operation and lower power consumption.

### 8.11 GPIO (p. 67)

The general purpose input/output (GPIO) peripheral lets the application write data out and read data in through the device pins. Through the use of the Port A and Port B GPIO peripheral, these devices support up to 45 GPIO pins.

The key features of the GPIO module include:

- Set/Clear/Toggle multiple bits without the need of a read-modify-write construct in software

- GPIOs with "Standard with Wake" drive functionality able to wake the device from SHUTDOWN mode

- "FastWake" feature enables low-power wakeup from STOP and STANDBY modes for any GPIO port

- User controlled input filtering

For more details, see the GPIO chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.12 IOMUX (p. 67)

The IOMUX peripheral enables IO pad configuration and controls digital data flow to and from the device pins. The key features of the IOMUX include:

- IO Pad configuration registers allow for programmable drive strength, speed, pullup-down, and more

- Digital pin muxing allows for multiple peripheral signals to be routed to the same IO pad

- Pin functions and capabilities are user-configured using the PINCM register

For more details, see the IOMUX chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.13 ADC (p. 67)

The 12-bit analog-to-digital converter (ADC) module in these devices support fast 12-bit conversions with single-ended inputs.

ADC features include:

- 12-bit output resolution at up to 1.6-Msps with 10.6-bit ENOB (external reference)

- Up to 27 external input channels

- Internal channels for temperature sensing, supply monitoring, and analog signal chain

- Software selectable reference: – Configurable internal dedicated ADC reference voltage of 1.4V or 2.5V (VREF) – MCU supply voltage (VDD) – Support for bringing in an external reference on VREF+/- device pins – Requires a decpoupling capacitor placed on VREF+/- pins for proper operation.

- Operates in RUN, SLEEP, and STOP modes and supports triggers from STANDBY mode

**Table 8-8. ADC0 Channel Mapping (pp. 67-68)**

| CHANNEL[0:7] | SIGNAL NAME | CHANNEL[8:15] | SIGNAL NAME |
| --- | --- | --- | --- |
| 0 | A0 | 16 | A16 |
| 1 | A1 | 17 | A17 |
| 2 | A2 | 18 | A18 |
| 3 | A3 | 19 | A19 |
| 4 | A4 | 20 | A20 |
| 5 | A5 | 21 | A21 |
| 6 | A6 | 22 | A22 |
| 7 | A7 | 23 | A23 |
| 8 | A8 | 24 | A24 |
| 9 | A9 | 25 | A25 |
| 10 | A10 | 26 | A26 |
| 11 | A11 | 27 | Reserved |
| 12 | A12 | 28 | Temperature Sensor |
| 13 | A13 | 29 | VREF |
| 14 | A14 | 30 | Reserved |
| 15 | A15 | 31 | Supply/Battery Monitor |

Italicized signal names are internal to the SoC. These signals are used for internal peripheral interconnections.

For more details, see the ADC chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.14 Temperature Sensor (p. 68)

The temperature sensor provides a voltage output that changes linearly with device temperature. The temperature sensor output is internally connected to one of ADC input channels to enable a temperature-to-digital conversion.

A unit-specific single-point calibration value for the temperature sensor is provided in the factory constants memory region. This calibration value represents the ADC conversion result (in ADC code format) corresponding to the temperature sensor being measured in 12-bit mode with the 1.4V internal VREF at the factory trim temperature (TS TRIM). This calibration value can be used with the temperature sensor temperature coefficient (TSc) to estimate the device temperature. See the temperature sensor section of the MSPM0 C-Series Microcontrollers Technical Reference Manual for guidance on estimating the device temperature with the factory trim value.

### 8.15 Low-Frequency Sub System (LFSS) (p. 68)

The Low-Frequency Sub-System (LFSS) is a sub-system which combines several functional peripherals under one shared subsystem. These peripherals are clocked by the low freqency clock (LFCLK) or need to be active during low power modes. The LFCLK has a typical frequency of 32kHz and is mainly intended for long-term timekeeping.

LFSS in this device contains following components:

- Real Time Clock with additional prescalar extension and timestamp captures

- An asynchronous IWDT

For more details, see the LFSS chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.16 VREF (p. 68)

The shared voltage reference module (VREF) in these devices contain a configurable voltage reference buffer which allows users to supply a stable reference to on-board analog peripherals. It also supports bringing in an external reference for applications where higher accuracy is required.

VREF features include:

- 1.4V and 2.5V user-selectable internal references. Same reference voltage will be selected for ADC and COMP

- Internal reference supports ADC operation up to 515ksps

- Support for bringing in an external reference on VREF+ and VREF- device pins

Figure 8-1. VREF module

For more details, see the VREF chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.17 COMP (p. 69)

The comparator peripheral in the device compares the voltage levels on two inputs terminals and provides a digital output based on this comparison. It supports the following key features:

- Programmable hysteresis

- Programmable reference voltage: – External reference voltage (VREF IO) – Integrated 8-bit reference DAC

- Configurable operation modes: – High speed mode – Lower power mode

- Programmable output glitch filter delay

- Supports 6 blanking sources from TIMx instances (see Table 8-10)

- Support output wake up device from all low power modes

- Output connected to advanced timer fault handling mechanism

- The IPSEL and IMSEL bits in comparator registers can be used to select the comparator channel inputs from device pins

- 8-bit reference DAC can be used to output to device pins

**Table 8-9. COMP0 Input Channel Selection (p. 69)**

| IPSEL / IMSEL BITS | POSITIVE TERMINAL INPUT | POSITIVE TERMINAL INPUT | NEGATIVE TERMINAL INPUT |
| --- | --- | --- | --- |
| 0x0 | COMP0_IN0+ |  | COMP0_IN0- |
| 0x1 | COMP0_IN1+ |  | COMP0_IN1- |
| 0x2 | COMP0_IN2+ |  | COMP0_IN2- |
| 0x3 | COMP0_IN3+ |  | - |
| 0x5 | - |  | Temperature Sensor |

**Table 8-10. COMP0 Blanking Source Table (pp. 69-70)**

| CTL2.BLANKSRC | Blanking Source Selected |
| --- | --- |
| 1 | TIMA0.CC2 |
| 2 | TIMA0.CC3 |
| 3 | TIMA0.CC1 |
| 4 | TIMG0.CC1 |
| 5 | TIMG1.CC1 |
| 6 | TIMG8.CC1 |

For more information about device analog connections, refer to Section 8.27.

For more details, see the COMP chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.18 Security (p. 70)

This device offers several security features, including:

- Debug security

- Unique Die ID

- Flexible firewalls for protecting code and data – Flash write-erase protection – Flash read-execute protection – Flash IP protection – SRAM write-execute mutual exclusion

- Secure boot

- Secure firmware update

- Customer secure code

- Cyclic redundancy checker (CRC-16) with support for custom polynomial

For more details, see the Security chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual

### 8.19 CRC (p. 70)

The cyclical redundancy check (CRC) module provides a signature for an input data sequence. Key features of the CRC module include:

- Support for 16-bit CRC based on CRC16-CCITT

- Support for bit reversal

For more details, see the CRC chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.20 UART (p. 70)

The UART peripherals (UART0, UART1, UART2) provide the following key features:

- Standard asynchronous communication bits for start, stop, and parity

- Fully programmable serial interface – 5, 6, 7 or 8 data bits – Even, odd, stick, or no-parity bit generation and detection – 1 or 2 stop bit generation – Line-break detection – Glitch filter on the input signals – Programmable baud rate generation with oversampling by 16, 8 or 3 – Local Interconnect Network (LIN) mode support

- Separated 4-entry transmit and receive FIFOs support DAM data transfer

- Support transmit and receive loopback mode operation

- See Table 8-11 for detail information on supported protocols.

**Table 8-11. UART Features (pp. 70-71)**

| UART Features | UART0(Extend) | UART1, UART2(Main) |
| --- | --- | --- |
| Active in Stop and Standby Mode | Yes | Yes |
| Separate transmit and receive FIFOs | Yes | Yes |
| Support hardware flow control | Yes | Yes |
| Support 9-bit configuration | Yes | Yes |
| Support LIN mode | Yes | - |
| Support DALI | Yes | - |
| Support IrDA | Yes | - |
| Support ISO7816 Smart Card | Yes | - |
| Support Manchester coding | Yes | - |

For more details, see the UART chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.21 I2C (p. 71)

The inter-integrated circuit interface (I 2 C) peripherals in these devices provide bidirectional data transfer with other I2C devices on the bus and support the following key features:

- 7-bit and 10-bit addressing mode with multiple 7-bit target addresses

- Multiple-controller transmitter or receiver mode

- Target receiver or transmitter mode with configurable clock stretching

- Support Standard-mode (Sm), with a bit rate up to 100kbps

- Support Fast-mode (Fm), with a bit rate up to 400kbps

- Support Fast-mode Plus (Fm+), with a bit rate up to 1Mbps

- Separated 4-entry transmit and receive FIFOs support DMA data transfer

- Support SMBus 3.0 with PEC, ARP, timeout detection and host support

- Wakeup from low power mode on address match

- Support analog and digital glitch filter for input signal glitch suppression

For more details, see the I2C chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.22 SPI (p. 71)

The serial peripheral interface (SPI) peripherals in these devices support the following key features:

- Support ULPCLK/2 bit rate and up to 16 Mbits/s in both controller and peripheral mode

- Configurable as a controller or a peripheral

- Configurable chip select for both controller and peripheral

- Programmable clock prescaler and bit rate

- Programmable data frame size from 4 bits to 16 bits (controller mode)

- Programmable data frame size from 7 bits to 16 bits (peripheral mode)

- Separated 4-entry transmit and receive FIFOs support DMA data transfer

- Supports TI mode, Motorola mode, and National Microwire format

For more details, see the SPI chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.23 IWDT (p. 71)

The independent watchdog timer (IWDT) in the LFSS is a device-independent supervisor which monitors code execution and overall hang up scenarios of the device. Due to the nature of LFSS, this IWDT has its own system independent power and clock source. If the application software does not successfully reset the watchdog within the programmed time, the watchdog generates a POR reset to the device.

Key features of the IWDT include:

- A 25-bit counter with closed and open window

- Counter driven from LFOSC (fixed 32kHz clock path) with a programmable clock divider

- Eight selectable watchdog timer periods

For more details, see the IWDT chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.24 WWDT (p. 72)

The windowed watchdog timer (WWDT) can be used to supervise the operation of the device, specifically code execution. The WWDT can be used to generate a reset or an interrupt if the application software does not successfully reset the watchdog within a specified window of time. Key features of the WWDT include:

- 25-bit counter

- Programmable clock divider

- Eight software selectable watchdog timer periods

- Eight software selectable window sizes

- Support for stopping the WWDT automatically when entering a sleep mode

- Interval timer mode for applications which do not require watchdog functionality

For more details, see the WWDT chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.25 RTC_B (p. 72)

The RTC_B instance of the real-time clock operates off of a 32kHz input clock source (typically a low frequency crystal) and provides a time base to the application with multiple options for interrupts to the CPU. The RTC_B provides common key features in relation to the Low-Frequency Sub System (LFSS).

Common key features of the RTC_B include:

- Counters for seconds, minutes, hours, day of the week, day of the month, month, and year

- Binary or BCD format

- Leap-year handling

- One customizable alarm interrupt based on minute, hour, day of the week, and day of the month

- Interval alarm interrupt to wake every minute, every hour, at midnight, or at noon

- Interval alarm interrupt providing periodic wake-up at 4096, 2048, 1024, 512, 256, or 128 Hz

- Interval alarm interrupt providing periodic wake-up at 64, 32, 16, 8, 4, 2, 1, and 0.5 Hz

- Calibration for crystal offset error (up to +/- 240ppm)

- Compensation for temperature drift (up to +/- 240ppm)

- RTC clock output to pin for calibration

Table 8-12 shows the RTC features supported in this device.

**Table 8-12. RTC_B Key Features (pp. 72-73)**

| RTC Features | RTC_B |
| --- | --- |
| Power enable register | - |
| Real-time clock and calendar mode providing seconds, minutes, hours, day of week, day of month, and year | Yes |
| Selectable binary or binary-coded decimal (BCD) format | Yes |
| Leap-year correction (valid for year 1901 through 2099) | Yes |
| Two customizable calendar alarm interrupts based on minute, hour, day of the week, and day of the month | Yes |
| Interval alarm interrupt to wake every minute, every hour, at midnight, or at noon | Yes |
| Periodic interrupt to wake at 4096, 2048, 1024, 512, 256, or 128Hz | Yes |
| Periodic interrupt to wake at 64, 32, 16, 8, 4, 2, 1, and 0.5Hz | Yes |
| Interrupt capability down to STANDBY mode with STOPCLKSTBY | Yes |
| Calibration for crystal offset error and crystal temperature drift (up to ±240 ppm total) | Yes |
| RTC clock output to pin for calibration (GPIO) | Yes |
| RTC clock output to pin for calibration (TIO) | - |
| Three -bit prescaler for heartbeat function with interrupt generation | - |
| RTC external clock selection of untrimmed 32kHz, trimmed 512Hz, 256Hz or 1Hz | - |
| RTC time stamp capture upon detection of a timer stamp event, including: • TIO event • VDD fail event | - |
| RTC counter lock function | - |

For more details, see the RTC chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.26 Timers (TIMx) (p. 73)

The timer peripherals in these devices support the following key features. For specific configuration, see Table 8-13.

Specific features for the general-purpose timer (TIMGx) include:

- 16-bit down, up/down, or up counter with repeat-reload mode

- Selectable and configurable clock source

- 8-bit programmable prescaler to divide the counter clock frequency

- Up to four independent CC channels for – Output compare – Input capture – PWM output – One-shot mode

- Support quadrature encoder interface (QEI) for positioning and movement sensing

- Support synchronization and cross trigger among different TIMx instances in the same power domain

- Support interrupt trigger generation and cross peripherals (such as ADC) trigger capability

- Cross-trigger event logic for Hall sensor inputs

Specific features for the advanced timer (TIMAx) include:

- 16-bit down or up-down counter, with repeat-reload mode

- Selectable and configurable clock source

- 8-bit programmable prescaler to divide the counter clock frequency

- Clock doubler to provide 2x clock source for improved timer resolution

- Repeat counter to generate an interrupt or event only after a given number of cycles of the counter

- Up to four independent CC channels for – Output compare – Input capture – PWM output – One-shot mode

- Shadow register for load and CC register available

- Complementary output PWM

- Asymmetric PWM with programmable dead band insertion

- Fault handling mechanism to keep the output signals in a safe user-defined state when a fault condition is encountered

- Support synchronization and cross trigger among different TIMx instances in the same power domain

- Support interrupt trigger generation and cross peripherals (such as ADC) trigger capability

- Two additional capture/compare channels for internal events

**Table 8-13. TIMx Configurations (p. 74)**

| TIMER NAME | POWER DOMAIN | RESOLUTION | PRESCALER | REPEAT COUNTER | CAPTURE / COMPARE CHANNELS | PHASE LOAD | SHADOW LOAD | SHADOW CC | DEAD-BAND | FAULT | QEI |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| TIMG14 | PD0 | 16 bit | 8 bit | – | 4 | – | – | – | – | – | – |
| TIMG1 | PD0 | 16 bit | 8 bit | – | 2 | – | – | – | – | – | – |
| TIMG2 | PD0 | 16 bit | 8 bit | – | 2 | – | – | – | – | – | – |
| TIMG8 | PD0 | 16 bit | 8 bit | – | 2 | – | – | – | – | – | Yes |
| TIMA0 | PD0 | 16 bit | 8 bit | 8-bit | 4 | Yes | Yes | Yes | Yes | Yes | – |

For more details, see the TIMx chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

MSPM0C1105, MSPM0C1106

### 8.27 Device Analog Connections (p. 75)

Figure 8-2 shows the internal analog connection of the device.

![Figure 8-2. Device Analog Connection](../assets/figures/p075-figure-8-2-device-analog-connection.png)
*Figure 8-2. Device Analog Connection (p. 75)*

### 8.28 Input/Output Diagrams (p. 76)

The IOMUX manages the selection of which peripheral function is to be used on a digital IO. It also provides the controls for the output driver, input path, and the wake-up logic for wakeup from SHUTDOWN mode. For more information, refer to the IOMUX section of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

The mixed-signal IO pin slice diagram for a full featured IO pin is shown in Figure 8-3. Not all pins will have analog functions, wake-up logic, drive strength control, and pullup or pulldown resistors available.

![Figure 8-3. Superset Input/Output Diagram](../assets/figures/p076-figure-8-3-superset-input-output-diagram.png)
*Figure 8-3. Superset Input/Output Diagram (p. 76)*

### 8.29 Serial Wire Debug Interface (p. 76)

A serial wire debug (SWD) two-wire interface is provided via an Arm compatible serial wire debug port (SW-DP) to enable access to multiple debug functions within the device.

**Table 8-14. Serial Wire Debug Pin Requirements and Functions (p. 77)**

| DEVICE SIGNAL | DIRECTION | SWD FUNCTION |
| --- | --- | --- |
| SWCLK | Input | Serial wire clock from debug probe |
| SWDIO | Input/Output | Bi-directional (shared) serial wire data |

For a complete description of the debug functionality offered on MSPM0 devices, see the Debug chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.30 DEBUGSS (p. 77)

The debug subsystem (DEBUGSS) interfaces the ARM Serial Wire Debug (SWD) two-wire physical interface to multiple debug functions within the device. MSPM0 devices support debugging of processor execution and the device state. The DEBUGSS also provides a mailbox system for communicating with software through SWD.

Key features provided by the debug subsystem include:

- The ARM Serial Wire Debug (SWD) two-wire (SWDIO, SWCLK) debug interface, compatible with both TI and 3 rd party debug probes – On-chip pullup and pulldown resistors for SWDIO and SWCLK, respectively, enabled by default – Support for disabling SWD functions to use SWD pins as general-purpose input/output pins – Support for debug on all low power modes

- Debug of the processor – Run, halt, and step debug support – 2 hardware breakpoints (BPU) – 1 hardware watchpoints (DWT) – Supporting software breakpoints

- Software-configurable peripheral behavior during processor debug – Ability to free run select peripherals through debug halt – Ability to halt select peripherals on a debug halt – Ability to request reset and mode changes to the PMCU

- Mailbox (DSSM) for passing data and control signals between the SWD interface and boot ROM (as well as application software)

- Support for various security features, including SWD lockout and password authenticated debugging

For more details, see the DEBUGSS chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual.

### 8.31 Device Factory Constants (p. 77)

All devices include a memory-mapped FACTORY region which provides read-only data describing the capabilities of a device as well as any factory-provided trim information for use by application software. Please refer to Factory Constants chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual for more information.

DEVICEID address is 0x41C4.0004, PARTNUM is bit 12 to 27, MANUFACTURER is bit 1 to 11.

**Table 8-15. DEVICEID (p. 77)**

| Device | PARTNUM | MANUFACTURER |
| --- | --- | --- |
| MSPM0C1105 | 0xBBBA | 0x17 |
| MSPM0C1106 | 0xBBBA | 0x17 |

USERID address is 0x41C4.0008, PART is bit 0 to 15, VARIANT is bit 16 to 23

**Table 8-16. USERID (p. 77)**

| Device | Part | Variant |
| --- | --- | --- |
| MSPM0C1105SPTR | CC25 | 31 |
| MSP32C031C6SPTR | CC25 | 53 |
| MSP32G031C6SPTR | CC25 | 54 |

MSPM0C1105SDGS28R CC25

USERID address is 0x41C4.0008, PART is bit 0 to 15, VARIANT is bit 16 to 23

**Table 8-16. USERID (continued) (p. 78)**

| Device | Part | Variant |
| --- | --- | --- |
| MSPM0C1105SDGS20R | CC25 | 34 |
| MSPM0C1105SRGZR | CC25 | 35 |
| MSPM0C1105SRHBR | CC25 | 36 |
| MSPM0C1105SRGER | CC25 | 37 |
| MSPM0C1105SRUKR | CC25 | 38 |
| MSP32G031C6SVFCR | CC25 | 39 |
| MSP32C031C6SVFCR | CC25 | 55 |
| MSPM0C1105SZCMR | CC25 | 60 |
| MSPM0C1106SPTR | F5CF | 3A |
| MSP32G031C8SPTR | F5CF | 56 |
| MSPM0C1106SDGS28R | F5CF | 3C |
| MSPM0C1106SDGS20R | F5CF | 3D |
| MSPM0C1106SRGZR | F5CF | 3E |
| MSPM0C11106SRHBR | F5CF | 3F |
| MSPM0C1106SRGER | F5CF | 40 |
| MSPM0C1106SRUKR | F5CF | 41 |
| MSP32G031C8SVFCR | F5CF | 42 |
| MSPM0C1106SZCMR | F5CF | 61 |

### 8.32 Identification (p. 78)

Revision and Device Identification

The hardware revision and device identification values are stored in the memory-mapped FACTORY region, refer to Device Factory Constants section, which provides read-only data describing the capabilities of a device as well as any factory-provided trim information for use by application software. Refer to the Factory Constants chapter of the MSPM0 C-Series Microcontrollers Technical Reference Manual for more information.

The device revision and identification information are also included as part of the top-side marking on the device package. The device-specific errata sheet describes these markings (see Section 10.4).


---

---
document: "MSPM0C1105, MSPM0C1106 Mixed-Signal Microcontrollers datasheet (Rev. B)"
section_id: "09"
section_title: "9 Applications, Implementation, and Layout"
pages: "79-79"
---

# 9 Applications, Implementation, and Layout (pp. 79-79)

## 9 Applications, Implementation, and Layout (p. 79)

### 9.1 Typical Application (p. 79)

> **Note:**

Information in the following applications sections is not part of the TI component specification, and TI does not warrant its accuracy or completeness. TI’s customers are responsible for determining suitability of components for their purposes, as well as validating and testing their design implementation to confirm system functionality.

#### 9.1.1 Schematic (p. 79)

TI recommends connecting a combination of a 10µF and a 0.1µF low-ESR ceramic decoupling capacitor to the VDD and VSS pins. Higher-value capacitors may be used but can impact supply rail ramp-up time. Decoupling capacitors must be placed as close as possible to the pins that they decouple (within a few millimeters).

The NRST reset pin is required to connect an external 47kΩ pullup resistor with a 1000pF pulldown capacitor.

For devices supporting external crystals, external bypass capacitors for the crystal oscillator pins are required. Refer to MSPM0 C-Series Microcontrollers Technical Reference Manual which explains how to calculate the

![Figure 9-1. Typical Application Schematic](../assets/figures/p079-figure-9-1-typical-application-schematic.png)
*Figure 9-1. Typical Application Schematic (p. 79)*


---

---
document: "MSPM0C1105, MSPM0C1106 Mixed-Signal Microcontrollers datasheet (Rev. B)"
section_id: "10"
section_title: "10 Device and Documentation Support"
pages: "80-81"
---

# 10 Device and Documentation Support (pp. 80-81)

## 10 Device and Documentation Support (p. 80)

TI offers an extensive line of development tools. Tools and software to evaluate the performance of the device, generate code, and develop solutions are listed below.

### 10.1 Getting Started and Next Steps (p. 80)

For more information on the MSP low-power microcontrollers and the tools and libraries that are available to help with development, visit the Texas Instruments Arm Cortex-M0+ MCUs page.

### 10.2 Device Nomenclature (p. 80)

To designate the stages in the product development cycle, TI assigns prefixes to the part numbers of all MSP MCU devices and support tools. . Each MSP MCU commercial family member has one of two prefixes: MSP or X. These prefixes represent evolutionary stages of product development from engineering prototypes (X) through fully qualified production devices (MSP).

X – Experimental device that is not necessarily representative of the final device's electrical specifications

MSP– Fully qualified production device

X devices are shipped against the following disclaimer:

"Developmental product is intended for internal evaluation purposes." MSP devices have been characterized fully, and the quality and reliability of the device have been demonstrated fully. TI's standard warranty applies. Predictions show that prototype devices (X) have a greater failure rate than the standard production devices. TI recommends that these devices not be used in any production system because their expected end-use failure rate still is undefined. Only qualified production devices are to be used.

TI device nomenclature also includes a suffix with the device family name. This suffix indicates the temperature range, package type, and distribution format. Figure 10-1 provides a legend for reading the complete device name.

Figure 10-1. Device Nomenclature

**Table 10-1. Device Nomenclature (p. 80)**

| Processor Family | MSP = Mixed-signal processor X= Experimental silicon |
| --- | --- |
| MCU Platform | M0 = Arm based 32-bit M0+ |
| Product Family | C = 32MHz frequency |
| Device Subfamily | 1105/6 = 32MHz frequency, ADC, RTC, CMP |
| Flash Memory | 5 = 32KB; 6= 64KB |
| Temperature Range | S = -40°C to 125°C |
| Package Type | See the Device Comparison section and https://www.ti.com/packaging |
| Distribution Format | R = Large reel No marking = Tube or tray |

For orderable part numbers of MSP devices in different package types, see the Package Option Addendum of this document, ti.com, or contact your TI sales representative.

### 10.3 Tools and Software (p. 81)

Design Kits and Evaluation Modules

MSPM0 LaunchPad (LP) Empowers you to immediately start developing on the industry’s best Boards: LP-MSPM0C1106 integrated analog and most cost-optimized general purpose MSPM0 MCU family. Exposes all device pins and functionality; includes some built-in circuitry, out-of-box software demos, and on-board XDS110 debug probe for programming/debugging/EnergyTrace. The LP ecosystem includes dozens of BoosterPack stackable plug-in modules to extend functionality. Embedded Software

MSPM0 Software Development Contains software drivers, middleware libraries, documentation, tools, and Kit (SDK) code examples that create a familiar and easy user experience for all MSPM0 devices.

Software Development Tools TI Cloud Tools Start your evaluation and development on a web browser without any installation. Cloud tools also have a downloadable, offline version. TI Resource Explorer Online portal to TI SDKs. Accessible in CCS IDE or in TI Cloud Tools. SysConfig Intuitive GUI to configure device and peripherals, resolve system conflicts, generate configuration code, and automate pin mux settings. Accessible in CCS IDE or in TI Cloud Tools. (offline version) MSP Academy Great starting point for all developers to learn about the MSPM0 MCU Platform with training modules that span a wide range of topics. Part of TI Rex. GUI Composer GUIs that simplify evaluation of certain MSPM0 features, such as configuring and monitoring a fully integrated analog signal chain without any code needed. IDE & compiler toolchains Code Composer Studio ™ Includes TI Arm-Clang compiler. Supports all TI Arm Cortex MCUs and (CCS) boasts competitive code size performance advantages, fast compile time, code coverage support, safety certification support, and completely free to use. IAR Embedded Workbench® IDE

Keil®MDK IDE

GNU Arm Embedded Toolchain

### 10.4 Documentation Support (p. 81)

To receive notification of documentation updates, navigate to the device product folder on ti.com. Click on Subscribe to updates to register and receive a weekly digest of any product information that has changed. For change details, review the revision history included in any revised document.

The following documents describe the MSPM0 MCUs. Copies of these documents are available on the Internet at www.ti.com.

Technical Reference Manual

MSPM0 C-Series This manual describes the modules and peripherals of the family of devices. Each Microcontrollers description presents the module or peripheral in a general sense. Not all features Technical Reference and functions of all modules or peripherals are present on all devices. In addition, Manual modules or peripherals can differ in their exact implementation on different devices.


---

---
document: "MSPM0C1105, MSPM0C1106 Mixed-Signal Microcontrollers datasheet (Rev. B)"
section_id: "11"
section_title: "11 Revision History"
pages: "82-82"
---

# 11 Revision History (pp. 82-82)

Pin functions, internal signal connections, and operational parameters differ from device to device. See the device-specific data sheet for these details.

### 10.5 Support Resources (p. 82)

TI E2E ™ support forums are an engineer's go-to source for fast, verified answers and design help — straight from the experts. Search existing answers or ask your own question to get the quick design help you need.

Linked content is provided "AS IS" by the respective contributors. They do not constitute TI specifications and do not necessarily reflect TI's views; see TI's Terms of Use.

### 10.6 Trademarks (p. 82)

LaunchPad ™ Code Composer Studio ™, and TI E2E ™ are trademarks of Texas Instruments. , Arm®and Cortex®are registered trademarks of Arm Limited. All trademarks are the property of their respective owners.

### 10.7 Electrostatic Discharge Caution (p. 82)

This integrated circuit can be damaged by ESD. Texas Instruments recommends that all integrated circuits be handled with appropriate precautions. Failure to observe proper handling and installation procedures can cause damage. ESD damage can range from subtle performance degradation to complete device failure. Precision integrated circuits may be more susceptible to damage because very small parametric changes could cause the device not to meet its published specifications.

### 10.8 Glossary (p. 82)

TI Glossary This glossary lists and explains terms, acronyms, and definitions.

## 11 Revision History (p. 82)

NOTE: Page numbers for previous revisions may differ from page numbers in the current version.

Changes from September 30, 2025 to January 30, 2026 (from Revision A (September 2025) to Revision B (January 2026)) Page

| • | Removed the ZCM table footnotes..................................................................................................................... 2 |
| --- | --- |
| • | Updated thermal information with ZCM............................................................................................................ 41 |
| • | Added VREF module block diagram................................................................................................................ 68 |
| • | Clarified the FIFO entries description............................................................................................................... 70 |
| • | Clarified the FIFO entries description............................................................................................................... 71 |
| • | Clarified the FIFO entries description............................................................................................................... 71 |


---

---
document: "MSPM0C1105, MSPM0C1106 Mixed-Signal Microcontrollers datasheet (Rev. B)"
section_id: "12"
section_title: "12 Mechanical, Packaging, and Orderable Information"
pages: "83-138"
---

# 12 Mechanical, Packaging, and Orderable Information (pp. 83-138)

## 12 Mechanical, Packaging, and Orderable Information (p. 83)

The following pages include mechanical, packaging, and orderable information. This information is the most current data available for the designated devices. This data is subject to change without notice and revision of this document. For browser-based versions of this data sheet, refer to the left-hand navigation.

## PACKAGE OPTION ADDENDUM (p. 104)

20-Aug-2026

PACKAGING INFORMATION

Orderable part number Status Material type Package | Pins Package qty | Carrier RoHS Lead finish/ MSL rating/ Op temp (°C) Part marking

(1) (2) (3) Ball material Peak reflow (6)

(4) (5) MSP32C031C6SPTR Active Production LQFP (PT) | 48 1000 | LARGE T&R Yes NIPDAU Level-2-260C-1 YEAR -40 to 125 M0C031C6S MSP32G031C6SPTR Active Production LQFP (PT) | 48 1000 | LARGE T&R Yes NIPDAU Level-2-260C-1 YEAR -40 to 125 M0G031C6S MSP32G031C8SPTR Active Production LQFP (PT) | 48 1000 | LARGE T&R Yes NIPDAU Level-2-260C-1 YEAR -40 to 125 M0G031C8S MSPM0C1105SDGS20R Active Production VSSOP (DGS) | 20 5000 | LARGE T&R Yes NIPDAU Level-2-260C-1 YEAR -40 to 125 M0C1105S MSPM0C1105SDGS28R Active Production VSSOP (DGS) | 28 5000 | LARGE T&R Yes NIPDAU Level-2-260C-1 YEAR -40 to 125 C1105S MSPM0C1105SPTR Active Production LQFP (PT) | 48 1000 | LARGE T&R Yes NIPDAU Level-2-260C-1 YEAR -40 to 125 M0C1105S MSPM0C1105SRGER Active Production VQFN (RGE) | 24 5000 | LARGE T&R Yes NIPDAU Level-1-260C-UNLIM -40 to 125 MSPM0 C1105S MSPM0C1105SRGZR Active Production VQFN (RGZ) | 48 4000 | LARGE T&R Yes NIPDAU Level-2-260C-1 YEAR -40 to 125 MSPM0 C1105S MSPM0C1105SRHBR Active Production VQFN (RHB) | 32 5000 | LARGE T&R Yes NIPDAU Level-2-260C-1 YEAR -40 to 125 MSPM0 C1105S MSPM0C1105SRUKR Active Production WQFN (RUK) | 20 5000 | LARGE T&R Yes NIPDAU Level-1-260C-UNLIM -40 to 125 C1105S MSPM0C1105SZCMR Active Production NFBGA (ZCM) | 48 2500 | LARGE T&R Yes SNAGCU Level-2-260C-1 YEAR -40 to 125 M0C1105S MSPM0C1106SDGS20R Active Production VSSOP (DGS) | 20 5000 | LARGE T&R Yes NIPDAU Level-2-260C-1 YEAR -40 to 125 M0C1106S MSPM0C1106SDGS28R Active Production VSSOP (DGS) | 28 5000 | LARGE T&R Yes NIPDAU Level-2-260C-1 YEAR -40 to 125 C1106S MSPM0C1106SPTR Active Production LQFP (PT) | 48 1000 | LARGE T&R Yes NIPDAU Level-2-260C-1 YEAR -40 to 125 M0C1106S MSPM0C1106SRGER Active Production VQFN (RGE) | 24 5000 | LARGE T&R Yes NIPDAU Level-1-260C-UNLIM -40 to 125 MSPM0 C1106S MSPM0C1106SRGZR Active Production VQFN (RGZ) | 48 4000 | LARGE T&R Yes NIPDAU Level-2-260C-1 YEAR -40 to 125 MSPM0 C1106S MSPM0C1106SRHBR Active Production VQFN (RHB) | 32 5000 | LARGE T&R Yes NIPDAU Level-2-260C-1 YEAR -40 to 125 MSPM0 C1106S MSPM0C1106SRUKR Active Production WQFN (RUK) | 20 5000 | LARGE T&R Yes NIPDAU Level-1-260C-UNLIM -40 to 125 C1106S MSPM0C1106SZCMR Active Production NFBGA (ZCM) | 48 2500 | LARGE T&R Yes SNAGCU Level-2-260C-1 YEAR -40 to 125 M0C1106S

(1) Status: For more details on status, see our product life cycle.

(2) Material type: When designated, preproduction parts are prototypes/experimental devices, and are not yet approved or released for full production. Testing and final process, including without limitation quality assurance, reliability performance testing, and/or process qualification, may not yet be complete, and this item is subject to further changes or possible discontinuation. If available for ordering, purchases will be subject to an additional waiver at checkout, and are intended for early internal evaluation purposes only. These items are sold without warranties of any kind.

Addendum-Page

## PACKAGE OPTION ADDENDUM (p. 105)

20-Aug-2026

(3) RoHS values: Yes, No, RoHS Exempt. See the TI RoHS Statement for additional information and value definition.

(4) Lead finish/Ball material: Parts may have multiple material finish options. Finish options are separated by a vertical ruled line. Lead finish/Ball material values may wrap to two lines if the finish value exceeds the maximum column width.

(5) MSL rating/Peak reflow: The moisture sensitivity level ratings and peak solder (reflow) temperatures. In the event that a part has multiple moisture sensitivity ratings, only the lowest level per JEDEC standards is shown. Refer to the shipping label for the actual reflow temperature that will be used to mount the part to the printed circuit board.

(6) Part marking: There may be an additional marking, which relates to the logo, the lot trace code information, or the environmental category of the part.

Multiple part markings will be inside parentheses. Only one part marking contained in parentheses and separated by a "~" will appear on a part. If a line is indented then it is a continuation of the previous line and the two combined represent the entire part marking for that device.

Important Information and Disclaimer: The information provided on this page represents TI's knowledge and belief as of the date that it is provided. TI bases its knowledge and belief on information provided by third parties, and makes no representation or warranty as to the accuracy of such information. Efforts are underway to better integrate information from third parties. TI has taken and continues to take reasonable steps to provide representative and accurate information but may not have conducted destructive testing or chemical analysis on incoming materials and chemicals. TI and TI suppliers consider certain information to be proprietary, and thus CAS numbers and other limited information may not be available for release.

In no event shall TI's liability arising out of such information exceed the total purchase price of the TI part(s) at issue in this document sold by TI to Customer on an annual basis.

OTHER QUALIFIED VERSIONS OF MSPM0C1105, MSPM0C1106 :

- Automotive : MSPM0C1105-Q1, MSPM0C1106-Q1

NOTE: Qualified Version Definitions:

- Automotive - Q100 devices qualified for high-reliability automotive applications targeting zero defects

Addendum-Page

3-Sep-2026

| Col 1 | Col 2 | TAPE AND REEL INFORMATION | TAPE AND REEL INFORMATION | Col 5 | Col 7 | Col 8 | Col 9 | Col 10 | Col 11 | Col 12 | Col 13 | Col 14 | P1 | Col 16 | Col 17 | Col 18 | Col 19 | Col 20 | Col 21 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  | Reel |  |  |  |  |  |  |  |  |  |  | W |  |  |  |
|  |  |  |  |  | Diameter |  |  |  |  |  | Cavity |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  | A0 |  |  |  |  | Dimension designed to accommodate the component width |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  | B0 |  |  |  |  | Dimension designed to accommodate the component length |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  | K0 |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  | W |  |  | Overall width of the carrier tape |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  | P1 |  |  |  | Pitch between successive cavity centers |  |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  | QUADRANT ASSIGNMENTS FOR PIN 1 ORIENTATION IN TAPE |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  |  |  | Q1 Q3 | Q2 Q4 |  | Q1 Q3 | Q2 Q4 |  |  |  | Sprocket Holes User Direction of Feed |  |  |  |  |  |
| *All dimensions are nominal | *All dimensions are nominal |  |  |  |  |  |  |  | Pocket Quadrants |  |  |  |  |  |  |  |  |  |  |
|  | Device |  |  | Package Type |  | Pins | Pins | SPQ |  | Reel Diameter (mm) |  | Reel Width W1 (mm) |  |  | B0 (mm) | K0 (mm) | P1 (mm) | W (mm) | Pin1 Quadrant |
|  | MSP32C031C6SPTR |  |  | LQFP |  | 48 | 48 | 1000 |  | 330.0 |  | 16.4 |  |  | 9.6 | 1.9 | 12.0 | 16.0 | Q2 |
|  | MSP32G031C6SPTR |  |  | LQFP |  | 48 | 48 | 1000 |  | 330.0 |  | 16.4 |  |  | 9.6 | 1.9 | 12.0 | 16.0 | Q2 |
|  | MSP32G031C8SPTR |  |  | LQFP |  | 48 | 48 | 1000 |  | 330.0 |  | 16.4 |  |  | 9.6 | 1.9 | 12.0 | 16.0 | Q2 |
|  | MSPM0C1105SDGS20R |  |  | VSSOP |  | 20 | 20 | 5000 |  | 330.0 |  | 16.4 |  |  | 5.4 | 1.45 | 8.0 | 16.0 | Q1 |
|  | MSPM0C1105SDGS28R |  |  | VSSOP |  | 28 | 28 | 5000 |  | 330.0 |  | 16.4 |  |  | 7.4 | 1.45 | 8.0 | 16.0 | Q1 |
|  | MSPM0C1105SPTR |  |  | LQFP |  | 48 | 48 | 1000 |  | 330.0 |  | 16.4 |  |  | 9.6 | 1.9 | 12.0 | 16.0 | Q2 |
|  | MSPM0C1105SRGER |  |  | VQFN |  | 24 | 24 | 5000 |  | 330.0 |  | 12.4 |  |  | 4.25 | 1.15 | 8.0 | 12.0 | Q2 |
|  | MSPM0C1105SRGZR |  |  | VQFN |  | 48 | 48 | 4000 |  | 330.0 |  | 16.4 |  |  | 7.3 | 1.1 | 12.0 | 16.0 | Q2 |
|  | MSPM0C1105SRHBR |  |  | VQFN |  | 32 | 32 | 5000 |  | 330.0 |  | 12.4 |  |  | 5.3 | 1.1 | 8.0 | 12.0 | Q2 |
|  | MSPM0C1105SRUKR |  |  | WQFN |  | 20 | 20 | 5000 |  | 330.0 |  | 12.4 |  |  | 3.3 | 1.1 | 8.0 | 12.0 | Q2 |
|  | MSPM0C1105SZCMR |  |  | NFBGA |  | 48 | 48 | 2500 |  | 330.0 |  | 12.4 |  |  | 3.7 | 1.4 | 8.0 | 12.0 | Q1 |
|  | MSPM0C1106SDGS20R |  |  | VSSOP |  | 20 | 20 | 5000 |  | 330.0 |  | 16.4 |  |  | 5.4 | 1.45 | 8.0 | 16.0 | Q1 |
|  | MSPM0C1106SDGS28R |  |  | VSSOP |  | 28 | 28 | 5000 |  | 330.0 |  | 16.4 |  |  | 7.4 | 1.45 | 8.0 | 16.0 | Q1 |
|  | MSPM0C1106SPTR |  |  | LQFP |  | 48 | 48 | 1000 |  | 330.0 |  | 16.4 |  |  | 9.6 | 1.9 | 12.0 | 16.0 | Q2 |
|  | MSPM0C1106SRGER |  |  | VQFN |  | 24 | 24 | 5000 |  | 330.0 |  | 12.4 |  |  | 4.25 | 1.15 | 8.0 | 12.0 | Q2 |
|  | MSPM0C1106SRGZR |  |  | VQFN |  | 48 | 48 | 4000 |  | 330.0 |  | 16.4 |  |  | 7.3 | 1.1 | 12.0 | 16.0 | Q2 |

3-Sep-2026

| Device | Package Type | Package Drawing | Pins | SPQ | Reel Diameter (mm) | Reel Width W1 (mm) | A0 (mm) | B0 (mm) | K0 (mm) | P1 (mm) | W (mm) | Pin1 Quadrant |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| MSPM0C1106SRHBR | VQFN | RHB | 32 | 5000 | 330.0 | 12.4 | 5.3 | 5.3 | 1.1 | 8.0 | 12.0 | Q2 |
| MSPM0C1106SRUKR | WQFN | RUK | 20 | 5000 | 330.0 | 12.4 | 3.3 | 3.3 | 1.1 | 8.0 | 12.0 | Q2 |
| MSPM0C1106SZCMR | NFBGA | ZCM | 48 | 2500 | 330.0 | 12.4 | 3.7 | 3.7 | 1.4 | 8.0 | 12.0 | Q1 |

3-Sep-2026

| *All dimensions are nominal | Col 6 | Col 8 | L | Col 10 | Width (mm) | Col 12 | Col 14 | Col 15 | Col 16 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Device | Package Type | Package Drawing |  | Pins | Pins | SPQ | Length (mm) | Width (mm) | Height (mm) |
| MSP32C031C6SPTR | LQFP | PT |  | 48 | 48 | 1000 | 336.6 | 336.6 | 31.8 |
| MSP32G031C6SPTR | LQFP | PT |  | 48 | 48 | 1000 | 336.6 | 336.6 | 31.8 |
| MSP32G031C8SPTR | LQFP | PT |  | 48 | 48 | 1000 | 336.6 | 336.6 | 31.8 |
| MSPM0C1105SDGS20R | VSSOP | DGS |  | 20 | 20 | 5000 | 353.0 | 353.0 | 32.0 |
| MSPM0C1105SDGS28R | VSSOP | DGS |  | 28 | 28 | 5000 | 353.0 | 353.0 | 32.0 |
| MSPM0C1105SPTR | LQFP | PT |  | 48 | 48 | 1000 | 336.6 | 336.6 | 31.8 |
| MSPM0C1105SRGER | VQFN | RGE |  | 24 | 24 | 5000 | 360.0 | 360.0 | 36.0 |
| MSPM0C1105SRGZR | VQFN | RGZ |  | 48 | 48 | 4000 | 360.0 | 360.0 | 36.0 |
| MSPM0C1105SRHBR | VQFN | RHB |  | 32 | 32 | 5000 | 360.0 | 360.0 | 36.0 |
| MSPM0C1105SRUKR | WQFN | RUK |  | 20 | 20 | 5000 | 360.0 | 360.0 | 36.0 |
| MSPM0C1105SZCMR | NFBGA | ZCM |  | 48 | 48 | 2500 | 338.1 | 338.1 | 20.6 |
| MSPM0C1106SDGS20R | VSSOP | DGS |  | 20 | 20 | 5000 | 353.0 | 353.0 | 32.0 |
| MSPM0C1106SDGS28R | VSSOP | DGS |  | 28 | 28 | 5000 | 353.0 | 353.0 | 32.0 |
| MSPM0C1106SPTR | LQFP | PT |  | 48 | 48 | 1000 | 336.6 | 336.6 | 31.8 |
| MSPM0C1106SRGER | VQFN | RGE |  | 24 | 24 | 5000 | 360.0 | 360.0 | 36.0 |
| MSPM0C1106SRGZR | VQFN | RGZ |  | 48 | 48 | 4000 | 360.0 | 360.0 | 36.0 |
| MSPM0C1106SRHBR | VQFN | RHB |  | 32 | 32 | 5000 | 360.0 | 360.0 | 36.0 |
| MSPM0C1106SRUKR | WQFN | RUK |  | 20 | 20 | 5000 | 360.0 | 360.0 | 36.0 |

3-Sep-2026

| Device | Package Type | Package Drawing | Pins | SPQ | Length (mm) | Width (mm) | Height (mm) |
| --- | --- | --- | --- | --- | --- | --- | --- |
| MSPM0C1106SZCMR | NFBGA | ZCM | 48 | 2500 | 338.1 | 338.1 | 20.6 |

## ZCM0048A NFBGA - 0.8 mm max height (p. 110)

| Col 2 | Col 3 | Col 4 | Col 5 | Col 6 | SCALE 4.000 | Col 8 | Col 10 | Col 11 | PLASTIC BALL GRID ARRAY |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  |  | 3.6 3.4 | A |  |  |
|  | BALL A1 CORNER |  |  |  |  |  | 3.6 3.4 |  |  |
|  | 0.80 0.61 |  |  |  |  |  |  |  |  |
|  | 0.61 |  |  |  |  |  |  |  |  |
|  | 0.23 0.13 |  |  |  |  | 3 TYP SYMM | (0.25) | 0.08 C |  |
|  | 3 TYP | G F E D |  |  |  |  | SYMM |  |  |
|  | 3 TYP | D C B A |  |  |  |  | 48X | 0.3 0.2 0.15 0.05 |  |
| 0.5 TYP |  |  | 1 2 | 2 | 0.5 TYP | 4 5 6 7 0.5 TYP |  | 0.05 | 4231499/A 02/2025 |

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M. 2. This drawing is subject to change without notice.

## ZCM0048A NFBGA - 0.8 mm max height (p. 111)

| Col 1 | Col 2 | Col 3 | Col 4 | Col 5 | Col 6 | PLASTIC BALL GRID ARRAY |
| --- | --- | --- | --- | --- | --- | --- |
| 48X ( 0.2) (0.5) TYP | 0.2) A (0.5) TYP B C D | 1 | (0.5) TYP 2 3 | 4 5 6 | 7 | SYMM |
| 0.05 MAX ALL AROUND SOLDER MASK OPENING | D E F G | ( NON-SOLDER MASK DEFINED (PREFERRED) | SYMM LAND PATTERN EXAMPLE EXPOSED METAL SHOWN SCALE: 25X EXPOSED METAL ( 0.2) METAL EDGE | SYMM LAND PATTERN EXAMPLE EXPOSED METAL SHOWN SCALE: 25X 0.05 MIN ALL AROUND EXPOSED METAL SOLDER MASK DETAILS NOT TO SCALE | SOLDER MASK DEFINED | METAL UNDER SOLDER MASK ( 0.2) SOLDER MASK OPENING 4231499/A 02/2025 |

NOTES: (continued)

3. Final dimensions may vary due to manufacturing tolerance considerations and also routing constraints. For information, see Texas Instruments literature number SPRAA99 (www.ti.com/lit/spraa99).

## ZCM0048A NFBGA - 0.8 mm max height (p. 112)

| Col 1 | Col 2 | Col 3 | Col 4 | Col 5 | PLASTIC BALL GRID ARRAY |
| --- | --- | --- | --- | --- | --- |
| 48X ( (0.5) TYP | 0.2) A (0.5) TYP B C D | 1 | (0.5) TYP 2 3 | 4 5 6 7 | SYMM |
|  | D E F G |  | SYMM | SYMM SOLDER PASTE EXAMPLE BASED ON 0.100 mm THICK STENCIL SCALE: 25X | 4231499/A 02/2025 |

NOTES: (continued)

4. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release.

## RUK 20 WQFN - 0.8 mm max height (p. 113)

3 x 3, 0.4 mm pitch PLASTIC QUAD FLATPACK - NO LEAD

This image is a representation of the package family, actual package may vary. Refer to the product data sheet for package details.

## RUK0020B WQFN - 0.8 mm max height (p. 114)

| Col 1 | Col 2 | Col 3 | Col 4 | Col 6 | Col 9 | Col 10 | Col 11 | SCALE 4.000 | SCALE 4.000 | Col 14 | Col 16 | Col 17 | Col 18 | Col 19 | Col 20 | Col 21 | Col 22 | PLASTIC QUAD FLATPACK - NO LEAD | Col 24 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | A |  |  | 3.1 2.9 |  |  |  |  |  |  | B |  |  |  |  |  |  |  |
| PIN 1 INDEX AREA |  |  |  |  |  |  |  |  |  |  |  | 3.1 2.9 |  |  |  | 0.25 0.15 |  | DETAIL OPTIONAL TERMINAL TYPICAL | 0.5 0.3 |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  | DIMENSION A OPTION 01 OPTION 02 | (0.1) (0.2) |
| 0.8 MAX | 0.8 MAX |  |  |  |  |  |  |  |  |  |  |  | C |  |  |  |  | OPTION 02 | (0.2) |
|  | 0.05 0.00 |  |  | 6 | 1.7 | 0.05 |  |  |  | 10 |  |  | EXPOSED | 0.08 | C | SEATING PLANE | (DIM A) TYP |  |  |
| 16X | 0.4 4X | 0.4 | 5 |  | 21 |  |  |  |  |  | 11 | SYMM | EXPOSED THERMAL PAD |  |  |  |  |  |  |
| SEE TERMINAL DETAIL | 1.6 | 1.6 | 1 |  |  |  |  |  |  |  | 15 |  | 20X | 0.25 0.15 0.1 | C | A |  |  |  |
|  | (OPTIONAL) | PIN 1 ID (OPTIONAL) |  |  | SYMM |  | 20X | 20X | 16 0.5 0.3 |  |  |  |  | 0.1 0.05 | C | A |  |  | 4222676/A 02/2016 |

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M. 2. This drawing is subject to change without notice. 3. The package thermal pad must be soldered to the printed circuit board for thermal and mechanical performance.

## RUK0020B WQFN - 0.8 mm max height (p. 115)

| Col 1 | Col 2 | Col 3 | Col 5 | Col 6 | Col 7 | Col 8 | Col 10 | Col 11 | Col 13 | Col 14 | Col 15 | Col 16 | Col 17 | PLASTIC QUAD FLATPACK - NO LEAD |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  |  | ( | 1.7) | 16 | 16 |  |  |  |  |  |  |
| 20X (0.6) | 20X (0.6) | 1 |  |  |  |  |  |  |  |  | 15 |  |  |  |
| 20X (0.2) |  |  |  | 21 |  |  |  |  |  |  | (0.6) TYP | (0.6) TYP SYMM | SYMM |  |
| 16X (0.4) (R0.05) TYP | 16X (0.4) ) | 5 |  |  |  |  |  |  |  |  | 11 |  | (2.8) | (2.8) |
| ( 0.2) 0.05 MAX ALL AROUND | )TYP VIA 0.05 MAX | NON SOLDER MASK DEFINED (PREFERRED) | SOLDER MASK |  |  | SOLDER MASK DETAILS |  | 10 | 0.05 MIN ALL AROUND | 0.05 MIN | SOLDER MASK DEFINED |  | SOLDER MASK OPENING | SOLDER MASK METAL UNDER SOLDER MASK 4222676/A 02/2016 |

NOTES: (continued)

4. This package is designed to be soldered to a thermal pad on the board. For more information, see Texas Instruments literature number SLUA271 (www.ti.com/lit/slua271). 5. Vias are optional depending on application, refer to device data sheet. If any vias are implemented, refer to their locations shown on this view. It is recommended that vias under paste be filled, plugged or tented.

## RUK0020B WQFN - 0.8 mm max height (p. 116)

| Col 1 | Col 2 | Col 3 | Col 4 | Col 6 | Col 7 | Col 8 | Col 9 | Col 12 | Col 13 | Col 14 | Col 17 | Col 18 | Col 19 | PLASTIC QUAD FLATPACK - NO LEAD |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  | (R | 0.05) | )TYP |  | 20 |  |  |  | 16 | (0.47) TYP |  |  |  |  |
| 20X (0.6) 20X (0.2) | 20X (0.6) | 1 |  |  |  |  | 21 |  |  |  | 15 (0.47) TYP | (0.47) TYP SYMM | SYMM |  |
| 16X (0.4) | METAL | 5 |  |  |  |  |  |  |  |  | 11 |  | (2.8) | (2.8) |
| TYP |  |  |  | 6 | 4X ( | 0.75 | 0.75) | 10 | 10 |  |  |  |  | 4222676/A 02/2016 |

NOTES: (continued)

6. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.

## PT0048A LQFP - 1.6 mm max height (p. 117)

| Col 1 | Col 2 | Col 5 | SCALE 2.000 | SCALE 2.000 | Col 10 | Col 12 | Col 13 | Col 14 | Col 15 | Col 16 | LOW PROFILE QUAD FLATPACK | Col 18 | Col 19 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  | 9.2 |  |  |  |  |  |  |  |  |  |  |
|  |  | 7.2 6.8 | 8.8 7.2 6.8 |  |  |  |  |  | A |  |  |  |  |
|  | 9.2 8.8 |  |  |  |  |  |  |  | 7.2 6.8 |  |  |  |  |
| SEE DETAIL A |  | 4X 5.5 | 4X 5.5 |  |  | 44X 0.5 | 44X 0.5 |  | 48X | 0.27 0.17 |  |  |  |
| 1.6 MAX |  | 0.25 -7 |  |  | DETAIL AA15.000 |  | 1.45 1.35 | 0.05 MIN |  | 0.1 C |  | SEATING PLANE | 4215159/B 11/2023 |

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M. 2. This drawing is subject to change without notice. 3. Reference JEDEC registration MS-026. 4. This may also be a thermally enhanced plastic package with leads conected to the die pads.

## PT0048A LQFP - 1.6 mm max height (p. 118)

| Col 1 | Col 2 | Col 3 | Col 5 | Col 6 | Col 7 | Col 8 | Col 11 | Col 12 | Col 14 | Col 15 | Col 18 | Col 19 | Col 20 | Col 21 | LOW PROFILE QUAD FLATPACK |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  | 48 |  |  | PKG SYMM | PKG |  |  |  | DETAILS |  | SEE SOLDER MASK |  |
| 48X (0.3) 44X (0.5) PKG SYMM | 48X (1.6) 44X (0.5) PKG SYMM |  |  |  |  |  |  |  |  |  |  |  | 36 | (8.2) | (8.2) |
| PKG SYMM (R0.05) TYP | PKG SYMM | 12 |  |  |  |  |  |  |  |  |  |  | 25 | (8.2) | (8.2) |
| EXPOSED METAL | SOLDER MASK | OPENING | 0.05 MAX ALLAROUND NON SOLDER MASK DEFINED | 13 | METAL EDGE | METAL EDGE | (8.2) LAND PATTERN EXAMPLE EXPOSED METAL SHOWN SCALE 10.000 SOLDER MASK DETAILS | (8.2) EXPOSED METAL | SOLDER MASK | OPENING | 0.05 MIN ALL AROUND SOLDER MASK DEFINED |  |  | METAL UNDER SOLDER MASK | 4215159/B 11/2023 |

NOTES: (continued)

5. Publication IPC-7351 may have alternate designs. 6. Solder mask tolerances between and around signal pads can vary based on board fabrication site.

## PT0048A LQFP - 1.6 mm max height (p. 119)

| Col 1 | Col 2 | Col 5 | Col 10 | Col 11 | Col 16 | Col 20 | LOW PROFILE QUAD FLATPACK |
| --- | --- | --- | --- | --- | --- | --- | --- |
|  |  | 48 | PKG SYMM | PKG | 37 |  |  |
| 48X (1.6) 48X (0.3) 44X (0.5) PKG SYMM |  |  |  |  |  | 36 (8.2) | (8.2) |
| PKG SYMM (R0.05) TYP | 12 |  |  |  |  | (8.2) 25 | (8.2) |
|  |  | 13 | (8.2) SOLDER PASTE EXAMPLE BASED ON 0.1 mm THICK STENCIL SCALE: 10X | (8.2) | 24 |  | 4215159/B 11/2023 |

NOTES: (continued)

7. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations. 8. Board assembly site may have different recommendations for stencil design.

## DGS0028A VSSOP - 1.1 mm max height (p. 120)

| Col 1 | Col 2 | Col 4 | Col 5 | Col 6 | Col 7 | Col 8 | Col 10 | Col 11 | Col 12 | Col 13 | Col 14 | SMALL OUTLINE PACKAGE | Col 16 | Col 17 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| PIN 1 INDEX | AREA | 5.1 4.7 | TYP |  |  |  |  |  |  |  |  | PLANE | SEATING PLANE |  |
| SEE DETAIL A | 1 14 | 3.1 2.9 | 3.1 2.9 | 4X (7 | 28 15 -15) | 0.275 0.165 0.25 | 4X (0 | -15 | ) |  |  |  | 1.1 MAX | 1.1 MAX |
|  |  |  |  |  |  | -8 |  |  |  | A 20 | DETAIL AA 20 TYPICAL |  | 0.15 0.05 4226365/A 10/2020 | 0.15 0.05 |

PowerPAD is a trademark of Texas Instruments. NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M. 2. This drawing is subject to change without notice. 3. This dimension does not include mold flash, protrusions, or gate burrs. Mold flash, protrusions, or gate burrs shall not exceed 0.15 mm per side. 4. No JEDEC registration as of September 2020. 5. Features may differ or may not be present.

## DGS0028A VSSOP - 1.1 mm max height (p. 121)

| Col 1 | Col 2 | Col 3 | Col 6 | Col 7 | Col 9 | Col 10 | Col 11 | SMALL OUTLINE PACKAGE |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 28X (1.45) 28X (0.3) (R0.05) TYP 26X (0.5) | 1 |  | SYMM | SYMM |  | 28 | SYMM |  |
| SOLDER MASK OPENING 0.05 MAX ALL AROUND | 14 | NON-SOLDER MASK DEFINED (PREFERRED) | (4.4) LAND PATTERN EXAMPLE SCALE: 13X METAL EXPOSED METAL 15.000 | (4.4) LAND PATTERN EXAMPLE SCALE: 13X METAL UNDER SOLDER MASK 0.05 MIN ALL AROUND SOLDER MASK DETAILS | SOLDER MASK DEFINED | 15 SOLDER MASK DEFINED |  | SOLDER MASK OPENING EXPOSED METAL 4226365/A 10/2020 |

NOTES: (continued)

6. Publication IPC-7351 may have alternate designs. 7. Solder mask tolerances between and around signal pads can vary based on board fabrication site. 8. This package is designed to be soldered to a thermal pad on the board. For more information, see Texas Instruments literature numbers SLMA002 (www.ti.com/lit/slma002) and SLMA004 (www.ti.com/lit/slma004). 9. Size of metal pad may vary due to creepage requirement. 10. Vias are optional depending on application, refer to device data sheet. It is recommended that vias under paste be filled, plugged or tented.

## DGS0028A VSSOP - 1.1 mm max height (p. 122)

| Col 1 | Col 2 | Col 5 | Col 6 | Col 9 | SMALL OUTLINE PACKAGE |
| --- | --- | --- | --- | --- | --- |
| 28X (1.45) 28X (0.3) (R0.05) TYP 26X (0.5) | 1 | SYMM | SYMM | 28 | SYMM |
|  | 14 | (4.4) | (4.4) SOLDER PASTE EXAMPLE BASED ON 0.125 mm THICK STENCIL SCALE: 13X | 15 | 4226365/A 10/2020 |

NOTES: (continued)

11. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations. 12. Board assembly site may have different recommendations for stencil design.

## RGE 24 VQFN - 1 mm max height (p. 123)

PLASTIC QUAD FLATPACK - NO LEAD

Images above are just a representation of the package family, actual package may vary. Refer to the product data sheet for package details.

4204104/H

## RGE0024B VQFN - 1 mm max height (p. 124)

| Col 1 | Col 2 | Col 3 | Col 4 | Col 5 | SCALE 3.000 | Col 7 | Col 9 | Col 10 | Col 11 | Col 13 | PLASTIC QUAD FLATPACK - NO LEAD | Col 15 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  | 4.1 3.9 |  |  |  | B |  |  |  |  |  |
| PIN 1 INDEX AREA |  |  |  |  |  |  | 4.1 3.9 |  | 0.3 0.2 | DETAIL OPTIONAL TERMINAL TYPICAL | 0.5 0.3 | 0.5 0.3 |
| 1 MAX |  |  |  |  |  |  |  |  |  |  |  |  |
| 0.05 0.00 |  | 7 | 2X 2.5 2.45 | 0.1 |  | 12 | EXPOSED | 0.08 C |  |  |  | (0.2) TYP |
| SEE TERMINAL DETAIL | 6 |  | 25 |  |  |  | EXPOSED SYMM |  |  |  |  |  |
| 20X 0.5 | 1 |  |  |  |  |  |  | 0.3 24X 0.2 |  |  |  |  |
|  |  |  | SYMM | 24X | 19 0.5 0.3 | 19 |  | 0.1 0.05 |  |  |  | 4219013/A 05/2017 |

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M. 2. This drawing is subject to change without notice. 3. The package thermal pad must be soldered to the printed circuit board for thermal and mechanical performance.

## RGE0024B VQFN - 1 mm max height (p. 125)

| Col 1 | Col 2 | Col 3 | Col 5 | Col 6 | Col 7 | Col 8 | Col 9 | Col 10 | Col 12 | Col 13 | Col 14 | Col 15 | Col 16 | Col 17 | PLASTIC QUAD FLATPACK - NO LEAD |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 24X (0.6) | 24X (0.6) |  |  |  |  | ( | 2.45) |  |  | 19 |  |  |  |  |  |
| 24X (0.25) (R0.05) TYP |  | 1 |  |  |  | 25 |  |  |  |  |  |  | 18 SYMM | SYMM |  |
| 20X (0.5) ( | 20X (0.5) 0.2) TYP | 6 |  |  |  |  |  |  |  |  |  |  | 13 | (3.8) | (3.8) |
| 0.07 MAX ALL AROUND EXPOSED METAL | VIA 0.07 MAX | VIA NON SOLDER MASK DEFINED (PREFERRED) | METAL OPENING | (0.975) TYP SOLDER MASK OPENING | SOLDER MASK | LAND PATTERN EXAMPLE EXPOSED METAL SHOWN SCALE:15X |  | SOLDER MASK DETAILS | EXPOSED | METAL | 0.07 MIN ALL AROUND | 0.07 MIN | SOLDER MASK DEFINED |  | SOLDER MASK OPENING METAL UNDER SOLDER MASK 4219013/A 05/2017 |

NOTES: (continued)

4. This package is designed to be soldered to a thermal pad on the board. For more information, see Texas Instruments literature number SLUA271 (www.ti.com/lit/slua271). 5. Vias are optional depending on application, refer to device data sheet. If any vias are implemented, refer to their locations shown on this view. It is recommended that vias under paste be filled, plugged or tented.

## RGE0024B VQFN - 1 mm max height (p. 126)

| Col 1 | Col 2 | Col 3 | Col 6 | Col 7 | Col 8 | Col 9 | Col 10 | Col 11 | Col 15 | Col 19 | PLASTIC QUAD FLATPACK - NO LEAD | PLASTIC QUAD FLATPACK - NO LEAD |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  | 24 | 4X ( | 4X ( | 1.08) |  |  | (0.64) TYP |  |  |  |
| 24X (0.6) |  | 1 |  |  |  | 25 | 25 |  |  | 18 |  |  |
| 24X (0.25) |  |  |  |  |  |  |  |  |  | 18 |  |  |
| (R0.05) TYP SYMM | SYMM |  |  |  |  |  |  |  |  | (0.64) TYP | (0.64) TYP |  |
| 20X (0.5) | 20X (0.5) |  |  |  |  |  |  |  |  |  | (3.8) | (3.8) |
|  |  | 6 METAL |  |  |  |  |  |  |  | 13 |  |  |
|  | TYP |  | 7 |  |  |  |  | SYMM (3.8) SOLDER PASTE EXAMPLE BASED ON 0.125 mm THICK STENCIL EXPOSED PAD 25 78% PRINTED SOLDER COVERAGE BY AREA UNDER PACKAGE SCALE:20X |  |  |  | 4219013/A 05/2017 |

NOTES: (continued)

6. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.

## RHB 32 VQFN - 1 mm max height (p. 127)

5 x 5, 0.5 mm pitch PLASTIC QUAD FLATPACK - NO LEAD

Images above are just a representation of the package family, actual package may vary. Refer to the product data sheet for package details.

## RHB0032E VQFN - 1 mm max height (p. 128)

| Col 1 | Col 2 | Col 3 | Col 4 | Col 5 | Col 6 | Col 7 | Col 9 | Col 10 | Col 11 | Col 12 | PLASTIC QUAD FLATPACK - NO LEAD | Col 16 | Col 17 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
|  |  |  |  | 5.1 4.9 |  |  |  |  |  |  |  |  |  |
| PIN 1 INDEX AREA |  |  |  |  |  |  |  | 5.1 |  |  | SIDE WALL DETAIL OPTIONAL METAL THICKNESS20.000 | (0.1) SIDE WALL DETAIL OPTIONAL METAL THICKNESS |  |
|  | 1 MAX |  |  |  |  |  |  |  |  |  |  |  |  |
|  | 0.05 0.00 |  | 9 | 2X 3.5 3.45 | 0.1 |  |  |  | 0.08 C |  |  |  | (0.2) TYP |
| 28X 0.5 |  |  |  |  |  |  | 17 |  | SEE SIDE WALL |  |  |  |  |
|  | 2X 3.5 |  |  |  | 33 |  | 17 | SYMM | SEE SIDE WALL |  |  |  |  |
|  | 3.5 |  |  |  |  |  | 24 |  | 0.3 0.2 0.1 0.05 | C A B C |  |  |  |
|  | (OPTIONAL) | PIN 1 ID |  | SYMM | SYMM | 25 0.5 0.3 |  |  |  |  |  |  | 4223442/B 08/2019 |

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M. 2. This drawing is subject to change without notice. 3. The package thermal pad must be soldered to the printed circuit board for thermal and mechanical performance.

## RHB0032E VQFN - 1 mm max height (p. 129)

| Col 1 | Col 2 | Col 3 | Col 4 | Col 6 | Col 9 | Col 10 | Col 11 | Col 12 | Col 13 | Col 15 | Col 17 | Col 18 | Col 19 | Col 21 | Col 22 | PLASTIC QUAD FLATPACK - NO LEAD | Col 24 | Col 25 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 32X (0.6) |  |  |  | 32 |  |  |  | ( | 3.45) |  |  |  | 25 |  |  |  |  |  |
|  |  | 1 |  |  |  |  |  |  |  |  |  |  |  | 24 |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  |  |  |  | 24 |  |  |  |  |
| 32X (0.25) 28X (0.5) |  |  |  |  |  |  |  |  | 33 |  |  |  |  |  | (1.475) | (1.475) SYMM | SYMM |  |
| ( 0.2) TYP | VIA | 8 |  |  |  |  |  |  |  |  |  |  |  | 17 |  |  |  | (4.8) |
| (R0.05) TYP | TYP | 8 |  |  |  |  |  |  |  |  |  |  |  | 17 |  |  |  |  |
|  |  |  |  | 9 |  |  |  |  |  | (1.475) |  |  |  |  |  |  |  |  |
|  |  |  | 0.07 MAX ALL AROUND | NON SOLDER MASK DEFINED (PREFERRED) | METAL | METAL OPENING | SOLDER MASK |  | SOLDER MASK DETAILS |  | ALL AROUND | 0.07 MIN | SOLDER MASK DEFINED |  | OPENING | SOLDER MASK OPENING METAL UNDER SOLDER MASK |  | 4223442/B 08/2019 |

NOTES: (continued)

4. This package is designed to be soldered to a thermal pad on the board. For more information, see Texas Instruments literature number SLUA271 (www.ti.com/lit/slua271). 5. Vias are optional depending on application, refer to device data sheet. If any vias are implemented, refer to their locations shown on this view. It is recommended that vias under paste be filled, plugged or tented.

## RHB0032E VQFN - 1 mm max height (p. 130)

| Col 1 | Col 2 | Col 3 | Col 6 | Col 7 | Col 9 | Col 10 | Col 11 | Col 13 | Col 14 | Col 15 | Col 19 | Col 20 | Col 24 | PLASTIC QUAD FLATPACK - NO LEAD | Col 26 | Col 27 | Col 28 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 32X (0.6) |  | (R0.05) TYP | 32 |  | 4X ( | 1.49) | 1.49) |  |  | (0.845) |  | 25 |  |  |  |  |  |
|  |  | 1 |  |  |  |  |  |  |  |  |  |  | 24 |  |  |  |  |
| 32X (0.25) 28X (0.5) |  |  |  |  |  |  |  |  | 33 |  |  |  | (0.845) | (0.845) SYMM | SYMM |  |  |
|  |  | 8 |  |  |  |  |  |  | 33 |  |  |  | 17 |  |  | (4.8) | (4.8) |
| TYP | METAL |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |  |
|  |  |  |  | 9 |  |  |  | SYMM (4.8) |  |  | 16 |  |  |  |  |  |  |
|  |  |  |  |  |  |  |  | (4.8) SOLDER PASTE EXAMPLE BASED ON 0.125 mm THICK STENCIL EXPOSED PAD 33: 75% PRINTED SOLDER COVERAGE BY AREA UNDER PACKAGE SCALE:20X |  |  |  |  |  |  |  | 4223442/B 08/2019 |  |

NOTES: (continued)

6. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.

## DGS0020A VSSOP - 1.1 mm max height (p. 131)

| Col 1 | Col 2 | Col 4 | Col 5 | Col 6 | Col 7 | Col 8 | Col 10 | Col 11 | Col 12 | Col 13 | Col 14 | SMALL OUTLINE PACKAGE | Col 16 | Col 17 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| PIN 1 INDEX | AREA | 5.1 4.7 | TYP |  |  |  |  |  |  |  |  |  | SEATING PLANE |  |
| SEE DETAIL A | 1 10 | 3.1 2.9 | 3.1 2.9 | 4X (7 | 20 11 -15) | 0.275 0.165 0.25 | 4X (0 | -15 | ) |  |  |  | 1.1 MAX | 1.1 MAX |
|  |  |  |  |  |  | -8 |  |  |  | A 20 | DETAIL AA 20 TYPICAL |  | 0.15 0.05 4226367/A 10/2020 | 0.15 0.05 |

PowerPAD is a trademark of Texas Instruments. NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M. 2. This drawing is subject to change without notice. 3. This dimension does not include mold flash, protrusions, or gate burrs. Mold flash, protrusions, or gate burrs shall not exceed 0.15 mm per side. 4. No JEDEC registration as of September 2020. 5. Features may differ or may not be present.

## DGS0020A VSSOP - 1.1 mm max height (p. 132)

| Col 1 | Col 2 | Col 3 | Col 4 | Col 5 | Col 6 | Col 9 | Col 10 | SMALL OUTLINE PACKAGE |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 20X (1.45) 20X (0.3) (R0.05) TYP 18X (0.5) | 1 |  |  | SYMM | SYMM |  | 20 | SYMM |
| SOLDER MASK OPENING 0.05 MAX ALL AROUND | 10 OPENING | NON-SOLDER MASK DEFINED (PREFERRED) | NON-SOLDER MASK | (4.4) LAND PATTERN EXAMPLE SCALE: 16X METAL EXPOSED METAL 15.000 | (4.4) LAND PATTERN EXAMPLE SCALE: 16X METAL UNDER SOLDER MASK 0.05 MIN ALL AROUND SOLDER MASK DETAILS | SOLDER MASK DEFINED | 11 | SOLDER MASK OPENING EXPOSED METAL 4226367/A 10/2020 |

NOTES: (continued)

6. Publication IPC-7351 may have alternate designs. 7. Solder mask tolerances between and around signal pads can vary based on board fabrication site. 8. This package is designed to be soldered to a thermal pad on the board. For more information, see Texas Instruments literature numbers SLMA002 (www.ti.com/lit/slma002) and SLMA004 (www.ti.com/lit/slma004). 9. Size of metal pad may vary due to creepage requirement. 10. Vias are optional depending on application, refer to device data sheet. It is recommended that vias under paste be filled, plugged or tented.

## DGS0020A VSSOP - 1.1 mm max height (p. 133)

| Col 1 | Col 2 | Col 5 | Col 6 | Col 9 | SMALL OUTLINE PACKAGE |
| --- | --- | --- | --- | --- | --- |
| 20X (1.45) 20X (0.3) (R0.05) TYP | 1 | SYMM | SYMM | 20 | SYMM |
| (18X 0.5) | 10 | (4.4) | (4.4) SOLDER PASTE EXAMPLE BASED ON 0.125 mm THICK STENCIL SCALE: 16X | 11 | 4226367/A 10/2020 |

NOTES: (continued)

11. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations. 12. Board assembly site may have different recommendations for stencil design.

## RGZ 48 VQFN - 1 mm max height (p. 134)

7 x 7, 0.5 mm pitch PLASTIC QUADFLAT PACK- NO LEAD

Images above are just a representation of the package family, actual package may vary. Refer to the product data sheet for package details.

## RGZ0048B VQFN - 1 mm max height (p. 135)

| Col 1 | Col 2 | Col 3 | Col 4 | Col 5 | Col 6 | SCALE 2.000 | Col 8 | Col 9 | Col 10 | Col 11 | Col 12 | Col 13 | Col 15 | Col 16 | PLASTIC QUAD FLATPACK - NO LEAD | Col 19 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| PIN 1 INDEX AREA |  |  |  | 7.15 6.85 | 7.15 6.85 |  |  |  |  | A 7.15 6.85 |  |  |  |  |  |  |
| 1 MAX |  |  |  |  |  |  |  |  |  |  | C |  |  |  |  |  |
|  |  |  |  |  |  |  |  |  |  |  | C |  |  |  |  |  |
| 44X 0.5 | 0.05 0.00 |  |  | 2X 5.5 4.1 | 2X 5.5 0.1 |  |  | 24 |  |  | EXPOSED | 0.08 C THERMAL PAD | SEATING PLANE | SEATING PLANE |  | (0.2) TYP |
| 44X 0.5 | 2X | 12 |  | 49 |  |  |  |  | 25 | SYMM |  | THERMAL PAD |  |  |  |  |
|  | 5.5 | 1 |  |  |  |  |  |  | 36 |  | 48X | 0.30 0.18 0.1 | C B A |  |  |  |
|  | PIN 1 ID (OPTIONAL) |  | 48 | SYMM | SYMM | 0.5 48X 0.3 | 37 0.5 0.3 |  |  |  |  | 0.1 0.05 | C B A |  |  | 4218795/B 02/2017 |

NOTES:

1. All linear dimensions are in millimeters. Any dimensions in parenthesis are for reference only. Dimensioning and tolerancing per ASME Y14.5M. 2. This drawing is subject to change without notice. 3. The package thermal pad must be soldered to the printed circuit board for thermal and mechanical performance.

## RGZ0048B VQFN - 1 mm max height (p. 136)

| Col 1 | Col 2 | Col 3 | Col 4 | Col 5 | Col 7 | Col 8 | Col 9 | Col 10 | PLASTIC QUAD FLATPACK - NO LEAD |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 48X (0.6) | 48X (0.6) |  |  | 48 | ( (1.115) TYP |  |  |  |  |
| 48X (0.24) |  | 1 |  |  |  |  | 36 |  |  |
| 44X (0.5) | SYMM | SYMM |  |  | 49 |  | (1.115) TYP (0.685) TYP |  |  |
| ( 0.2) TYP (R0.05) TYP | VIA |  |  |  |  |  |  | (6.8) | (6.8) |
|  |  | 12 |  |  |  |  | 25 |  |  |
| EXPOSED METAL |  | 0.07 MAX ALL AROUND | 0.07 MAX | 13 NON SOLDER MASK DEFINED (PREFERRED) | SYMM (6.8) LAND PATTERN EXAMPLE EXPOSED METAL SHOWN SCALE:12X | SOLDER MASK DETAILS |  | SOLDER MASK OPENING | SOLDER MASK METAL UNDER SOLDER MASK 4218795/B 02/2017 |

NOTES: (continued)

4. This package is designed to be soldered to a thermal pad on the board. For more information, see Texas Instruments literature number SLUA271 (www.ti.com/lit/slua271). 5. Vias are optional depending on application, refer to device data sheet. If any vias are implemented, refer to their locations shown on this view. It is recommended that vias under paste be filled, plugged or tented.

## RGZ0048B VQFN - 1 mm max height (p. 137)

| Col 1 | Col 2 | Col 3 | Col 8 | Col 9 | Col 11 | Col 12 | PLASTIC QUAD FLATPACK - NO LEAD |
| --- | --- | --- | --- | --- | --- | --- | --- |
| 48X (0.6) | 48X (0.6) |  |  |  |  |  |  |
| 48X (0.24) |  | 1 |  |  | 36 |  |  |
| 44X (0.5) | SYMM | SYMM |  | 49 | (1.37) TYP | (1.37) |  |
| (R0.05) TYP |  |  |  |  |  | (6.8) | (6.8) |
| METAL TYP |  |  |  |  | 9X ( | 9X 1.17) |  |
|  |  | 12 |  |  | 25 |  |  |
|  |  |  | SYMM (6.8) SOLDER PASTE EXAMPLE BASED ON 0.125 mm THICK STENCIL EXPOSED PAD 49 73% PRINTED SOLDER COVERAGE BY AREA UNDER PACKAGE SCALE:12X |  |  |  | 4218795/B 02/2017 |

NOTES: (continued)

6. Laser cutting apertures with trapezoidal walls and rounded corners may offer better paste release. IPC-7525 may have alternate design recommendations.

IMPORTANT NOTICE AND DISCLAIMER TI PROVIDES TECHNICAL AND RELIABILITY DATA (INCLUDING DATASHEETS), DESIGN RESOURCES (INCLUDING REFERENCE DESIGNS), APPLICATION OR OTHER DESIGN ADVICE, WEB TOOLS, SAFETY INFORMATION, AND OTHER RESOURCES “AS IS” AND WITH ALL FAULTS, AND DISCLAIMS ALL WARRANTIES, EXPRESS AND IMPLIED, INCLUDING WITHOUT LIMITATION ANY IMPLIED WARRANTIES OF MERCHANTABILIT Y, FITNESS FOR A PARTICULAR PURPOSE OR NON-INFRINGEMENT OF THIRD PARTY INTELLECTUAL PROPERTY RIGHTS. These resources are intended for skilled developers designing with TI products. Y ou are solely responsible for (1) selecting the appropriate TI products for your application, (2) designing, validating and testing your application, and (3) ensuring your application meets applicable standards, and any other safety, security, regulatory or other requirements. These resources are subject to change without notice. TI grants you permission to use these resources only for development of an application that uses the TI products described in the resource. Other reproduction and display of these resources is prohibited. No license is granted to any other TI intellectual property right or to any third party intellectual property right. TI disclaims responsibility for, and you fully indemnify TI and its representatives against any claims, damages, costs, losses, and liabilities arising out of your use of these resources. TI’s products are provided subject to TI’s T erms of Sale, TI’s General Quality Guidelines, or other applicable terms available either on ti.com or provided in conjunction with such TI products. TI’s provision of these resources does not expand or otherwise alter TI’s applicable warranties or warranty disclaimers for TI products. Unless TI explicitly designates a product as custom or customer-specified, TI products are standard, catalog, general purpose devices. TI objects to and rejects any additional or different terms you may propose. IMPORTANT NOTICE

6, Texas Instruments Incorporated Last updated 10/2025


---

