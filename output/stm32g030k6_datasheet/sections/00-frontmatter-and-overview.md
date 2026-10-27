---
document: "Datasheet - STM32G030x6/x8 - Arm® Cortex®-M0+ 32-bit MCU, up to 64 KB Flash, 8 KB RAM, 2x USART, timers, ADC, comm. I/Fs, 2.0-3.6 V"
section_id: "00"
section_title: "Frontmatter and Overview"
pages: "1-7"
---

# Frontmatter and Overview (pp. 1-7)

| Col 1 | Col 2 | Col 3 | STM32G030x6/x8 |
| --- | --- | --- | --- |
|  | ® Arm | Cortex® | -M0+ 32-bit MCU, up to 64 KB Flash, 8 KB RAM, 2x USART, timers, ADC, comm. I/Fs, 2.0-3.6 V |
|  |  |  | Datasheet - production data |
| Features • • • • | Features technology® Core: Arm frequency up to 64 MHz Memories | Includes ST state-of-the-art patented ® 32-bit Cortex -M0+ CPU, -40°C to 85°C operating temperature – Up to 64 Kbytes of flash memory with | SO8N TSSOP20 LQFP32 (4.9 × 6 mm) (6.4 × 4.4 mm) (7 × 7 mm) LQFP48 (7 × 7 mm) |

protection • Communication interfaces2 – 8 Kbytes of SRAM with HW parity check – Two I C-bus interfaces supporting Fast-

- CRC calculation unit mode Plus (1 Mbit/s) with extra current

- Reset and power management sink, one supporting SMBus/PMBus and wakeup from Stop mode – Voltage range: 2.0 V to 3.6 V – Two USARTs with master/slave – Power-on/Power-down reset (POR/PDR) synchronous SPI; one supporting ISO7816 – Low-power modes: interface, LIN, IrDA capability, auto baud Sleep, Stop, Standby rate detection and wakeup feature –V BAT supply for RTC and backup registers – Two SPIs (32 Mbit/s) with 4- to 16-bit

- Clock management programmable bitframe, one multiplexed2 – 4 to 48 MHz crystal oscillator with I S interface; two extra SPIs through – 32 kHz crystal oscillator with calibration USARTs – Internal 16 MHz RC with PLL option • Development support: serial wire debug (SWD) – Internal 32 kHz RC oscillator (±5 %)

- All packages ECOPACK 2 compliant

- Up to 44 fast I/Os – All mappable on external interrupt vectors Table 1. Device summary – Multiple 5 V-tolerant I/Os

| Col 1 | Reference Part number |
| --- | --- |
| STM32G030x6 | STM32G030C6, STM32G030F6, STM32G030J6, STM32G030K6 |
|  | STM32G030x8 STM32G030C8, STM32G030K8 |

- 5-channel DMA controller with flexible mapping

- 12-bit, 0.4 µs ADC (up to 16 ext. channels) – Up to 16-bit with hardware oversampling – Conversion range: 0 to 3.6V

- 8 timers: 16-bit for ad vanced motor control, four 16-bit general-purpose, two watchdogs, SysTick timer

- Calendar RTC with alarm and periodic wakeup from Stop/Standby

June 2025 DS12991 Rev 6 1/100

This is information on a product in full production. www.st.com

## Contents (p. 2)

1 Introduction . . . . . . . . . . . .

2 Description . . . . . . . .

3 Functional overview . . . . . .

3.1 Arm® Cortex®

3.13.1 Nested vectored interrupt controller (NVIC) 3.13.2 Extended interrupt/event controller (EXTI)

3.14.1 Temperature sensor . . . . . . . . . . . . . . 3.14.2 Internal voltage reference (V REFINT 3.14.3 V BAT

3.15.2 General-purpose timers (TIM3, 14, 16, 17

2

### 3.17 Inter-integrated circuit interface (I (p. 3)

3.18 Universal synchronous/asynchronous re ceiver transmitter (USART) . . . 27

4 Pinouts, pin descripti on and alternate functions . . .

5 Electrical characteristi cs . . . . . . . . . . .

5.3.1 General operating conditions 5.3.2 Operating conditions at power-up / powe 5.3.3 Embedded reset and power control bloc

5.3.6 Wakeup time from low-power modes and voltage scaling

5.3.8 Internal clock source charac

5.3.16 Extended interrupt and event controller input (EXTI) characteristics . . . 67

5.3.18 Analog-to-digital converter characteristics

5.3.20 V BAT

5.3.22 Characteristics of communication interfac

6 Package information . . . . . . .

7 Ordering information . . . . . .

8 Important security notice . . . .

9 Revision history . . . . . . . . .

## List of tables (p. 5)

Table 2. STM32G030x6/x8 family device Table 3. Access status versus readout protection level

Table 5. Temperature sensor calibration values. . . . . . . . . Table 6. Internal voltage reference calibrati Table 7. Timer feature comparison. . . . . .2. . 23 Table 8. I 6

Table 11. Terms and symbols used in Pin assignment and description

Table 13. Port A alternate function mapping. 35 Table 14. Port B alternate function mapping. 36 Table 15. Port C alternate function mapping. 37 Table 16. Port D alternate function mapping. 37

Table 20. Thermal characteristics.. . . . 41

Table 23. Embedded reset and power control block characterist

Table 25. Current consumption in Run and Low-power run modes

Table 26. Current consumption in Sleep and Low-power slee

9 Table 33. Regulator mode transition times .. 50 Table 34. High-speed external user clock characteristics. . . Table 35. Low-speed external user clock characteristics . . . .

Table 37. LSE oscillator characteristics (f LSE

Table 42. Flash memory endurance and data

Table 45. ESD absolute maximum ratings . . 59 Table 46. Electrical sensitivity. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

Table 51. Non-FT_c I/O output timing characteristics . . . . .

Table 57. Maximum ADC R AIN Table 58. ADC accuracy. . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . Table 59. TS characteristics . . . . . . . . . . . . . . . . . . Table 60. V BAT Table 61. V BAT

Table 63. IWDG min/max timeout period at 32 kHz LSI clock .

Table 66. SPI characteristics . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . 2

Table 68. USART characteristics in SPI mode . 0

Table 73. Package thermal characteristics.. 95

## List of figures (p. 7)

Figure 1. Block diagram . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

Figure 4. STM32G030Fx TSSOP20 pinout . . . . . . . . . . . . . .

Figure 8. Pin input voltage . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

Figure 10. Current consumption measurement scheme . . . . . . Figure 11. V REFINT Figure 12. High-speed external clock source AC timing diagra Figure 13. Low-speed external clock source AC timing diagram.

Figure 15. Typical application with a 32.768 kHz crystal . . . .

Figure 17. Current injection into FT_e input with diode active

Figure 19. Recommended NRST pin protection . . . . . . . . . .

Figure 24. SPI timing diagram - master mode . . . . . . . . . .2 78 Figure 25. I2 Figure 26. I S master timing diagram (Ph Figure 27. USART timing diagram in SPI ma

Figure 29. SO8N -Outline . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .

Figure 33. LQFP32 - Outline . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . Figure 34. LQFP32 – Footprint example . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . . .(15). . 91 Figure 35. LQFP48 - Outline
