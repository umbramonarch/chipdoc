---
document: "STM32G0x0 advanced Arm®-based 32-bit MCUs - Reference manual"
section_id: "07"
section_title: "7 System configuration controller (SYSCFG)"
pages: "189-207"
---

# 7 System configuration controller (SYSCFG) (pp. 189-207)

## 7 System configuration controller (SYSCFG) (p. 189)

The devices feature a set of configuration registers. The main purposes of the system configuration controller are the following: 2

- Enabling/disabling I C Fast Mode Plus on some I/O ports

- Enabling/disabling the analog switch booster

- Configuring the IR modulation signal and its output polarity

- Remapping of some I/O ports

- Remapping the memory located at t he beginning of the code area

- Flag pending interrupts from each interrupt line

- Managing robustness feature

### 7.1 SYSCFG registers (p. 189)

### 7.1.1 [SYSCFG.SYSCFG_CFGR1] Offset: 0x00 | Reset: 0x0000000X (p. 189)

This register is used for specific configurations of memory and DMA requests remap and to control special I/O features.

Two bits are used to configure the type of memory accessible at address 0x0000 0000. These bits are used to select th e physical remap by software an d so, bypass the hardware BOOT selection. After reset thes e bits take the value selected by the actual boot mode configuration.

Address offset: 0x00

Reset value: 0x0000 000X (X is the memory mode selected by the actual boot mode configuration

1. Only significant on devices i ntegrating the corresponding peripheral or function, otherwise reserved. Refer to Section 1.4: Availability of peripherals.

Bits 31:25 Reserved, must be kept at reset value.

Bit 24 I2C3_FMP: Fast Mode Plus (FM +) enable for I2C3 (1) This bit is set and cleared by software. It enables I2C FM+ driving capability on I/O ports configured as I2C3 throug h GPIOx_AFR registers. 0: Disable 1: Enable With this bit in disable state, the I2C FM+ driving capability on I/O ports configured as I2C32 can be enabled through their co rresponding I2Cx_FMP bit. When I C FM+ is enabled, the speed control is ignored.

Bit 23 I2C_PA10_FMP: Fast Mode Plus (FM+) enable for PA10 This bit is set and cleared by software. It enables I2C FM+ driving capability on PA10 I/O port. 0: Disable 1: Enable With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled2 through one of I2Cx_FMP bits. When I C FM+ is enabled, the s peed control is ignored.

Bit 22 I2C_PA9_FMP: Fast Mode Plus (FM+) enable for PA9 This bit is set and cleared by software. It enables I2C FM+ driving capability on PA9 I/O port. 0: Disable 1: Enable With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled2 through one of I2Cx_FMP bits. When I C FM+ is enabled, the s peed control is ignored.

Bit 21 I2C2_FMP: Fast Mode Plus (FM +) enable for I2C2 This bit is set and cleared by software. It enables I2C FM+ driving capability on I/O ports configured as I2C2 throug h GPIOx_AFR registers. 0: Disable 1: Enable With this bit in disable state, the I2C FM+ driving capability on I/O ports configured as I2C22 can be enabled through their co rresponding I2Cx_FMP bit. When I C FM+ is enabled, the speed control is ignored.

Bit 20 I2C1_FMP: Fast Mode Plus (FM +) enable for I2C1 This bit is set and cleared by software. It enables I2C FM+ driving capability on I/O ports configured as I2C1 throug h GPIOx_AFR registers. 0: Disable 1: Enable With this bit in disable state, the I2C FM+ driving capability on I/O ports configured as I2C12 can be enabled through their co rresponding I2Cx_FMP bit. When I C FM+ is enabled, the speed control is ignored.

Bit 19 I2C_PB9_FMP: Fast Mode Plus (FM+) enable for PB9 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB9 I/O port. 0: Disable 1: Enable With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled2 through one of I2Cx_FMP bits. When I C FM+ is enabled, the s peed control is ignored.

Bit 18 I2C_PB8_FMP: Fast Mode Plus (FM+) enable for PB8 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB8 I/O port. 0: Disable 1: Enable With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled2 through one of I2Cx_FMP bits. When I C FM+ is enabled, the s peed control is ignored.

Bit 17 I2C_PB7_FMP: Fast Mode Plus (FM+) enable for PB7 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB7 I/O port. 0: Disable 1: Enable With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled2 through one of I2Cx_FMP bits. When I C FM+ is enabled, the s peed control is ignored.

Bit 16 I2C_PB6_FMP: Fast Mode Plus (FM+) enable for PB6 This bit is set and cleared by software. It enables I2C FM+ driving capability on PB6 I/O port. 0: Disable 1: Enable With this bit in disable state, the I2C FM+ driving capability on this I/O port can be enabled2 through one of I2Cx_FMP bits. When I C FM+ is enabled, the s peed control is ignored.

Bits 15:11 Reserved, must be kept at reset value.

Bit 10 UCPD2_STROBE: PD0 and PD2 pull-down configuration strobe (1) Upon power on, internal pull-dow n resistors on PD0 and PD2 pins are enabled (connected). Setting this bit disab les (disconnects) the internal pull-down resistors.

Bit 9 UCPD1_STROBE: PB15 and PA8 pull-down configuration strobe (1) Upon power on, internal pull-down resistors on PB15 and PA8 pin s are enabled (connected). Setting this bit disab les (disconnects) the internal pull-down resistors.

Bit 8 BOOSTEN: I/O analog switch voltage booster enable This bit selects the way of supplying I/O analog switches: 0: V DD 1: Dedicated voltage b ooster (supplied by V DD) When using the analog inputs , setting to 0 is recommended for high V DD, setting to 1 for low V DD (less than 2.4 V).

Bits 7:6 IR_MOD[1:0]: IR Modulation Envelope signal selection This bitfield selects the sign al for IR modulation envelope: 00: TIM16 01: USART1 10: USART4 on STM32G070xx as we ll as STM32G0B0xx, USART2 on STM 32G030xx as well as STM32G050xx 11: Reserved

Bit 5 IR_POL: IR output polarity selection 0: Output of IRTIM (IR_OUT) is not inverted 1: Output of IRTIM (IR_OUT) is inverted

Bit 4 PA12_RMP: PA12 pin remapping This bit is set and cleared by software. When set, it remaps th e PA12 pin to operate as PA10 GPIO port, instead as PA12 GPIO port. 0: No remap (PA12) 1: Remap (PA10)

Bit 3 PA11_RMP: PA11 pin remapping This bit is set and cleared by s oftware. When set, it remaps th e PA11 pin to operate as PA9 GPIO port, instead as PA11 GPIO port. 0: No remap (PA11) 1: Remap (PA9)

Bit 2 Reserved, must be kept at reset value.

Bits 1:0 MEM_MODE[1:0]: Memory mapping selection bits These bits are set and cleared by software. They control the me mory internal mapping at address 0x0000 0000. After reset these bits take on the value s elected by the actual boot mode configuration. Refer to Section 2.5: Boot configuration for more details. x0: Main Flash memory mapped at 0x0000 0000 01: System Flash memory mapped at 0x0000 0000 11: Embedded SRAM m apped at 0x0000 0000

### 7.1.2 [SYSCFG.SYSCFG_CFGR2] Offset: 0x18 | Reset: 0x00000000 (p. 192)

Address offset: 0x18

System reset value: 0x0000 0000

1. Only significant on devices i ntegrating switchable clamping d iodes, otherwise reserved. Refer to Section 1.4: Availability of peripherals.

Bits 31:24 Reserved, must be kept at reset value

Bit 23 PB2_CDEN: PB2 clamping diode enable bit (1) This bit is set and cleared by software. It enables (connects) a clamping diode to V DD on PB2 pin. 0: Disable 1: Enable

Bit 22 PB1_CDEN: PB1 clamping diode enable bit (1) This bit is set and cleared by software. It enables (connects) a clamping diode to V DD on PB1 pin. 0: Disable 1: Enable

Bit 21 PB0_CDEN: PB0 clamping diode enable bit (1) This bit is set and cleared by software. It enables (connects) a clamping diode to V DD on PB0 pin. 0: Disable 1: Enable

Bit 20 PA13_CDEN: PA13 clamping diode enable bit (1) This bit is set and cleared by s oftware. It enables (connects) a clamping diode to V DD on PA13 pin. 0: Disable 1: Enable

Bit 19 PA6_CDEN: PA6 clamping diode enable bit (1) This bit is set and cleared by software. It enables (connects) a clamping diode to V DD on PA6 pin. 0: Disable 1: Enable

Bit 18 PA5_CDEN: PA5 clamping diode enable bit (1) This bit is set and cleared by software. It enables (connects) a clamping diode to V DD on PA5 pin. 0: Disable 1: Enable

Bit 17 PA3_CDEN: PA3 clamping diode enable bit (1) This bit is set and cleared by software. It enables (connects) a clamping diode to V DD on PA3 pin. 0: Disable 1: Enable

Bit 16 PA1_CDEN: PA1 clamping diode enable bit (1) This bit is set and cleared by software. It enables (connects) a clamping diode to V DD on PA1 pin. 0: Disable 1: Enable

Bits 15:9 Reserved, must be kept at reset value

Bit 8 SRAM_PEF: SRAM parity error flag This bit is set by hardware when a n SRAM parity error is detected. It is cleared by software by writing 1. 0: No SRAM parity error detected 1: SRAM parity error detected

Bits 7:4 Reserved, must be kept at reset value.

Bit 3 ECC_LOCK: ECC error lock bit This bit is set by software and clea red by a system reset. It can be used to enable and lock the Flash ECC 2-bit e rror detection signal connection to TIM1/1 5/16/17 Break input. 0: ECC error disconnected from TIM1/15/16/17 Break input 1: ECC error connected to TIM1/15/16/17 Break input

Bit 2 Reserved, must be kept at reset value.

Bit 1 SRAM_PARITY_LOCK: SRAM parity lock bit This bit is set by software and clea red by a system reset. It can be used to enable and lock the SRAM parity error signal connection to TIM1/15/16/17 Break input. 0: SRAM parity error disconnected from TIM1/15/16/17 Break inpu t 1: SRAM parity error connecte d to TIM1/15/16/17 Break input

Bit 0 LOCKUP_LOCK: Cortex®-M0+ LOCKUP bit enable bit This bit is set by software and cleared by a system reset. It c® an be use to enable and lock the connection of Cortex® -M0+ LOCKUP (Hardfault) outpu t to TIM1/15/16/17 Break input. 0: Cortex®-M0+ LOCKUP output disconnected from TIM1/15/16/17 Break input 1: Cortex -M0+ LOCKUP output connected to TIM1/15/16/17 Break input

### 7.1.3 [SYSCFG.SYSCFG_ITLINE0] Offset: 0x80 | Reset: 0x00000000 (p. 194)

A dedicated set of registers is im plemented on the device to co llect all pending interrupt sources associated with each interrupt line into a single register. This allows users to check by single read which peripheral requires service in case more t han one source is associated to the interrupt line.

All bits in those registers are read only, set by hardware when there is corresponding interrupt request pend ing and cleared by re setting the interrupt source flags in the peripheral registers.

Address offset: 0x80

System reset value: 0x0000 0000

Res. Res. Res. Res. Res. Res. Res. Re s. Res. Res. Res. Res. Res. Res. Res. WWDG

r

Bits 31:1 Reserved, must be kept at reset value.

Bit 0 WWDG: Window watchdog interrupt pending flag

### 7.1.4 [SYSCFG.SYSCFG_ITLINE2] Offset: 0x88 | Reset: 0x00000000 (p. 194)

Address offset: 0x88

System reset value: 0x0000 0000

Bits 31:2 Reserved, must be kept at reset value.

Bit 1 RTC: RTC interrupt request pending (EXTI line 19)

Bit 0 TAMP: Tamper interrupt request pending (EXTI line 21)

### 7.1.5 [SYSCFG.SYSCFG_ITLINE3] Offset: 0x8C | Reset: 0x00000000 (p. 195)

Address offset: 0x8C

System reset value: 0x0000 0000

Bits 31:2 Reserved, must be kept at reset value.

Bit 1 FLASH_ECC: Flash interface ECC in terrupt request pending

Bit 0 FLASH_ITF: Flash interface in terrupt request pending

### 7.1.6 [SYSCFG.SYSCFG_ITLINE4] Offset: 0x90 | Reset: 0x00000000 (p. 195)

Address offset: 0x90

System reset value: 0x0000 0000

Res. Res. Res. Res. Res. Res. Res. Re s. Res. Res. Res. Res. Res. Res. Res. RCC

r

Bits 31:1 Reserved, must be kept at reset value.

Bit 0 RCC: Reset and clock control interrupt request pending

### 7.1.7 [SYSCFG.SYSCFG_ITLINE5] Offset: 0x94 | Reset: 0x00000000 (p. 196)

Address offset: 0x94

System reset value: 0x0000 0000

Bits 31:2 Reserved, must be kept at reset value.

Bit 1 EXTI1: EXTI line 1 interrupt request pending

Bit 0 EXTI0: EXTI line 0 interrupt request pending

### 7.1.8 [SYSCFG.SYSCFG_ITLINE6] Offset: 0x98 | Reset: 0x00000000 (p. 196)

Address offset: 0x98

System reset value: 0x0000 0000

Bits 31:2 Reserved, must be kept at reset value.

Bit 1 EXTI3: EXTI line 3 interrupt request pending

Bit 0 EXTI2: EXTI line 2 interrupt request pending

### 7.1.9 [SYSCFG.SYSCFG_ITLINE7] Offset: 0x9C | Reset: 0x00000000 (p. 196)

Address offset: 0x9C

System reset value: 0x0000 0000

Bits 31:12 Reserved, must be kept at reset value.

Bit 11 EXTI15: EXTI line 15 interrupt request pending

Bit 10 EXTI14: EXTI line 14 interrupt request pending

Bit 9 EXTI13: EXTI line 13 interrupt request pending

Bit 8 EXTI12: EXTI line 12 interrupt request pending

Bit 7 EXTI11: EXTI line 11 interrupt request pending

Bit 6 EXTI10: EXTI line 10 interrupt request pending

Bit 5 EXTI9: EXTI line 9 interrupt request pending

Bit 4 EXTI8: EXTI line 8 interrupt request pending

Bit 3 EXTI7: EXTI line 7 interrupt request pending

Bit 2 EXTI6: EXTI line 6 interrupt request pending

Bit 1 EXTI5: EXTI line 5 interrupt request pending

Bit 0 EXTI4: EXTI line 4 interrupt request pending

### 7.1.10 [SYSCFG.SYSCFG_ITLINE8] Offset: 0xA0 | Reset: 0x00000000 (p. 197)

Address offset: 0xA0

System reset value: 0x0000 0000

(1)

r

1. Only significant on devices integrating the corresponding peripheral, otherwise reserved. Refer to Section 1.4: Availability of peripherals.

Bits 31:3 Reserved, must be kept at reset value.

Bit 2 USB: USB interrupt request pending (1)

Bit 1 Reserved, must be kept at reset value.

Bit 0 Reserved, must be kept at reset value.

### 7.1.11 [SYSCFG.SYSCFG_ITLINE9] Offset: 0xA4 | Reset: 0x00000000 (p. 197)

Address offset: 0xA4

System reset value: 0x0000 0000

CH1

r

Bits 31:1 Reserved, must be kept at reset value.

Bit 0 DMA1_CH1: DMA1 channel 1 interrupt request pending

### 7.1.12 [SYSCFG.SYSCFG_ITLINE10] Offset: 0xA8 | Reset: 0x00000000 (p. 198)

Address offset: 0xA8

System reset value: 0x0000 0000

Bits 31:2 Reserved, must be kept at reset value.

Bit 1 DMA1_CH3: DMA1 channel 3 interrupt request pending

Bit 0 DMA1_CH2: DMA1 channel 2 interrupt request pending

### 7.1.13 [SYSCFG.SYSCFG_ITLINE11] Offset: 0xAC | Reset: 0x00000000 (p. 198)

Address offset: 0xAC

System reset value: 0x0000 0000

1. Only significant on devices i ntegrating the corresponding DMA instance and channel, otherwise reserved. Refer to Section 1.4: Availability of peripherals.

Bits 31:10 Reserved, must be kept at reset value.

Bit 9 DMA2_CH5: DMA2 channel 5 interrupt request pending (1)

Bit 8 DMA2_CH4: DMA2 channel 4 interrupt request pending (1)

Bit 7 DMA2_CH3: DMA2 channel 3 interrupt request pending (1)

Bit 6 DMA2_CH2: DMA2 channel 2 interrupt request pending (1)

Bit 5 DMA2_CH1: DMA2 channel 1 interrupt request pending (1)

Bit 4 DMA1_CH7: DMA1 channel 7 interrupt request pending (1)

Bit 3 DMA1_CH6 DMA1 channel 6 interrupt request pending (1)

Bit 2 DMA1_CH5: DMA1 channel 5 interrupt request pending

Bit 1 DMA1_CH4: DMA1 channel 4 interrupt request pending

Bit 0 DMAMUX: DMAMUX interrupt request pending

### 7.1.14 [SYSCFG.SYSCFG_ITLINE12] Offset: 0xB0 | Reset: 0x00000000 (p. 199)

Address offset: 0xB0

System reset value: 0x0000 0000

Res. Res. Res. Res. Res. Res. Res. Re s. Res. Res. Res. Res. Res. Res. Res. ADC

r

Bits 31:1 Reserved, must be kept at reset value.

Bit 0 ADC: ADC interrupt request pending

### 7.1.15 [SYSCFG.SYSCFG_ITLINE13] Offset: 0xB4 | Reset: 0x00000000 (p. 199)

Address offset: 0xB4

System reset value: 0x0000 0000

Bits 31:4 Reserved, must be kept at reset value.

Bit 3 TIM1_BRK: Timer 1 break interrupt request pending

Bit 2 TIM1_UPD: Timer 1 update interrupt request pending

Bit 1 TIM1_TRG: Timer 1 trigger interrupt request pending

Bit 0 TIM1_CCU: Timer 1 commutation interrupt request pending

### 7.1.16 [SYSCFG.SYSCFG_ITLINE14] Offset: 0xB8 | Reset: 0x00000000 (p. 199)

Address offset: 0xB8

System reset value: 0x0000 0000

CC

r

Bits 31:1 Reserved, must be kept at reset value.

Bit 0 TIM1_CC: Timer 1 capture compare interrupt request pending

### 7.1.17 [SYSCFG.SYSCFG_ITLINE16] Offset: 0xC0 | Reset: 0x00000000 (p. 200)

Address offset: 0xC0

System reset value: 0x0000 0000

1. Only significant on devices i ntegrating TIM4, otherwise reserved. Refer to Section 1.4: Availability of peripherals.

Bits 31:2 Reserved, must be kept at reset value.

Bit 1 TIM4: Timer 4 interrupt request pending

Bit 0 TIM3: Timer 3 interrupt request pending

### 7.1.18 [SYSCFG.SYSCFG_ITLINE17] Offset: 0xC4 | Reset: 0x00000000 (p. 200)

Address offset: 0xC4

System reset value: 0x0000 0000

Res. Res. Res. Res. Res. Res. Res. Re s. Res. Res. Res. Res. Res. Res. Res. TIM6 (1)

r

1. Only significant on devices i ntegrating TIM4, otherwise reserved. Refer to Section 1.4: Availability of peripherals.

Bits 31:1 Reserved, must be kept at reset value.

Bit 0 TIM6: Timer 6 interrupt request pending (1)

### 7.1.19 [SYSCFG.SYSCFG_ITLINE18] Offset: 0xC8 | Reset: N/A (p. 200)

Address offset: 0xC8

System reset value: 0x0000 0000

Res. Res. Res. Res. Res. Res. Res. Re s. Res. Res. Res. Res. Res. Res. Res. TIM7 (1)

r

1. Only significant on devices i ntegrating TIM4, otherwise reserved. Refer to Section 1.4: Availability of peripherals.

Bits 31:1 Reserved, must be kept at reset value.

Bit 0 TIM7: Timer 7 interrupt request pending (1)

### 7.1.20 [SYSCFG.SYSCFG_ITLINE19] Offset: 0xCC | Reset: 0x00000000 (p. 201)

Address offset: 0xCC

System reset value: 0x0000 0000

Res. Res. Res. Res. Res. Res. Res. Re s. Res. Res. Res. Res. Res. Res. Res. TIM14

r

Bits 31:1 Reserved, must be kept at reset value.

Bit 0 TIM14: Timer 14 interrupt request pending

### 7.1.21 [SYSCFG.SYSCFG_ITLINE20] Offset: 0xD0 | Reset: 0x00000000 (p. 201)

Address offset: 0xD0

System reset value: 0x0000 0000

Res. Res. Res. Res. Res. Res. Res. Re s. Res. Res. Res. Res. Res. Res. Res. TIM15 (1)

r

1. Only significant on devices integrating TIM15, otherwise rese rved. Refer to Section 1.4: Availability of peripherals.

Bits 31:1 Reserved, must be kept at reset value.

Bit 0 TIM15: Timer 15 interrupt request pending (1)

### 7.1.22 [SYSCFG.SYSCFG_ITLINE21] Offset: 0xD4 | Reset: N/A (p. 201)

Address offset: 0xD4

System reset value: 0x0000 0000

Res. Res. Res. Res. Res. Res. Res. Re s. Res. Res. Res. Res. Res. Res. Res. TIM16

r

Bits 31:3 Reserved, must be kept at reset value.

Bit 0 TIM16: Timer 16 interrupt request pending

### 7.1.23 [SYSCFG.SYSCFG_ITLINE22] Offset: 0xD8 | Reset: 0x00000000 (p. 202)

Address offset: 0xD8

System reset value: 0x0000 0000

Res. Res. Res. Res. Res. Res. Res. Re s. Res. Res. Res. Res. Res. Res. Res. TIM17

r

Bits 31:3 Reserved, must be kept at reset value.

Bit 0 TIM17: Timer 17 interrupt request pending

### 7.1.24 [SYSCFG.SYSCFG_ITLINE23] Offset: 0xDC | Reset: 0x00000000 (p. 202)

Address offset: 0xDC

System reset value: 0x0000 0000

Res. Res. Res. Res. Res. Res. Res. Re s. Res. Res. Res. Res. Res. Res. Res. I2C1

r

Bits 31:1 Reserved, must be kept at reset value.

Bit 0 I2C1: I2C1 interrupt request pendi ng, combined with EXTI line 23

### 7.1.25 [SYSCFG.SYSCFG_ITLINE24] Offset: 0xE0 | Reset: 0x00000000 (p. 202)

Address offset: 0xE0

System reset value: 0x0000 0000

1. Only significant on devices i ntegrating I2C3, otherwise reserved. Refer to Section 1.4: Availability of peripherals.

Bits 31:2 Reserved, must be kept at reset value.

Bit 1 I2C3: I2C3 interrupt request pending (EXTI line 22) (1)

Bit 0 I2C2: I2C2 interrupt request pending

### 7.1.26 [SYSCFG.SYSCFG_ITLINE25] Offset: 0xE4 | Reset: 0x00000000 (p. 203)

Address offset: 0xE4

System reset value: 0x0000 0000

Res. Res. Res. Res. Res. Res. Res. Re s. Res. Res. Res. Res. Res. Res. Res. SPI1

r

Bits 31:1 Reserved, must be kept at reset value.

Bit 0 SPI1: SPI1 interrupt request pending

### 7.1.27 [SYSCFG.SYSCFG_ITLINE26] Offset: 0xE8 | Reset: 0x00000000 (p. 203)

Address offset: 0xE8

System reset value: 0x0000 0000

1. Only significant on devices i ntegrating SPI3, otherwise reserved. Refer to Section 1.4: Availability of peripherals.

Bits 31:2 Reserved, must be kept at reset value.

Bit 1 SPI3: SPI3 interrupt request pending (1)

Bit 0 SPI2: SPI2 interrupt request pending

### 7.1.28 [SYSCFG.SYSCFG_ITLINE27] Offset: 0xEC | Reset: 0x00000000 (p. 204)

Address offset: 0xEC

System reset value: 0x0000 0000

Res. Res. Res. Res. Res. Res. Res. Re s. Res. Res. Res. Res. Res. Res. Res. USART1

r

Bits 31:1 Reserved, must be kept at reset value.

Bit 0 USART1: USART1 interrupt request pendi ng, combined with EXTI line 25

### 7.1.29 [SYSCFG.SYSCFG_ITLINE28] Offset: 0xF0 | Reset: 0x00000000 (p. 204)

Address offset: 0xF0

System reset value: 0x0000 0000

Res. Res. Res. Res. Res. Res. Res. Re s. Res. Res. Res. Res. Res. Res. Res. USART2

r

Bits 31:1 Reserved, must be kept at reset value.

Bit 0 USART2: USART2 interrupt reques t pending (EXTI line 26)

### 7.1.30 [SYSCFG.SYSCFG_ITLINE29] Offset: 0xF4 | Reset: 0x00000000 (p. 204)

Address offset: 0xF4

System reset value: 0x0000 0000

(1) (1) (1) (1)

rr rr

1. Only significant on devices i ntegrating the corresponding peripheral, otherwise reserved. Refer to Section 1.4: Availability of peripherals.

Bits 31:5 Reserved, must be kept at reset value.

Bit 4 USART6: USART6 interrupt request pending (1)

Bit 3 USART5: USART5 interrupt request pending (1)

Bit 2 Reserved, must be kept at reset value.

Bit 1 USART4: USART4 interrupt request pending (1)

Bit 0 USART3: USART3 interrupt reques t pending (EXTI line 28) (1)

#### 7.1.31 SYSCFG register map (p. 205)

The following table gi ves the SYSCFG registe r map and the reset values.

**Table 31. SYSCFG register map and reset values (p. 205)**

| Col 1 | Offset Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0x00 | SYSCFG_CFGR1 | Res. | Res. | Res. | Res. | Res. | Res. | Res. | I2C3_FMP | I2C_PA10_FMP | I2C_PA9_FMP | I2C2_FMP | I2C1_FMP | I2C_PB9_FMP | I2C_PB8_FMP | I2C_PB7_FMP | I2C_PB6_FMP | Res. | Res. | Res. | Res. | Res. | UCPD2_STROBE | UCPD2_STROBE | BOOSTEN | IR_MOD | IR_MOD | IR_POL | PA12_RMP | PA11_RMP | Res. | MEM_MODE[1:0] | MEM_MODE[1:0] |

Reset value 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 0 X X

| 0x04 to 0x17 | Reserved | Col 3 | Col 4 | Col 5 | Col 6 | Col 7 | Col 8 | Col 9 | Col 10 | Col 11 | Col 12 | Col 13 | Col 14 | Col 15 | Col 16 | Col 17 | Reserved | Reserved | Col 20 | Col 21 | Col 22 | Col 23 | Col 24 | Col 25 | Col 26 | Col 27 | Col 28 | Col 29 | Col 30 | Col 31 | Col 32 | Col 33 | Col 34 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0x18 | SYSCFG_CFGR2 | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | SRAM_PEF | Res. | Res. | Res. | Res. | ECC_LOCK | Res. | SRAM_PARITY_LOCK | LOCUP_LOCK |

Reset value 0 0 00

| 0x1D to 0x7F | Reserved | Col 3 | Col 4 | Col 5 | Col 6 | Col 7 | Col 8 | Col 9 | Col 10 | Col 11 | Col 12 | Col 13 | Col 14 | Col 15 | Col 16 | Col 17 | Reserved | Reserved | Col 20 | Col 21 | Col 22 | Col 23 | Col 24 | Col 25 | Col 26 | Col 27 | Col 28 | Col 29 | Col 30 | Col 31 | Col 32 | Col 33 | Col 34 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0x80 | SYSCFG_ITLINE0 | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | WWDG |

Reset value 0

| 0x84 | Reserved | Col 3 | Col 4 | Col 5 | Col 6 | Col 7 | Col 8 | Col 9 | Col 10 | Col 11 | Col 12 | Col 13 | Col 14 | Col 15 | Col 16 | Col 17 | Reserved | Reserved | Col 20 | Col 21 | Col 22 | Col 23 | Col 24 | Col 25 | Col 26 | Col 27 | Col 28 | Col 29 | Col 30 | Col 31 | Col 32 | Col 33 | Col 34 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0x88 | SYSCFG_ITLINE2 | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | RTC | TAMP |

Reset value 00 0x8C SYSCFG_ITLINE3 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. FLASH_ECC FLASH_ITF Reset value 00 0x90 SYSCFG_ITLINE4 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. RCC Reset value 0 0x94 SYSCFG_ITLINE5 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. EXTI1 EXTI0 Reset value 00 0x98 SYSCFG_ITLINE6 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. EXTI3 EXTI2 Reset value 00

**Table 31. SYSCFG reg ister map and reset values (c ontinued) (p. 206)**

| Col 1 | Offset Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0x9C | SYSCFG_ITLINE7 | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | EXTI15 | EXTI14 | EXTI13 | EXTI12 | EXTI11 | EXTI10 | EXTI9 | EXTI8 | EXTI7 | EXTI6 | EXTI5 | EXTI4 |

Reset value 000000000000 0xA0 SYSCFG_ITLINE8 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. USB Res. Res. Reset value 0 0xA4 SYSCFG_ITLINE9 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. DMA1_CH1 Reset value 0 0xA8 SYSCFG_ITLINE10 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. DMA1_CH3 DMA1_CH2 Reset value 00 0xAC SYSCFG_ITLINE11 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. DMA2_CH5 DMA2_CH4 DMA2_CH3 DMA2_CH2 DMA2_CH1 DMA1_CH7 DMA1_CH6 DMA1_CH5 DMA1_CH4 DMAMUX Reset value 0000000000 0xB0 SYSCFG_ITLINE12 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. ADC Reset value 0 0xB4 SYSCFG_ITLINE13 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. TIM1_BRK TIM1_UPD TIM1_TRG TIM1_CCU Reset value 0000 0xB8 SYSCFG_ITLINE14 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. TIM1_CC Reset value 0

| 0xBC | Reserved | Col 3 | Col 4 | Col 5 | Col 6 | Col 7 | Col 8 | Col 9 | Col 10 | Col 11 | Col 12 | Col 13 | Col 14 | Col 15 | Col 16 | Col 17 | Reserved | Reserved | Col 20 | Col 21 | Col 22 | Col 23 | Col 24 | Col 25 | Col 26 | Col 27 | Col 28 | Col 29 | Col 30 | Col 31 | Col 32 | Col 33 | Col 34 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0xC0 | SYSCFG_ITLINE16 | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | TIM4 | TIM3 |

Reset value 00 0xC4 SYSCFG_ITLINE17 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. TIM6 Reset value 0 0xC8 SYSCFG_ITLINE18 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. TIM7 Reset value 0 0xCC SYSCFG_ITLINE19 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. TIM14 Reset value 0 0xD0 SYSCFG_ITLINE20 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. TIM15 Reset value 0 0xD4 SYSCFG_ITLINE21 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. TIM16 Reset value 000 0xD8 SYSCFG_ITLINE22 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. TIM17 Reset value 000

**Table 31. SYSCFG reg ister map and reset values (c ontinued) (p. 207)**

| Col 1 | Offset Register | 31 | 30 | 29 | 28 | 27 | 26 | 25 | 24 | 23 | 22 | 21 | 20 | 19 | 18 | 17 | 16 | 15 | 14 | 13 | 12 | 11 | 10 | 9 | 8 | 7 | 6 | 5 | 4 | 3 | 2 | 1 | 0 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| 0xDC | SYSCFG_ITLINE23 | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | Res. | I2C1 |

Reset value 0 0xE0 SYSCFG_ITLINE24 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. I2C3 I2C2 Reset value 00 0xE4 SYSCFG_ITLINE25 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. SPI1 Reset value 0 0xE8 SYSCFG_ITLINE26 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. SPI3 SPI2 Reset value 00 0xEC SYSCFG_ITLINE27 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. USART1 Reset value 0 0xF0 SYSCFG_ITLINE28 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. USART2 Reset value 0 0xF4 SYSCFG_ITLINE29 Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. Res. USART6 USART5 Res. USART4 USART3 Reset value 00 00 0xF8 - Reserved Reserved 0xFF

Refer to Section 2.2 on page 44 for the register boundary addresses.
