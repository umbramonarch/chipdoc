---
document: "STM32G0x0 advanced Arm®-based 32-bit MCUs - Reference manual"
section_id: "11"
section_title: "11 Nested vectored interrupt controller (NVIC)"
pages: "250-252"
---

# 11 Nested vectored interrupt controller (NVIC) (pp. 250-252)

## 11 Nested vectored interrupt controller (NVIC) (p. 250)

### 11.1 Main features (p. 250)

®

- 32 maskable interrupt channels ( not including the sixteen Cortex -M0+ interrupt lines)

- 4 programmable priority levels (2 bits of interrupt priority are used)

- Low-latency exception and interrupt handling

- Power management control

- Implementation of system control registers

The NVIC and the processor core interface are closely coupled, which enables low-latency interrupt processing and efficien t processing of late arriving interrupts.

All interrupts including the core exceptions are managed by the NVIC. For more information on exceptions and NVIC programmi ng, refer to the programming ma nual PM0223.

### 11.2 SysTick calibration value register (p. 250)

The SysTick calibration value is set to 6500, which gives a reference time base of 1 ms with the SysTick clock se t to 6.5 MHz (max f HCLK/8).

### 11.3 Interrupt and exception vectors (p. 250)

Table 45 is the vector table. Information pertaining to a peripheral on ly applies to devices containing that peripheral.

**Table 45. Vector table (1) (pp. 250-252)**

| Position Priority | Col 2 | Type of priority | Col 4 | Acronym Description Address | Col 6 |
| --- | --- | --- | --- | --- | --- |
| - | - | - | - | Reserved | 0x0000_0000 |
| - | -3 | fixed | Reset | Reset | 0x0000_0004 |
| - | -2 | fixed | NMI_Handler | Non maskable interrupt. The SRAM parity err., Flash ECC double err., HSE CSS and LSE CSS are linked to the NMI vector. | 0x0000_0008 |
| - | -1 | fixed | HardFault_Handler | All class of fault | 0x0000_000C |
| - | - | - | - | Reserved | 0x0000_0010 0x0000_0014 0x0000_0018 0x0000_001C 0x0000_0020 0x0000_0024 0x0000_0028 |
| - | 3 | settable | SVC_Handler | System service call via SWI instruction | 0x0000_002C |
| - |  | - | - | - | Reserved |
| - |  | 5 | settable | PendSV_Handler | Pendable request for system service |
| - |  | 6 | settable | SysTick_Handler | System tick timer |
|  |  |  |  |  | 0 7 settable WWDG Window watchdog interrupt 0x0000_0040 |
|  |  |  |  |  | 1 - - - Reserved 0x0000_0044 |
|  |  |  | 2 9 settable RTC / TAMP |  | RTC and TAMP interrupts (combined EXTI lines 19 & 21) |
|  |  |  |  |  | 3 10 settable FLASH Flash global interrupt 0x0000_004C |
|  |  |  |  |  | 4 11 settable RCC RCC global interrupt 0x0000_0050 |
|  |  |  |  |  | 5 12 settable EXTI0_1 EXTI line 0 & 1 interrupt 0x0000_0054 |
|  |  |  |  |  | 6 13 settable EXTI2_3 EXTI line 2 & 3 interrupt 0x0000_0058 |
|  |  |  |  |  | 7 14 settable EXTI4_15 EXTI line 4 to 15 interrupt 0x0000_005C |
|  |  |  |  |  | 8 - - - Reserved 0x0000_0060 |
|  |  |  |  |  | 9 16 settable DMA1_Channel1 DMA1 channel 1 interrupt 0x0000_0064 |
|  |  |  |  | 10 17 settable DMA1_Channel2_3 DMA1 c | hannel 2 & 3 interrupts 0x0000_ |
|  |  | 11 18 settable |  | DMA1_Channel4_5_6 _7 / DMAMUX / DMA2_Channel1_2_3 _4_5 | DMA1 channel 4, 5, 6, 7, DMAMUX, DMA2 channel 1, 2, 3, 4, 5 interrupts |
|  |  | 12 19 settable ADC | 12 19 settable ADC |  | ADC interrupt (ADC combined with EXTI 17 & 18) |
|  |  | 13 20 settable |  | TIM1_BRK_UP_TRG_COM | TIM1 break, update, trigger and commutation interrupts |
|  |  |  |  | 14 21 settable TIM1_CC TIM1 Captur | e Compare interrupt 0x0000_0078 |
|  |  |  |  |  | 15 - - - Reserved 0x0000_007C |
|  |  |  |  |  | 16 23 settable TIM3+TIM4 TIM3 global interrupt 0x0000_0080 |
|  |  |  |  |  | 17 24 settable TIM6 TIM6 global interrupt 0x0000_0084 |
|  |  |  |  |  | 18 25 settable TIM7 TIM7 global interrupt 0x0000_0088 |
|  |  |  |  |  | 19 26 settable TIM14 TIM14 global interrupt 0x0000_008C |
|  |  |  |  |  | 20 27 settable TIM15 TIM15 global interrupt 0x0000_0090 |
|  |  |  |  |  | 21 28 settable TIM16 TIM16 global interrupt 0x0000_0094 |
|  |  |  |  |  | 22 29 settable TIM17 TIM17 global interrupt 0x0000_0098 |
|  |  | 23 30 settable I2C1 |  |  | I2C1 global interrupt (combined with EXTI 23) |
|  |  |  |  | 24 31 settable I2C2 / I2C3 I2C2 and I2C3 global interrupt 0x0000_00A | 24 31 settable I2C2 / I2C3 I2C2 and I2C3 global interrupt 0x0000_00A |
|  |  |  |  |  | 25 32 settable SPI1 SPI1 global interrupt 0x0000_00A4 |
|  |  |  |  | 26 33 settable SPI2 / SPI3 SPI2 global interrupt 0x0000_00A8 |  |
|  |  | 27 34 settable USART1 |  | USART1 global interrupt (combined with EXTI 25) | 0x0000_00AC |
|  |  | 28 35 settable USART2 |  | USART2 global interrupt (combined with EXTI 26) | 0x0000_00B0 |
|  | 29 36 settable |  | USART3 / USART4 / USART5 / USART6 | USART3/4/5/6 global interrupt (combined with EXTI 28) | 0x0000_00B4 |
|  |  |  |  | 30 - - - Reserved 0x0000_00B8 |  |
|  |  |  |  | 31 - - - Reserved 0x0000_00BC |  |
|  | 1. The grayed cells correspond to the Cortex |  | ®-M0+ interrupts. |  |  |
