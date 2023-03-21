# System Evaluation: bThermo
This branch contains an example of battery-powered thermostat appliance, *bThermo*, which communicates over Bluetooth and MQTT with a server that provides updates to the components and the appliance control commands.

**NOTE: In this branch, HBF is a discontinued alias of CBF. Refer to the main branch for the related documentation.**

The target board is an STMicroelectronics *NUCLEO-L476RG*. Its *STM32L476RG* SoC is intended for the IoT market and is equipped with 1 MiB flash memory, 128 KiB (we will consider only the first 64 KiB of SRAM1) SRAM, dual-bank flash support, and small 2 KiB erase pages.

The idea is to create a thermostat that reads a stream of values from a temperature sensor and sends them back to the server, while manipulating four outputs according to sixteen custom programs. Thus, apart from the above-mentioned SoC, we assembled the hardware of the thermostat using popular low-cost components, such as:
- `TMP117`, an I2C temperature sensor from Texas Instruments. It provides a 16-bit temperature result with a resolution of 0.0078 °C and an accuracy of up to ±0.1 °C.
- `DS3231`, a low-cost, extremely accurate I2C real-time clock with integrated temperature compensation. This device is needed to enable custom programs, as internal kernel ticks are not accurate for measuring time over a long period.
- `HC-06`, a popular simple Bluetooth 2.0 module, as no communication module is provided by this SoC.
- Four `SRD-05VDC-SL-C`, arranged in a relay module, that act as output optoisolators to protect the pins of the microcontroller. Each output can be independently controlled from custom programs.
