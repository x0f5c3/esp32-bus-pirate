# Agent Invocation Guide - ESP32 Bus Pirate Rust Migration

**Created**: 2025-12-08  
**Purpose**: Step-by-step guide to invoke specialized agents for Phase 1-6 implementation  
**Goal**: Maximize parallel work to complete migration in minimum time

---

## Quick Start - Phase 1 Immediate Actions

### Step 1: Start HAL Implementation (NOW - No Dependencies)

**Command**: Invoke `@hal-peripherals-developer` with this message:

```
@hal-peripherals-developer

Please complete Task #1: HAL Implementation as specified in docs/tasks/TASK-001-HAL-Implementation.md

Key requirements:
- Implement rust/hal/src/peripherals/i2c.rs (I2C at 100kHz)
- Implement rust/hal/src/peripherals/spi.rs (SPI at 40MHz with DMA)
- Implement rust/hal/src/peripherals/uart.rs (all baud rates)
- Implement rust/hal/src/peripherals/gpio.rs (PWM, interrupts)
- Complete rust/hal/src/board.rs (board initialization)

Pin mappings (Waveshare ESP32-S3-Touch-LCD-2.8):
- Display SPI: MOSI=GPIO45, SCLK=GPIO40, CS=GPIO42, DC=GPIO41, RST=GPIO39, BL=GPIO5
- Touch I2C: SDA=GPIO1, SCL=GPIO3, INT=GPIO4, RST=GPIO2
- SD Card SPI: MISO=GPIO16, MOSI=GPIO17, SCLK=GPIO14, CS=GPIO21

Target: no_std, xtensa-esp32s3-none-elf
Timeline: 10-14 days
Success: All tests pass, cargo clippy passes, documentation complete
```

### Step 2: Start Protocol Tests (NOW - Partial Independence)

**Command**: Invoke `@protocol-cli-developer` with this message:

```
@protocol-cli-developer

Please complete Task #3: Protocol Tests & USB CDC as specified in docs/tasks/TASK-003-Protocol-USB-CDC.md

Phase A - Start immediately (no dependencies):
- Add 50+ unit tests for MessageCodec in rust/protocol/tests/codec_tests.rs
- Create integration tests in rust/protocol/tests/integration_tests.rs
- Test CRC validation with corrupted data
- Test version mismatch handling
- Create Python test client in tools/test_client.py

Phase B - Start after HAL USB peripheral is ready:
- Implement USB CDC class in rust/firmware/src/transport/usb_cdc.rs
- Create frame transport layer with RX/TX buffers
- Handle USB connect/disconnect events
- Test on actual hardware

Target: no_std, xtensa-esp32s3-none-elf
Timeline: 7-10 days
Success: 50+ tests pass, USB enumerates as serial port, Python client works
```

---

## Phase 1 Continued (Week 2)

### Step 3: Start Display & Touch Drivers (After HAL Completes)

**Dependency**: Wait for Step 1 (@hal-peripherals-developer) to complete HAL

**Command**: Invoke `@display-touch-developer` with this message:

```
@display-touch-developer

Please complete Task #2: Display & Touch Drivers as specified in docs/tasks/TASK-002-Display-Touch-Drivers.md

Requirements:
- Implement ST7789 display driver in rust/drivers/src/display/st7789.rs
- Use embedded-graphics for drawing primitives
- Implement CST328 touch controller in rust/drivers/src/touch/cst328.rs
- PWM backlight control (0-100% via GPIO5)
- Framebuffer in PSRAM if available
- Multi-touch support (up to 5 points)
- Create demo applications in rust/firmware/examples/

Hardware:
- Display: 240×320 portrait, RGB565, SPI via HAL
- Touch: I2C address 0x5A, interrupt on GPIO4

Target: no_std, xtensa-esp32s3-none-elf
Timeline: 10-14 days
Success: Display shows text, touch accurate, >10 FPS, no artifacts
```

---

## Phase 2 - Core Bus Modes (Week 3-6)

**Dependency**: Wait for Steps 1 and 2 to complete (HAL + Protocol/USB ready)

### Step 4A: I2C & 1-Wire Modes (Parallel Agent #1)

**Command**: Invoke `@bus-mode-engineer` (instance 1) with this message:

```
@bus-mode-engineer

Please complete Task #4: I2C, 1-Wire, 2-Wire, 3-Wire Modes as specified in docs/tasks/TASK-004-I2C-1Wire-Modes.md

Implement in rust/bus-modes/src/:
- i2c.rs - I2C scan (7/10-bit), register read/write, sniffer, EEPROM dump
- onewire.rs - Reset/presence, ROM search, DS18B20 temperature, iButton
- twowire.rs - Smart card communication
- threewire.rs - Microwire EEPROM

C reference: src/Controllers/I2cController.cpp, OneWireController.cpp, etc.

Timeline: 14-21 days
Success: I2C scan works, DS18B20 reads temp, EEPROM dump works
```

### Step 4B: SPI, DIO & LED Modes (Parallel Agent #2)

**Command**: Invoke `@bus-mode-engineer` (instance 2) with this message:

```
@bus-mode-engineer

Please complete Task #5: SPI, DIO, LED Modes as specified in docs/tasks/TASK-005-SPI-DIO-LED-Modes.md

Implement in rust/bus-modes/src/:
- spi.rs - All SPI modes (0-3), Flash ID, SD card, slave mode
- dio.rs - GPIO control, PWM output, pulse measurement
- led.rs - WS2812/APA102 via RMT peripheral

C reference: src/Controllers/SpiController.cpp, DioController.cpp, LedController.cpp

Timeline: 14-21 days
Success: Flash ID reads, SD card works, GPIO PWM works, WS2812 displays
```

### Step 4C: UART, Infrared & I2S Modes (Parallel Agent #3)

**Command**: Invoke `@bus-mode-engineer` (instance 3) with this message:

```
@bus-mode-engineer

Please complete Task #6: UART, Infrared, I2S Modes as specified in docs/tasks/TASK-006-UART-IR-I2S-Modes.md

Implement in rust/bus-modes/src/:
- uart.rs - All baud rates, bridge mode, AT commands, auto-baud
- hduart.rs - Half-duplex UART
- infrared.rs - IR TX/RX via RMT, 10+ protocols (NEC, RC5, Sony)
- i2s.rs - Audio playback/recording via PCM5101A

C reference: src/Controllers/UartController.cpp, InfraredController.cpp, I2sController.cpp

Timeline: 14-21 days
Success: UART bridge works, IR controls TV, audio playback clear
```

---

## Phase 3 - GUI Implementation (Week 5+)

**Dependency**: Wait for Step 3 (@display-touch-developer) to complete

### Step 5: GUI with Slint or embedded-graphics

**Command**: Invoke `@ui-developer` with this message:

```
@ui-developer

Please complete Task #7: GUI Implementation as specified in docs/tasks/TASK-007-GUI-Implementation.md

Implement in rust/firmware/src/gui/:
- main_menu.rs - Mode selection grid
- mode_screen.rs - Current mode status
- keyboard.rs - Virtual keyboard
- widgets.rs - Buttons, sliders, etc.

Choose framework: Slint (if no_std compatible) or embedded-graphics

Requirements:
- Touch-based navigation (<50ms latency)
- 240×320 display layout
- Mode switching
- Settings screens
- Pin configuration UI

Timeline: 14-21 days
Success: GUI renders >10 FPS, touch works smoothly, all screens fit
```

---

## Phase 4 - Advanced Bus Modes (Week 7+)

**Dependency**: Wait for Phase 2 bus modes to complete

### Step 6A: Wireless Modes (Parallel)

**Command**: Invoke `@bus-mode-engineer` (instance 4):

```
@bus-mode-engineer

Implement advanced wireless modes:
- rust/bus-modes/src/bluetooth.rs - BLE scan, HID, sniffing
- rust/bus-modes/src/wifi.rs - AP scan, packet sniffing
- rust/bus-modes/src/ethernet.rs - Packet send/receive

C reference: src/Controllers/BluetoothController.cpp, WifiController.cpp, EthernetController.cpp
Timeline: 21-28 days
```

### Step 6B: Radio & RFID Modes (Parallel)

**Command**: Invoke `@bus-mode-engineer` (instance 5):

```
@bus-mode-engineer

Implement radio and RFID modes:
- rust/bus-modes/src/subghz.rs - CC1101 frequency scanning
- rust/bus-modes/src/rfid.rs - PN532 NFC/RFID card operations
- rust/bus-modes/src/rf24.rs - NRF24L01+ packet sniffing
- rust/bus-modes/src/usb.rs - USB HID emulation

C reference: src/Controllers/SubGhzController.cpp, RfidController.cpp, etc.
Timeline: 21-28 days
```

### Step 6C: Advanced Protocols (Parallel)

**Command**: Invoke `@bus-mode-engineer` (instance 6):

```
@bus-mode-engineer

Implement advanced protocol modes:
- rust/bus-modes/src/can.rs - CAN frame TX/RX, sniffer
- rust/bus-modes/src/jtag.rs - JTAG/SWD scanning, boundary scan

C reference: src/Controllers/CanController.cpp, JtagController.cpp
Timeline: 14-21 days
```

---

## Phase 5 - Testing & Optimization (Week 11+)

### Step 7: Comprehensive Testing

**Command**: Invoke `@testing-ci-engineer` with this message:

```
@testing-ci-engineer

Set up comprehensive testing and CI/CD:
- Create unit test suite (80%+ coverage)
- Write integration tests for each bus mode
- Perform hardware-in-the-loop testing
- Set up GitHub Actions CI
- Stress testing and error recovery
- Performance profiling and optimization
- Binary size reduction

Timeline: 14 days
Success: All tests pass, CI green, performance targets met
```

---

## Parallel Execution Timeline

### Week 1 (Now)
```
🟡 @hal-peripherals-developer     → HAL (full speed)
🟡 @protocol-cli-developer        → Protocol tests (partial)
```

### Week 2
```
🟢 @hal-peripherals-developer     → HAL completing
🟡 @protocol-cli-developer        → USB CDC (needs HAL)
🟡 @display-touch-developer       → Display & Touch (needs HAL)
```

### Week 3-6 (Maximum Parallelization)
```
🟡 @bus-mode-engineer #1  → I2C, 1-Wire
🟡 @bus-mode-engineer #2  → SPI, DIO, LED
🟡 @bus-mode-engineer #3  → UART, IR, I2S
🟡 @ui-developer          → GUI (needs Display/Touch)
```

### Week 7-10
```
🟡 @bus-mode-engineer #4  → Bluetooth, Wi-Fi, Ethernet
🟡 @bus-mode-engineer #5  → SubGHz, RFID, RF24, USB
🟡 @bus-mode-engineer #6  → CAN, JTAG
```

### Week 11-14
```
🟡 @testing-ci-engineer   → Testing, optimization, documentation
```

---

## Progress Tracking

Monitor progress in these files:
- **Phase 1**: `docs/tasks/PHASE-1-INITIATION.md`
- **Task #1**: `docs/tasks/TASK-001-HAL-Implementation.md`
- **Task #2**: `docs/tasks/TASK-002-Display-Touch-Drivers.md`
- **Task #3**: `docs/tasks/TASK-003-Protocol-USB-CDC.md`
- **Tasks #4-7**: Similar task files in `docs/tasks/`

Update status in task files as work progresses:
- 🔴 Not Started
- 🟡 In Progress
- 🟢 Complete

---

## Success Metrics

### Code Quality
- ✅ `cargo clippy` passes with zero warnings
- ✅ `cargo fmt --check` passes
- ✅ All tests pass
- ✅ No undocumented `unsafe` code

### Functionality
- ✅ All 20+ bus modes working
- ✅ GUI responsive (<50ms touch latency)
- ✅ CLI compatible with Bus Pirate syntax
- ✅ USB serial communication reliable

### Performance
- ✅ Binary size <1MB
- ✅ GUI >10 FPS
- ✅ Bus speeds ≥ C version
- ✅ Startup time <2 seconds

---

## Emergency Contacts

If an agent gets blocked or needs coordination:
- Mention `@coordinator-planner` in the task file
- Review dependencies in `docs/AGENT-COORDINATION.md`
- Check C reference files for implementation details

---

**Total Timeline**: 14 weeks  
**Target Completion**: Early March 2025  
**Maximum Parallelization**: 7 agents working simultaneously

**Start now with Steps 1 and 2 above!**
