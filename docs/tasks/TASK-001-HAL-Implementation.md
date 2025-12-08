# Task #1: Complete HAL Peripheral Implementations

**Status**: 🔴 Not Started  
**Assigned to**: `@hal-peripherals-developer`  
**Priority**: CRITICAL (blocking all other work)  
**Dependencies**: None  
**Estimated time**: 10-14 days  
**Started**: TBD  
**Completed**: TBD

## Description

Implement the HAL peripheral wrappers to provide safe abstractions for all ESP32-S3 peripherals used on the Waveshare ESP32-S3-Touch-LCD-2.8 board.

## Files to Implement

- [ ] `rust/hal/src/peripherals/i2c.rs` - I2C peripheral wrapper
- [ ] `rust/hal/src/peripherals/spi.rs` - SPI peripheral wrapper  
- [ ] `rust/hal/src/peripherals/uart.rs` - UART peripheral wrapper
- [ ] `rust/hal/src/peripherals/gpio.rs` - GPIO utilities and interrupts
- [ ] `rust/hal/src/board.rs` - Complete board initialization

## Requirements

### I2C Peripheral
1. Safe wrapper around `esp-hal` I2C
2. Expose `embedded-hal` I2C traits
3. Configure at 100kHz for touch/IMU/RTC communication
4. Support 7-bit and 10-bit addressing
5. Error handling and timeout support
6. Document usage examples

### SPI Peripheral
1. Safe wrapper around `esp-hal` SPI
2. Expose `embedded-hal` SPI traits
3. Support 40MHz operation for display
4. DMA support for efficient transfers
5. Multiple chip select support
6. Configurable SPI modes (0-3)

### UART Peripheral
1. Safe wrapper around `esp-hal` UART
2. Expose `embedded-hal` serial traits
3. Support all standard baud rates (9600-921600+)
4. Configurable parity (none, even, odd)
5. Configurable stop bits (1, 1.5, 2)
6. RX/TX buffer management

### GPIO Peripheral
1. Safe GPIO abstractions
2. PWM support for backlight control (0-100%)
3. Interrupt support for touch INT pin
4. Pull-up/pull-down configuration
5. Pin mode switching (input/output/alternate)

### Board Initialization
1. System clock configuration
2. Initialize all required peripherals
3. Configure pin mappings per Waveshare specs
4. Provide delay provider
5. Safe singleton access to peripherals

## Pin Mappings (Waveshare Board)

### Display (ST7789 via SPI)
- MOSI: GPIO45
- SCLK: GPIO40
- CS: GPIO42
- DC: GPIO41
- RST: GPIO39
- BL (backlight): GPIO5

### Touch Controller (CST328 via I2C)
- SDA: GPIO1
- SCL: GPIO3
- INT: GPIO4
- RST: GPIO2

### SD Card (via SPI)
- MISO: GPIO16
- MOSI: GPIO17
- SCLK: GPIO14
- CS: GPIO21

### USB
- D+: GPIO19
- D-: GPIO20

## Success Criteria

- ✅ All modules compile without warnings
- ✅ `cargo clippy` passes with no warnings
- ✅ Pin mappings match Waveshare documentation
- ✅ Basic smoke test works (LED blink, UART echo)
- ✅ Unit tests pass for each peripheral
- ✅ Documentation is complete with examples

## Reference Materials

### C Code References
- `lib/TFT_eSPI/Processors/TFT_eSPI_ESP32_S3.c`
- `lib/TFT_eSPI/Processors/TFT_eSPI_ESP32_S3.h`

### Hardware Documentation
- [Waveshare Wiki](https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-2.8)
- [ESP32-S3 TRM](https://www.espressif.com/sites/default/files/documentation/esp32-s3_technical_reference_manual_en.pdf)

### Software Documentation
- [esp-hal Documentation](https://docs.rs/esp-hal/latest/esp_hal/)
- [embedded-hal Traits](https://docs.rs/embedded-hal/latest/embedded_hal/)

## Agent: Start Work

When ready to begin, mention `@hal-peripherals-developer` in a comment to this file.
