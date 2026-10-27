---
document: "MSPM0 C-Series 24-MHz Microcontrollers Technical Reference Manual (Rev. C)"
section_id: "01"
section_title: "Table of Contents"
pages: "3-9"
---

# Table of Contents (pp. 3-9)

Table of Contents

Read This First 9 About This Manual 9 Notational Conventions 9 Glossary 9 Related Documentation 9 Support Resources 9

## 1 Architecture 10 (p. 3)

1.1 Architecture Overview 11 1.2 Bus Organization 11 1.3 Platform Memory Map 13 1.3.1 Code Region 13 1.3.2 SRAM Region 13 1.3.3 Peripheral Region 15 1.3.4 Subsystem Region 15 1.3.5 System PPB Region 16 1.4 Boot Configuration 16 1.4.1 Configuration Memory (NONMAIN) 16 1.4.2 Boot Configuration Routine (BCR) 17 1.4.3 NONMAIN Layout Types 21 1.4.4 NONMAIN_TYPEB Registers 22 1.4.5 NONMAIN_TYPED Registers 28 1.5 Factory Constants 52 1.5.1 FACTORYREGION Registers 53

## 2 PMCU 59 (p. 3)

2.1 PMCU Overview 60 2.1.1 Power Domains 61 2.1.2 Operating Modes 61 2.2 Power Management (PMU) 64 2.2.1 Power Supply 65 2.2.2 Core Regulator 65 2.2.3 Supply Supervisors 65 2.2.4 Bandgap Reference 66 2.2.5 VBOOST for Analog Muxes 67 2.2.6 Peripheral Enable 69 2.3 Clock Module (CKM) 70 2.3.1 Oscillators 70 2.3.2 Clocks 75 2.3.3 Clock Tree 82 2.3.4 Clock Monitors 84 2.3.5 Frequency Clock Counter (FCC) 87 2.4 System Controller (SYSCTL) 90 2.4.1 Resets and Device Initialization 90 2.4.2 Operating Mode Selection 98 2.4.3 Asynchronous Fast Clock Requests 99 2.4.4 SRAM Write Protection 101 2.4.5 Flash Wait States 102 2.4.6 Shutdown Mode Handling (if present) 102 2.4.7 Configuration Lockout 103

2.4.9 Error Handling 103 2.4.10 SYSCTL Events 105 2.5 Quick Start Reference 106 2.5.1 Default Device Configuration 106 2.5.2 Leveraging MFCLK 107 2.5.3 Optimizing Power Consumption in STOP Mode 107 2.5.4 Optimizing Power Consumption in STANDBY Mode 107 2.5.5 Increasing MCLK and ULPCLK Precision 108 2.5.6 High Speed Clock (SYSPLL, HFCLK) Handling in Low-Power Modes 108 2.5.7 Optimizing for Lowest Wakeup Latency 108 2.5.8 Optimizing for Lowest Peak Current in RUN/SLEEP Mode 108 2.6 SYSCTL_C1103_C1104 Registers 109 2.7 SYSCTL_C1105_C1106 Registers 154

## 3 CPU 204 (p. 4)

3.1 Overview 205 3.2 Arm Cortex-M0+ CPU 205 3.2.1 CPU Register File 206 3.2.2 Stack Behavior 208 3.2.3 Execution Modes and Privilege Levels 208 3.2.4 Address Space and Supported Data Sizes 208 3.3 Interrupts and Exceptions 209 3.3.1 Peripheral Interrupts (IRQs) 210 3.3.2 Interrupt and Exception Table 214 3.3.3 Processor Lockup Scenario 216 3.4 CPU Peripherals 216 3.4.1 System Control Block (SCB) 216 3.4.2 System Tick Timer (SysTick) 217 3.5 Read-Only Memory (ROM) 218 3.6 CPUSS Registers 219 3.7 WUC Registers 227

## 4 SECURITY 229 (p. 4)

4.1 Overview 229 4.1.1 Secure Boot 230 4.1.2 Customer Secure Code (CSC) 230 4.2 Boot and Startup Sequence 230 4.2.1 CSC Programming Overview 231 4.3 Flash Memory Protection 232 4.3.1 Write Protection 233 4.3.2 Read-Execute Protection 233 4.3.3 IP Protection 233 4.4 SRAM Protection 234 4.5 SECURITY Registers 235

## 5 Direct Memory Access (DMA) 246 (p. 4)

5.1 DMA Overview 247 5.2 DMA Operation 248 5.2.1 Addressing Modes 249 5.2.2 Channel Types 250 5.2.3 Transfer Modes 251 5.2.4 Extended Modes 253 5.2.5 Initiating DMA Transfers 255 5.2.6 Stopping DMA Transfers 256 5.2.7 Channel Priorities 256 5.2.8 Burst Block Mode 256 5.2.9 Using DMA with System Interrupts 256 5.2.10 DMA Controller Interrupts 257 5.2.11 DMA Trigger Event Status 257 5.2.12 DMA Operating Mode Support 257 5.2.13 DMA Address and Data Errors 258 5.2.14 Interrupt and Event Support 258

6.1 NVM Overview 309 6.1.1 Key Features 309 6.1.2 System Components 309 6.1.3 Terminology 309 6.2 Flash Memory Bank Organization 310 6.2.1 Banks 310 6.2.2 Flash Memory Regions 310 6.2.3 Addressing 310 6.2.4 Memory Organization Examples 311 6.3 Flash Controller 312 6.3.1 Overview of Flash Controller Commands 312 6.3.2 NOOP Command 313 6.3.3 PROGRAM Command 313 6.3.4 ERASE Command 318 6.3.5 READVERIFY Command 318 6.3.6 BLANKVERIFY Command 319 6.3.7 Command Diagnostics 320 6.3.8 Overriding the System Address With a Bank ID, Region ID, and Bank Address 321 6.3.9 FLASHCTL Events 321 6.4 Write Protection 321 6.4.1 Write Protection Resolution 322 6.4.2 Static Write Protection 322 6.4.3 Dynamic Write Protection 322 6.5 Read Interface 323 6.5.1 Bank Address Swapping 323 6.6 FLASHCTL Registers 325

## 7 Events 411 (p. 5)

7.1 Events Overview 412 7.1.1 Event Publisher 412 7.1.2 Event Subscriber 412 7.1.3 Event Fabric Routing 412 7.1.4 Event Routing Map 414 7.1.5 Event Propagation Latency 415 7.2 Events Operation 415 7.2.1 CPU Interrupt 415 7.2.2 DMA Trigger 416 7.2.3 Peripheral to Peripheral Event 417 7.2.4 Extended Module Description Register 417 7.2.5 Using Event Registers 417

## 8 Low Frequency Subsystem (LFSS_B) 421 (p. 5)

8.1 Overview 422 8.2 Clock System 422 8.3 LFSS Reset 423 8.4 Real Time Counter (RTC_x) 423 8.5 Independent Watchdog Timer (IWDT) 424 8.6 Lock Function of RTC and IWDT 424 8.7 LFSS Registers 425

## 9 IOMUX 546 (p. 5)

9.1 IOMUX Overview 547 9.1.1 IO Types and Analog Sharing 547 9.2 IOMUX Operation 550 9.2.1 Peripheral Function (PF) Assignment 550 9.2.2 Logic High to Hi-Z Conversion 550 9.2.3 Logic Inversion 551 9.2.4 SHUTDOWN Mode Wakeup Logic 551 9.2.5 Pullup/Pulldown Resistors 552 9.2.6 Drive Strength Control 552 9.2.7 Hysteresis and Logic Level Control 552 9.3 IOMUX Registers 554

10.2 GPIO Operation 558 10.2.1 GPIO Ports 559 10.2.2 GPIO Read/Write Interface 559 10.2.3 GPIO Input Glitch Filtering and Synchronization 559 10.2.4 GPIO Fast Wake 560 10.2.5 GPIO DMA Interface 561 10.2.6 Event Publishers and Subscribers 561 10.3 GPIO Registers 562

## 11 ADC 670 (p. 6)

11.1 ADC Overview 671 11.2 ADC Operation 672 11.2.1 ADC Core 672 11.2.2 Voltage Reference Options 673 11.2.3 Generic Resolution Modes 673 11.2.4 Hardware Averaging 673 11.2.5 ADC Clocking 674 11.2.6 Common ADC Use Cases 675 11.2.7 Power Down Behavior 676 11.2.8 Sampling Trigger Sources and Sampling Modes 676 11.2.9 Sampling Period 678 11.2.10 Conversion Modes 679 11.2.11 Data Format 680 11.2.12 Advanced Features 680 11.2.13 Status Register 684 11.2.14 ADC Events 684 11.3 ADC0 Registers 687

## 12 COMP 733 (p. 6)

12.1 Comparator Overview 734 12.2 Comparator Operation 735 12.2.1 Comparator Configuration 735 12.2.2 Comparator Channels Selection 735 12.2.3 Comparator Output 736 12.2.4 Output Filter 736 12.2.5 Sampled Output Mode 737 12.2.6 Blanking Mode 737 12.2.7 Reference Voltage Generator 737 12.2.8 Comparator Hysteresis 739 12.2.9 Input SHORT Switch 739 12.2.10 Interrupt and Events Support 740 12.3 COMP0 Registers 742

## 13 VREF 771 (p. 6)

13.1 VREF Overview 772 13.2 VREF Operation 773 13.2.1 Internal Reference Generation 774 13.2.2 External Reference Input 774 13.2.3 Analog Peripheral Interface 774 13.3 VREF Registers 775

## 14 UART 784 (p. 6)

14.1 UART Overview 785 14.1.1 Purpose of the Peripheral 785 14.1.2 Features 785 14.1.3 Functional Block Diagram 786 14.2 UART Operation 786 14.2.1 Clock Control 786 14.2.2 Signal Descriptions 787 14.2.3 General Architecture and Protocol 787 14.2.4 Low Power Operation 801 14.2.5 Reset Considerations 801 14.2.6 Initialization 802

14.3 UART Registers 805

## 15 SPI 862 (p. 7)

15.1 SPI Overview 863 15.1.1 Purpose of the Peripheral 863 15.1.2 Features 863 15.1.3 Functional Block Diagram 864 15.1.4 External Connections and Signal Descriptions 864 15.2 SPI Operation 866 15.2.1 Clock Control 866 15.2.2 General Architecture 866 15.2.3 Protocol Descriptions 871 15.2.4 Reset Considerations 876 15.2.5 Initialization 876 15.2.6 Interrupt and Events Support 876 15.2.7 Emulation Modes 878 15.32 SPI Registers 879

## 16 I C 922 (p. 7)

16.1 I2C Overview 923 16.1.1 Purpose of the Peripheral 923 16.1.2 Features 923 16.1.3 Functional Block Diagram 924 16.1.4 Environment and External Connections 924 16.2 I2C Operation 925 16.2.1 Clock Control 925 16.2.2 Signal Descriptions 926 16.2.3 General Architecture 926 16.2.4 Protocol Descriptions 934 16.2.5 Reset Considerations 945 16.2.6 Initialization 946 16.2.7 Interrupt and Events Support 946 16.2.8 Emulation Modes 948 16.3 I2C Registers 949

## 17 CRC 1013 (p. 7)

17.1 CRC Overview 1014 17.1.1 CRC16-CCITT 1014 17.2 CRC Operation 1014 17.2.1 CRC Generator Implementation 1015 17.2.2 Configuration 1015 17.3 CRCP0 Registers 1017

## 18 Timers (TIMx) 1030 (p. 7)

18.1 TIMx Overview 1031 18.1.1 TIMG Overview 1031 18.1.2 TIMA Overview 1032 18.1.3 TIMx Instance Configuration 1034 18.2 TIMx Operation 1034 18.2.1 Timer Counter 1035 18.2.2 Counting Mode Control 1037 18.2.3 Capture/Compare Module 1043 18.2.4 Shadow Load and Shadow Compare 1057 18.2.5 Output Generator 1059 18.2.6 Fault Handler (TIMA only) 1069 18.2.7 Synchronization With Cross Trigger 1075 18.2.8 Low Power Operation 1077 18.2.9 Interrupt and Event Support 1078 18.2.10 Debug Handler (TIMA Only) 1081 18.3 TIMx Registers 1082

## 19 RTC 1176 (p. 7)

19.1 Overview 1177 19.1.1 RTC Instances 1177

19.3.1 Clocking 1180 19.3.2 Reading and Writing to RTC Peripheral Registers 1180 19.3.3 Binary vs. BCD 1181 19.3.4 Leap Year Handling 1181 19.3.5 Calendar Alarm Configuration 1181 19.3.6 Interval Alarm Configuration 1182 19.3.7 Periodic Alarm Configuration 1182 19.3.8 Calibration 1183 19.3.9 RTC Prescaler Extension 1186 19.3.10 RTC Timestamp Capture 1187 19.3.11 RTC Events 1189 19.4 RTC Registers 1191

## 20 IWDT 1258 (p. 8)

20.2 IWDT Clock Configuration 1259 20.3 IWDT Period Selection 1259 20.4 Debug Behavior of the IWDT 1259 20.5 IWDT Registers 1259

## 21 Window Watchdog Timer (WWDT) 1260 (p. 8)

21.1 WWDT Overview 1261 21.1.1 Watchdog Mode 1261 21.1.2 Interval Timer Mode 1262 21.2 WWDT Operation 1262 21.2.1 Mode Selection 1262 21.2.2 Clock Configuration 1263 21.2.3 Low-Power Mode Behavior 1264 21.2.4 Debug Behavior 1264 21.2.5 WWDT Events 1265 21.3 WWDT Registers 1266

## 22 Debug 1284 (p. 8)

22.1 DEBUGSS Overview 1285 22.1.1 Debug Interconnect 1285 22.1.2 Physical Interface 1286 22.1.3 Debug Access Ports 1287 22.2 DEBUGSS Operation 1288 22.2.1 Debug Features 1288 22.2.2 Behavior in Low Power Modes 1289 22.2.3 Restricting Debug Access 1290 22.2.4 Mailbox (DSSM) 1292 22.3 DEBUGSS Registers 1295

## 23 Revision History 1311 (p. 8)

Preface
