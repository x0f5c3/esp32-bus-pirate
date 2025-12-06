# Agent Coordination Plan for ESP32 Bus Pirate Rust Migration

This document provides specific task assignments for coordinating multiple agents to complete the Rust migration in parallel.

## Overview

The migration is divided into **6 phases** with **10 specialized agents** working in parallel where dependencies allow. Total estimated time: **12-14 weeks** with proper parallelization.

---

## Phase 1: Foundation (Weeks 1-2)

### Agent 1: HAL & Peripherals Developer

**Task**: Complete Hardware Abstraction Layer  
**Priority**: CRITICAL (blocks all other work)  
**Files to create/modify**:
- `rust/hal/src/peripherals/i2c.rs`
- `rust/hal/src/peripherals/spi.rs`
- `rust/hal/src/peripherals/uart.rs`
- `rust/hal/src/peripherals/gpio.rs`
- `rust/hal/src/board.rs` (complete initialization)

**Reference C files**:
- `lib/TFT_eSPI/Processors/TFT_eSPI_ESP32_S3.*`

**Deliverables**:
1. Safe I2C abstraction with error handling
2. SPI with DMA support at 40MHz for display
3. UART with all baud rates and configurations
4. GPIO with PWM, interrupts, pull-up/down
5. Board initialization that sets up all peripherals
6. Unit tests for each peripheral

**Success criteria**:
- All HAL modules compile without warnings
- Pin mappings verified against Waveshare docs
- Basic smoke tests pass (LED blink, UART echo)

**Estimated time**: 10-14 days

---

### Agent 2: Display & Touch Developer

**Task**: Integrate ST7789 and CST328 drivers  
**Priority**: HIGH (needed for GUI)  
**Dependencies**: Agent 1 (HAL) must complete SPI and I2C first

**Files to create/modify**:
- `rust/drivers/src/display/st7789.rs`
- `rust/drivers/src/display/mod.rs`
- `rust/drivers/src/touch/cst328.rs`
- `rust/drivers/src/touch/mod.rs`

**Deliverables**:
1. ST7789 driver integrated with `embedded-graphics`
2. Display initialization (240×320, portrait mode)
3. Backlight PWM control (0-100%)
4. Framebuffer in PSRAM (if available)
5. Double buffering implementation
6. CST328 I2C driver with interrupt support
7. Touch event detection (press, release, move)
8. Multi-touch support (up to 5 points)
9. Demo app showing "Hello World" and touch points

**Success criteria**:
- Display shows clear text without artifacts
- Touch is accurate across entire screen
- No ghost touches or missed events
- Frame rate >10 FPS for GUI updates

**Estimated time**: 10-14 days

---

### Agent 3: Protocol & USB Developer

**Task**: Test protocol and add USB CDC transport  
**Priority**: HIGH (needed for host communication)  
**Dependencies**: Agent 1 (HAL) for USB peripheral

**Files to create/modify**:
- `rust/protocol/tests/codec_tests.rs` (new)
- `rust/protocol/tests/integration_tests.rs` (new)
- `rust/firmware/src/transport/usb_cdc.rs` (new)
- `rust/firmware/src/transport/mod.rs` (new)

**Deliverables**:
1. 50+ unit tests for MessageCodec
2. Integration tests for all message types
3. CRC validation tests with corrupted data
4. Python test client script
5. USB CDC serial implementation
6. Frame transport layer
7. RX/TX circular buffers
8. USB connect/disconnect handling

**Success criteria**:
- All tests pass consistently
- Codec handles malformed data gracefully
- Python client can communicate with device
- USB appears as serial port on all platforms

**Estimated time**: 7-10 days

---

## Phase 2: Core Bus Modes (Weeks 3-6)

### Agent 4: I2C & 1-Wire Specialist

**Task**: Complete I2C and 1-Wire modes  
**Priority**: HIGH (most commonly used modes)  
**Dependencies**: Agent 1 (HAL)

**Files to create/modify**:
- `rust/bus-modes/src/i2c.rs` (expand)
- `rust/bus-modes/src/onewire.rs` (new)
- `rust/bus-modes/src/twowire.rs` (new)
- `rust/bus-modes/src/threewire.rs` (new)

**C reference files**:
- `src/Controllers/I2cController.cpp`
- `src/Services/I2cService.cpp`
- `src/Controllers/OneWireController.cpp`
- `src/Services/OneWireService.cpp`
- `src/Controllers/TwoWireController.cpp`
- `src/Controllers/ThreeWireController.cpp`

**Deliverables**:

**I2C Mode**:
1. Bus scan (7-bit and 10-bit addressing)
2. Register read/write (single and burst)
3. I2C sniffer (passive bus monitoring)
4. Clock stretching support
5. Repeated start capability
6. EEPROM dump utility
7. Common device identification

**1-Wire Mode**:
1. Reset and presence detect
2. ROM search algorithm
3. iButton support (DS1990A)
4. DS18B20 temperature sensor
5. EEPROM operations (DS2431, DS2433)
6. Parasite power mode

**2-Wire & 3-Wire**:
1. Smart card communication (2-wire)
2. Microwire EEPROM (3-wire)

**Success criteria**:
- I2C scan finds all devices
- Can read DS18B20 temperature
- Can read/write i2c EEPROM
- Sniffer captures transactions accurately

**Estimated time**: 14-21 days

---

### Agent 5: SPI & DIO Specialist

**Task**: Complete SPI, DIO, and LED modes  
**Priority**: HIGH (SPI very common, DIO essential)  
**Dependencies**: Agent 1 (HAL)

**Files to create/modify**:
- `rust/bus-modes/src/spi.rs` (expand)
- `rust/bus-modes/src/dio.rs` (new)
- `rust/bus-modes/src/led.rs` (new)

**C reference files**:
- `src/Controllers/SpiController.cpp`
- `src/Services/SpiService.cpp`
- `src/Controllers/DioController.cpp`
- `src/Controllers/LedController.cpp`
- `src/Services/LedService.cpp`

**Deliverables**:

**SPI Mode**:
1. Support all SPI modes (0-3)
2. Full-duplex transfers
3. Flash ID reading (JEDEC)
4. Flash chip erase/program
5. SD card initialization and R/W
6. SPI slave mode
7. SPI EEPROM (25-series) operations

**DIO Mode**:
1. GPIO read/write/toggle
2. Pull-up/pull-down configuration
3. PWM output (frequency, duty cycle)
4. Pulse measurement (duration)
5. Servo control

**LED Mode**:
1. WS2812 (NeoPixel) via RMT peripheral
2. APA102 (DotStar) via RMT
3. LED animations
4. RGB color control

**Success criteria**:
- Can read SPI flash ID
- SD card reads/writes correctly
- GPIO PWM works smoothly
- WS2812 LEDs display correctly

**Estimated time**: 14-21 days

---

### Agent 6: UART & Audio Specialist

**Task**: Complete UART, HD-UART, Infrared, and I2S modes  
**Priority**: MEDIUM-HIGH  
**Dependencies**: Agent 1 (HAL)

**Files to create/modify**:
- `rust/bus-modes/src/uart.rs` (expand)
- `rust/bus-modes/src/hduart.rs` (new)
- `rust/bus-modes/src/infrared.rs` (new)
- `rust/bus-modes/src/i2s.rs` (new)
- `rust/drivers/src/audio/mod.rs` (expand)

**C reference files**:
- `src/Controllers/UartController.cpp`
- `src/Services/UartService.cpp`
- `src/Controllers/HdUartController.cpp`
- `src/Services/HdUartService.cpp`
- `src/Controllers/InfraredController.cpp`
- `src/Services/InfraredService.cpp`
- `src/Controllers/I2sController.cpp`
- `src/Services/I2sService.cpp`

**Deliverables**:

**UART Mode**:
1. All baud rates (9600-921600+)
2. Parity (none, even, odd)
3. Stop bits (1, 1.5, 2)
4. UART bridge mode
5. AT command detection
6. Auto baud detection
7. Line ending options

**HD-UART**:
1. Half-duplex UART

**Infrared Mode**:
1. IR transmit via RMT
2. IR receive via RMT
3. 10+ common protocols (NEC, RC5, Sony, etc.)
4. Device-B-Gone functionality

**I2S Mode**:
1. Audio playback (PCM5101A)
2. Audio recording (if mic available)
3. Sample rate configuration

**Success criteria**:
- UART bridge works reliably
- Auto-baud detects common rates
- IR can control TV/AC
- Audio playback is clear

**Estimated time**: 14-21 days

---

### Agent 7: Advanced Protocols Specialist

**Task**: Complete CAN and JTAG modes  
**Priority**: MEDIUM  
**Dependencies**: Agent 1 (HAL)

**Files to create/modify**:
- `rust/bus-modes/src/can.rs` (new)
- `rust/bus-modes/src/jtag.rs` (new)

**C reference files**:
- `src/Controllers/CanController.cpp`
- `src/Services/CanService.cpp`
- `src/Controllers/JtagController.cpp`
- `src/Services/JtagService.cpp`

**Deliverables**:

**CAN Mode**:
1. CAN frame send/receive
2. CAN bus sniffer
3. Filter configuration
4. Baud rate configuration

**JTAG Mode**:
1. JTAG pin scanning
2. SWD support
3. Boundary scan
4. Device ID reading

**Success criteria**:
- CAN sniffer captures automotive frames
- JTAG can identify ARM chips

**Estimated time**: 14-21 days

---

## Phase 3: Wireless & Advanced (Weeks 7-10)

### Agent 8: Wireless Communications Specialist

**Task**: Complete Bluetooth, Wi-Fi, and Ethernet modes  
**Priority**: MEDIUM (optional features)  
**Dependencies**: Agent 1 (HAL)

**Files to create/modify**:
- `rust/bus-modes/src/bluetooth.rs` (new)
- `rust/bus-modes/src/wifi.rs` (new)
- `rust/bus-modes/src/ethernet.rs` (new)
- `rust/firmware/src/network/` (new module)

**C reference files**:
- `src/Controllers/BluetoothController.cpp`
- `src/Services/BluetoothService.cpp`
- `src/Controllers/WifiController.cpp`
- `src/Services/WifiService.cpp`
- `src/Controllers/EthernetController.cpp`
- `src/Services/EthernetService.cpp`
- `src/Servers/*.cpp`

**Deliverables**:

**Bluetooth Mode**:
1. BLE scan
2. BLE HID (keyboard, mouse)
3. BLE sniffing

**Wi-Fi Mode**:
1. AP scan
2. Packet sniffing
3. Network scanning tools

**Ethernet**:
1. Ethernet initialization
2. Packet send/receive

**Success criteria**:
- BLE can scan nearby devices
- Wi-Fi can scan networks
- Ethernet link established

**Estimated time**: 21-28 days

---

### Agent 9: Radio & RFID Specialist

**Task**: Complete SubGHz, RFID, RF24, and USB modes  
**Priority**: LOW-MEDIUM (specialized features)  
**Dependencies**: Agent 1 (HAL)

**Files to create/modify**:
- `rust/bus-modes/src/subghz.rs` (new)
- `rust/bus-modes/src/rfid.rs` (new)
- `rust/bus-modes/src/rf24.rs` (new)
- `rust/bus-modes/src/usb.rs` (new)

**C reference files**:
- `src/Controllers/SubGhzController.cpp`
- `src/Services/SubGhzService.cpp`
- `src/Controllers/RfidController.cpp`
- `src/Services/RfidService.cpp`
- `src/Controllers/Rf24Controller.cpp`
- `src/Services/Rf24Service.cpp`
- `src/Controllers/UsbS3Controller.cpp`

**Deliverables**:

**SubGHz Mode** (requires external CC1101):
1. Frequency scanning
2. Signal analysis
3. Replay functionality

**RFID Mode** (requires external PN532):
1. NFC/RFID card reading
2. Card emulation
3. Card cloning

**RF24 Mode** (requires external NRF24L01+):
1. Channel scanning
2. Packet sniffing

**USB Mode**:
1. USB HID emulation (keyboard, mouse)
2. USB storage mode

**Success criteria**:
- CC1101 can scan 433MHz/868MHz/915MHz
- PN532 can read Mifare cards
- RF24 can sniff packets
- USB HID works as keyboard

**Estimated time**: 21-28 days

---

## Phase 4: Application Layer (Weeks 7-9)

### Agent 10: GUI & CLI Developer

**Task**: Implement GUI and CLI  
**Priority**: HIGH (user interface)  
**Dependencies**: Agent 2 (Display & Touch)

**Files to create**:
- `rust/firmware/src/gui/` (new module)
  - `main_menu.rs`
  - `mode_screen.rs`
  - `keyboard.rs`
  - `theme.rs`
- `rust/firmware/src/cli/` (new module)
  - `parser.rs`
  - `commands.rs`
  - `history.rs`
  - `help.rs`
- `rust/firmware/src/main.rs` (event loop)

**C reference files**:
- `src/Dispatchers/ActionDispatcher.cpp`
- `src/Transformers/TerminalCommandTransformer.cpp`
- `src/Transformers/InstructionTransformer.cpp`
- `src/Views/*.cpp`
- `src/Inputs/*.cpp`

**Deliverables**:

**GUI**:
1. Main menu with mode selection
2. Mode status screens
3. Touch-based navigation
4. Virtual keyboard
5. Logic analyzer viewer
6. Pin configuration UI
7. Settings screens

**CLI**:
1. Command parser (tokenizer)
2. Bus Pirate syntax (`[`, `]`, `r`, `w`, etc.)
3. Help system
4. Command history (up/down arrows)
5. Tab completion
6. Script execution

**Main Application**:
1. Event loop
2. Mode switching logic
3. Input routing (touch vs serial)
4. State management

**Success criteria**:
- GUI is responsive (<50ms touch latency)
- CLI accepts all Bus Pirate commands
- Mode switching works seamlessly
- Help is comprehensive

**Estimated time**: 21-28 days

---

## Phase 5: Advanced Features (Weeks 11-12)

### All Agents (parallel tasks)

**File System** (Agent 10):
- `rust/firmware/src/fs/` module
- LittleFS for internal flash
- SD card FAT32 support
- **Time**: 7 days

**Network Services** (Agent 8):
- HTTP server
- WebSocket server
- Wi-Fi AP/STA modes
- **Time**: 14 days

**Additional Drivers** (Agent 2):
- QMI8658 IMU driver
- PCF8563 RTC driver
- PCM5101A audio improvements
- **Time**: 7 days

**Scripting Engine** (Agent 10):
- Bytecode interpreter
- Script file execution
- **Time**: 7 days

**Utility Mode** (Agent 5):
- System utilities
- Voltage/frequency measurement
- **Time**: 7 days

---

## Phase 6: Testing & Documentation (Weeks 13-14)

### All Agents (contribute to their domains)

**Testing**:
- Unit tests (80% coverage target)
- Integration tests
- Hardware-in-the-loop tests
- Stress testing
- **Time**: 7 days

**Optimization**:
- Binary size reduction
- Performance profiling
- Memory optimization
- **Time**: 7 days

**Documentation**:
- API documentation (rustdoc)
- User manual
- Developer guide
- Examples and tutorials
- **Time**: 7 days (ongoing)

---

## Coordination Mechanism

### Daily Standups (Async)
Each agent reports via GitHub issues:
- What was completed yesterday
- What's being worked on today
- Any blockers

### Weekly Sync
- Review progress against plan
- Adjust timeline if needed
- Resolve integration issues

### Integration Points
- **Week 2**: HAL, Display, Touch integration test
- **Week 4**: Core modes with CLI test
- **Week 7**: GUI with core modes
- **Week 10**: All modes integrated
- **Week 12**: Feature freeze
- **Week 14**: Release candidate

---

## Risk Management

### High Risk Items
1. **ESP toolchain installation** - Currently blocked by GitHub rate limit
   - Mitigation: Retry later, use alternative download source
2. **Hardware availability** - Agents may not have physical board
   - Mitigation: Simulator/mock testing, delegate to those with hardware
3. **Dependency on HAL** - Blocks most work
   - Mitigation: Prioritize Agent 1, consider mock HAL for early testing

### Medium Risk Items
1. **PSRAM availability** - May not be usable for framebuffer
   - Mitigation: Single buffer fallback
2. **RMT peripheral complexity** - LED/IR modes may be challenging
   - Mitigation: Allocate extra time, consult esp-rs examples
3. **Wireless integration** - esp-wifi crate may have issues
   - Mitigation: Mark as optional, implement last

---

## Success Metrics

### Code Quality
- ✅ Passes `cargo clippy` with no warnings
- ✅ Passes `cargo fmt --check`
- ✅ All tests pass
- ✅ No `unsafe` without justification and documentation

### Functionality
- ✅ All 20+ bus modes working
- ✅ GUI responsive and intuitive
- ✅ CLI compatible with Bus Pirate syntax
- ✅ USB serial communication reliable

### Performance
- ✅ Binary size <1MB
- ✅ GUI frame rate >10 FPS
- ✅ SPI/I2C speeds match or exceed C version
- ✅ Startup time <2 seconds

### Documentation
- ✅ All public APIs documented
- ✅ User manual complete
- ✅ Examples for each mode

---

**Last Updated**: 2025-12-06  
**Maintainer**: Coordinator-Planner Agent
