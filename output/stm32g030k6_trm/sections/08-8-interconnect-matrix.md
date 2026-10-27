---
document: "STM32G0x0 advanced Arm®-based 32-bit MCUs - Reference manual"
section_id: "08"
section_title: "8 Interconnect matrix"
pages: "208-212"
---

# 8 Interconnect matrix (pp. 208-212)

## 8 Interconnect matrix (p. 208)

### 8.1 Introduction (p. 208)

Several peripherals have dire ct connections between them.

This allows autonomous communication and/or synchronization between peripherals, saving CPU resources thus power consumption.

In addition, these hardware conne ctions remove software latency and allow design of predictable systems.

Depending on peripherals, these interconnections can operate in Run, Sleep, Low-power run, Low-power sleep, Stop 0, and Stop 1 modes.

For availability of peripherals on differen t STM32G0x0 products, refer to Section 1.4: Availability of peripherals.

### 8.2 Connection summary (p. 208)

**Table 32. Interconnect matrix (1)(2) (pp. 208-209)**

| Col 2 | Col 3 | Col 4 | Col 5 | Col 6 | Destination | Destination | Col 9 | Col 10 | Col 11 | Col 12 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Source | TIM1 | TIM3 | TIM4 | TIM14 | TIM15 | TIM16 | TIM17 | ADC | DMAMUX | IRTIM |
| TIM1 TIM3 | -8.3.1 | 8.3.1 8.3.1 - | 8.3.1 8.3.1 - | -- | -8.3.1 | -- | -- | 8.3.2 8.3.2 | -- | -- |
| TIM4 | 8.3.1 | - | - | - | 8.3.1 | - | - | 8.3.2 | - | - |
| TIM14 | - | 8.3.1 8.3.1 | 8.3.1 8.3.1 | - | - | - | - | - | 8.3.8 | - |
| TIM15 |  | 8.3.1 8.3.1 8.3.1 |  | - | - | - | - | 8.3.2 | - | - |
| TIM16 | - | - | - | - | 8.3.1 | - | - | - | - | 8.3.7 |
| TIM17 | 8.3.1 | - | - | - | 8.3.1 | - | - | - | - | 8.3.7 |
| TIM6 | - | - | - | - | - | - | - | 8.3.2 | - | - |
| USART1 | - | - | - | - | - | - | - | - | - | 8.3.7 |
| USART4 | - | - | - | - | - | - | - | - | - | 8.3.7 |
| ADC | 8.3.3 | - | - | - | - | - | - | - | - | - |
| T. sensor | - | - | - | - | - | - | - | 8.3.5 | - | - |
| VBAT | - | - | - | - | - | - | - | 8.3.5 | - | - |
| VREFINT | - | - | - |  | - | - | - | 8.3.5 | - | - |
| HSE | - | - | - | 8.3.4 | - | - | 8.3.4 | - | - | - |
| LSE | - | - | - | - | - | 8.3.4 | - | - | - | - |
| LSI | - | - | - | - | - | 8.3.4 | - | - | - | - |
| Source MCO | TIM1 - | TIM3 - | TIM4 - | TIM14 8.3.4 | TIM15 - | TIM16 - | TIM17 8.3.4 | ADC - | DMAMUX - | IRTIM - |
| MCO2 EXTI | -- | -- | -- | 8.3.4 - | -- | -- | 8.3.4 - | -8.3.2 | -- | -- |
| RTC and TAMP | - | - | - | 8.3.4 | - | 8.3.4 | - | - | - | - |
| SYST ERR |  | 8.3.6 8.3.6 8.3.6 |  | - |  | 8.3.6 8.3.6 8.3.6 |  | - | - | - |

1. Numbers in the table are links to corresponding sub-sections in Section 8.3: Interconnection details. 2. The “-” symbol in grayed ce lls means “no interconnection”.

### 8.3 Interconnection details (p. 209)

#### 8.3.1 From TIM1, TIM3, TIM 4, TIM15, TIM16, and TIM17, (p. 209)

to TIM1, TIM3, TIM4, and TIM15

Purpose

Some of the TIMx timers are linked together internally for time r synchronization or chaining.

When one timer is configured in master mode, it can reset, start, stop or clock the counter of another timer configured in slave mode.

A description of the feature is provided in: Section 16.3.19: Timer synchronization.

The modes of synchronization are detailed in:

- Section 15.3.26: Timer synchronization for advanced-control timer TIM1

- Section 16.3.18: Timers and external trigger synchronization for general-purpose timer TIM3/TIM4

- Section 19.4.19: External trigger synchronization (TIM15 only) for general-purpose timer TIM15

Triggering signals

The output (from master) is on s ignal TIMx_TRGO (and TIMx_TRGOx), following a configurable timer event.

With TIM14, TIM16, and TIM17 ti mers that do not have a trigger output, the output compare 1 is used instead.

The input (to slave) is on signals TIMx_ITR0/ITR1/ITR2/ITR3.

The input and output signals for TIM1 are shown in Figure 55: Advanced-control timer block diagram.

The possible master/slave connections are given in Table 69: TIM1 internal trigger connection.

Relevant power modes

These interconnections operate in Run, Sleep, Low-power run, an d Low-power sleep power modes.

#### 8.3.2 From TIM1, TIM3, TIM4, T IM6, TIM15, and EXTI, to ADC (p. 210)

Purpose

The general-purpose timers TIM3, TIM4, and TIM15, basic timer T IM6, advanced-control timer TIM1, and EXTI can be used to generate an ADC triggering event.

TIMx synchronization is described in: Section 15.3.27: ADC synchronization.

ADC synchronization is described in: Section 14.4: Conversion on external trigger and trigger polarity (EXTSEL, EXTEN).

Triggering signals

The output (from timer) is on si gnal TIMx_TRGO, TIMx_TRGO2 or T IMx_CCx event.

The input (to ADC) is on s ignal EXT[15:0], JEXT[15:0].

The connection between time rs and ADC is provided in Relevant power modes

These interconnections operate in Run, Sleep, Low-power run, an d Low-power sleep power modes.

#### 8.3.3 From ADC to TIM1 (p. 210)

Purpose

ADC can provide trigger event through watchdog signals to the a dvanced-control timer TIM1.

A description of the ADC analog wa tchdog setting is provided in: Section 14.7: Analog window watchdog (AWD1EN, AWD1SGL, AWD1CH, ADC_AWDxCR, ADC_AWDxTR).

Trigger settings on the timer are provided in: Section 15.3.4: External trigger input.

Triggering signals

The output (from ADC) is on signals ADCn_AWDx_OUT n = 1 (for AD C) x = 1, 2, 3 (three watchdogs per ADC) and the input (to timer) on signal TIMx_ETR (external trigger).

Relevant power modes

This interconnection operates in Run, Sleep, Low-power run, and Low-power sleep power modes.

8.3.4 From HSE, LSE, LSI, MCO, MCO2, RTC and TAMP, to TIM14, TIM16, and TIM17

Purpose

External clocks (HSE, LSE), internal clock (LSI), microcontroll er output clock (MCO and MCO2), RTC clock, RTC wakeup in terrupt, and GPIO can be selecte d as inputs to capture channel 1 of some of T IM14/16/TIM17 timers.

The timers allow calibrating or p recisely measuring internal cl ocks such as HSI16 or LSI, using accurate clocks such as LS E or HSE/32 for timing reference. See details in Section 5.2.15: Internal/external clock measurement with TIM14/TIM16/TIM17.

When low-speed exter nal (LSE) oscillator is used, no additional hardware connections are required.

Relevant power modes

These interconnections operate in Run, Sleep, Low-power run, an d Low-power sleep power modes.

8.3.5

From internal an alog sources to ADC

Purpose

Internal temperature sensor output voltage V TS, internal reference voltage V REFINT and V BAT monitoring channel are connected to ADC input channels.

More information is in:

- Section 14.2: ADC main features

- Section 14.3.8: Channel selection (CHSEL, SCANDIR, CHSELRMOD)

- Figure 14.9: Temperature sensor and internal reference voltage

- Figure 14.10: Battery voltage monitoring

Relevant power modes

These interconnections operate in Run, Sleep, Low-power run, an d Low-power sleep power modes.

#### 8.3.6 From system errors to TI M1, TIM3, TIM4, TIM15, TIM16, (p. 211)

and TIM17

Purpose

CSS, CPU hardfault, RAM parity error and FLASH ECC double error detection can generate system errors in the fo rm of timer break toward TIM1, TIM3, TIM 4, TIM15, TIM16, and TIM17.

The purpose of the break function is to protect power switches driven by PWM signals from the timers.

List of possible source of break are described in:

- Section 15.3.16: Using the break function (TIM1)

- Section 19.4.13: Using the break function (TIM15/TIM16/TIM17)

- Figure 190: TIM15 block diagram

- Figure 191: TIM16/TIM17 block diagram

Relevant power modes

These interconnections operate in Run, Sleep, Low-power run, an d Low-power sleep power modes.

#### 8.3.7 From TIM16, TIM17, USART1, and USART4, to IRTIM (p. 212)

Purpose

TIMx_OC1 output channe l of TIM17 timers, a ssociated with USART1 or USART4 transmission signal, ca n generate the infrared output waveform.

The functionality is described in Section 20: Infrared interface (IRTIM).

Relevant power modes

These interconnections operate in Run, Sleep, Low-power run, an d Low-power sleep power modes.

#### 8.3.8 From TIM14 to DMAMUX (p. 212)

Purpose

TIM14 general-purpose timer and EXTI can be used as triggering event to DMAMUX.

Relevant power modes

These interconnections operate in Run, Sleep, Low-power run, an d Low-power sleep power modes.
