---
document: "STM32G0x0 advanced Arm®-based 32-bit MCUs - Reference manual"
section_id: "01"
section_title: "1 Documentation conventions"
pages: "39-41"
---

# 1 Documentation conventions (pp. 39-41)

## 1 Documentation conventions (p. 39)

### 1.1 General information (p. 39)

The STM32G0x0 dev ices have an Arm® (a) Cortex®-M0+ core.

### 1.2 List of abbreviations for registers (p. 39)

The following abbreviations (b) are used in register descriptions:

read/write (rw) Software can read and write to this bit.

read-only (r) Software can only read this bit.

write-only (w) Software can onl y write to this b it. Reading this bit returns the reset value.

read/clear write0 (rc_w0) Software can read as we ll as clear thi s bit by writing 0. Writing 1 has no effect on the bit value.

read/clear write1 (rc_w1) Software can read as we ll as clear thi s bit by writing 1. Writing 0 has no effect on the bit value.

read/clear write (rc_w) Software can read as well as clear this bit by writing to t he register. The value written to this b it is not important.

read/clear by read (rc_r) Software can read this bit. Reading th is bit automatically clears it to 0. Writing this bit has no effect on the bit value.

read/set by read (rs_r) Software can read this bi t. Reading this bit automatica lly sets it to 1. Writing this bit has no effect on the bit value.

read/set (rs) Software can read a s well as set this bit. Writing 0 has no effect on the bit value.

read/write once (rwo) Software ca n only write once to this bit a nd can also read it at any time. Only a reset can return t he bit to its reset value.

toggle (t) The software can toggle this bit by writing 1. Writin g 0 has no effect.

read-only write trigger (rt_w1) S oftware can read this bit. Writ ing 1 triggers an event but has no effect on the bit value.

Reserved (Res.) Reserved bit, m ust be kept at reset value.

a. Arm is a registered trademark of Arm Limited (or its subsidia ries) in the US and/or elsewhere. b. This is an exhaustive list of a ll abbreviations applicable to STMicroelectronics microcontrollers, some of them may not be used in the current document.

### 1.3 Glossary (p. 40)

This section gives a brief defin ition of acronyms and abbreviat ions used in this document:

- Word: data of 32-bit length.

- Half-word: data of 16-bit length.

- Byte: data of 8-bit length.

- SWD-DP (SWD DEBUG PORT): SWD-DP provides a 2-pin (clock and data) interface® based on the Serial Wire Debug ( SWD) protocol. P lease refer to the Cortex -M0+ technical reference manual.

- IAP (in-application programming): IAP is the ability to re-program the Flash memory of a microcontrolle r while the u ser program is running.

- ICP (in-circuit programming): ICP is the ability to program the Flash m emory of a microcontroller using the SWD protocol or the bootloader while the device is mounted on the user application board.

- Option bytes: product configuration bits stored in the Flash memory.

- OBL: option byte loader.

- AHB: advanced high-performance bus.

- APB: advanced peripheral bus.

### 1.4 Availability of peripherals (p. 40)

For availability of p eripherals and their number across all sal es types, refer to the particular device datasheet.

The following table shows per-pro duct availability of periphera ls that are not common to all STM32G0x0 products.

**Table 1. Peripherals versus products (pp. 40-41)**

| Col 1 | Feature STM32G030 STM32G050 STM32G070 STM32G0B0 | Col 4 |
| --- | --- | --- |
|  | TIM4 No No No Yes |  |
|  | TIM6 and TIM7 No Yes Yes Yes |  |
|  | TIM15 No No Yes Yes |  |
|  | I2C3 No No No Yes |  |
|  | SPI3 No No No Yes |  |
|  | I2S2 No No No Yes |  |
|  | USART3, USART4 No No Yes Yes |  |
|  | USART5, USART6 No No No Yes |  |
| USART3 independent clock selection | N/A N/A No Yes | N/A N/A No Yes |
| I2C2 independent clock selection | No No No Yes | No No No Yes |
|  | USB No No No Yes |  |
|  | UCPDx_STROBE bits No No Yes Yes |  |
|  | DMA2 No No No Yes |  |
|  | MCO2 No No No Yes |  |
|  | GPIO port E No No No Yes |  |
| Switchable I/O clamping diode | Yes Yes No Yes | Yes Yes No Yes |
|  | PLLQCLK No No No Yes |  |
