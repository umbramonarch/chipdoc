# MSPM0C1105, MSPM0C1106 Mixed-Signal Microcontrollers datasheet (Rev. B)

> **Agent Navigation Guide**:
> - Prefer 1-turn CLI queries: `chipdoc reg <name|hex>`, `chipdoc table <query> --show`, `chipdoc xref <symbol>`, `chipdoc page <N>`.
> - For fast 1-line-per-bitfield register lookups, grep `index/registers.tsv`.
> - For cross-chapter hardware relationships (pins, interrupts, memory map, registers), grep `index/xref.tsv`.
> - For electrical specification or summary tables, grep `index/tables.tsv`.
> - For chapter/peripheral details, grep `sections/` or open the corresponding file below.

## Document Summary

- **Source File**: `mspm0c1105_datasheet.pdf`
- **Author**: Texas Instruments, Incorporated [SLASFJ6,B]
- **Document ID / Keywords**: `SLASFJ6B`
- **Pages Converted**: 138 / 138
- **Section Files**: 12 (`sections/*.md`)
- **Tables Extracted**: 84 (34 stitched across pages) -> `index/tables.tsv`
- **Registers / Bitfields Indexed**: 0 registers / 0 bitfields -> `index/registers.tsv`
- **Figures Detected**: 21 (21 rendered to `assets/figures/`) -> `index/figures.tsv`
- **Cross-References Indexed**: 2479 symbols -> `index/xref.tsv`
- **Sanity Score**: 100/100 (~56904 tokens)

## Section Shards (`sections/*.md`)

| ID | Section Title | File | Pages | Lines | Tables | Registers | Figures |
| --- | --- | --- | --- | ---: | ---: | ---: | ---: |
| `01` | 1 Features / 2 Applications | [sections/01-1-features-2-applications.md](sections/01-1-features-2-applications.md) | 1-1 | 43 | 0 | 0 | 0 |
| `02` | 3 Description | [sections/02-3-description.md](sections/02-3-description.md) | 2-3 | 63 | 1 | 0 | 0 |
| `03` | 4 Functional Block Diagram | [sections/03-4-functional-block-diagram.md](sections/03-4-functional-block-diagram.md) | 4-4 | 16 | 0 | 0 | 1 |
| `04` | Table of Contents | [sections/04-table-of-contents.md](sections/04-table-of-contents.md) | 5-5 | 25 | 0 | 0 | 0 |
| `05` | 5 Device Comparison | [sections/05-5-device-comparison.md](sections/05-5-device-comparison.md) | 6-6 | 44 | 1 | 0 | 0 |
| `06` | 6 Pin Configuration and Functions | [sections/06-6-pin-configuration-and-functions.md](sections/06-6-pin-configuration-and-functions.md) | 7-40 | 908 | 15 | 0 | 10 |
| `07` | 7 Specifications | [sections/07-7-specifications.md](sections/07-7-specifications.md) | 41-57 | 700 | 33 | 0 | 5 |
| `08` | 8 Detailed Description | [sections/08-8-detailed-description.md](sections/08-8-detailed-description.md) | 58-78 | 869 | 17 | 0 | 3 |
| `09` | 9 Applications, Implementation, and Layout | [sections/09-9-applications-implementation-and-layout.md](sections/09-9-applications-implementation-and-layout.md) | 79-79 | 30 | 0 | 0 | 1 |
| `10` | 10 Device and Documentation Support | [sections/10-10-device-and-documentation-support.md](sections/10-10-device-and-documentation-support.md) | 80-82 | 92 | 1 | 0 | 1 |
| `11` | 11 Revision History | [sections/11-11-revision-history.md](sections/11-11-revision-history.md) | 82-82 | 23 | 1 | 0 | 0 |
| `12` | 12 Mechanical, Packaging, and Orderable Information | [sections/12-12-mechanical-packaging-and-orderable-information.md](sections/12-12-mechanical-packaging-and-orderable-information.md) | 83-138 | 692 | 15 | 0 | 0 |

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
  - **10.5 Support Resources** — p. 82 (`sections/10-10-device-and-documentation-support.md`)
  - **10.6 Trademarks** — p. 82 (`sections/10-10-device-and-documentation-support.md`)
  - **10.7 Electrostatic Discharge Caution** — p. 82 (`sections/10-10-device-and-documentation-support.md`)
  - **10.8 Glossary** — p. 82 (`sections/10-10-device-and-documentation-support.md`)
- **11 Revision History** — p. 82 (`sections/11-11-revision-history.md`)
- **12 Mechanical, Packaging, and Orderable Information** — p. 83 (`sections/12-12-mechanical-packaging-and-orderable-information.md`)
