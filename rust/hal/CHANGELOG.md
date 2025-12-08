# Changelog

All notable changes to the ESP32 Bus Pirate HAL will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2024-12-08

### Added
- Initial HAL implementation for Waveshare ESP32-S3-Touch-LCD-2.8 board
- I2C peripheral wrapper (`peripherals::i2c`)
  - Support for 100kHz standard mode
  - 7-bit addressing support
  - Extension trait with register operations and bus scanning
  - `embedded-hal` I2C traits implementation
- SPI peripheral wrapper (`peripherals::spi`)
  - Support for SPI2 and SPI3 peripherals
  - Configurable modes (Mode0-Mode3)
  - Frequencies up to 40MHz
  - `SpiDevice` wrapper with automatic chip select management
  - `embedded-hal` SPI traits implementation
- UART peripheral wrapper (`peripherals::uart`)
  - Support for UART0 and UART1
  - Configurable baud rates (9600-921600+)
  - Configurable parity (None, Even, Odd)
  - Configurable stop bits (1, 2)
  - Configurable data bits (5, 6, 7, 8)
  - `embedded-io` Read/Write traits implementation
- GPIO utilities (`peripherals::gpio`)
  - PWM configuration for backlight brightness control
  - Interrupt mode configuration
  - Pull-up/pull-down configuration
  - `GpioExt` extension trait for convenient operations
- Board initialization (`board`)
  - Complete board structure with all peripherals
  - System clock configuration (240 MHz)
  - Display SPI2 @ 40MHz
  - SD card SPI3 @ 20MHz
  - I2C0 @ 100kHz for touch/IMU/RTC
  - Helper methods for display and touch initialization
  - `BoardConfig` for customizable initialization
- Pin definitions (`pins`)
  - Display (ST7789) pins
  - Touch controller (CST328) pins with I2C address
  - IMU (QMI8658C) pins with I2C address
  - RTC (PCF85063) pins with I2C address
  - SD card pins
  - USB pins
  - Available GPIO pins for bus modes
- Comprehensive documentation
  - Module-level documentation with examples
  - README with usage guide
  - API documentation for all public items
  - Hardware pin mapping reference

### Changed
- Fixed Cargo.toml async feature configuration

### Technical Details
- Target: `xtensa-esp32s3-none-elf`
- Build mode: `no_std`
- Dependencies: esp-hal 0.21, embedded-hal 1.0, embedded-io 0.6
- Clock speed: 240 MHz
- All peripherals use standard embedded-hal traits for portability

### Notes
- Code requires Xtensa ESP32-S3 toolchain installed via `espup`
- Unit tests included but require hardware for full validation
- DMA support configured but not yet fully utilized
- 10-bit I2C addressing not implemented (hardware limitation)
