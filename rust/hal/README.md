# ESP32 Bus Pirate HAL

Hardware Abstraction Layer for the Waveshare ESP32-S3-Touch-LCD-2.8 board.

## Overview

This crate provides safe, `no_std` abstractions over the ESP32-S3 peripherals, implementing standard `embedded-hal` traits for portability.

## Features

- **I2C**: 100kHz standard mode for touch controller, IMU, and RTC
- **SPI**: Dual SPI buses (SPI2 @ 40MHz for display, SPI3 @ 20MHz for SD card)
- **UART**: Configurable UART0 and UART1 with all standard baud rates
- **GPIO**: PWM support for backlight, interrupt configuration for touch
- **Board**: Complete board initialization with all peripherals configured

## Hardware Pin Mappings

### Display (ST7789 via SPI2)
- MOSI: GPIO45
- SCLK: GPIO40
- CS: GPIO42
- DC: GPIO41
- RST: GPIO39
- Backlight: GPIO5

### Touch Controller (CST328 via I2C0)
- SDA: GPIO1
- SCL: GPIO3
- INT: GPIO4
- RST: GPIO2
- I2C Address: 0x5A

### IMU (QMI8658C via I2C0)
- SDA: GPIO1 (shared with touch)
- SCL: GPIO3 (shared with touch)
- I2C Address: 0x6B

### RTC (PCF85063 via I2C0)
- SDA: GPIO1 (shared with touch)
- SCL: GPIO3 (shared with touch)
- I2C Address: 0x51

### SD Card (via SPI3)
- MISO: GPIO16 (SD_D0)
- MOSI: GPIO17 (SD_CMD)
- SCLK: GPIO14 (SD_SCK)
- CS: GPIO21 (SD_D3)

### USB
- D+: GPIO20
- D-: GPIO19

## Usage

### Basic Board Initialization

```rust
use esp32_bus_pirate_hal::WaveshareS3Board;

// Initialize the board with default configuration
let mut board = WaveshareS3Board::new();

// Initialize display
board.init_display();

// Initialize touch controller
board.init_touch();

// Turn on backlight
board.set_backlight(true);
```

### I2C Example

```rust
use esp32_bus_pirate_hal::peripherals::i2c::{I2cBus, I2cConfig, I2cExt};
use embedded_hal::i2c::I2c;

// Board provides i2c0 already configured
let mut i2c = board.i2c0;

// Scan for I2C devices
let devices = i2c.scan();
for addr in devices {
    println!("Found device at 0x{:02X}", addr);
}

// Read from a register
let value = i2c.read_register(0x5A, 0x00)?;

// Write to a register
i2c.write_register(0x5A, 0x01, 0xFF)?;
```

### SPI Example

```rust
use embedded_hal::spi::SpiBus;

// Board provides display_spi already configured at 40MHz
let mut spi = board.display_spi;
let mut cs = board.display_cs;

// Perform SPI transfer
cs.set_low();
spi.write(&[0x2A, 0x00, 0x00, 0x00, 0xEF])?;
cs.set_high();
```

### GPIO and PWM Example

```rust
use esp32_bus_pirate_hal::peripherals::gpio::{PwmChannel, PwmConfig};

// For simple on/off control
board.display_bl.set_high(); // Backlight on
board.display_bl.set_low();  // Backlight off

// For PWM brightness control, you would need to set up LEDC:
// (This is more advanced and requires additional setup)
let config = PwmConfig::default()
    .with_frequency(5000)     // 5kHz
    .with_duty_percent(75);   // 75% brightness
```

### UART Example

```rust
use esp32_bus_pirate_hal::peripherals::uart::{UartBus0, UartConfig};
use embedded_io::{Read, Write};

// UART configuration
let config = UartConfig::default()
    .with_baudrate(115200)
    .with_parity(Parity::None);

// Note: You would need to initialize UART separately as it's not
// included in the default board struct to save resources.
```

## API Documentation

### I2C Peripheral (`peripherals::i2c`)

- `I2cBus`: Main I2C wrapper implementing `embedded_hal::i2c::I2c`
- `I2cConfig`: Configuration for I2C frequency and timeout
- `I2cExt`: Extension trait with helper methods:
  - `scan()`: Scan for devices on the bus
  - `read_register()`: Read a single register
  - `write_register()`: Write to a single register
  - `read_registers()`: Read multiple registers
  - `write_registers()`: Write to multiple registers

### SPI Peripheral (`peripherals::spi`)

- `SpiBus2` / `SpiBus3`: SPI wrappers implementing `embedded_hal::spi::SpiBus`
- `SpiConfig`: Configuration for frequency, mode, and DMA
- `SpiDeviceWithCs`: SPI device with automatic chip select management
- `SpiMode`: Mode0-Mode3 configuration

### UART Peripheral (`peripherals::uart`)

- `UartBus0` / `UartBus1`: UART wrappers implementing `embedded_io::{Read, Write}`
- `UartConfig`: Configuration for baudrate, parity, stop bits, data bits
- `Parity`: None, Even, Odd
- `StopBits`: One, Two
- `DataBits`: Five, Six, Seven, Eight

### GPIO Utilities (`peripherals::gpio`)

- `PwmConfig`: PWM configuration for frequency and duty cycle
- `PwmChannel`: PWM channel control for backlight brightness
- `GpioUtils`: Static utility functions for GPIO configuration
- `GpioExt`: Extension trait for convenient GPIO operations
- `InterruptMode`: Rising, Falling, AnyEdge, LowLevel, HighLevel

### Board (`board`)

- `WaveshareS3Board`: Main board structure with all peripherals
- `BoardConfig`: Configuration options for board initialization
- Helper methods:
  - `new()`: Initialize board with defaults
  - `init_display()`: Reset sequence for ST7789
  - `init_touch()`: Reset sequence for CST328
  - `set_backlight()`: Simple on/off backlight control
  - `touch_interrupt_active()`: Check touch INT pin state

## Building

This crate requires the Xtensa ESP32-S3 toolchain:

```bash
# Install espup
cargo install espup

# Install the Xtensa toolchain
espup install

# Source the environment
source ~/export-esp.sh

# Build the HAL
cargo build --release
```

## Testing

Unit tests are included but most require hardware to validate:

```bash
# Run tests (requires ESP32-S3 hardware)
cargo test --release
```

## Dependencies

- `esp-hal`: ESP32-S3 hardware abstraction
- `embedded-hal`: Standard embedded traits (v1.0)
- `embedded-io`: Standard I/O traits
- `heapless`: Static data structures for `no_std`
- `fugit`: Time units
- `bitflags`: Bit flag operations
- `log`: Logging facade

## References

- [Waveshare ESP32-S3-Touch-LCD-2.8 Wiki](https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-2.8)
- [ESP32-S3 Technical Reference Manual](https://www.espressif.com/sites/default/files/documentation/esp32-s3_technical_reference_manual_en.pdf)
- [esp-hal Documentation](https://docs.rs/esp-hal/latest/esp_hal/)
- [embedded-hal Documentation](https://docs.rs/embedded-hal/latest/embedded_hal/)

## License

MIT OR Apache-2.0
