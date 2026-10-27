---
document: "STM32G0x0 advanced Arm®-based 32-bit MCUs - Reference manual"
section_id: "00"
section_title: "Frontmatter and Overview"
pages: "1-38"
---

# Frontmatter and Overview (pp. 1-38)

## RM0454 (p. 1)

## Reference manual (p. 1)

STM32G0x0 advanced Arm®-based 32-bit MCUs

## Introduction (p. 1)

This reference manual complemen ts the datasheets of the STM32G0x0 microcontrollers, providing information required for application and in particula r for software de velopment. It pertains to the superset of feat ure sets available on STM32G0x0 microcontrollers.

For feature set, ordering information, and mechanical and elect rical characteristics of a particular STM32G0x0 device, refer to its corresponding datashe et. For information on the Arm® Cortex®-M0+ core, refer to the Cortex®-M0+ technical reference manual.

## Related documents (p. 1)

- “Cortex®-M0+ Technical Reference Manual”, available from: http://infocenter.arm.com

- PM0223 programming manual for Cortex®-M0+ core (a)

- STM32G0x0 datasheets (a)

- AN2606 application note on booting STM32 MCUs (a)

a. Available on STMicroelectronics website www.st.com

November 2020 RM0454 Rev 5 1/989

## Contents (p. 2)

1.1 General information . . . . . . . . . . . . . . . . . . . .

1.2 List of abbreviations for registers . . . . . . . . . . . .

1.3 Glossary . . . . . . . . . . . . . . . . . . . . . . . . . .

1.4 Availability of peripherals . . . . . . . . . . . . . . . .

2.1 System architecture . . . . . . . . . . . . . . . . . . . .

2.2 Memory organization . . . . . . . . . . . . . . . . . . . . 2.2.1 Introduction . . . . . . . . . . . . . . . . . . . . . .

2.2.2 Memory map and register boundary addresses . . . . . . .

2.3 Embedded SRAM . . . . . . . . . . . . . . . . . . . . . . .

2.4 Flash memory overview . . . . . . . . . . . . . . . . . . .

2.5 Boot configuration . . . . . . . . . . . . . . . . . . . .

3.1 FLASH Introduction . . . . . . . . . . . . . . . . . . . .

3.2 FLASH main features . . . . . . . . . . . . . . . . . . . .

3.3 FLASH functional description . . . . . . . . . . . . . . . . 3.3.1 FLASH memory organization . . . . . . . . . . . . . . . . 3.3.2 FLASH empty check . . . . . . . . . . . . . . . . . . . . 3.3.3 FLASH error code correction (ECC) . . . . . . . . . . . . 3.3.4 FLASH read access latency . . . . . . . . . . . . . . . . 3.3.5 FLASH memory accelerati on . . . . . . . . . . . . . . . . 3.3.6 FLASH program and erase operations . . . . . . . . . . . 3.3.7 FLASH Main memory erase sequences 3.3.8 FLASH Main memory programming sequences . . . . . . . . . 3.3.9 Read-while-write (RWW) function . . . . . . . . . . . . .

3.4 FLASH option bytes . . . . . . . . . . . . . . . . . . . . . 3.4.1 FLASH option byte description . . . . . . . . . . . . . . 3.4.2 FLASH option byte programming . . . . . . . . . . . . . .

3.5 FLASH memory protection . . . . . . . . . . . . . . . . . . 3.5.1 FLASH read protection (RDP) . . . . . . . . . . . . . . .

3.5.2 FLASH write protection (WRP) . . . . . . . . . . . . . .

3.6 FLASH interrupts . . . . . . . . . . . . . . . . . . . . .

3.7 FLASH registers . . . . . . . . . . . . . . . . . . . . . . 3.7.1 FLASH access control reg ister (FLASH_ACR) . . . . . . . . 3.7.2 FLASH key register (FLASH_KEYR) . . . . . . . . . . . . . 3.7.3 FLASH option key register (FLASH_OPTKEYR) . . . .. . . . 3.7.4 FLASH status register (FLASH_SR) . . . . . . . . . . . . 3.7.5 FLASH control register (FLASH_CR) . . . . . . . . . . . . 3.7.6 FLASH ECC register (FLASH_ECCR) . . . . . . . . . . . . . 3.7.7 FLASH ECC register 2 (F LASH_ECCR2) . . . . . . . . . . . . 3.7.8 FLASH option register (FLASH_OPTR) . . . . . . . . . . . 3.7.9 FLASH WRP area A address register (FLASH_WRP1AR) . . . . 3.7.10 FLASH WRP area B address register (FLASH_WRP1BR) . . . .

3.7.13 FLASH register map . . . . . . . . . . . . . . . . . . .

4.1 Power supplies . . . . . . . . . . . . . . . . . . . . . . . 4.1.1 ADC reference voltage . . . . . . . . . . . . . . . . . . 4.1.2 Battery backup of RTC domain . . . . . . . . . . . . . . 4.1.3 Voltage regulator . . . . . . . . . . . . . . . . . . . . 4.1.4 Dynamic voltage scaling management . . . . . . . . . . .

4.2 Power supply supervisor . . . . . . . . . . . . . . . . . . 4.2.1 Power-on reset (POR) / power-down reset (PDR) . . . . . .

4.3 Low-power modes . . . . . . . . . . . . . . . . . . . . . . 4.3.1 Run mode . . . . . . . . . . . . . . . . . . . . . . . .

4.3.2 Low-power run mode (LP run) . . . . . . . . . . . . . . . 4.3.3 Low-power modes . . . . . . . . . . . . . . . . . . . . . 4.3.4 Sleep mode . . . . . . . . . . . . . . . . . . . . . . . 4.3.5 Low-power sleep mode (L P sleep) . . . . . . . . . . . . . 4.3.6 Stop 0 mode . . . . . . . . . . . . . . . . . . . . . . . 4.3.7 Stop 1 mode . . . . . . . . . . . . . . . . . . . . . . . 4.3.8 Standby mode . . . . . . . . . . . . . . . . . . . . . . 4.3.9 Auto-wakeup from low-power mode . . . . . . . . . . . . .

4.4 PWR registers . . . . . . . . . . . . . . . . . . . . . . .

4.4.1 Power control register 1 (PWR_CR1) . . . . . . . . . . . 4.4.2 Power control register 2 (PWR_CR2) . . . . . . . . . . . 4.4.3 Power control register 3 (PWR_CR3) . . . . . . . . . . . 4.4.4 Power control register 4 (PWR_CR4) . . . . . . . . . . . 4.4.5 Power status register 1 (PWR_SR1) . . . . . . . . . . . . 4.4.6 Power status register 2 (PWR_SR2) . . . . . . . . . . . . 4.4.7 Power status clear register (PWR_SCR) . . . . . . . . . . 4.4.8 Power Port A pull-up control register (PWR_PUCRA) . . . . 4.4.9 Power Port A pull-down control register (PWR_PDCRA) . . . 4.4.10 Power Port B pull-up cont rol register (PWR_PUCRB) . . . . 4.4.11 Power Port B pull-down co ntrol register (PWR_PDCRB) . . 4.4.12 Power Port C pull-up contro l register (PWR_PUCRC) . . . 4.4.13 Power Port C pull-down control register (PWR_PDCRC) . . 4.4.14 Power Port D pull-up contro l register (PWR_PUCRD) . . . 4.4.15 Power Port D pull-down control register (PWR_PDCRD) . . 4.4.16 Power Port E pull-up cont rol register (PWR_PUCRE) . . . . 4.4.17 Power Port E pull-down co ntrol register (PWR_PDCRE) . . 4.4.18 Power Port F pull-up control register (PWR_PUCRF) . . . . 4.4.19 Power Port F pull-down con trol register (PWR_PDCRF) . . 4.4.20 PWR register map . . . . . . . . . . . . . . . . . . . .

## 5 Reset and clock control (RCC) (p. 4)

5.1 Reset . . . . . . . . . . . . . . . . . . . . . . . . . . .

5.1.1 Power reset . . . . . . . . . . . . . . . . . . . . . . .

5.1.2 System reset . . . . . . . . . . . . . . . . . . . . . . 5.1.3 RTC domain reset . . . . . . . . . . . . . . . . . . . .

5.2 Clocks . . . . . . . . . . . . . . . . . . . . . . . . . . .

5.2.1 HSE clock . . . . . . . . . . . . . . . . . . . . . . . .

5.2.2 HSI16 clock . . . . . . . . . . . . . . . . . . . . . . .

5.2.3 PLL . . . . . . . . . . . . . . . . . . . . . . . . . . .

5.2.4 LSE clock . . . . . . . . . . . . . . . . . . . . . . . .

5.2.5 LSI clock . . . . . . . . . . . . . . . . . . . . . . . .

5.2.6 System clock (SYSCLK) se lection . . . .. . . . .. . . . 5.2.7 Clock source frequency ve rsus voltage scaling . . . . . . 5.2.8 Clock security system (CSS) . . . . . . . . . . . . . . . 5.2.9 Clock security system for LSE clock (LSECSS) . . . . . . 5.2.10 ADC clock . . . . . . . . . . . . . . . . . . . . . . .

5.2.11 RTC clock . . . . . . . . . . . . . . . . . . . . . . .

5.2.12 Timer clock . . . . . . . . . . . . . . . . . . . . . . .

5.2.13 Watchdog clock . . . . . . . . . . . . . . . . . . . . . 5.2.14 Clock-out capability . . . . . . . . . . . . . . . . . . 5.2.15 Internal/external clock m easurement with TIM14/TIM16/TIM1 7 . . . . . 131

5.2.16 Peripheral clock enable registers . . . . . . . . . . . .

5.3 Low-power modes . . . . . . . . . . . . . . . . . . . . . .

5.4 RCC registers . . . . . . . . . . . . . . . . . . . . . . . 5.4.1 Clock control register (RCC_CR) . . . . . . . . . . . . . 5.4.2 Internal clock source calib ration register (RCC_ICSCR) . . 5.4.3 Clock configuration regi ster (RCC_CFGR) . . . . . . . . . 5.4.4 PLL configuration register (RCC_PLLCFGR) . . . . . . . . 5.4.5 Clock interrupt enable re gister (RCC_CIER) . . . . . . . . 5.4.6 Clock interrupt flag register (RCC_CIFR) . . . . . . . . 5.4.7 Clock interrupt clear reg ister (RCC_CICR) . . . . . . . . 5.4.8 I/O port reset register (RCC_IOPRSTR) . . . . . . . . . . 5.4.9 AHB peripheral reset register (RCC_AHBRSTR) . . . . . . . 5.4.10 APB peripheral reset register 1 (RCC_APBRSTR1) . . . . . 5.4.11 APB peripheral reset register 2 (R CC_APBRSTR2) . . . . . 5.4.12 I/O port clock enable reg ister (RCC_IOPENR) . . . . . . 5.4.13 AHB peripheral clock enab le register (RCC_AHBENR) . . . 5.4.14 APB peripheral cl ock enable register 1 (RCC_APBENR1) . . 5.4.15 APB peripheral clock enabl e register 2( RCC_APBENR2) . . . 5.4.16 I/O port in Sleep mode cl ock enable register (RCC_IOPSMEN R) . . . 157 5.4.17 AHB peripheral clock enab le in Sleep/Stop mode register (RCC_AHBSMENR) . . . . . . . . . . . . . . . . . . . . . . . . 5.4.18 APB peripheral cl ock enable in Sleep/Stop mode register 1 (RCC_APBSMENR1) . .. . . . . 5.4.19 APB peripheral cl ock enable in Sleep/Stop mode register 2 (RCC_APBSMENR2) . .. . . . . 5.4.20 Peripherals independent clock configuration register (RCC_CCIPR) . 163 5.4.21 Peripherals independent clock configuration register 2 (RCC_CCIPR2) . . . . . . . . . . . . . . . . . . . . . . . . . 5.4.22 RTC domain control regi ster (RCC_BDCR) . . . . . . . . . 5.4.23 Control/status register (RCC_CSR) . . . . . . . . . . . . 5.4.24 RCC register map . . . . . . . . . . . . . . . . . . . .

6 General-purpose I/Os (GPIO) .

6.1 Introduction . . . . . . . . . . . . . . . . . . . . . . .

6.2 GPIO main features . . . . . . . . . . . . . . . . . . . .

6.3 GPIO functional description . . . . . . . . . . . . . . . . 6.3.1 General-purpo se I/O (GPIO) . . . . . . . . . . . . . . . 6.3.2 I/O pin alternate functio n multiplexer and mapping . . . . 6.3.3 I/O port control registers . . . . . . . . . . . . . . . 6.3.4 I/O port data registers . . . . . . . . . . . . . . . . . 6.3.5 I/O data bitwise handling . . . . . . . . . . . . . . . . 6.3.6 GPIO locking mechanism . . . . . . . . . . . . . . . . . 6.3.7 I/O alternate function i nput/output . . . . . . . . . . . 6.3.8 External inte rrupt/wakeup lines . . . . . . . . . . . . . 6.3.9 Input configuration . . . . . . . . . . . . . . . . . . . 6.3.10 Output configuration . . . . . . . . . . . . . . . . . . 6.3.11 Alternate function configuration . . . . . . . . . . . . 6.3.12 Analog configuration . . . . . . . . . . . . . . . . . . 6.3.13 Using the HSE or LSE oscillator pins as GPIOs . . . . . 6.3.14 Using the GPIO pins in the RTC domain . . . . . . . . . .

6.4 GPIO registers . . . . . . . . . . . . . . . . . . . . . . 6.4.1 GPIO port mode re gister (GPIOx_MODER) (x =A to F) . . . . . . . . . . . . . . . . . . . . . . . . . 6.4.2 GPIO port output ty pe register (GPIOx_OTYPER) (x = A to F) . . . . . . . . . . . . . . . . . . . . . . . . . 6.4.3 GPIO port output spee d register (GPIOx_OSPEEDR) (x = A to F) . . . . . . . . . . . . . . . . . . . . . . . . . 6.4.4 GPIO port pull-up/pull-down register (GPIOx_PUPDR) (x = A to F) . . . . . . . . . . . . . . . . . . . . . . . . . 6.4.5 GPIO port input data register (GPIOx_IDR) (x = A to F) . . . . . . . . . . . . . . . . . . . . . . . . . 6.4.6 GPIO port output da ta register (GPIOx_ODR) (x = A to F) . . . . . . . . . . . . . . . . . . . . . . . . . 6.4.7 GPIO port bit set/res et register (GPIOx_BSRR) (x = A to F) . . . . . . . . . . . . . . . . . . . . . . . . . 6.4.8 GPIO port configuration lock register (GPIOx_LCKR) (x = A to F) . . . . . . . . . . . . . . . . . . . . . . . . . 6.4.9 GPIO alternate function low register (GPIOx_AFRL) (x = A to F) . . . . . . . . . . . . . . . . . . . . . . . . . 6.4.10 GPIO alternate functio n high register (GPIOx_AFRH) (x = A to F) . . . . . . . . . . . . . . . . . . . . . . . . . 6.4.11 GPIO port bit reset registe r (GPIOx_BRR) (x = A to F) . 6.4.12 GPIO register map . . . . . . . . . . . . . . . . . . . .

7.1 SYSCFG registers . . . . . . . . . . . . . . . . . . . . . 7.1.1 SYSCFG configuration regis ter 1 (SYSCFG_CFGR1) . . . . . 7.1.2 SYSCFG configuration regis ter 2 (SYSCFG_CFGR2) . . . . . 7.1.3 SYSCFG interrupt line 0 status register (SYSCFG_ITLINE0) . 7.1.4 SYSCFG interrupt line 2 status register (SYSCFG_ITLINE2) . 7.1.5 SYSCFG interrupt line 3 status register (SYSCFG_ITLINE3) . 7.1.6 SYSCFG interrupt line 4 status register (SYSCFG_ITLINE4) . 7.1.7 SYSCFG interrupt line 5 status register (SYSCFG_ITLINE5) . 7.1.8 SYSCFG interrupt line 6 status register (SYSCFG_ITLINE6) . 7.1.9 SYSCFG interrupt line 7 status register (SYSCFG_ITLINE7) . 7.1.10 SYSCFG interrupt line 8 status register (SYSCFG_ITLINE8) 7.1.11 SYSCFG interrupt line 9 status register (SYSCFG_ITLINE9) 7.1.12 SYSCFG interrupt line 10 status regi ster (SYSCFG_ITLINE10) . . . . . 198 7.1.13 SYSCFG interrupt line 11 status register (SYSCFG_ITLINE11) . . . . . 198 7.1.14 SYSCFG interrupt line 12 status regi ster (SYSCFG_ITLINE12) . . . . . 199 7.1.15 SYSCFG interrupt line 13 status regi ster (SYSCFG_ITLINE13) . . . . . 199 7.1.16 SYSCFG interrupt line 14 status regi ster (SYSCFG_ITLINE14) . . . . . 199 7.1.17 SYSCFG interrupt line 16 status regi ster (SYSCFG_ITLINE16) . . . . . 200 7.1.18 SYSCFG interrupt line 17 status regi ster (SYSCFG_ITLINE17) . . . . . 200 7.1.19 SYSCFG interrupt line 18 status regi ster (SYSCFG_ITLINE18) . . . . . 200 7.1.20 SYSCFG interrupt line 19 status regi ster (SYSCFG_ITLINE19) . . . . . 201 7.1.21 SYSCFG interrupt line 20 status regi ster (SYSCFG_ITLINE20) . . . . . 201 7.1.22 SYSCFG interrupt line 21 status regi ster (SYSCFG_ITLINE21) . . . . . 201 7.1.23 SYSCFG interrupt line 22 status regi ster (SYSCFG_ITLINE22) . . . . . 202 7.1.24 SYSCFG interrupt line 23 status regi ster (SYSCFG_ITLINE23) . . . . . 202 7.1.25 SYSCFG interrupt line 24 status regi ster (SYSCFG_ITLINE24) . . . . . 202 7.1.26 SYSCFG interrupt line 25 status regi ster (SYSCFG_ITLINE25) . . . . . 203 7.1.27 SYSCFG interrupt line 26 status regi ster (SYSCFG_ITLINE26) . . . . . 203 7.1.28 SYSCFG interrupt line 27 status regi ster (SYSCFG_ITLINE27) . . . . . 204 7.1.29 SYSCFG interrupt line 28 status regi ster (SYSCFG_ITLINE28) . . . . . 204 7.1.30 SYSCFG interrupt line 29 status regi ster (SYSCFG_ITLINE29) . . . . . 204 7.1.31 SYSCFG register map. . . . . .. . . . . . .. . . . . .

8.1 Introduction . . . . . . . . . . . . . . . . . . . . . . .

8.2 Connection summary . . . . . . . . . . . . . . . . . . . . .

8.3 Interconnection details . . . . . . . . . . . . . . . . . .

8.3.1 From TIM1, TIM3, TIM4, TIM15, TIM16, and TIM17, to TIM1, TIM3, TIM4, and T IM15 . . . . . . . . . . . . . . . .

8.3.2 From TIM1, TIM3, TIM4, TIM6, TIM15, and EXTI, to ADC . .

8.3.3 From ADC to TIM1 . . . . . . . . . . . . . . . . . . . . .

8.3.4 From HSE, LSE, LSI, MCO, MCO2, RTC and TAMP, to TIM14, TIM16, and TIM17 . . . . . . . . . . . . . . . . . . . . . . .

8.3.5

From internal analog sources to ADC 211 8.3.6 From system errors to TIM 1, TIM3, TIM4, TIM15, TIM16, and TIM17 . . . . . . . . . . . . . . . . . . . . . . . . . .

8.3.7 From TIM16, TIM17, USART1, and USART4, to IRTIM . . . . .

8.3.8 From TIM14 to DMAMUX . . . . . . . . . . . . . . . . . .

9.1 Introduction . . . . . . . . . . . . . . . . . . . . . . .

9.2 DMA main features . . . . . . . . . . . . . . . . . . . . .

9.3 DMA implementation . . . . . . . . . . . . . . . . . . . . 9.3.1 DMA . . . . . . . . . . . . . . . . . . . . . . . . . . .

9.3.2 DMA request mapping . . . . . . . . . . . . . . . . . . .

9.4 DMA functional description . . . . . . . . . . . . . . . . . 9.4.1 DMA block diagram . . . . . . . . . . . . . . . . . . . . 9.4.2 DMA pins and internal s ignals . . . . . . . . . . . . . . 9.4.3 DMA transfers . . . . . . . . . . . . . . . . . . . . . .

9.4.4 DMA arbitration . . . . . . . . . . . . . . . . . . . . .

9.4.5 DMA channels . . . . . . . . . . . . . . . . . . . . . .

9.4.6 DMA data width, alignment and endianness . . . . . . . . 9.4.7 DMA error management . . . . . . . . . . . . . . . . . .

9.5 DMA interrupts . . . . . . . . . . . . . . . . . . . . . .

9.6 DMA registers . . . . . . . . . . . . . . . . . . . . . . . 9.6.1 DMA interrupt status register (DMA_ISR) . . . . . . . . . 9.6.2 DMA interrupt flag clear register (DMA_ IFCR) . . . . . . 9.6.3 DMA channel x configuration register (DMA_CCRx). . . . . 9.6.4 DMA channel x number of data to transfer register (DMA_CND TRx) . 229 9.6.5 DMA channel x peripheral address register (DMA_CPARx) . .

9.6.7 DMA register map . . . . . . . . . . . . . . . . . . . .

10.1 Introduction . . . . . . . . . . . . . . . . . . . . . . .

10.2 DMAMUX main features . . . . . . . . . . . . . . . . . . .

10.3 DMAMUX implementation . . . . . . . . . . . . . . . . . . 10.3.1 DMAMUX instantiation . . . . . . . . . . . . . . . . . .

10.3.2 DMAMUX mapping . . . . . . . . . . . . . . . . . . . . .

10.4 DMAMUX functional description . . . . . . . . . . . . . . . 10.4.1 DMAMUX block diagram . . . . . . . . . . . . . . . . . . 10.4.2 DMAMUX signals . . . . . . . . . . . . . . . . . . . . . 10.4.3 DMAMUX channels . . . . . . . . . . . . . . . . . . . . .

10.4.4 DMAMUX request line multiplexer . . . . . . . . . . . . . 10.4.5 DMAMUX request generator . . . . . . . . . . . . . . . .

10.5 DMAMUX interrupts . . . . . . . . . . . . . . . . . . . . .

10.6 DMAMUX registers . . . . . . . . . . . . . . . . . . . . . 10.6.1 DMAMUX request line multip lexer channel x configuration register (DMAMUX_CxCR) . . . . . . . . . . . . . . . . . . . . . . . . . 10.6.2 DMAMUX request line multiplexer interrupt channel status register (DMAMUX_CSR) . . . . . . . . . . . . . . . . . . . . . . . . . 10.6.3 DMAMUX request line multip lexer interrupt clear flag regi ster (DMAMUX_CFR) . . . . . . . . . . . . . . . . . . . . . . . . . 10.6.4 DMAMUX request generator channel x configuration register (DMAMUX_RGxCR) . . . . . . . . . . . . . . . . . . . . . . . . 10.6.5 DMAMUX request generator interrupt status register (DMAMUX_RGSR) . . . . . . . . . . . . . . . . . . . . . . . . 10.6.6 DMAMUX request generator interrupt clear flag register (DMAMUX_RGCFR) . . . . . . . . . . . . . . . . . . . . . . . . 10.6.7 DMAMUX register map . . . . . . . . . . . . . . . . . .

11.1 Main features . . . . . . . . . . . . . . . . . . . . . .

11.2 SysTick calibration value register . . . . . . . . . . . .

11.3 Interrupt and exception vectors . . . . . . . . . . . . .

12.1 EXTI main features . . . . . . . . . . . . . . . . . . . .

12.2 EXTI block diagram . . . . . . . . . . . . . . . . . . . .

12.2.1 EXTI connections between peripherals and CPU . . . . . .

12.3 EXTI functional description . . . . . . . . . . . . . . . . 12.3.1 EXTI configurable event input wakeup . . . . . . . . . . 12.3.2 EXTI direct event input wakeup . . . . . . . . . . . . . 12.3.3 EXTI mux . . . . . . . . . . . . . . . . . . . . . . . .

12.4 EXTI functional behavior . . . . . . . . . . . . . . . . .

12.5 EXTI registers . . . . . . . . . . . . . . . . . . . . . . 12.5.1 EXTI rising trigger selecti on register (EXTI_RTSR1) . . . 12.5.2 EXTI falling trigger selecti on register 1 (EXTI_FTSR1) . 12.5.3 EXTI software interrupt event register 1 ( EXTI_SWIER1) . 12.5.4 EXTI rising edge pending re gister 1 (EXTI_RPR1) . . . . . 12.5.5 EXTI falling edge pending register 1 ( EXTI_FPR1) . . . . 12.5.6 EXTI external interrupt sel ection register (EXTI_EXTICRx) 12.5.7 EXTI CPU wakeup with interrupt mask register (EXTI_IMR1) 12.5.8 EXTI CPU wakeup with event m ask register (EXTI_EMR1) . . 12.5.9 EXTI register map . . . . . . . . . . . . . . . . . . .

13 Cyclic redundancy check calcu lation unit (CRC) . . . . . . .

13.1 Introduction . . . . . . . . . . . . . . . . . . . . . . .

13.2 CRC main features . . . . . . . . . . . . . . . . . . . . .

13.3 CRC functional description . . . . . . . . . . . . . . . . 13.3.1 CRC block diagram . . . . . . . . . . . . . . . . . . . 13.3.2 CRC internal signals . . . . . . . . . . . . . . . . . . 13.3.3 CRC operation . . . . . . . . . . . . . . . . . . . . . .

13.4 CRC registers . . . . . . . . . . . . . . . . . . . . . . . 13.4.1 CRC data register (CRC_DR) . . . . . . . . . . . . . . . 13.4.2 CRC independent data register (CRC_IDR) . . . . . . . . 13.4.3 CRC control register (CRC_CR) . . . . . . . . . . . . . 13.4.4 CRC initial value (CRC_INIT) . . . . . . . . . . . . . . 13.4.5 CRC polynomial (CRC_POL) . . . . . . . . . . . . . . . . 13.4.6 CRC register map . . . . . . . . . . . . . . . . . . . .

14 Analog-to-digital converter (A DC) . . . . . .

14.1 Introduction . . . . . . . . . . . . . . . . . . . . . . .

14.2 ADC main features . . . . . . . . . . . . . . . . . . . . .

14.3 ADC functional description . . . . . . . . . . . . . . . .

14.3.1 ADC pins and internal signals . . . . . . . . . . . . . 14.3.2 ADC voltage regulator (ADVREGEN) . . . . . . . . . . . . 14.3.3 Calibration (ADCAL) 14.3.4 ADC on-off control (ADEN, ADDIS, ADRDY) . . . . . . . . . 14.3.5 ADC clock (CKMODE, PRESC[3:0]) . . . . . . . . . . . . . 14.3.6 ADC connectivity . . . . . . . . . . . . . . . . . . . . 14.3.7 Configuring the ADC . . . . . . . . . . . . . . . . . . 14.3.8 Channel selection (CHSEL, SCANDIR, CHSELRMOD) . . . . . 14.3.9 Programmable sampling ti me (SMPx[2:0]) . . . . . . . . . 14.3.10 Single conversion mode (CONT = 0) . . . . . . . . . . . 14.3.11 Continuous conversion mode (CONT = 1) . . . . . . . . . 14.3.12 Starting conversions (ADSTART) . . . . . . . . . . . . 14.3.13 Timings . . . . . . . . . . . . . . . . . . . . . . . . 14.3.14 Stopping an ongoing conversion (ADSTP) . . . . . . . .

14.4 Conversion on external trigger and trigger polarity (EXTSEL, EXTEN) . 289 14.4.1 Discontinuous mode (DISCEN) . . . . . . . . . . . . . . . 14.4.2 Programmable resolution ( RES) - Fast conversion mode . . 14.4.3 End of conversion, end of s ampling phase (EOC, EOSMP flag s) . . . . 291 14.4.4 End of conversion sequence (EOS flag) . . . . . . . . . 14.4.5 Example timing diagra ms (single/continuous modes hardware/software triggers) . . . . . . . . . . . . . . . . . . 14.4.6 Low frequency trigger mode . . . . . . . . . . . . . . .

14.5 Data management . . . . . . . . . . . . . . . . . . . . . 14.5.1 Data register and data alig nment (ADC_DR, ALIGN) . . . . 14.5.2 ADC overrun (OVR, OVRMOD) . . . . . . . . . . . . . . . . 14.5.3 Managing a sequence of da ta converted withou t using the D MA . . . . 296 14.5.4 Managing converted data without using the DMA without ove rrun . . . 296 14.5.5 Managing converted data using the DMA . . . . . . . . .

14.6 Low-power features . . . . . . . . . . . . . . . . . . . . 14.6.1 Wait mode conversion . . . . . . . . . . . . . . . . . .

14.6.2 Auto-off mode (AUTOFF) . . . . . . . . . . . . . . . . .

### 14.7 Analog window watchdog (AWD1EN, AWD1SGL, AWD1CH, (p. 11)

ADC_AWDxCR, ADC_AWDxTR) . . . . . . . . . . . . . . . . . . . . 14.7.1 Description of analog wa tchdog 1 . . . . . . . . . . . . 14.7.2 Description of analog wa tchdog 2 and 3 14.7.3 ADC_AWDx_OUT output signal generation . . . . . . . . . 14.7.4 Analog Watchdog threshold control . . . . . . . . . . . .

14.8 Oversampler . . . . . . . . . . . . . . . . . . . . . . . . 14.8.1 ADC operating modes supported when oversampling . . . . 14.8.2 Analog watchdog . . . . . . . . . . . . . . . . . . . . . 14.8.3 Triggered mode . . . . . . . . . . . . . . . . . . . . .

14.9 Temperature sensor and internal reference voltage . . . . .

14.10 Battery voltage monitoring . . . . . . . . . . . . . . .

14.11 ADC interrupts . . . . . . . . . . . . . . . . . . . . .

14.12 ADC registers . . . . . . . . . . . . . . . . . . . . . . 14.12.1 ADC interrupt and status register (ADC_ISR) . . . . . . 14.12.2 ADC interrupt enable register (ADC_IER) . . . . . . . . 14.12.3 ADC control register (ADC_CR) . . . . . . . . . . . . . 14.12.4 ADC configurat ion register 1 (ADC_CFGR1) . . . . . . . 14.12.5 ADC configurat ion register 2 (ADC_CFGR2) . . . . . . . 14.12.6 ADC sampling time register (ADC_SMPR) . . . . . . . . . 14.12.7 ADC watchdog threshold register (ADC_AWD1TR) . . . . . 14.12.8 ADC watchdog threshold register (ADC_AWD2TR) . . . . . 14.12.9 ADC channel selection re gister [alternate] (ADC_CHSELR) 14.12.10 ADC channel selection re gister [alternate] (ADC_CHSELR) 14.12.11 ADC watchdog threshold register (ADC_AWD3TR) . . . . . 14.12.12 ADC data register (ADC_DR) . . . . . . . . . . . . . . 14.12.13 ADC Analog Watchdog 2 Configuration register (ADC_AWD2C R) . . . 330 14.12.14 ADC Analog Watchdog 3 Configuration register (ADC_AWD3C R) . . . 330 14.12.15 ADC Calibration facto r (ADC_CALFACT) . . . . . . . . . 14.12.16 ADC common configurati on register (ADC_CCR) . . . . . .

14.13 ADC register map . . . . . . . . . . . . . . . . . . . . .

15 Advanced-control timer (TIM1). . . . . . .

15.1 TIM1 introduction . . . . . . . . . . . . . . . . . . . .

15.2 TIM1 main features . . . . . . . . . . . . . . . . . . . .

15.3 TIM1 functional description . . . . . . . . . . . . . . . 15.3.1 Time-base unit . . . . . . . . . . . . . . . . . . . . .

15.3.2 Counter modes . . . . . . . . . . . . . . . . . . . . .

15.3.3 Repetition counter . . . . . . . . . . . . . . . . . . . 15.3.4 External trigger input . . . . . . . . . . . . . . . . . 15.3.5 Clock selection . . . . . . . . . . . . . . . . . . . . .

15.3.6 Capture/compare channels . . . . . . . . . . . . . . . .

15.3.7 Input capture mode . . . . . . . . . . . . . . . . . . . 15.3.8 PWM input mode . . . . . . . . . . . . . . . . . . . . . 15.3.9 Forced output mode . . . . . . . . . . . . . . . . . . . 15.3.10 Output compare mode . . . . . . . . . . . . . . . . . . 15.3.11 PWM mode . . . . . . . . . . . . . . . . . . . . . . .

15.3.12 Asymmetric PWM mode . . . . . . . . . . . . . . . . . . 15.3.13 Combined PWM mode . . . . . . . . . . . . . . . . . . .

15.3.14 Combined 3-phase PWM mode . . . . . . . . . . . . . . . 15.3.15 Complementary outputs and dead-time insertion . . . . . 15.3.16 Using the break function . . . . . . . . . . . . . . . 15.3.17 Bidirectional break inputs . . . . . . . . . . . . . . . 15.3.18 Clearing the OCxREF signa l on an external event . . . . 15.3.19 6-step PWM generation . . . . . . . . . . . . . . . . . 15.3.20 One-pulse mode . . . . . . . . . . . . . . . . . . . . 15.3.21 Retriggerable one pul se mode . . . . . . . . . . . . . . 15.3.22 Encoder interface mode . . . . . . . . . . . . . . . . .

15.3.23 UIF bit remapping . . . . . . . . . . . . . . . . . . . 15.3.24 Timer input XOR function . . . . . . . . . . . . . . . . 15.3.25 Interfacing with Hall sensors . . . . . . . . . . . . . 15.3.26 Timer synchronization 15.3.27 ADC synchronization . . . . . . . . . . . . . . . . . . 15.3.28 DMA burst mode . . . . . . . . . . . . . . . . . . . .

15.3.29 Debug mode . . . . . . . . . . . . . . . . . . . . . .

15.4 TIM1 registers . . . . . . . . . . . . . . . . . . . . . . 15.4.1 TIM1 control register 1 (TIM1_CR1) . . . . . . . . . . . 15.4.2 TIM1 control register 2 (TIM1_CR2) . . . . . . . . . . . 15.4.3 TIM1 slave mode control re gister (TIM1_SMCR) . . . . . . 15.4.4 TIM1 DMA/interrupt enable register (TIM1_DIER) . . . . . 15.4.5 TIM1 status register (TIM1_SR) . . . . . . . . . . . . . 15.4.6 TIM1 event generation register (TIM1_EGR) . . . . . . . 15.4.7 TIM1 capture/compare mode register 1 [alternate] (TIM1_CCMR1) . . . . . . . . . . . . . . . . . . . . . . . . . 15.4.8 TIM1 capture/compare mode register 1 [alternate] (TIM1_CCMR1) . . . . . . . . . . . . . . . . . . . . . . . . . 15.4.9 TIM1 capture/compare mode register 2 [alternate] (TIM1_CCMR2) . . . . . . . . . . . . . . . . . . . . . . . . . 15.4.10 TIM1 capture/compare mode register 2 [alternate] (TIM1_CCMR2) . . . . . . . . . . . . . . . . . . . . . . . . .

15.4.11 TIM1 capture/com pare enable register (TIM1_CCER) . . . . . . . . . . . . . . . . . . . . . . . . . 15.4.12 TIM1 counter (TIM1_CNT) . . . . . . . . . . . . . . . . 15.4.13 TIM1 prescaler (TIM1_PSC) . . . . . . . . . . . . . . . 15.4.14 TIM1 auto-relo ad register (TIM1_ARR) . . . . . . . . . . 15.4.15 TIM1 repetition counter register (TIM1_RCR) . . . . . . 15.4.16 TIM1 capture/compare register 1 (TIM1_CCR1) . . . . . . 15.4.17 TIM1 capture/compare register 2 (TIM1_CCR2) . . . . . . 15.4.18 TIM1 capture/compare register 3 (TIM1_CCR3) . . . . . . 15.4.19 TIM1 capture/compare register 4 (TIM1_CCR4) . . . . . . 15.4.20 TIM1 break and dead-time register (TIM1_BDTR) . . . . . . . . . . . . . . . . . . . . . . . . . 15.4.21 TIM1 DMA control register (TIM1_DCR) . . . . . . . . . 15.4.22 TIM1 DMA addre ss for full transfer (TIM1_DMAR) . . . . . . . . . . . . . . . . . . . . . . . . . 15.4.23 TIM1 capture/compare mode register 3 (TIM1_CCMR3) . . . . . . . . . . . . . . . . . . . . . . . . . 15.4.24 TIM1 capture/compare register 5 (TIM1_CCR5) . . . . . . 15.4.25 TIM1 capture/compare register 6 (TIM1_CCR6) . . . . . . 15.4.26 TIM1 alternate function option register 1 (TIM1_AF1) . . 15.4.27 TIM1 Alternate function register 2 (TIM1_AF2) . . . . . 15.4.28 TIM1 timer input selectio n register (TIM1_TISEL) . . . . 15.4.29 TIM1 register map . . . . . . . . . . . . . . . . . . .

16.1 TIM3/TIM4 introduction . . . . . . . . . . . . . . . . . .

16.2 TIM3/TIM4 main features . . . . . . . . . . . . . . . . . .

16.3 TIM3/TIM4 functional description . . . . . . . . . . . . . 16.3.1 Time-base unit . . . . . . . . . . . . . . . . . . . . .

16.3.2 Counter modes . . . . . . . . . . . . . . . . . . . . .

16.3.3 Clock selection . . . . . . . . . . . . . . . . . . . . .

16.3.4 Capture/Compare channels . . . . . . . . . . . . . . . . 16.3.5 Input capture mode . . . . . . . . . . . . . . . . . . . 16.3.6 PWM input mode . . . . . . . . . . . . . . . . . . . . . 16.3.7 Forced output mode . . . . . . . . . . . . . . . . . . . 16.3.8 Output compare mode . . . . . . . . . . . . . . . . . . . 16.3.9 PWM mode . . . . . . . . . . . . . . . . . . . . . . . .

16.3.10 Asymmetric PWM mode . . . . . . . . . . . . . . . . . .

16.3.11 Combined PWM mode . . . . . . . . . . . . . . . . . . .

16.3.12 Clearing the OCxREF signa l on an external event . . . . 16.3.13 One-pulse mode . . . . . . . . . . . . . . . . . . . . 16.3.14 Retriggerable one pul se mode . . . . . . . . . . . . . . 16.3.15 Encoder interface mode . . . . . . . . . . . . . . . . .

16.3.16 UIF bit remapping . . . . . . . . . . . . . . . . . . . 16.3.17 Timer input XOR function . . . . . . . . . . . . . . . . 16.3.18 Timers and external tr igger synchronization . . . . . . 16.3.19 Timer synchronization 16.3.20 DMA burst mode . . . . . . . . . . . . . . . . . . . .

16.3.21 Debug mode . . . . . . . . . . . . . . . . . . . . . .

16.4 TIM3/TIM4 registers . . . . . . . . . . . . . . . . . . . . 16.4.1 TIMx control register 1 ( TIMx_CR1)(x = 3 to 4) . . . . . 16.4.2 TIMx control register 2 ( TIMx_CR2)(x = 3 to 4) . . . . . 16.4.3 TIMx slave mode control register (TIMx_SM CR)(x = 3 to 4) 16.4.4 TIMx DMA/Interrupt enable register (TIMx_DIER)(x = 3 to 4 16.4.5 TIMx status register (T IMx_SR)(x = 3 to 4) . . . . . . . 16.4.6 TIMx event generation regi ster (TIMx_EGR)(x = 3 to 4) . 16.4.7 TIMx capture/compare mode register 1 [alternate] (TIMx_CC MR1) (x = 3 to 4) . . . . . . . . . . . . . . . . . . . . . . . . . 16.4.8 TIMx capture/compare mode register 1 [alternate] (TIMx_CC MR1) (x = 3 to 4) . . . . . . . . . . . . . . . . . . . . . . . . . 16.4.9 TIMx capture/compare mode register 2 [alternate] (TIMx_CC MR2) (x = 3 to 4) . . . . . . . . . . . . . . . . . . . . . . . . . 16.4.10 TIMx capture/compare mode register 2 [alternate] (TIMx_C CMR2) (x = 3 to 4) . . . . . . . . . . . . . . . . . . . . . . . . . 16.4.11 TIMx capture/compare enable register (TIMx_CCER)(x = 3 to 4) . . . . . . . . . . . . . . . . . . . . 16.4.12 TIMx counter [alternate] (TIMx_CNT)(x = 3 to 4) . . . . 16.4.13 TIMx counter [alternate] (TIMx_CNT)(x = 3 to 4) . . . . 16.4.14 TIMx prescaler (TIMx_PSC)(x = 3 to 4) . . . . . . . . . 16.4.15 TIMx auto-relo ad register (TIMx_AR R)(x = 3 to 4) . . . 16.4.16 TIMx capture/compare reg ister 1 (TIMx_CCR1)(x = 3 to 4) 16.4.17 TIMx capture/compare reg ister 2 (TIMx_CCR2)(x = 3 to 4) 16.4.18 TIMx capture/compare reg ister 3 (TIMx_CCR3)(x = 3 to 4) 16.4.19 TIMx capture/compare reg ister 4 (TIMx_CCR4)(x = 3 to 4) 16.4.20 TIMx DMA control register (TIMx_DCR)(x = 3 to 4) . . . . 16.4.21 TIMx DMA address for full transfer (TIMx_DMAR)(x = 3 to

16.4.22 TIM3 alternate function option register 1 (TIM3_AF1) . . 16.4.23 TIM4 alternate function option register 1 (TIM4_AF1) . . 16.4.24 TIM3 timer input selectio n register (TIM3_TISEL) . . . . 16.4.25 TIM4 timer input selectio n register (TIM4_TISEL) . . . . 16.4.26 TIMx register map . . . . . . . . . . . . . . . . . . .

17 Basic timers (TIM6/TIM7) .

17.1 TIM6/TIM7 introduction . . . . . . . . . . . . . . . . . .

17.2 TIM6/TIM7 main features . . . . . . . . . . . . . . . . . .

17.3 TIM6/TIM7 functional description . . . . . . . . . . . . . 17.3.1 Time-base unit . . . . . . . . . . . . . . . . . . . . .

17.3.2 Counting mode . . . . . . . . . . . . . . . . . . . . . 17.3.3 UIF bit remapping . . . . . . . . . . . . . . . . . . . 17.3.4 Clock source . . . . . . . . . . . . . . . . . . . . . .

17.3.5 Debug mode . . . . . . . . . . . . . . . . . . . . . . .

17.4 TIM6/TIM7 registers . . . . . . . . . . . . . . . . . . . . 17.4.1 TIMx control register 1 ( TIMx_CR1)(x = 6 to 7) . . . . . 17.4.2 TIMx control register 2 ( TIMx_CR2)(x = 6 to 7) . . . . . 17.4.3 TIMx DMA/Interrupt enable register (TIMx_DIER)(x = 6 to 7 17.4.4 TIMx status register (T IMx_SR)(x = 6 to 7) . . . . . . . 17.4.5 TIMx event generation regi ster (TIMx_EGR)(x = 6 to 7) . 17.4.6 TIMx counter (TIMx_CNT)(x = 6 to 7) . . . . . . . . . . 17.4.7 TIMx prescaler (TIMx_PSC)(x = 6 to 7) . . . . . . . . . . 17.4.8 TIMx auto-reload register (TIMx_ARR)(x = 6 to 7) . . . . 17.4.9 TIMx register map . . . . . . . . . . . . . . . . . . .

18 General-purpose timers (TIM14). . . . . . .

18.1 TIM14 introduction . . . . . . . . . . . . . . . . . . . .

18.2 TIM14 main features . . . . . . . . . . . . . . . . . . .

18.2.1 TIM14 main features . . . . . . . . . . . . . . . . . .

18.3 TIM14 functional description . . . . . . . . . . . . . . . 18.3.1 Time-base unit . . . . . . . . . . . . . . . . . . . . .

18.3.2 Counter modes . . . . . . . . . . . . . . . . . . . . .

18.3.3 Clock selection . . . . . . . . . . . . . . . . . . . . .

18.3.4 Capture/compare channels . . . . . . . . . . . . . . . . 18.3.5 Input capture mode . . . . . . . . . . . . . . . . . . .

18.3.6 Forced output mode . . . . . . . . . . . . . . . . . . . 18.3.7 Output compare mode . . . . . . . . . . . . . . . . . . . 18.3.8 PWM mode . . . . . . . . . . . . . . . . . . . . . . . .

18.3.9 One-pulse mode . . . . . . . . . . . . . . . . . . . . . 18.3.10 UIF bit remapping . . . . . . . . . . . . . . . . . . . 18.3.11 Using timer output as trigger for other timers (TIM14) . 18.3.12 Debug mode . . . . . . . . . . . . . . . . . . . . . .

18.4 TIM14 registers . . . . . . . . . . . . . . . . . . . . . 18.4.1 TIM14 control register 1 (TIM14_CR1) . . . . . . . . . . 18.4.2 TIM14 Interrupt enable re gister (TIM14_DIER) . . . . . . 18.4.3 TIM14 status register ( TIM14_SR) . . . . . . . . . . . . 18.4.4 TIM14 event generation re gister (TIM14_EGR) . . . . . . 18.4.5 TIM14 capture/compare mode register 1 [alternate] (TIM14_CCMR1) 539 18.4.6 TIM14 capture/compare mode register 1 [alternate] (TIM14_CCMR1) 540 18.4.7 TIM14 capture/compare ena ble register (TIM14_CCER) . . . 18.4.8 TIM14 counter (TIM14_CNT) . . . . . . . . . . . . . . . . 18.4.9 TIM14 prescaler (TIM14_PSC) . . . . . . . . . . . . . . 18.4.10 TIM14 auto-reload regis ter (TIM14_ARR) . . . . . . . . . 18.4.11 TIM14 capture/compare re gister 1 (TIM14_CCR1) . . . . . 18.4.12 TIM14 timer input selecti on register (TIM14_TISEL) . . . 18.4.13 TIM14 register map . . . . . . . . . . . . . . . . . .

19.1 TIM15/TIM16/TIM17 introduction . . . . . . . . . . . . . .

19.2 TIM15 main features . . . . . . . . . . . . . . . . . . .

19.3 TIM16/TIM17 main features . . . . . . . . . . . . . . . . .

19.4 TIM15/TIM16/TIM17 functional description . . . . . . . . . 19.4.1 Time-base unit . . . . . . . . . . . . . . . . . . . . .

19.4.2 Counter modes . . . . . . . . . . . . . . . . . . . . .

19.4.3 Repetition counter . . . . . . . . . . . . . . . . . . . 19.4.4 Clock selection . . . . . . . . . . . . . . . . . . . . .

19.4.5 Capture/compare channels . . . . . . . . . . . . . . . . 19.4.6 Input capture mode . . . . . . . . . . . . . . . . . . . 19.4.7 PWM input mode (only for TIM15) . . . . . . . . . . . . . 19.4.8 Forced output mode . . . . . . . . . . . . . . . . . . . 19.4.9 Output compare mode . . . . . . . . . . . . . . . . . . . 19.4.10 PWM mode . . . . . . . . . . . . . . . . . . . . . . .

19.4.11 Combined PWM mode (TIM 15 only) . . . . . . . . . . . . . 19.4.12 Complementary outputs and dead-time insertion . . . . . 19.4.13 Using the break function . . . . . . . . . . . . . . . 19.4.14 Bidirectional break inputs . . . . . . . . . . . . . . . 19.4.15 One-pulse mode . . . . . . . . . . . . . . . . . . . . 19.4.16 Retriggerable one pulse mode (TIM15 only) . . . . . . . 19.4.17 UIF bit remapping . . . . . . . . . . . . . . . . . . . 19.4.18 Timer input XOR functio n (TIM15 only) . . . . . . . . . 19.4.19 External trigger synchronization (TIM15 only) . . . . . 19.4.20 Slave mode – combined res et + trigger mode . . . . . . . 19.4.21 DMA burst mode . . . . . . . . . . . . . . . . . . . .

19.4.22 Timer synchronization ( TIM15) . . . . . . . . . . . . . 19.4.23 Using timer output as trig ger for other timers (TIM16/TI 19.4.24 Debug mode . . . . . . . . . . . . . . . . . . . . . .

19.5 TIM15 registers . . . . . . . . . . . . . . . . . . . . . 19.5.1 TIM15 control register 1 (TIM15_CR1) . . . . . . . . . . 19.5.2 TIM15 control register 2 (TIM15_CR2) . . . . . . . . . . 19.5.3 TIM15 slave mode control register (TIM15_SMCR) . . . . . 19.5.4 TIM15 DMA/interrupt enable register (TIM15_DIER) . . . . 19.5.5 TIM15 status register ( TIM15_SR) . . . . . . . . . . . . 19.5.6 TIM15 event generation re gister (TIM15_EGR) . . . . . . 19.5.7 TIM15 capture/compare mode register 1 [alternate] (TIM15_CCMR1) . . . . . . . . . . . . . . . . . . . . . . . . . 19.5.8 TIM15 capture/compare mode register 1 [alternate] (TIM15_CCMR1) . . . . . . . . . . . . . . . . . . . . . . . . . 19.5.9 TIM15 capture/compare ena ble register (TIM15_CCER) . . . 19.5.10 TIM15 counter (TIM15_CNT) . . . . . . . . . . . . . . . 19.5.11 TIM15 prescaler (TIM 15_PSC) . . . . . . . . . . . . . . 19.5.12 TIM15 auto-reload regis ter (TIM15_ARR) . . . . . . . . . 19.5.13 TIM15 repetition counte r register (TIM15_RCR) . . . . . 19.5.14 TIM15 capture/compare re gister 1 (TIM15_CCR1) . . . . . 19.5.15 TIM15 capture/compare re gister 2 (TIM15_CCR2) . . . . . 19.5.16 TIM15 break and dead-time register (TIM15_BDTR) . . . . 19.5.17 TIM15 DMA control register (TIM15_DCR) . . . . . . . . 19.5.18 TIM15 DMA address for fu ll transfer (TIM15_DMAR) . . . 19.5.19 TIM15 alternate register 1 (TIM15_AF1) . . . . . . . . . 19.5.20 TIM15 input selection re gister (TIM15_TISEL) . . . . .

19.5.21 TIM15 register map . . . . . . . . . . . . . . . . . .

19.6 TIM16/TIM17 registers . . . . . . . . . . . . . . . . . . . 19.6.1 TIMx control register 1 (TIMx_CR1)(x = 16 to 17) . . . . 19.6.2 TIMx control register 2 (TIMx_CR2)(x = 16 to 17) . . . . 19.6.3 TIMx DMA/interrupt enable re gister (TIMx_DIER)(x = 16 to 17) . . . . . 614 19.6.4 TIMx status register (TIMx_SR)(x = 16 to 17) . . . . . . 19.6.5 TIMx event generation regi ster (TIMx_EGR)(x = 16 to 17) 19.6.6 TIMx capture/compare mode register 1 [alternate] (TIMx_CC MR1) (x = 16 to 17) . . . . . . . . . . . . . . . . . . . . . . . . 19.6.7 TIMx capture/compare mode register 1 [alternate] (TIMx_CC MR1) (x = 16 to 17) . . . . . . . . . . . . . . . . . . . . . . . . 19.6.8 TIMx capture/compare enable register (TIMx_CCER)(x = 16 to 17) . . 620 19.6.9 TIMx counter (TIMx_CNT)(x = 16 to 17) . . . . . . . . . 19.6.10 TIMx prescaler (TIMx_PSC)(x = 16 to 17) . . . . . . . . 19.6.11 TIMx auto-reload register (TIMx_ARR)(x = 16 to 17) . . 19.6.12 TIMx repetition counter register (TIMx_RCR)(x = 16 to 17 19.6.13 TIMx capture/compare reg ister 1 (TIMx_CCR1)(x = 16 to 17 19.6.14 TIMx break and d ead-time register (T IMx_BDTR)(x = 16 to 17) . . . . . 625 19.6.15 TIMx DMA control register (TIMx_DCR)(x = 16 to 17) . . . 19.6.16 TIMx DMA address for full transfer (TIMx_DMAR)(x = 16 to 17) . . . . . 628 19.6.17 TIM16 alternate function register 1 (TIM16_AF1) . . . . 19.6.18 TIM16 input selection re gister (TIM16_TISEL) . . . . . 19.6.19 TIM17 alternate function register 1 (TIM17_AF1) . . . . 19.6.20 TIM17 input selection re gister (TIM17_TISEL) . . . . . 19.6.21 TIM16/TIM17 register map . . . . . . . . . . . . . . .

## 21 Independent watchdog (IWDG) (p. 19)

21.1 Introduction . . . . . . . . . . . . . . . . . . . . . . .

21.2 IWDG main features . . . . . . . . . . . . . . . . . . . .

21.3 IWDG functional description . . . . . . . . . . . . . . . . 21.3.1 IWDG block diagram . . . . . . . . . . . . . . . . . . . 21.3.2 Window option . . . . . . . . . . . . . . . . . . . . . . 21.3.3 Hardware watchdog . . . . . . . . . . . . . . . . . . . . 21.3.4 Register access protection . . . . . . . . . . . . . . . 21.3.5 Debug mode . . . . . . . . . . . . . . . . . . . . . . .

21.4 IWDG registers . . . . . . . . . . . . . . . . . . . . . . 21.4.1 IWDG key register (IWDG_KR) . . . . . . . . . . . . . . . 21.4.2 IWDG prescaler register (IWDG_PR) . . . . . . . . . . . 21.4.3 IWDG reload register (IWDG_RLR) . . . . . . . . . . . . . 21.4.4 IWDG status register (IWDG_SR) . . . . . . . . . . . . . 21.4.5 IWDG window register (IW DG_WINR) . . . . . . . . . . . . 21.4.6 IWDG register map . . . . . . . . . . . . . . . . . . .

22.1 Introduction . . . . . . . . . . . . . . . . . . . . . . .

22.2 WWDG main features . . . . . . . . . . . . . . . . . . . .

22.3 WWDG functional description . . . . . . . . . . . . . . . 22.3.1 WWDG block diagram . . . . . . . . . . . . . . . . . . . 22.3.2 Enabling the watchdog . . . . . . . . . . . . . . . . . 22.3.3 Controlling the down-co unter . . . . . . . . . . . . . . 22.3.4 How to program the watchdog timeout . . . . . . . . . . 22.3.5 Debug mode . . . . . . . . . . . . . . . . . . . . . . .

22.4 WWDG interrupts . . . . . . . . . . . . . . . . . . . . . .

22.5 WWDG registers . . . . . . . . . . . . . . . . . . . . . . 22.5.1 WWDG control register (WWDG_CR) . . . . . . . . . . . . . 22.5.2 WWDG configuration regi ster (WWDG_CFR) . . . . . . . . . 22.5.3 WWDG status register (WWDG_SR) . . . . . . . . . . . . . 22.5.4 WWDG register map . . . . . . . . . . . . . . . . . . .

23.1 Introduction . . . . . . . . . . . . . . . . . . . . . . .

23.2 RTC main features . . . . . . . . . . . . . . . . . . . . .

23.3 RTC functional description . . . . . . . . . . . . . . . . 23.3.1 RTC block diagram . . . . . . . . . . . . . . . . . . . 23.3.2 RTC pins and internal signals . . . . . . . . . . . . . 23.3.3 GPIOs controlled by the RTC and TAMP . . . . . . . . . . 23.3.4 Clock and prescalers . . . . . . . . . . . . . . . . . . 23.3.5 Real-time clock and calendar . . . . . . . . . . . . . .

23.3.6 Programmable alarms . . . . . . . . . . . . . . . . . . . 23.3.7 Periodic auto-wakeup . . . . . . . . . . . . . . . . . . 23.3.8 RTC initialization and configuration . . . . . . . . . .

23.3.9 Reading the calendar . . . . . . . . . . . . . . . . . . 23.3.10 Resetting the RTC . . . . . . . . . . . . . . . . . . . 23.3.11 RTC synchronization . . . . . . . . . . . . . . . . . . 23.3.12 RTC reference clock detection . . . . . . . . . . . . .

23.3.13 RTC smooth digital calibration . . . . . . . . . . . . . 23.3.14 Timestamp function . . . . . . . . . . . . . . . . . . 23.3.15 Calibration clock out put . . . . . . . . . . . . . . . . 23.3.16 Tamper and alarm output . . . . . . . . . . . . . . . .

23.4 RTC low-power modes . . . . . . . . . . . . . . . . . . . .

23.5 RTC interrupts . . . . . . . . . . . . . . . . . . . . . .

23.6 RTC registers . . . . . . . . . . . . . . . . . . . . . . . 23.6.1 RTC time register (RTC_TR) . . . . . . . . . . . . . . . 23.6.2 RTC date register (RTC_DR) . . . . . . . . . . . . . . . 23.6.3 RTC sub second register (RTC_SSR) . . . . . . . . . . . . 23.6.4 RTC initialization control and status register (RTC_ICSR) 23.6.5 RTC prescaler register (RTC_PRER) . . . . . . . . . . . 23.6.6 RTC wakeup timer register (RTC_WUTR) . . . . . . . . . . 23.6.7 RTC control register (RTC_CR) . . . . . . . . . . . . . . 23.6.8 RTC write protection register (RTC_WPR) . . . . . . . . 23.6.9 RTC calibration registe r (RTC_CALR) . . . . . . . . . . . 23.6.10 RTC shift control registe r (RTC_SHIFTR) . . . . . . . . 23.6.11 RTC timestamp time register (RTC_TSTR) . . . . . . . . . 23.6.12 RTC timestamp date register (RTC_TSDR) . . . . . . . . 23.6.13 RTC timestamp sub second register (RTC_TSSSR) . . . . . 23.6.14 RTC alarm A register (RTC_ALRMAR) . . . . . . . . . . . 23.6.15 RTC alarm A sub second register (RTC_ALRMASSR) . . . . 23.6.16 RTC alarm B register (RTC_ALRMBR) . . . . . . . . . . . 23.6.17 RTC alarm B sub second register (RTC_ALR MBSSR) . . . . . 23.6.18 RTC status register (RTC_SR) . . . . . . . . . . . . . 23.6.19 RTC masked interrupt status register (RTC_MISR) . . . . 23.6.20 RTC status clear register (RTC_SCR) . . . . . . . . . . 23.6.21 RTC register map . . . . . . . . . . . . . . . . . . .

24.1 Introduction . . . . . . . . . . . . . . . . . . . . . . .

24.2 TAMP main features . . . . . . . . . . . . . . . . . . . .

24.3 TAMP functional description . . . . . . . . . . . . . . . . 24.3.1 TAMP block diagram . . . . . . . . . . . . . . . . . . . 24.3.2 TAMP pins and internal signals . . . . . . . . . . . . . 24.3.3 TAMP register write protection . . . . . . . . . . . . . 24.3.4 Tamper detection . . . . . . . . . . . . . . . . . . . .

24.4 TAMP low-power modes . . . . . . . . . . . . . . . . . . .

24.5 TAMP interrupts . . . . . . . . . . . . . . . . . . . . .

24.6 TAMP registers . . . . . . . . . . . . . . . . . . . . . . 24.6.1 TAMP control register 1 (TAMP_CR1) . . . . . . . . . . . 24.6.2 TAMP control register 2 (TAMP_CR2) . . . . . . . . . . . 24.6.3 TAMP filter control register (TAMP_FLTCR) . . . . . . . 24.6.4 TAMP interrupt enable register (TAMP_IER) . . . . . . . . 24.6.5 TAMP status register (TAMP_SR) . . . . . . . . . . . . . 24.6.6 TAMP masked interrupt statu s register (TAMP_MISR) . . . 24.6.7 TAMP status clear register (TAMP_SCR) . . . . . . . . . 24.6.8 TAMP backup x register (TAMP_BKPxR) .. . . . .. . . . 24.6.9 TAMP register map . . . . . . . . . . . . . . . . . . .

25 Inter-integrated circuit (I2C) interface . . .. . . . . . .

25.1 Introduction . . . . . . . . . . . . . . . . . . . . . . .

25.2 I2C main features . . . . . . . . . . . . . . . . . . . . .

25.3 I2C implementation . . . . . . . . . . . . . . . . . . . .

25.4 I2C functional description . . . . . . . . . . . . . . . . 25.4.1 I2C1 block diagram . . . . . . . . . . . . . . . . . . . 25.4.2 I2C2 block diagram . . . . . . . . . . . . . . . . . . . 25.4.3 I2C pins and internal signals . . . . . . . . . . . . . 25.4.4 I2C clock requirements . . . . . . . . . . . . . . . . . 25.4.5 Mode selection . . . . . . . . . . . . . . . . . . . . .

25.4.6 I2C initialization . . . . . . . . . . . . . . . . . . .

25.4.7 Software reset . . . . . . . . . . . . . . . . . . . . .

25.4.8 Data transfer . . . . . . . . . . . . . . . . . . . . .

25.4.9 I2C slave mode . . . . . . . . . . . . . . . . . . . . .

25.4.10 I2C master mode . . . . . . . . . . . . . . . . . . . .

25.4.11 I2C_TIMINGR register configuration examples . . . . . . 25.4.12 SMBus specific features . . . . . . . . . . . . . . . . 25.4.13 SMBus initialization . . . . . . . . . . . . . . . . .

25.4.14 SMBus: I2C_TIMEOUTR register configuration examples . . 25.4.15 SMBus slave mode . . . . . . . . . . . . . . . . . . .

25.4.16 Wakeup from Stop mode on address match . . . . . . . . 25.4.17 Error conditions . . . . . . . . . . . . . . . . . . .

25.4.18 DMA requests . . . . . . . . . . . . . . . . . . . . . 25.4.19 Debug mode . . . . . . . . . . . . . . . . . . . . . .

25.5 I2C low-power modes . . . . . . . . . . . . . . . . . . . .

25.6 I2C interrupts . . . . . . . . . . . . . . . . . . . . . .

25.7 I2C registers . . . . . . . . . . . . . . . . . . . . . . . 25.7.1 I2C control register 1 (I2C_CR1) . . . . . . . . . . . . 25.7.2 I2C control register 2 (I2C_CR2) . . . . . . . . . . . . 25.7.3 I2C own address 1 register (I2C_OAR1) . . . . . . . . . . 25.7.4 I2C own address 2 register (I2C_OAR2) . . . . . . . . . . 25.7.5 I2C timing register (I2C_TIMINGR) . . . . . . . . . . . . 25.7.6 I2C timeout register (I 2C_TIMEOUTR) . . . . . . . . . . 25.7.7 I2C interrupt and status register (I2C_ISR) . . . . . . . 25.7.8 I2C interrupt clear reg ister (I2C_ICR) . . . . . . . . . 25.7.9 I2C PEC register (I2C_PECR) . . . . . . . . . . . . . . . 25.7.10 I2C receive da ta register (I2C_RXDR) . . . . . . . . . 25.7.11 I2C transmit data register (I2C_TXDR) . . . . . . . . . 25.7.12 I2C register map . . . . . . . . . . . . . . . . . . .

26 Universal synchonous receiver tra nsmitter (USART) . . . . .

26.1 USART introduction . . . . . . . . . . . . . . . . . . . .

26.2 USART main features . . . . . . . . . . . . . . . . . . .

26.3 USART extended features . . . . . . . . . . . . . . . . . .

26.4 USART implementation . . . . . . . . . . . . . . . . . . .

26.5 USART functional description . . . . . . . . . . . . . . . 26.5.1 USART block diagram . . . . . . . . . . . . . . . . . . . 26.5.2 USART signals . . . . . . . . . . . . . . . . . . . . . . 26.5.3 USART character description . . . . . . . . . . . . . . 26.5.4 USART FIFOs and thresholds . . . . . . . . . . . . . . .

26.5.5 USART transmitter . . . . . . . . . . . . . . . . . . . .

26.5.6 USART receiver . . . . . . . . . . . . . . . . . . . . .

26.5.7 USART baud rate generation . . . . . . . . . . . . . . . 26.5.8 Tolerance of the USART re ceiver to clock deviation . . .

26.5.9 USART Auto baud rate detection . . . . . . . . . . . . .

26.5.10 USART multiprocessor communication . . . . . . . . . . . 26.5.11 USART Modbus communication . . . . . . . . . . . . . .

26.5.12 USART parity control . . . . . . . . . . . . . . . . . . 26.5.13 USART LIN (local interconnection network) mode . . . . 26.5.14 USART synchronous mode . . . . . . . . . . . . . . . . . 26.5.15 USART single-wire Half-d uplex communication . . . . . . 26.5.16 USART receiver timeout . . . . . . . . . . . . . . . .

26.5.17 USART Smartcard mode

26.5.18 USART IrDA SIR ENDEC block . . . . . . . . . . . . . . .

26.5.19 Continuous communication using USART and DMA . . . . . . 26.5.20 RS232 Hardware flow control and RS485 Driver Enable . .

26.5.21 USART low-power management . . . . . . . . . . . . . . .

26.6 USART in low-power modes . . . . . . . . . . . . . . . . .

26.7 USART interrupts . . . . . . . . . . . . . . . . . . . . .

26.8 USART registers . . . . . . . . . . . . . . . . . . . . . 26.8.1 USART control register 1 [alternate] (USART_CR1) . . . . 26.8.2 USART control register 1 [alternate] (USART_CR1) . . . . 26.8.3 USART control register 2 (USART_CR2) . . . . . . . . . . 26.8.4 USART control register 3 (USART_CR3) . . . . . . . . . . 26.8.5 USART baud rate register (USART_BRR) . . . . . . . . . . 26.8.6 USART guard time and prescaler regis ter (USART_GTPR) . . 26.8.7 USART receiver timeout re gister (USART_RTOR) . . . . . . 26.8.8 USART request register (USART_RQR) . . . . . . . . . . . 26.8.9 USART interrupt and statu s register [alternate] (USART_IS 26.8.10 USART interrupt and statu s register [alternate] (USART_I 26.8.11 USART interrupt flag clear register (USART_ICR) . . . . 26.8.12 USART receive data register (USART_RDR) . . . . . . . . 26.8.13 USART transmit data register (USART_TDR) . . . . . . . . 26.8.14 USART prescaler register (USART_PRESC) . . . . . . . . 26.8.15 USART register map . . . . . . . . . . . . . . . . . .

27 Serial peripheral interface / integrated interchip sound (SPI /I2S) . . 863

27.1 Introduction . . . . . . . . . . . . . . . . . . . . . . .

27.2 SPI main features . . . . . . . . . . . . . . . . . . . . .

27.3 I2S main features . . . . . . . . . . . . . . . . . . . . .

27.4 SPI/I2S implementation . . . . . . . . . . . . . . . . . .

27.5 SPI functional description . . . . . . . . . . . . . . . . 27.5.1 General description . . . . . . . . . . . . . . . . . . 27.5.2 Communications between o ne master and one slave . . . . .

27.5.3 Standard multi-slave communication . . . . . . . . . . .

27.5.4 Multi-master communication . . . . . . . . . . . . . . .

27.5.5 Slave select (NSS) pin management . . . . . . . . . . . . 27.5.6 Communication formats . . . . . . . . . . . . . . . . . .

27.5.7 Configuration of SPI . . . . . . . . . . . . . . . . . . 27.5.8 Procedure for enabling SPI . . . . . . . . . . . . . . . 27.5.9 Data transmission and reception procedures . . . . . . . 27.5.10 SPI status flags . . . . . . . . . . . . . . . . . . . 27.5.11 SPI error flags . . . . . . . . . . . . . . . . . . . . 27.5.12 NSS pulse mode . . . . . . . . . . . . . . . . . . . . 27.5.13 TI mode . . . . . . . . . . . . . . . . . . . . . . . .

27.5.14 CRC calculation . . . . . . . . . . . . . . . . . . . .

27.6 SPI interrupts . . . . . . . . . . . . . . . . . . . . . .

27.7 I2S functional description . . . . . . . . . . . . . . . . 27.7.1 I2S general description . . . . . . . . . . . . . . . . 27.7.2 Supported audio protocols . . . . . . . . . . . . . . . . 27.7.3 Start-up description . . . . . . . . . . . . . . . . . . 27.7.4 Clock generator . . . . . . . . . . . . . . . . . . . . . 27.7.5 I2S master mode . . . . . . . . . . . . . . . . . . . . . . . . 27.7.6 I2S slave mode . . . . . . . . . . . . . . . . . . . . . . . . .

27.7.7 I2S status flags . . . . . . . . . . . . . . . . . . . . 27.7.8 I2S error flags . . . . . . . . . . . . . . . . . . . . 27.7.9 DMA features . . . . . . . . . . . . . . . . . . . . . .

27.8 I2S interrupts . . . . . . . . . . . . . . . . . . . . . .

27.9 SPI and I2S registers . . . . . . . . . . . . . . . . . . . 27.9.1 SPI control register 1 (SPIx_CR1) . . . . . . . . . . . . 27.9.2 SPI control register 2 (SPIx_CR2) . . . . . . . . . . . . 27.9.3 SPI status register (SPIx_SR) . . . . . . . . . . . . . . 27.9.4 SPI data register (SPIx_DR) . . . . . . . . . . . . . . 27.9.5 SPI CRC polynomial register (SPIx_CRCPR) . . . . . . . . 27.9.6 SPI Rx CRC register (SPIx_RXCRCR) . . . . . . . . . . . . 27.9.7 SPI Tx CRC register (SPIx_TXCRCR) . . . . . . . . . . .

27.9.8 SPIx_I2S configuration register (SPIx_I2SCFGR) . . . . . 27.9.9 SPIx_I2S prescaler register (SPIx_I2SPR) . . . . . . . . 27.9.10 SPI/I2S register map . . . . . . . . . . . . . . . . .

28 Universal serial bus full-speed host/device interface (USB) .

28.1 Introduction . . . . . . . . . . . . . . . . . . . . . . .

28.2 USB main features . . . . . . . . . . . . . . . . . . . . .

28.3 USB implementation . . . . . . . . . . . . . . . . . . . .

28.4 USB functional description . . . . . . . . . . . . . . . . 28.4.1 Description of USB blocks u sed in both Device and Host mo des . . . . 923 28.4.2 Description of host frame scheduler (HFS) specific to Host mode . . . 924

28.5 Programming considerations for Device and Host modes . . . 28.5.1 Generic USB Device programming . . . . . . . . . . . . . 28.5.2 System and power-on reset . . . . . . . . . . . . . . . . 28.5.3 Double-buffered endpoints and usage in Device mode . . . 28.5.4 Double buffered channels: usage in Host mode . . . . . . 28.5.5 Isochronous transfers in Device mode . . . . . . . . . .

28.5.6 Isochronous transfers in Host mode . . . . . . . . . . .

28.5.7 Suspend/resume events . . . . . . . . . . . . . . . . .

28.6 USB and USB SRAM registers . . . . . . . . . . . . . . . . 28.6.1 Common registers . . . . . . . . . . . . . . . . . . . . 28.6.2 Buffer descriptor table . . . . . . . . . . . . . . . . 28.6.3 USB register map . . . . . . . . . . . . . . . . . . . .

29 Debug support (DBG) . . . . .

29.1 Overview . . . . . . . . . . . . . . . . . . . . . . . . .

29.2 Reference Arm documentation . . . . . . . . . . . . . . . .

29.3 Pinout and debug port pins . . . . . . . . . . . . . . . . 29.3.1 SWD port pins . . . . . . . . . . . . . . . . . . . . . 29.3.2 SW-DP pin assignment . . . . . . . . . . . . . . . . . . 29.3.3 Internal pull-up & pull-down on SWD pins . . . . . . . .

29.4 ID codes and locking mechanism . . . . . . . . . . . . . .

29.5 SWD port . . . . . . . . . . . . . . . . . . . . . . . . . 29.5.1 SWD protocol introduction . . . . . . . . . . . . . . . . 29.5.2 SWD protocol sequence 29.5.3 SW-DP state machine (reset, idle states, ID code) . . .

29.5.4 DP and AP read/write accesses . . . . . . . . . . . . . .

29.5.5 SW-DP registers . . . . . . . . . . . . . . . . . . . . 29.5.6 SW-AP registers . . . . . . . . . . . . . . . . . . . .

29.6 Core debug . . . . . . . . . . . . . . . . . . . . . . . .

29.7 BPU (Break Point Unit) . . . . . . . . . . . . . . . . . . 29.7.1 BPU functionality . . . . . . . . . . . . . . . . . . . .

29.8 DWT (Data Watchpoint) . . . . . . . . . . . . . . . . . . . 29.8.1 DWT functionality . . . . . . . . . . . . . . . . . . . . 29.8.2 DWT Program Counter Sample Register . . . . . . . . . . .

29.9 MCU debug component (DBG) . . . . . . . . . . . . . . . . 29.9.1 Debug support for low-power modes . . . . . . . . . . . . 2 29.9.2 Debug support for timers, watchdog and I

29.10 DBG registers . . . . . . . . . . . . . . . . . . . . . . 29.10.1 DBG device ID code register (DBG_IDCODE) . . . . . . . 29.10.2 DBG configuration register (DBG_CR) . . . . . . . . . . 29.10.3 DBG APB freeze register 1 (DBG_APB_FZ1) 29.10.4 DBG APB freeze register 2 (DBG_APB_FZ2) 29.10.5 DBG register map . . . . . . . . . . . . . . . . . . .

30.1 Flash memory size data register . . . . . . . . . . . . .

30.2 Package data register . . . . . . . . . . . . . . . . . .

## List of tables (p. 28)

Table 2. STM32G0B0xx memory bou ndary addresses . . . . . . . . . Table 3. STM32G070xx memory b oundary addresses . . . . . . . . . Table 4. STM32G030xx an d STM32G050xx memory

Table 5. STM32G0x0 peripheral reg ister boundary addresses . . . Table 6. SRAM size . . . . . . . . . . . . . . . . . . . . . . . Table 7. Boot modes. . . . . . . . . . . . . . . . . . . . . . . Table 8. Flash memory organization for single-bank devices. . . Table 9. Flash memory organizati on for 512 Kbytes dual-bank devi Table 10. Number of wait states a ccording to Flash memory clock Table 11. Option byte format . . . . . . . . . . . . . . . . . .

Table 14. Access status versus protection level and execution mo

Table 19. Functionalities depend ing on the working mode. . . . . Table 20. Low-power run . . . . . . . . . . . . . . . . . . . .

Table 31. SYSCFG register map and reset valu es. . . . . . . . .

Table 35. Programmable data width and endian behavior (when PINC

Table 39. DMAMUX: assignment of multiplexer inputs to resources Table 40. DMAMUX: assignment of trigger inputs to resources. . . Table 41. DMAMUX: assignment of synchronization inputs to resour

Table 43. DMAMUX interrupts. . . 243 Table 44. DMAMUX register map and reset values . . . . . . . . . Table 45. Vector table. . . . . . . . . . . . . . . . . . . . .

Table 47. EVG pin overview. . . . 254 Table 48. EXTI event input configurations and register control .

Table 52. EXTI contro ller register map and reset values . . . .

Table 57. External triggers . . . . . . . . . . . . . . . . . . Table 58. Latency between trigger and start of conversion . . .

Table 62. Analog watchdog 1 ch annel selection . . . . . . . . . Table 63. Maximum output results vs N and M. Grayed values indic Table 64. ADC interrupts . . . . . . . . . . . . . . . . . . . .

Table 66. Behavior of timer outpu ts versus BRK/BRK2 inputs. . .

Table 68. Counting direction versus encoder signals . . . . . . .

Table 70. Output control bits for complementary OCx and OCxN cha nnels with break feature. . . . 416

Table 72. Counting direction versus encoder signals . . . . . . .

Table 74. Output control bit for standard OCx channels. . . . . Table 75. TIM3/TIM4 register ma p and reset values . . . . . . .

Table 77. Output control bit for standard OCx channels. . . . .

Table 81. Output control bits for complementary OCx and OCxN cha nnels with break feature

Table 83. Output control bits for complementary OCx and OCxN cha nnels with break feature

Table 84. TIM16/TIM17 register map and reset values . . . . . .

Table 86. WWDG register map and reset values . . . . . . . . . .

Table 94. Interrupt requests . . . . . . . . . . . . . . . . . .

Table 99. Effect of low-power m odes on TAMP . . . . . . . . . . . Table 100. Interrupt requests . . . . . . . . . . . . . . . . .

Table 106. I2C-SMBus specificatio n data setup and hold times . . Table 107. I2C configuration. . . . . . . . . . . . . . . . . .

Table 109. Examples of timing settings for fI2CCLK = 8 MHz . . . Table 110. Examples of timings settings for fI2CCLK = 16 MHz . . Table 111. Examples of timings settings for fI2CCLK = 48 MHz . .

Table 114. Examples of TIMEOUTA s ettings for various I2CCLK freq uencies (max t TIMEOUT = 25 ms) . . . . . . . . . . . . . . . . . . . . . . . . . . . Table 115. Examples of TIMEOUTB s ettings for various I2CCLK freq Table 116. Examples of TIMEOUTA s ettings for various I2CCLK freq uencies (max t IDLE = 50 µs) . . . . . . . . . . . . . . . . . . . . . . . . . . .

Table 123. Tolerance of the USART receiver when BRR [3:0] = 0000 Table 124. Tolerance of the USART receiver when BRR[3:0] is diff

Table 126. Effect of low-power modes on the USART . . . . . . . .

Table 128. USART register map and reset values . . . . . . . . . Table 129. STM32G0x0 SPI and SPI /I2S implementation . . . . . . .

Table 131. Audio-frequency precis ion using standard 8 MHz HSE .

Table 134. STM32G0x0 USB implementation . . . . . . . . . . . .

Table 136. Bulk double-buffering memory buffers usage (Device mo Table 137. Bulk double-buffering memory buffers usage (Host mode

Table 153. 32-bit debug port registers addressed through the shi

## List of figures (p. 32)

Figure 2. Memory map. . . .

Figure 8. Clock tree . . . . . . . . . . . . . . . . . . . . . .

Figure 10. Frequency measurement with TIM14 in capture mode. . . Figure 11. Frequency measurement with TIM16 in capture mode. . . Figure 12. Frequency measurement with TIM17 in capture mode. . .

Figure 14. Input floating/pull up/ pull down configurations . . . 8

Figure 20. Synchronization mode of the DMAMUX request line multi Figure 21. Event generation of t he DMA request line multiplexer

Figure 23. Configurable event tri gger logic CPU wakeup . . . . .

Figure 28. ADC calibration. . . . . . . . . . . . . . . . . . .

Figure 32. ADC connectivity. . . . . 283

Figure 36. Single conversions of a sequence, software trigger . . Figure 37. Continuous conversion of a sequence, software trigger Figure 38. Single conversions of a sequence, hardware trigger . Figure 39. Continuous conversions of a sequence, hardware trigge

Figure 41. Example of overrun (OVR) .. . 295 Figure 42. Wait mode conversion (continuous mode, software trigg Figure 43. Behavior with WAIT = 0, AUTOFF = 1 . . . . . . . . . Figure 44. Behavior with WAIT = 1, AUTOFF = 1 . . . . . . . . .

Figure 46. ADC_AWDx_OUT signal generation . . . . . . . . . . . Figure 47. ADC_AWDx_OUT signal gen eration (AWDx flag not cleared Figure 48. ADC_AWDx_OUT signal gen eration (on a single channel)

Figure 51. Numerical example wit h 5-bits shift and rounding . . Figure 52. Triggered oversampling mode (TOVS bit = 1) . . . . . Figure 53. Temperature sensor and VREFINT channel block diagram

Figure 56. Counter timing diagram with prescaler division change Figure 57. Counter timing diagram with prescaler division change Figure 58. Counter timing diagram, internal clock divided by 1 . Figure 59. Counter timing diagram, internal clock divided by 2 . Figure 60. Counter timing diagram, internal clock divided by 4 . Figure 61. Counter timing diagram, internal clock divided by N. Figure 62. Counter timing diagram, update event when ARPE=0 (TIM x_ARR not preloaded) . . . . . 343 Figure 63. Counter timing diagram, update event when ARPE=1 (TIM Figure 64. Counter timing diagram, internal clock divided by 1 . Figure 65. Counter timing diagram, internal clock divided by 2 . Figure 66. Counter timing diagram, internal clock divided by 4 . Figure 67. Counter timing diagram, internal clock divided by N. Figure 68. Counter timing diagram, update event w hen repetition counter is not use Figure 69. Counter timing diagram, internal clock divided by 1, Figure 70. Counter timing diagram, internal clock divided by 2 . Figure 71. Counter timing diagram, internal clock divided by 4, Figure 72. Counter timing diagram, internal clock divided by N. Figure 73. Counter timing diagram, update event with ARPE=1 (cou Figure 74. Counter timing diagram, Update event with ARPE=1 (cou Figure 75. Update rate examples d epending on mode and TIMx_RCR r

Figure 78. Control circuit in norma l mode, internal clock divide

Figure 83. Capture/compare cha nnel (example: channel 1 input sta Figure 84. Capture/co mpare channel 1 main circuit . . . . . . . Figure 85. Output stage of capture/compare channel (channel 1, i Figure 86. Output stage of capture/compare channel (channel 4). Figure 87. Output stage of capture/compare channel (channel 5, i

Figure 89. Output com pare mode, toggle on OC1. . . . . . . . . . Figure 90. Edge-aligned PWM waveforms (ARR=8) . . . . . . . . . . Figure 91. Center-alig ned PWM waveforms (ARR=8) . . . . . . . . . Figure 92. Generation of 2 phase-s hifted PWM signals with 50% du Figure 93. Combined PWM mode on channel 1 and 3 . . . . . . . . Figure 94. 3-phase combined PWM s ignals with mult iple trigger pu Figure 95. Complementary output with dead -time insertion . . . . Figure 96. Dead-time waveforms with delay greater than the negat Figure 97. Dead-time waveforms with delay greater than the posit

Figure 99. Various output behavior in response to a break event Figure 100. PWM output state following BRK and BRK2 pins asserti

Figure 101. PWM outpu t state following BRK assertion (OSSI=0) . Figure 102. Output redirection (BRK2 request not represented) . .

Figure 104. 6-step generation, COM example (OSSR=1) . . . . . . .

Figure 107. Example of counter o peration in encoder interface mo Figure 108. Example of encoder interface mode with TI1FP1 polari Figure 109. Measuring time interval between edges on 3 signals .

Figure 114. Control circuit in external clock mode 2 + trigger m

Figure 116. Counter timing diagra m with prescaler division chang Figure 117. Counter timing diagra m with prescaler division chang Figure 118. Counter timing diagra m, internal clock divided by 1 . Figure 119. Counter timing diagra m, internal clock divided by 2 . Figure 120. Counter timing diagra m, internal clock divided by 4 . Figure 121. Counter timing diagra m, internal clock divided by N. Figure 122. Counter timing diagra m, Update event when ARPE=0 (TI Mx_ARR not preloaded). . . . . 440 Figure 123. Counter timing diagra m, Update event when ARPE=1 (TI Figure 124. Counter timing diagra m, internal clock divided by 1 . Figure 125. Counter timing diagra m, internal clock divided by 2 . Figure 126. Counter timing diagra m, internal clock divided by 4 . Figure 127. Counter timing diagra m, internal clock divided by N. Figure 128. Counter timing diagram, Update event when repetition counter

Figure 129. Counter timing diagra m, internal cloc k divided by 1, Figure 130. Counter timing diagra m, internal clock divided by 2 . Figure 131. Counter timing diagra m, internal cloc k divided by 4, Figure 132. Counter timing diagra m, internal clock divided by N. Figure 133. Counter timing diagra m, Update event with ARPE=1 (co Figure 134. Counter timing diagra m, Update event with ARPE=1 (co Figure 135. Control circuit in normal mode, internal clock divid

Figure 140. Capture/Compare channel (example: channel 1 input st Figure 141. Capture/Compare cha nnel 1 main circuit . . . . . . . Figure 142. Output stage of Capture/Compare channel (channel 1) .

Figure 144. Output compare mod e, toggle on OC1. . . . . . . . . Figure 145. Edge-aligned PWM waveforms (ARR=8) . . . . . . . . . Figure 146. Center-aligned PWM waveforms (ARR=8) . . . . . . . . Figure 147. Generation of 2 phase -shifted PWM signals with 50% d Figure 148. Combined PWM mode o n channels 1 and 3 . . . . . . .

Figure 152. Example of counter o peration in encoder interface mo Figure 153. Example of encoder interface mode with TI1FP1 polari

Figure 157. Control circuit in external clock mode 2 + trigger m

Figure 159. Master/slave connecti on example with 1 channel only

Figure 164. Triggering TIMy and TIMz with TIMy TI1 input . . . .

Figure 166. Counter timing diagra m with prescaler division chang Figure 167. Counter timing diagra m with prescaler division chang Figure 168. Counter timing diagra m, internal clock divided by 1 . Figure 169. Counter timing diagra m, internal clock divided by 2 . Figure 170. Counter timing diagra m, internal clock divided by 4 . Figure 171. Counter timing diagra m, internal clock divided by N. Figure 172. Counter timing diagra m, update event when ARPE = 0 (TIMx_ARR not

Figure 173. Counter timing diagra m, update event when ARPE=1 (TI Mx_ARR

Figure 174. Control circuit in normal mode, internal clock divid Figure 175. General-purpose time r block diagram (TIM14). . . . . Figure 176. Counter timing diagra m with prescaler division chang Figure 177. Counter timing diagra m with prescaler division chang Figure 178. Counter timing diagra m, internal clock divided by 1 . Figure 179. Counter timing diagra m, internal clock divided by 2 . Figure 180. Counter timing diagra m, internal clock divided by 4 . Figure 181. Counter timing diagra m, internal clock divided by N. Figure 182. Counter timing diagra m, update event when ARPE=0 (TI Mx_ARR not

Figure 183. Counter timing diagra m, update event when ARPE=1 (TI Mx_ARR

Figure 184. Control circuit in normal mode, internal clock divid Figure 185. Capture/compare channel (example: channel 1 input st

Figure 187. Output stage of capture/compare channel (channel 1). Figure 188. Output compare mode, toggle on OC1. . . . . . . . . Figure 189. Edge-aligned PWM waveforms (ARR=8) . . . . . . . . .

Figure 192. Counter timing diagra m with prescaler division chang Figure 193. Counter timing diagra m with prescaler division chang Figure 194. Counter timing diagra m, internal clock divided by 1 . Figure 195. Counter timing diagra m, internal clock divided by 2 . Figure 196. Counter timing diagra m, internal clock divided by 4 . Figure 197. Counter timing diagra m, internal clock divided by N. Figure 198. Counter timing diagra m, update event when ARPE=0 (TI Mx_ARR not

Figure 199. Counter timing diagra m, update event when ARPE=1 (TI Mx_ARR

Figure 200. Update rate examples depending on mode and TIMx_RCR Figure 201. Control circuit in normal mode, internal clock divid

Figure 204. Capture/compare channel (example: channel 1 input st

Figure 206. Output stage of capture/compare channel (channel 1). Figure 207. Output stage of capture/compare channel (channel 2 f

Figure 209. Output compare mod e, toggle on OC1. . . . . . . . . Figure 210. Edge-aligned PWM waveforms (ARR=8) . . . . . . . . . Figure 211. Combined PWM mode o n channel 1 and 2 . . . . . . . . Figure 212. Complementary output with dead-time insertion. . . . Figure 213. Dead-time waveforms with delay greater than the nega Figure 214. Dead-time waveforms with delay greater than the posi

Figure 217. Output redirecti on . . . . . . . . . . . . . . . . .

Figure 220. Measuring time interval between edges on 2 signals .

Figure 232. I2C bus protoco l . . . . . . . . . . . . . . . . . .

Figure 235. Data reception Figure 236. Data transmission . . . . . . . . . . . . . . . . .

Figure 238. Transfer sequence flowchart for I2C slave transmitte r, NOSTRETCH= 0 . . . . . . . . . . . . . . . . . . . . . . . . . Figure 239. Transfer sequence flowchart for I2C slave transmitte r, NOSTRETCH= 1 . . . . . . . . . . . . . . . . . . . . . . . . . Figure 240. Transfer bus diagrams for I2C slave transmitter. . . Figure 241. Transfer sequence flowchart for slave receiver with Figure 242. Transfer sequence flowchart for slave receiver with Figure 243. Transfer bus diagrams for I2C slave receiver . . .

Figure 246. 10-bit address read access with HEAD10R=0 . . . . . Figure 247. 10-bit address read access with HEAD10R=1 . . . . .

Figure 248. Transfer sequence flowchart for I2C master transmitt Figure 249. Transfer sequence flowchart for I2C master transmitt Figure 250. Transfer bus diagrams for I2C master transmitter . . Figure 251. Transfer sequence flowchart for I2C master receiver Figure 252. Transfer sequence flowchart for I2C master receiver Figure 253. Transfer bus diagra ms for I2C master receiver . . . Figure 254. Timeout intervals for t LOW:SEXT, t LOW:MEXT. . . . 742 Figure 255. Transfer sequence flowchart for SMBus slave transmit Figure 256. Transfer bus diagrams for SMBus slave transmitter (S Figure 257. Transfer sequence flowchart for SMBus slave receiver Figure 258. Bus transfer diagrams for SMBus slave receiver (SBC= Figure 259. Bus transfer diagrams for SMBus master transmitter. Figure 260. Bus transfer diagrams for SMBus master receiver . . .

Figure 262. Word le ngth programming .. . 782

Figure 265. Start bit detection when ov ersampling by 16 or 8. .

Figure 267. Data sampling when oversampling by 16 . . . . . . . . Figure 268. Data samp ling when oversampli ng by 8 . . . . . . . .

Figure 270. Mute mode using address mark detection . . . . . . . Figure 271. Break detection in LI N mode (11-bit break length - L Figure 272. Break detection in LIN mode vs. Framing error detect Figure 273. USART example of synch ronous master transmission. . Figure 274. USART data c lock timing diagram in synchronous maste r mode

Figure 275. USART data c lock timing diagram in synchronous maste r mode

Figure 276. USART data c lock timing diagram in synchronous slave mode

Figure 278. Parity error detection using the 1.5 stop bits . . . 812

Figure 280. IrDA data modulation (3/16) - Normal mode. . . . . .

Figure 283. Hardware flow control between 2 USARTs . . . . . . .

Figure 286. Wakeup event verified (wakeup event = address match, Figure 287. Wakeup event not veri fied (wakeup event = address ma tch,

Figure 288. SPI block diagram. . . . . . . . . . . . . . . . . . Figure 289. Full-duplex single ma ster/ single slave application. Figure 290. Half-duplex single mas ter/ single slave application Figure 291. Simplex single master/single slave application (master in transmit-only/ slave in receive-only mode) . . . . . . . . . . . . . . . . . .

Figure 294. Hardware/software s lave select management . . . . .

Figure 296. Data alignment when data length is not equal to 8-bi Figure 297. Packing data in FIFO for transmission and reception.

Figure 300. Master fu ll-duplex communication with CRC . . . . . Figure 301. Master full-duplex communication in packed mode . . Figure 302. NSSP pulse generation in Motorol a SPI master mode. . Figure 303. TI mode transfer . . . . . . . . . . . . . . . . . . Figure 304. I2S block diagr2 am . . . . . . . . . . . . . . . . . Figure 305. I2S Philips protocol waveforms ( 16/32-bit full accuracy). . . . . Figure 306. I S Philips standard waveforms (2 4-bit frame) . . . . . . . . . .

Figure 309. I S Philips standard (16-bit exten ded to 32-bit p acket frame) . . Figure 310. Example of 16-bit data frame extended to 32-bit chan Figure 311. MSB Justified 16-bit o r 32-bit full-accuracy length

Figure 313. MSB justifie d 16-bit extended to 32-bit packet frame Figure 314. LSB justified 16-bit or 32-bit full-accuracy . . . .. 895

Figure 316. Operations required to transmit 0x3478AE. . . . . . Figure 317. Operations required to receive 0x3478AE . . . . . . Figure 318. LSB justified 16-bit extended to 32-bit packet frame Figure 319. Example of 16-bit data frame extended to 32-bit chan

Figure 321. PCM standard waveforms (16-bit extended to 32-bit pa

Figure 324. I S clock generator architecture . . . . . . . . . . . . . . . . .

Figure 326. Packet buffer areas with examples of buffer descript® Figure 327. Block diagram of STM32G0x0 MCU and Cortex -M0+-level debug sup
