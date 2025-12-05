# Implementation Tasks for ESP32 Bus Pirate Rust Migration

This document outlines the tasks that should be delegated to specialized agents for completing the Rust migration.

## Phase 1: Foundation (Weeks 1-2)

### Task 1.1: Complete HAL Implementation
**Agent**: HAL & Peripherals Developer  
**Priority**: High  
**Dependencies**: None

**Description**: Complete the Hardware Abstraction Layer for the Waveshare ESP32-S3-Touch-LCD-2.8 board.

**Deliverables**:
- [ ] Implement `hal/src/peripherals/i2c.rs` with safe I²C abstractions
- [ ] Implement `hal/src/peripherals/spi.rs` with safe SPI abstractions
- [ ] Implement `hal/src/peripherals/uart.rs` with safe UART abstractions
- [ ] Implement `hal/src/peripherals/gpio.rs` with GPIO utilities
- [ ] Add PWM support for display backlight
- [ ] Add DMA support for SPI (if needed for performance)
- [ ] Verify all pin mappings against Waveshare schematic
- [ ] Add unit tests for peripheral configurations

**Acceptance Criteria**:
- All HAL modules compile without errors
- Pin mappings match Waveshare board documentation
- Code passes `cargo clippy` with no warnings
- Basic smoke tests pass on hardware

---

### Task 1.2: ST7789 Display Driver Integration
**Agent**: Display & Touch Developer  
**Priority**: High  
**Dependencies**: Task 1.1 (HAL)

**Description**: Integrate the ST7789 display driver and create a working framebuffer implementation.

**Deliverables**:
- [ ] Complete `drivers/src/display/st7789.rs`
- [ ] Initialize ST7789 with correct parameters (240×320, portrait)
- [ ] Implement `embedded_graphics::DrawTarget` trait
- [ ] Add backlight PWM control (0-100% brightness)
- [ ] Create framebuffer in PSRAM if available
- [ ] Implement double buffering to avoid tearing
- [ ] Add "Hello World" demo that displays text
- [ ] Optimize drawing performance

**Acceptance Criteria**:
- Display shows clear text without artifacts
- Backlight brightness is adjustable
- Drawing operations are reasonably fast (<100ms for full screen clear)
- No memory leaks or buffer overflows

---

### Task 1.3: CST328 Touch Controller Driver
**Agent**: Display & Touch Developer  
**Priority**: High  
**Dependencies**: Task 1.1 (HAL)

**Description**: Implement a complete driver for the CST328 capacitive touch controller.

**Deliverables**:
- [ ] Complete `drivers/src/touch/cst328.rs`
- [ ] Implement I²C initialization sequence
- [ ] Read touch coordinates (X, Y)
- [ ] Detect touch events (press, release, move)
- [ ] Support multi-touch (up to 5 points)
- [ ] Implement interrupt-driven reading (using INT pin)
- [ ] Add calibration routine
- [ ] Add touch event demo

**Acceptance Criteria**:
- Touch coordinates are accurate across the entire screen
- No ghost touches or false positives
- Interrupt latency is low (<10ms)
- Multi-touch detection works correctly

---

## Phase 2: Protocol Layer (Week 3)

### Task 2.1: Protocol Testing and Validation
**Agent**: Protocol & CLI Developer  
**Priority**: High  
**Dependencies**: None (protocol code is already implemented)

**Description**: Test and validate the binary protocol implementation.

**Deliverables**:
- [ ] Add comprehensive unit tests for `MessageCodec`
- [ ] Add unit tests for all `Message` variants
- [ ] Test CRC validation with corrupted frames
- [ ] Test version mismatch handling
- [ ] Create Python test client for protocol validation
- [ ] Test large message handling (up to 1KB)
- [ ] Fuzz test the decoder with random data
- [ ] Document protocol examples

**Acceptance Criteria**:
- All unit tests pass
- Codec handles malformed data gracefully
- Python client can encode/decode messages
- No panics or undefined behavior

---

### Task 2.2: USB CDC Serial Implementation
**Agent**: Protocol & CLI Developer  
**Priority**: High  
**Dependencies**: Task 2.1

**Description**: Implement USB CDC serial communication for protocol transport.

**Deliverables**:
- [ ] Enable USB peripheral in HAL
- [ ] Implement USB CDC class (serial emulation)
- [ ] Create transport layer that reads/writes protocol frames
- [ ] Add circular buffer for RX/TX
- [ ] Handle USB connect/disconnect events
- [ ] Test protocol over USB serial

**Acceptance Criteria**:
- Device enumerates as a USB serial port on PC
- Protocol frames can be sent/received reliably
- No data corruption or dropped frames
- Works on Windows, Linux, and macOS

---

## Phase 3: Core Bus Modes (Weeks 4-6)

### Task 3.1: Complete I²C Mode Implementation
**Agent**: Bus Mode Engineer (I²C Specialist)  
**Priority**: High  
**Dependencies**: Task 1.1, Task 2.2

**Description**: Implement all I²C mode features.

**Deliverables**:
- [ ] Complete `bus-modes/src/i2c.rs`
- [ ] Implement I²C scan with 7-bit and 10-bit addressing
- [ ] Add register read/write operations
- [ ] Implement bulk read/write
- [ ] Add I²C sniffer (passive monitoring)
- [ ] Implement clock stretching support
- [ ] Add repeated start support
- [ ] Create EEPROM dump utility
- [ ] Implement "identify device" feature (read common ID registers)

**Acceptance Criteria**:
- I²C scan finds all devices on the bus
- Can read/write EEPROM successfully
- Sniffer captures I²C transactions
- Works with common I²C devices (RTC, IMU, touch controller)

---

### Task 3.2: Complete SPI Mode Implementation
**Agent**: Bus Mode Engineer (SPI Specialist)  
**Priority**: High  
**Dependencies**: Task 1.1, Task 2.2

**Description**: Implement all SPI mode features.

**Deliverables**:
- [ ] Complete `bus-modes/src/spi.rs`
- [ ] Support all SPI modes (0-3)
- [ ] Implement full-duplex transfer
- [ ] Add Flash ID reading
- [ ] Implement Flash chip erase
- [ ] Implement Flash page read/write
- [ ] Add SD card initialization and read/write
- [ ] Support SPI slave mode (ESP32-S3 as slave)
- [ ] Add EEPROM operations (25-series)

**Acceptance Criteria**:
- Can read Flash ID from SPI Flash chip
- Can read/write SD card sectors
- Can dump SPI EEPROM contents
- SPI slave mode works correctly

---

### Task 3.3: Complete UART Mode Implementation
**Agent**: Bus Mode Engineer (UART Specialist)  
**Priority**: High  
**Dependencies**: Task 1.1, Task 2.2

**Description**: Implement all UART mode features.

**Deliverables**:
- [ ] Complete `bus-modes/src/uart.rs`
- [ ] Support all standard baud rates (9600-921600)
- [ ] Support custom baud rates
- [ ] Implement parity (none, even, odd)
- [ ] Implement stop bits (1, 1.5, 2)
- [ ] Add UART bridge mode (transparent pass-through)
- [ ] Implement AT command detection
- [ ] Add automatic baud rate detection
- [ ] Implement Half-Duplex UART mode
- [ ] Add line ending options (CR, LF, CRLF)

**Acceptance Criteria**:
- Can communicate with UART devices at all baud rates
- Bridge mode works reliably
- Baud rate auto-detection works for common rates
- No data corruption or dropped bytes

---

### Task 3.4: 1-Wire Mode Implementation
**Agent**: Bus Mode Engineer (1-Wire Specialist)  
**Priority**: Medium  
**Dependencies**: Task 1.1, Task 2.2

**Description**: Implement 1-Wire protocol support.

**Deliverables**:
- [ ] Create `bus-modes/src/onewire.rs`
- [ ] Implement 1-Wire reset and presence detect
- [ ] Implement ROM search algorithm
- [ ] Support 1-Wire read/write bytes
- [ ] Add iButton support (DS1990A)
- [ ] Add temperature sensor support (DS18B20)
- [ ] Implement EEPROM operations (DS2431, DS2433)
- [ ] Add parasite power mode

**Acceptance Criteria**:
- Can detect 1-Wire devices on the bus
- ROM search finds all devices
- Can read DS18B20 temperature
- Can read/write iButton IDs

---

## Phase 4: GUI and Menu System (Week 7)

### Task 4.1: Embedded Graphics GUI Framework
**Agent**: Display & Touch Developer  
**Priority**: High  
**Dependencies**: Task 1.2, Task 1.3

**Description**: Create a touch-based GUI using `embedded-graphics`.

**Deliverables**:
- [ ] Create `firmware/src/gui/` module
- [ ] Implement main menu with mode selection
- [ ] Create mode status screens (current mode, pins, config)
- [ ] Add touch-based navigation
- [ ] Implement virtual keyboard for text input
- [ ] Create logic analyzer trace viewer
- [ ] Add pin assignment configurator
- [ ] Design color scheme and UI theme
- [ ] Add icons and graphics

**Acceptance Criteria**:
- GUI is responsive to touch input (<50ms latency)
- All screens are readable and well-organized
- Navigation is intuitive
- UI fits within 240×320 resolution

---

### Task 4.2: CLI Parser and Command Executor
**Agent**: Protocol & CLI Developer  
**Priority**: High  
**Dependencies**: Task 2.2

**Description**: Implement command-line interface for text-based control.

**Deliverables**:
- [ ] Create `firmware/src/cli/` module
- [ ] Implement command parser (tokenizer, argument parser)
- [ ] Add command help system
- [ ] Implement mode switching commands
- [ ] Add Bus Pirate syntax support (`[`, `]`, `r`, `w`, etc.)
- [ ] Create command history (up/down arrows)
- [ ] Add tab completion
- [ ] Implement script execution (run commands from file)

**Acceptance Criteria**:
- CLI accepts all standard Bus Pirate commands
- Help text is clear and comprehensive
- Command history works correctly
- Script execution is reliable

---

## Phase 5: Advanced Features (Weeks 8-10)

### Task 5.1: Additional Bus Modes
**Agent**: Bus Mode Engineers  
**Priority**: Medium  
**Dependencies**: Phase 3 completion

**Description**: Implement remaining bus modes.

**Sub-tasks**:
- [ ] 2-Wire mode (smartcard, raw clock/data)
- [ ] 3-Wire mode (Microwire EEPROM)
- [ ] DIO mode (raw GPIO control, PWM, pulse measurement)
- [ ] LED mode (WS2812, APA102, etc. using RMT peripheral)
- [ ] Infrared mode (TX/RX using RMT peripheral)
- [ ] I²S audio mode (playback, recording)
- [ ] CAN bus mode (using external MCP2515 or similar)

---

### Task 5.2: File System Support
**Agent**: System Integration Engineer  
**Priority**: Medium  
**Dependencies**: Task 1.1

**Description**: Add file system support for scripts and data.

**Deliverables**:
- [ ] Integrate `embedded-sdmmc` for SD card FAT32 support
- [ ] Integrate `littlefs2` for internal flash storage
- [ ] Implement file read/write operations
- [ ] Add directory listing
- [ ] Create configuration file loader
- [ ] Implement script execution from filesystem
- [ ] Add HTTP file server (if Wi-Fi is implemented)

**Acceptance Criteria**:
- Can read/write files on SD card
- Can store configuration in internal flash
- File operations are reliable (no corruption)

---

### Task 5.3: Wi-Fi Integration (Optional)
**Agent**: Network Engineer  
**Priority**: Low  
**Dependencies**: Task 5.2

**Description**: Add Wi-Fi support for network-based control.

**Deliverables**:
- [ ] Enable Wi-Fi peripheral using `esp-wifi`
- [ ] Implement AP mode (device creates network)
- [ ] Implement STA mode (device joins existing network)
- [ ] Add WebSocket server for CLI over Wi-Fi
- [ ] Add HTTP server for file access
- [ ] Implement network configuration UI

**Acceptance Criteria**:
- Device creates functional Wi-Fi AP
- Can join existing Wi-Fi networks
- WebSocket CLI works from browser
- HTTP file server is accessible

---

## Phase 6: Testing and Optimization (Weeks 11-12)

### Task 6.1: Comprehensive Testing
**Agent**: Testing & CI Engineer  
**Priority**: High  
**Dependencies**: All previous tasks

**Description**: Perform thorough testing of all features.

**Deliverables**:
- [ ] Create unit test suite (80%+ coverage)
- [ ] Write integration tests for each bus mode
- [ ] Perform hardware-in-the-loop (HIL) testing
- [ ] Stress test with long-running operations
- [ ] Test error handling and recovery
- [ ] Validate protocol implementation against specification
- [ ] Test on multiple boards (if available)

**Acceptance Criteria**:
- All tests pass consistently
- No memory leaks or crashes
- Error handling is robust

---

### Task 6.2: Performance Optimization
**Agent**: Performance Engineer  
**Priority**: Medium  
**Dependencies**: Task 6.1

**Description**: Optimize firmware for size and speed.

**Deliverables**:
- [ ] Profile firmware size and identify bloat
- [ ] Optimize critical paths (display updates, protocol parsing)
- [ ] Use LTO and size optimizations
- [ ] Reduce heap allocations where possible
- [ ] Optimize I²C/SPI transfer speeds
- [ ] Benchmark performance vs. C version

**Acceptance Criteria**:
- Binary size fits comfortably in flash (<1MB)
- Display refresh rate is acceptable (>10 FPS)
- Bus speeds match or exceed C version

---

### Task 6.3: Documentation and Examples
**Agent**: Documentation Lead  
**Priority**: High  
**Dependencies**: All previous tasks

**Description**: Create comprehensive user and developer documentation.

**Deliverables**:
- [ ] User manual (how to use each mode)
- [ ] Developer guide (how to add new modes)
- [ ] API documentation (rustdoc)
- [ ] Example scripts and use cases
- [ ] Troubleshooting guide
- [ ] Migration guide from C version

**Acceptance Criteria**:
- All public APIs are documented
- User manual covers all features
- Examples are clear and functional

---

## Task Assignment Matrix

| Task | Agent | Estimated Time | Dependencies |
|------|-------|----------------|--------------|
| 1.1 | HAL Developer | 1 week | None |
| 1.2 | Display Developer | 1 week | 1.1 |
| 1.3 | Touch Developer | 1 week | 1.1 |
| 2.1 | Protocol Developer | 3 days | None |
| 2.2 | Protocol Developer | 4 days | 2.1 |
| 3.1 | I²C Engineer | 1 week | 1.1, 2.2 |
| 3.2 | SPI Engineer | 1 week | 1.1, 2.2 |
| 3.3 | UART Engineer | 1 week | 1.1, 2.2 |
| 3.4 | 1-Wire Engineer | 1 week | 1.1, 2.2 |
| 4.1 | GUI Developer | 1 week | 1.2, 1.3 |
| 4.2 | CLI Developer | 1 week | 2.2 |
| 5.1 | Mode Engineers | 2 weeks | Phase 3 |
| 5.2 | System Engineer | 1 week | 1.1 |
| 5.3 | Network Engineer | 1 week | 5.2 |
| 6.1 | QA Engineer | 1 week | All |
| 6.2 | Performance Engineer | 1 week | 6.1 |
| 6.3 | Tech Writer | 1 week | All |

---

## Issue Template

When creating GitHub issues, use this template:

```markdown
## [Task X.Y]: [Task Name]

**Assignee**: [Agent Role]  
**Priority**: [High/Medium/Low]  
**Estimated Time**: [Time]  
**Dependencies**: [List of tasks]

### Description
[Detailed description of the task]

### Deliverables
- [ ] Item 1
- [ ] Item 2
- [ ] ...

### Acceptance Criteria
- [ ] Criterion 1
- [ ] Criterion 2
- [ ] ...

### Resources
- [Link to design doc]
- [Link to hardware datasheet]
- [Link to relevant code]

### Notes
[Any additional context or considerations]
```

---

**Next Steps**:
1. Create GitHub issues for all Phase 1 tasks
2. Assign tasks to available agents
3. Set up CI/CD pipeline for automated testing
4. Begin implementation following the task order
