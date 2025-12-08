# Coordinating Agents for ESP32 Bus Pirate Rust Migration

This document provides specific GitHub issue templates and coordination instructions for using the custom agents to complete the migration.

## Available Custom Agents

1. **coordinator-planner** (you) - Overall migration planning and coordination
2. **hal-peripherals-developer** - HAL and peripheral driver implementation  
3. **display-touch-developer** - ST7789 display and CST328 touch drivers
4. **protocol-cli-developer** - Binary protocol and CLI implementation
5. **bus-mode-engineer** - Bus mode implementations (I2C, SPI, UART, etc.)
6. **ui-developer** - Slint-based user interface
7. **testing-ci-engineer** - Testing and CI/CD setup

## Immediate Actions Required

### Step 1: Complete Foundation Layer (Priority: CRITICAL)

Create the following GitHub issues and assign to custom agents:

#### Issue #1: Complete HAL Peripheral Implementations
**Assign to**: `hal-peripherals-developer`
**Dependencies**: None (blocking all other work)
**Estimated time**: 10-14 days

```markdown
## Task: Complete Hardware Abstraction Layer

Implement the HAL peripheral wrappers to provide safe abstractions for all ESP32-S3 peripherals.

### Files to Implement:
- [ ] `rust/hal/src/peripherals/i2c.rs`
- [ ] `rust/hal/src/peripherals/spi.rs`
- [ ] `rust/hal/src/peripherals/uart.rs`
- [ ] `rust/hal/src/peripherals/gpio.rs`
- [ ] `rust/hal/src/board.rs` (complete initialization)

### Requirements:
1. Use `esp-hal` crate and expose `embedded-hal` traits
2. I2C at 100kHz for touch/IMU/RTC
3. SPI at 40MHz for display with DMA support
4. UART with all baud rates (9600-921600+)
5. GPIO with PWM for backlight, interrupts for touch
6. Safe error handling throughout
7. Unit tests for each peripheral

### Pin Mappings (Waveshare Board):
- **Display SPI**: MOSI=GPIO45, SCLK=GPIO40, CS=GPIO42, DC=GPIO41, RST=GPIO39, BL=GPIO5
- **Touch I2C**: SDA=GPIO1, SCL=GPIO3, INT=GPIO4, RST=GPIO2
- **SD Card SPI**: MISO=GPIO16, MOSI=GPIO17, SCLK=GPIO14, CS=GPIO21
- **USB**: D+=GPIO19, D-=GPIO20

### Success Criteria:
- ✅ All modules compile without warnings
- ✅ `cargo clippy` passes
- ✅ Pin mappings match Waveshare docs
- ✅ Basic smoke test (LED blink) works on hardware

### Reference:
- `lib/TFT_eSPI/Processors/TFT_eSPI_ESP32_S3.*`
- Waveshare wiki: https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-2.8
```

---

#### Issue #2: Implement Display and Touch Drivers
**Assign to**: `display-touch-developer`
**Dependencies**: Issue #1 (HAL must be complete)
**Estimated time**: 10-14 days

```markdown
## Task: Implement ST7789 Display and CST328 Touch Drivers

Integrate display and touch drivers for the Waveshare ESP32-S3-Touch-LCD-2.8 board.

### Files to Implement:
- [ ] `rust/drivers/src/display/st7789.rs`
- [ ] `rust/drivers/src/display/mod.rs`
- [ ] `rust/drivers/src/touch/cst328.rs`
- [ ] `rust/drivers/src/touch/mod.rs`

### Display Requirements:
1. Use `st7789` crate + `embedded-graphics`
2. Initialize as 240×320 portrait mode
3. Implement `DrawTarget` trait
4. PWM backlight control (0-100% brightness)
5. Framebuffer in PSRAM if available
6. Double buffering to avoid tearing
7. Demo showing "Hello World" text

### Touch Requirements:
1. CST328 I2C driver at address 0x5A
2. Read touch coordinates (X, Y)
3. Detect events: press, release, move
4. Multi-touch support (up to 5 points)
5. Interrupt-driven reading via INT pin
6. Calibration routine
7. Demo showing touch points on screen

### Success Criteria:
- ✅ Display shows clear text without artifacts
- ✅ Backlight adjustable
- ✅ Frame rate >10 FPS
- ✅ Touch accurate across entire screen
- ✅ No ghost touches or missed events
- ✅ Multi-touch works correctly

### Reference:
- ST7789 datasheet
- CST328 datasheet
- Waveshare examples
```

---

#### Issue #3: Add Protocol Tests and USB CDC
**Assign to**: `protocol-cli-developer`
**Dependencies**: Issue #1 (HAL) for USB peripheral
**Estimated time**: 7-10 days

```markdown
## Task: Test Protocol and Implement USB CDC Transport

Validate the binary protocol and add USB CDC serial transport.

### Files to Create/Modify:
- [ ] `rust/protocol/tests/codec_tests.rs` (new)
- [ ] `rust/protocol/tests/integration_tests.rs` (new)
- [ ] `rust/firmware/src/transport/usb_cdc.rs` (new)
- [ ] `rust/firmware/src/transport/mod.rs` (new)
- [ ] `tools/test_client.py` (new - Python test client)

### Protocol Testing Requirements:
1. 50+ unit tests for `MessageCodec`
2. Test all message types (I2C, SPI, UART, etc.)
3. CRC validation with corrupted data
4. Version mismatch handling
5. Large message handling (up to 1KB)
6. Fuzz testing with random data
7. Documentation with examples

### USB CDC Requirements:
1. USB CDC class implementation
2. Frame transport layer (START/LENGTH/PAYLOAD/CRC/END)
3. RX/TX circular buffers (1KB each)
4. USB connect/disconnect events
5. Flow control
6. Error recovery

### Success Criteria:
- ✅ All unit tests pass
- ✅ Codec handles malformed data gracefully
- ✅ Python client can communicate with device
- ✅ USB enumerates as serial port on all platforms
- ✅ No data corruption or dropped frames

### Reference:
- `docs/protocol.md`
- `esp-hal` USB examples
```

---

### Step 2: Implement Core Bus Modes (Parallel Work)

Once HAL is complete, create these issues for bus mode engineers:

#### Issue #4: I2C and 1-Wire Modes
**Assign to**: `bus-mode-engineer`
**Label**: `i2c-onewire-specialist`

```markdown
## Task: Implement I2C, 1-Wire, 2-Wire, and 3-Wire Modes

### C Reference Files:
- `src/Controllers/I2cController.cpp`
- `src/Services/I2cService.cpp`
- `src/Controllers/OneWireController.cpp`
- `src/Services/OneWireService.cpp`
- `src/Controllers/TwoWireController.cpp`
- `src/Controllers/ThreeWireController.cpp`

### Files to Create/Modify:
- [ ] `rust/bus-modes/src/i2c.rs` (expand existing stub)
- [ ] `rust/bus-modes/src/onewire.rs` (new)
- [ ] `rust/bus-modes/src/twowire.rs` (new)
- [ ] `rust/bus-modes/src/threewire.rs` (new)

### I2C Mode Deliverables:
1. Bus scan (7-bit and 10-bit addressing)
2. Register read/write (single and burst)
3. I2C sniffer (passive monitoring)
4. Clock stretching support
5. Repeated start
6. EEPROM dump utility
7. Common device identification (read 0x00, 0x75 registers)

### 1-Wire Mode Deliverables:
1. Reset and presence detect
2. ROM search algorithm
3. iButton support (DS1990A)
4. DS18B20 temperature sensor
5. EEPROM operations (DS2431, DS2433)
6. Parasite power mode

### 2-Wire & 3-Wire:
1. Smart card communication (2-wire)
2. Microwire EEPROM (3-wire)

### Success Criteria:
- ✅ I2C scan finds all devices on bus
- ✅ Can read DS18B20 temperature
- ✅ Can dump I2C EEPROM
- ✅ Sniffer captures transactions
- ✅ All modes tested with real hardware

**Estimated time**: 14-21 days
```

#### Issue #5: SPI, DIO, and LED Modes
**Assign to**: `bus-mode-engineer`
**Label**: `spi-dio-specialist`

(Similar template covering SPI, DIO, and LED modes)

#### Issue #6: UART, Infrared, and I2S Modes
**Assign to**: `bus-mode-engineer`
**Label**: `uart-audio-specialist`

(Similar template covering UART, HD-UART, Infrared, and I2S modes)

---

### Step 3: GUI and CLI (After Display/Touch Complete)

#### Issue #7: Implement GUI with Slint or embedded-graphics
**Assign to**: `ui-developer`
**Dependencies**: Issue #2 (Display & Touch)

```markdown
## Task: Implement Touch-Based GUI

Choose between:
1. **Slint** - If `no_std` support is adequate
2. **embedded-graphics** - Manual widget implementation

### Files to Create:
- [ ] `rust/firmware/src/gui/main_menu.rs`
- [ ] `rust/firmware/src/gui/mode_screen.rs`
- [ ] `rust/firmware/src/gui/keyboard.rs`
- [ ] `rust/firmware/src/gui/theme.rs`
- [ ] `rust/firmware/src/gui/widgets.rs`
- [ ] `rust/firmware/src/gui/mod.rs`

### Requirements:
1. Main menu with mode selection (grid of icons)
2. Mode status screens (current mode, pins, config)
3. Touch-based navigation (tap, swipe)
4. Virtual keyboard for text input
5. Settings screens
6. Pin configuration UI
7. Responsive (<50ms touch latency)

### Success Criteria:
- ✅ GUI renders at >10 FPS
- ✅ Touch navigation is smooth
- ✅ All screens fit 240×320 resolution
- ✅ UI is intuitive and readable

**Estimated time**: 14-21 days
```

---

## Coordination Workflow

### Daily Process
1. Each agent updates their assigned GitHub issue with:
   - What was completed
   - What's being worked on
   - Any blockers
   
2. Coordinator reviews all issues daily and:
   - Unblocks agents
   - Resolves integration questions
   - Adjusts timeline if needed

### Weekly Sync
- Review progress vs. plan
- Integration testing
- Adjust priorities

### Integration Milestones
- **Week 2**: HAL + Display + Touch working together
- **Week 4**: Core modes (I2C, SPI, UART) with CLI
- **Week 7**: GUI integrated with modes
- **Week 10**: All modes functional
- **Week 12**: Feature freeze
- **Week 14**: Release candidate

---

## Current Status (2025-12-06)

### Completed
- ✅ Project planning and design
- ✅ Rust project scaffolding (5 crates)
- ✅ Protocol message definitions
- ✅ Documentation (design, protocol, tasks)

### In Progress
- ⚠️  ESP32 toolchain installation (blocked by GitHub rate limit)

### Blocked
- ❌ All implementation work (waiting for toolchain)

### Next Actions
1. Retry `espup install` when rate limit clears
2. Create GitHub issues #1-#7 above
3. Assign to custom agents
4. Begin Phase 1 work

---

## Notes for Custom Agents

### For hal-peripherals-developer
- Start with GPIO and SPI (needed for display)
- Then I2C (needed for touch)
- UART can be done in parallel
- Focus on `embedded-hal` trait compatibility

### For display-touch-developer
- Wait for HAL SPI and I2C to be ready
- Test display first (easier to debug)
- Then add touch
- Provide simple test pattern for calibration

### For protocol-cli-developer
- Protocol code already exists, focus on tests
- USB CDC is highest priority for host communication
- Python test client should be simple and well-documented

### For bus-mode-engineer (multiple instances)
- Study the C reference implementation first
- Start with simplest functionality, then add features
- Each mode should compile and test independently
- Provide examples showing usage

### For ui-developer
- Decide on Slint vs embedded-graphics early
- Keep UI simple - don't over-engineer
- Focus on essential functions first
- Touch calibration screen is high priority

---

**Maintainer**: Coordinator-Planner Agent  
**Last Updated**: 2025-12-06
