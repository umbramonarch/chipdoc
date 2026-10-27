---
document: "STM32G0x0 advanced Arm®-based 32-bit MCUs - Reference manual"
section_id: "30"
section_title: "30 Device electronic signature"
pages: "980-981"
---

# 30 Device electronic signature (pp. 980-981)

## 30 Device electronic signature (p. 980)

The device electronic signature is stored in the System memory area of the F lash memory module, and can be read using th e debug interface or by the CPU. It contains factory-programmed identification and calibration data that allow the u ser firmware or other external devices to automatica lly match to the characteristics of the ST M32G0x0 microcontroller.

### 30.1 [DBG.FLASH_MEMORY_SIZE] Offset: 0x00 | Reset: 0xXXXX (p. 980)

Base address: 0x1FFF 75E0

Address offset: 0x00

Read only = 0xXXXX where X is factory-p rogrammed

Bits 15:0 FLASH_SIZE[15:0]: Flash memory size This bitfield indicates the size of the device Flash memory exp ressed in Kbytes. As an example, 0x040 corresponds to 64 Kbytes.

### 30.2 [DBG.PACKAGE] Offset: 0x00 | Reset: 0xXXXX (p. 980)

Base address: 0x1FFF 7500

Address offset: 0x00

Read only = 0xXXXX where X is factory-p rogrammed

Bits 15:4 Reserved

Bits 3:0 PKG[3:0]: Package type For STM32G070xx: 0100: LQFP32 1000: LQFP48 1100: LQPF64 Others: Reserved For STM32G030xx and STM32G050xx: 0100: LQFP32 0111: LQFP48 1100: LQPF64 Others: Reserved For STM32G0B0xx: 0000: LQFP100 0001: LQFP32 0100: LQFP48 0111: LQFP64 Others: Reserved
