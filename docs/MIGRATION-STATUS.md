# ESP32 Bus Pirate Rust Migration - Current Status

**Last Updated**: 2025-12-06  
**Overall Progress**: ~5% complete (HAL: 10%, Drivers: 3%, Protocol: 85%, Bus Modes: 5%, Firmware: 2%)

## Executive Summary

The Rust migration project has completed the planning and scaffolding phase. All crates are created with proper structure, but **implementations are minimal**. This document tracks what remains to be migrated from the C/ESP-IDF codebase.

## Crate Status Overview

### HAL Crate (`rust/hal/`)
**Status**: 📝 Scaffolded (10% complete)
- ✅ Board structure defined
- ✅ Pin mappings documented  
- ❌ Peripheral implementations are empty stubs

**Needs Implementation**:
- [ ] `peripherals/i2c.rs` - I2C wrapper with safe abstractions
- [ ] `peripherals/spi.rs` - SPI wrapper with DMA support
- [ ] `peripherals/uart.rs` - UART wrapper with baud rate config
- [ ] `peripherals/gpio.rs` - GPIO utilities, PWM, interrupts
- [ ] `board.rs` - Complete initialization sequence

**C Files to Port**:
- Reference: `lib/TFT_eSPI/Processors/TFT_eSPI_ESP32_S3.*` for pin configs

---

### Drivers Crate (`rust/drivers/`)
**Status**: 📝 Scaffolded (5% complete)
- ✅ Module structure exists
- ✅ Error types defined
- ❌ All drivers are empty stubs (3-line placeholders)

**Needs Implementation**:

#### Display Driver (`drivers/src/display/`)
- [ ] ST7789 integration with `embedded-graphics`
- [ ] Backlight PWM control
- [ ] Framebuffer in PSRAM
- [ ] Double buffering for smooth updates
- **Estimated LOC**: 200-300

#### Touch Driver (`drivers/src/touch/`)
- [ ] CST328 I2C communication
- [ ] Touch event detection (press, release, move)
- [ ] Multi-touch support (up to 5 points)
- [ ] Interrupt-driven reading
- [ ] Calibration routine
- **Estimated LOC**: 300-400

#### IMU Driver (`drivers/src/imu/`)
- [ ] QMI8658 I2C communication
- [ ] Accelerometer data reading
- [ ] Gyroscope data reading
- [ ] Configuration and calibration
- **Estimated LOC**: 200-300

#### RTC Driver (`drivers/src/rtc/`)
- [ ] PCF8563 I2C communication
- [ ] Time read/write operations
- [ ] Alarm functionality
- **Estimated LOC**: 150-200

#### Audio Driver (`drivers/src/audio/`)
- [ ] PCM5101A I2S setup
- [ ] Audio playback
- [ ] Sample rate configuration
- **Estimated LOC**: 200-300

**C Files to Port**:
- No direct C equivalents (uses Arduino/ESP-IDF libraries)
- Reference Waveshare datasheets and existing Arduino examples

---

### Protocol Crate (`rust/protocol/`)
**Status**: ✅ Complete (95% done)
- ✅ Message types defined
- ✅ Codec with CRC implemented
- ✅ Version management
- ⚠️  Needs comprehensive unit tests

**Needs Implementation**:
- [ ] Add 50+ unit tests for codec
- [ ] Add integration tests
- [ ] Python/JavaScript test clients
- [ ] Fuzz testing
- **Estimated LOC**: 500-1000 (tests)

**C Files to Port**:
- None (new protocol, not in original C code)

---

### Bus Modes Crate (`rust/bus-modes/`)
**Status**: 📝 Scaffolded (15% complete)
- ✅ Trait definitions exist
- ✅ Basic I2C/SPI/UART structures
- ❌ Most functionality is stub/placeholder

**Needs Implementation**:

#### I2C Mode (`bus-modes/src/i2c.rs`)
**C Source**: `src/Controllers/I2cController.cpp`, `src/Services/I2cService.cpp`
- [ ] I2C scan (7-bit and 10-bit addressing)
- [ ] Register read/write
- [ ] Bulk operations
- [ ] I2C sniffer (passive monitoring)
- [ ] Clock stretching support
- [ ] Repeated start
- [ ] EEPROM dump utility
- [ ] Device identification
- **Estimated LOC**: 500-800

#### SPI Mode (`bus-modes/src/spi.rs`)
**C Source**: `src/Controllers/SpiController.cpp`, `src/Services/SpiService.cpp`
- [ ] All SPI modes (0-3)
- [ ] Full-duplex transfer
- [ ] Flash ID reading
- [ ] Flash erase/program
- [ ] SD card operations
- [ ] SPI slave mode
- [ ] EEPROM operations
- **Estimated LOC**: 600-900

#### UART Mode (`bus-modes/src/uart.rs`)
**C Source**: `src/Controllers/UartController.cpp`, `src/Services/UartService.cpp`
- [ ] All baud rates (9600-921600+)
- [ ] Parity options (none, even, odd)
- [ ] Stop bits (1, 1.5, 2)
- [ ] Bridge mode (transparent pass-through)
- [ ] AT command detection
- [ ] Auto baud detection
- [ ] Line ending options
- **Estimated LOC**: 400-600

#### 1-Wire Mode (NEW FILE: `bus-modes/src/onewire.rs`)
**C Source**: `src/Controllers/OneWireController.cpp`, `src/Services/OneWireService.cpp`
- [ ] Reset and presence detect
- [ ] ROM search algorithm
- [ ] Read/write bytes
- [ ] iButton support (DS1990A)
- [ ] Temperature sensor (DS18B20)
- [ ] EEPROM operations
- [ ] Parasite power mode
- **Estimated LOC**: 400-600

#### 2-Wire Mode (NEW FILE: `bus-modes/src/twowire.rs`)
**C Source**: `src/Controllers/TwoWireController.cpp`, `src/Services/TwoWireService.cpp`
- [ ] Smart card communication
- [ ] Raw clock/data control
- **Estimated LOC**: 200-300

#### 3-Wire Mode (NEW FILE: `bus-modes/src/threewire.rs`)
**C Source**: `src/Controllers/ThreeWireController.cpp`, `src/Services/ThreeWireService.cpp`
- [ ] Microwire EEPROM support
- **Estimated LOC**: 200-300

#### DIO Mode (NEW FILE: `bus-modes/src/dio.rs`)
**C Source**: `src/Controllers/DioController.cpp`
- [ ] Raw GPIO control (read, write, toggle)
- [ ] Pull-up/pull-down configuration
- [ ] PWM output
- [ ] Pulse measurement
- [ ] Servo control
- **Estimated LOC**: 300-400

#### LED Mode (NEW FILE: `bus-modes/src/led.rs`)
**C Source**: `src/Controllers/LedController.cpp`, `src/Services/LedService.cpp`
- [ ] WS2812 (NeoPixel) via RMT
- [ ] APA102 (DotStar) via RMT
- [ ] 50+ LED protocols
- [ ] Animations
- **Estimated LOC**: 400-600

#### Infrared Mode (NEW FILE: `bus-modes/src/infrared.rs`)
**C Source**: `src/Controllers/InfraredController.cpp`, `src/Services/InfraredService.cpp`
- [ ] IR transmit via RMT
- [ ] IR receive via RMT
- [ ] 80+ IR protocols (NEC, RC5, Sony, etc.)
- [ ] Device-B-Gone functionality
- [ ] Universal remote
- **Estimated LOC**: 600-1000

#### I2S Mode (NEW FILE: `bus-modes/src/i2s.rs`)
**C Source**: `src/Controllers/I2sController.cpp`, `src/Services/I2sService.cpp`
- [ ] Audio playback
- [ ] Audio recording
- [ ] Speaker testing
- [ ] Microphone testing
- **Estimated LOC**: 300-400

#### CAN Mode (NEW FILE: `bus-modes/src/can.rs`)
**C Source**: `src/Controllers/CanController.cpp`, `src/Services/CanService.cpp`
- [ ] CAN frame send/receive
- [ ] CAN sniffer
- [ ] Filter configuration
- **Estimated LOC**: 300-500

#### JTAG Mode (NEW FILE: `bus-modes/src/jtag.rs`)
**C Source**: `src/Controllers/JtagController.cpp`, `src/Services/JtagService.cpp`
- [ ] JTAG pin scanning
- [ ] SWD support
- [ ] Boundary scan
- **Estimated LOC**: 400-600

#### Bluetooth Mode (NEW FILE: `bus-modes/src/bluetooth.rs`)
**C Source**: `src/Controllers/BluetoothController.cpp`, `src/Services/BluetoothService.cpp`
- [ ] BLE scan
- [ ] BLE HID (keyboard, mouse)
- [ ] BLE spoofing
- [ ] BLE sniffing
- **Estimated LOC**: 600-800

#### Wi-Fi Mode (NEW FILE: `bus-modes/src/wifi.rs`)
**C Source**: `src/Controllers/WifiController.cpp`, `src/Services/WifiService.cpp`
- [ ] AP scan
- [ ] Deauth attack
- [ ] Packet sniffing
- [ ] Nmap-like scanning
- **Estimated LOC**: 600-800

#### Ethernet Mode (NEW FILE: `bus-modes/src/ethernet.rs`)
**C Source**: `src/Controllers/EthernetController.cpp`, `src/Services/EthernetService.cpp`
- [ ] Ethernet initialization
- [ ] Packet send/receive
- **Estimated LOC**: 300-400

#### SubGHz Mode (NEW FILE: `bus-modes/src/subghz.rs`)
**C Source**: `src/Controllers/SubGhzController.cpp`, `src/Services/SubGhzService.cpp`
- [ ] CC1101 communication
- [ ] Frequency scanning
- [ ] Signal analysis
- [ ] Replay attacks
- **Estimated LOC**: 600-800

#### RFID Mode (NEW FILE: `bus-modes/src/rfid.rs`)
**C Source**: `src/Controllers/RfidController.cpp`, `src/Services/RfidService.cpp`
- [ ] PN532 NFC/RFID
- [ ] Card read/write
- [ ] Card cloning
- **Estimated LOC**: 500-700

#### RF24 Mode (NEW FILE: `bus-modes/src/rf24.rs`)
**C Source**: `src/Controllers/Rf24Controller.cpp`, `src/Services/Rf24Service.cpp`
- [ ] NRF24L01+ communication
- [ ] Channel scanning
- [ ] Packet sniffing
- **Estimated LOC**: 400-600

#### USB Mode (NEW FILE: `bus-modes/src/usb.rs`)
**C Source**: `src/Controllers/UsbS3Controller.cpp`, `src/Services/UsbS3Service.cpp`
- [ ] USB HID emulation (keyboard, mouse, gamepad)
- [ ] USB storage
- **Estimated LOC**: 500-700

#### Half-Duplex UART (NEW FILE: `bus-modes/src/hduart.rs`)
**C Source**: `src/Controllers/HdUartController.cpp`, `src/Services/HdUartService.cpp`
- [ ] Half-duplex UART communication
- **Estimated LOC**: 200-300

#### Utility Mode (NEW FILE: `bus-modes/src/utility.rs`)
**C Source**: `src/Controllers/UtilityController.cpp`
- [ ] System utilities
- [ ] Pin voltage reading
- [ ] Frequency measurement
- **Estimated LOC**: 200-300

---

### Firmware Crate (`rust/firmware/`)
**Status**: 📝 Scaffolded (5% complete)
- ✅ Main entry point exists
- ❌ No event loop, GUI, or CLI

**Needs Implementation**:

#### Main Application Loop (`firmware/src/main.rs`)
- [ ] Event loop architecture
- [ ] Mode management/switching
- [ ] Command dispatch
- **Estimated LOC**: 200-400

#### GUI Module (NEW: `firmware/src/gui/`)
- [ ] Main menu
- [ ] Mode selection screen
- [ ] Status displays
- [ ] Touch navigation
- [ ] Virtual keyboard
- [ ] Logic analyzer viewer
- [ ] Pin configuration UI
- **Estimated LOC**: 1500-2500

#### CLI Module (NEW: `firmware/src/cli/`)
- [ ] Command parser (tokenizer, args)
- [ ] Bus Pirate syntax support (`[`, `]`, `r`, `w`, etc.)
- [ ] Help system
- [ ] Command history
- [ ] Tab completion
- [ ] Script execution
- **Estimated LOC**: 800-1200

**C Files to Port**:
- `src/Dispatchers/ActionDispatcher.cpp` - Command dispatch logic
- `src/Transformers/TerminalCommandTransformer.cpp` - Command parsing
- `src/Transformers/InstructionTransformer.cpp` - Bytecode interpretation
- `src/Views/*.cpp` - Display logic (GUI reference)
- `src/Inputs/*.cpp` - Input handling

---

## Additional Components Not Yet Started

### File System Support
**C Files**: `src/Services/LittleFsService.cpp`, `src/Services/SdService.cpp`
- [ ] LittleFS integration for internal flash
- [ ] SD card FAT32 support
- [ ] File read/write/list operations
- [ ] Configuration file loading
- [ ] Script file execution
- **Estimated LOC**: 600-1000

### Network Components (Optional)
**C Files**: `src/Servers/HttpServer.cpp`, `src/Servers/WebSocketServer.cpp`, `src/Services/WifiService.cpp`
- [ ] Wi-Fi AP and STA modes
- [ ] HTTP server for file access
- [ ] WebSocket server for CLI
- [ ] Network configuration UI
- **Estimated LOC**: 1000-1500

### Scripting Engine
**C Files**: `src/Transformers/InstructionTransformer.cpp`, `src/Models/Instruction.h`
- [ ] Bytecode interpreter
- [ ] Script execution from files
- [ ] Python integration (via serial)
- **Estimated LOC**: 400-600

---

## Migration Statistics

### Lines of Code Estimate

| Component | Current | Needed | Total Target |
|-----------|---------|--------|--------------|
| HAL | 240 | 500 | 740 |
| Drivers | 130 | 1200 | 1330 |
| Protocol | 326 | 500 (tests) | 826 |
| Bus Modes | 261 | 9500 | 9761 |
| Firmware | 46 | 3000 | 3046 |
| **Total** | **1003** | **14700** | **15703** |

### C Files to Port

| Category | C Files | Status |
|----------|---------|--------|
| Controllers | 22 files | 0% ported |
| Services | 33 files | 0% ported |
| Dispatchers | 1 file | 0% ported |
| Transformers | 7 files | 0% ported |
| Views | 7 files | 0% ported |
| Inputs | 7 files | 0% ported |
| Shells | 12 files | 0% ported |
| Managers | 4 files | 0% ported |
| **Total** | **93 files** | **~1% ported** |

---

## Implementation Priority Matrix

### Phase 1: Critical Path (Weeks 1-2)
1. **HAL peripherals** - Foundation for everything
2. **Display driver** - Needed for GUI
3. **Touch driver** - Needed for GUI
4. **Protocol tests** - Needed for CLI

### Phase 2: Core Modes (Weeks 3-6)
1. **I2C mode** - Most commonly used
2. **SPI mode** - Very common
3. **UART mode** - Essential for many use cases
4. **1-Wire mode** - DS18B20 is popular
5. **USB CDC** - For host communication

### Phase 3: GUI & CLI (Week 7)
1. **GUI framework** - User interface
2. **CLI parser** - Command-line control
3. **Event loop** - Application glue

### Phase 4: Extended Modes (Weeks 8-10)
1. **DIO mode** - GPIO control
2. **I2S mode** - Audio
3. **LED mode** - Visual feedback
4. **Infrared mode** - Universal remote
5. **CAN mode** - Automotive
6. **JTAG mode** - Debugging

### Phase 5: Advanced Features (Weeks 11-12)
1. **Bluetooth mode** - BLE
2. **Wi-Fi mode** - Network
3. **RFID mode** - NFC
4. **SubGHz mode** - Radio
5. **File system** - Scripts and config

### Phase 6: Refinement (Weeks 13-14)
1. Testing and optimization
2. Documentation
3. Examples and tutorials

---

## Agent Assignment Recommendations

### HAL & Peripherals Agent
- Complete `rust/hal/` crate
- Skills: ESP-IDF, embedded Rust, hardware
- Time: 1-2 weeks

### Display & Touch Agent
- Complete `rust/drivers/src/display/` and `rust/drivers/src/touch/`
- Skills: SPI, I2C, graphics, embedded-graphics
- Time: 1-2 weeks

### Protocol Agent
- Add tests to `rust/protocol/`
- Implement USB CDC
- Skills: Serialization, testing, USB
- Time: 1 week

### Bus Mode Engineers (4-6 agents)
Each takes 3-5 modes based on similarity:
- **Agent A**: I2C, 1-Wire, 2-Wire, 3-Wire
- **Agent B**: SPI, DIO, LED
- **Agent C**: UART, HD-UART, Infrared, I2S
- **Agent D**: CAN, JTAG
- **Agent E**: Bluetooth, Wi-Fi, Ethernet
- **Agent F**: SubGHz, RFID, RF24, USB
- Time: 2-4 weeks each

### GUI Agent
- Complete `rust/firmware/src/gui/`
- Skills: embedded-graphics, UI design
- Time: 2 weeks

### CLI Agent
- Complete `rust/firmware/src/cli/`
- Skills: Parsers, command processing
- Time: 1-2 weeks

### Integration Agent
- Main loop, mode switching, testing
- Skills: System integration, testing
- Time: 2 weeks

---

## Next Steps

1. **Install ESP32 toolchain** (espup) - ⚠️ Blocked by GitHub rate limit
2. **Fix Cargo.toml issues** - ✅ Done (removed optional from workspace defmt)
3. **Verify basic build** - Pending toolchain installation
4. **Create GitHub issues** for each phase
5. **Assign agents** to parallel tasks
6. **Begin Phase 1** implementations

---

## Resources

### Documentation
- [Rust Migration Design](./rust-migration-design.md)
- [Protocol Specification](./protocol.md)
- [Implementation Tasks](./implementation-tasks.md)

### Hardware
- [Waveshare ESP32-S3-Touch-LCD-2.8](https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-2.8)
- [ESP32-S3 Technical Reference](https://www.espressif.com/sites/default/files/documentation/esp32-s3_technical_reference_manual_en.pdf)

### Software
- [esp-hal Documentation](https://docs.rs/esp-hal/latest/esp_hal/)
- [embedded-graphics](https://docs.rs/embedded-graphics/latest/embedded_graphics/)
- [Original C Project](https://github.com/geo-tp/ESP32-Bus-Pirate)

---

**Last Updated**: 2025-12-06  
**Maintainer**: Coordinator-Planner Agent
