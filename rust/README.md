# ESP32 Bus Pirate - Rust Implementation

This directory contains the Rust implementation of the ESP32 Bus Pirate firmware, targeting the **Waveshare ESP32-S3-Touch-LCD-2.8** board with a `no_std` environment.

## Project Structure

```
rust/
├── Cargo.toml                  # Workspace manifest
├── .cargo/config.toml         # Build configuration
├── rust-toolchain.toml        # Rust toolchain specification
├── hal/                       # Hardware Abstraction Layer
├── drivers/                   # Device drivers (display, touch, etc.)
├── protocol/                  # Binary protocol implementation
├── bus-modes/                 # Bus Pirate mode implementations
└── firmware/                  # Main application binary
```

## Prerequisites

### 1. Install Rust (Nightly)

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustup install nightly
```

### 2. Install Xtensa Toolchain

```bash
cargo install espup
espup install
source ~/export-esp.sh
```

### 3. Install espflash

```bash
cargo install espflash
```

## Building

Build the firmware:

```bash
cd firmware
cargo build --release
```

## Flashing

Flash to the ESP32-S3 board:

```bash
cd firmware
cargo run --release
```

Or manually:

```bash
espflash flash target/xtensa-esp32s3-none-elf/release/esp32-bus-pirate --monitor
```

## Development

### Crate Overview

#### `esp32-bus-pirate-hal`
Hardware abstraction layer for the Waveshare board. Provides:
- Board initialization
- Pin mappings
- Peripheral configuration

#### `esp32-bus-pirate-drivers`
Device drivers for on-board peripherals:
- ST7789 display controller
- CST328 touch controller
- QMI8658C IMU
- PCF85063 RTC
- PCM5101A audio codec

#### `esp32-bus-pirate-protocol`
Binary protocol for CLI communication:
- Message definitions (I2C, SPI, UART, etc.)
- Postcard-based serialization
- CRC validation
- Framing logic

#### `esp32-bus-pirate-bus-modes`
Bus Pirate mode implementations:
- I2C (scan, read, write)
- SPI (transfer, flash operations)
- UART (read, write, bridge)
- More modes to be added

#### `esp32-bus-pirate-firmware`
Main application:
- Event loop
- Mode management
- GUI rendering
- Command processing

### Testing

Run unit tests:

```bash
cargo test --lib
```

Note: Hardware-dependent code cannot be tested on the host. Use `defmt-test` for on-device testing.

### Code Quality

Run clippy:

```bash
cargo clippy --all-targets -- -D warnings
```

Format code:

```bash
cargo fmt --all
```

Check for issues:

```bash
cargo check --all-targets
```

## Hardware Pin Mappings

See `hal/src/pins.rs` for complete pin definitions.

### Display (ST7789)
- MOSI: GPIO45
- SCLK: GPIO40
- CS: GPIO42
- DC: GPIO41
- RESET: GPIO39
- Backlight: GPIO5

### Touch (CST328)
- SDA: GPIO1
- SCL: GPIO3
- INT: GPIO4
- RST: GPIO2

## Features

### Implemented
- [x] HAL initialization
- [x] Board pin mappings
- [x] Protocol message definitions
- [x] Protocol codec (encode/decode with CRC)
- [x] I2C mode (basic structure)
- [x] SPI mode (basic structure)
- [x] UART mode (basic structure)

### In Progress
- [ ] Display driver integration
- [ ] Touch controller driver
- [ ] GUI framework
- [ ] Command parser
- [ ] Full I2C mode implementation

### Planned
- [ ] All other bus modes
- [ ] File system support
- [ ] Wi-Fi integration
- [ ] USB support
- [ ] Scripting engine

## Documentation

For detailed design and architecture, see the **mdbook documentation**:
- [Full Documentation](../book/book/index.html) - Complete user and developer guide
- [Rust Migration Guide](../book/src/development/rust-migration.md)
- [Design Document](../book/src/development/design.md)
- [Protocol Specification](../book/src/development/protocol.md)
- [Slint GUI Framework](../book/src/development/slint.md)
- [Conventional Commits](../book/src/development/conventional-commits.md)

Or view the original docs:
- [Design Document](../docs/rust-migration-design.md)
- [Protocol Specification](../docs/protocol.md)

## Contributing

Contributions are welcome! Please follow these guidelines:

1. Use `rustfmt` and `clippy` before committing
2. Add tests for new functionality
3. Update documentation as needed
4. Follow Rust API guidelines

## License

MIT OR Apache-2.0 (same as parent project)

## Resources

- [Embedded Rust Book](https://docs.rust-embedded.org/book/)
- [esp-rs Documentation](https://esp-rs.github.io/book/)
- [esp-hal Documentation](https://docs.rs/esp-hal/latest/esp_hal/)
- [Waveshare Board Wiki](https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-2.8)
