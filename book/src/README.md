# ESP32 Bus Pirate Documentation

Welcome to the comprehensive documentation for the **ESP32 Bus Pirate** project - a powerful, open-source multi-protocol debugging and hacking tool based on the ESP32-S3 microcontroller.

## What is ESP32 Bus Pirate?

ESP32 Bus Pirate is a versatile hardware tool that supports 20+ communication protocols, inspired by the [legendary Bus Pirate](https://buspirate.com/). It transforms your Waveshare ESP32-S3-Touch-LCD-2.8 board into a universal interface for:

- **Digital Protocols**: I²C, SPI, UART, 1-Wire, 2-Wire, 3-Wire
- **Wireless**: Bluetooth, Wi-Fi, Sub-GHz, RFID, RF24
- **Other Interfaces**: JTAG, CAN, USB, Infrared, I²S Audio

## Key Features

### 🎨 Touch Interface
- 2.8" capacitive touchscreen (240×320)
- Intuitive GUI built with Slint framework
- Real-time visualization of bus traffic
- Configuration without connecting to a computer

### 🔋 Portable Operation
- Native LiPo battery support
- Save measurements to SD card
- Export data for later analysis
- Mobile app for on-the-go control

### 📱 Mobile Companion
- Flutter/Tauri mobile application
- Control device wirelessly
- View saved measurements
- Real-time monitoring

### 🔧 Protocol Support
- 20+ bus protocols
- Sniffing and analysis capabilities
- EEPROM/Flash dump tools
- Automated device identification

### 🏗️ Modular Design
- M5Stack-style magnetic pogo connectors (planned)
- Easy expansion with add-on modules
- Clean cable management
- Hot-swappable peripherals

## Quick Links

- [Quick Start Guide](./getting-started/quick-start.md) - Get up and running in minutes
- [Hardware Setup](./hardware/waveshare-s3.md) - Board specifications and connections
- [Bus Modes](./modes/i2c.md) - Learn about different protocol modes
- [Contributing](./development/contributing.md) - Help improve the project

## Project Status

This project is undergoing a **comprehensive migration from C/ESP-IDF to Rust** (no_std), bringing:

- ✅ Memory safety without garbage collection
- ✅ Zero-cost abstractions
- ✅ Better tooling and documentation
- ✅ Modern async/await support
- ✅ Cross-platform development

See the [Rust Migration](./development/rust-migration.md) section for details.

## Getting Help

- **Issues**: [GitHub Issues](https://github.com/x0f5c3/esp32-bus-pirate/issues)
- **Discussions**: [GitHub Discussions](https://github.com/x0f5c3/esp32-bus-pirate/discussions)
- **Wiki**: [Project Wiki](https://github.com/x0f5c3/esp32-bus-pirate/wiki)

## License

This project is licensed under MIT OR Apache-2.0 - see the [LICENSE](https://github.com/x0f5c3/esp32-bus-pirate/blob/main/LICENSE) file for details.

## Acknowledgments

- Original [Bus Pirate](https://buspirate.com/) by Dangerous Prototypes
- [ESP32 Bus Pirate (C version)](https://github.com/geo-tp/ESP32-Bus-Pirate) by geo-tp
- [Waveshare](https://www.waveshare.com/) for the ESP32-S3-Touch-LCD-2.8 board
- The Rust embedded community

---

**Let's get started!** → [Quick Start Guide](./getting-started/quick-start.md)
