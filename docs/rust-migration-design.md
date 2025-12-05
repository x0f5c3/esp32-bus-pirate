# ESP32 Bus Pirate - Rust Migration Design Document

## Executive Summary

This document outlines the comprehensive plan for migrating the ESP32 Bus Pirate firmware from C/ESP-IDF (Arduino framework) to Rust with `no_std` targeting the **Waveshare ESP32-S3-Touch-LCD-2.8** board. The migration will maintain all existing Bus Pirate functionality while leveraging Rust's safety guarantees and the embedded ecosystem's powerful abstractions.

**Target Board:** Waveshare ESP32-S3-Touch-LCD-2.8
**Target MCU:** ESP32-S3 (Xtensa LX7 dual-core)
**Build Target:** `xtensa-esp32s3-none-elf`
**Environment:** `no_std` (bare-metal, no ESP-IDF dependency)

---

## Table of Contents

1. [Hardware Specifications](#1-hardware-specifications)
2. [Current Architecture Analysis](#2-current-architecture-analysis)
3. [Rust Project Structure](#3-rust-project-structure)
4. [Hardware Abstraction Layer (HAL)](#4-hardware-abstraction-layer-hal)
5. [Display and Touch Layer](#5-display-and-touch-layer)
6. [Binary Protocol Design](#6-binary-protocol-design)
7. [Bus Mode Implementations](#7-bus-mode-implementations)
8. [Application Layer](#8-application-layer)
9. [Memory Management](#9-memory-management)
10. [Error Handling Strategy](#10-error-handling-strategy)
11. [Testing Strategy](#11-testing-strategy)
12. [Migration Phases](#12-migration-phases)
13. [Implementation Guidelines](#13-implementation-guidelines)
14. [References](#14-references)

---

## 1. Hardware Specifications

### 1.1 Waveshare ESP32-S3-Touch-LCD-2.8 Board

**Display:**
- **Size:** 2.8 inch TFT LCD
- **Resolution:** 240×320 pixels
- **Controller:** ST7789VW
- **Interface:** SPI
- **Pin Mapping:**
  - MOSI: GPIO45
  - SCLK: GPIO40
  - CS: GPIO42
  - DC: GPIO41
  - RESET: GPIO39
  - Backlight (BL): GPIO5

**Touch Controller:**
- **Model:** CST328 (capacitive touch)
- **Interface:** I²C
- **Pin Mapping:**
  - SDA: GPIO1
  - SCL: GPIO3
  - INT: GPIO4
  - RST: GPIO2

**IMU:**
- **Model:** QMI8658C
- **Interface:** I²C (shared with touch)
- **Features:** 6-axis accelerometer + gyroscope

**RTC:**
- **Model:** PCF85063 (note: documentation mentions PCF8563, but PCF85063 is more common)
- **Interface:** I²C (shared bus)

**Audio Codec:**
- **Model:** PCM5101A
- **Interface:** I²S
- **Features:** Digital-to-analog converter for speaker output

**Storage:**
- **TF Card Slot:** MicroSD card via SPI

**Other Peripherals:**
- **Speaker:** Via audio codec
- **USB:** Native USB-OTG support (ESP32-S3)
- **Flash:** 16MB (typical for this board)
- **PSRAM:** 8MB QSPI PSRAM

### 1.2 ESP32-S3 Features

- **CPU:** Xtensa LX7 dual-core @ 240MHz
- **RAM:** 512KB SRAM
- **PSRAM:** Up to 8MB external
- **Flash:** 16MB external
- **Peripherals:**
  - 2× UART
  - 2× I²C
  - 4× SPI (including QSPI for flash/PSRAM)
  - 2× I²S
  - USB 1.1 OTG
  - GPIO: 45 programmable GPIOs
  - ADC, DAC, PWM, RMT (remote control)
  - Wi-Fi 802.11 b/g/n
  - Bluetooth LE 5.0

---

## 2. Current Architecture Analysis

### 2.1 Existing Codebase Overview

The current ESP32 Bus Pirate firmware (~45,000 lines of C++) is organized as follows:

**Core Components:**
- **Controllers:** Handle each protocol mode (I2C, SPI, UART, 1-Wire, etc.)
- **Services:** Low-level peripheral drivers and protocol implementations
- **Views:** Display and terminal output interfaces
- **Inputs:** User input handling (serial, web, device buttons)
- **Managers:** High-level coordination (UserInputManager, etc.)
- **Shells:** Interactive sub-modes (e.g., I2cEepromShell)
- **Transformers:** Data parsing and conversion
- **Dispatchers:** Main event loop and action routing
- **States:** Global configuration and runtime state

**Supported Modes (20+):**
1. HiZ (default/idle)
2. I²C (scan, sniff, slave, dump, EEPROM)
3. SPI (EEPROM, Flash, SD card, slave)
4. UART / Half-Duplex UART (bridge, read/write)
5. 1-Wire (iButton, EEPROM)
6. 2-Wire / 3-Wire (sniff, smartcard, EEPROM)
7. DIO (GPIO control, PWM)
8. Infrared (TX/RX, universal remote)
9. USB (HID: mouse/keyboard/gamepad)
10. Bluetooth (BLE HID, scan, spoof)
11. Wi-Fi / Ethernet (sniff, deauth, netcat)
12. JTAG (pinout scan, SWD)
13. LED (FastLED protocols)
14. I²S (audio playback/recording)
15. CAN (sniff, send/receive frames)
16. Sub-GHz (CC1101 radio)
17. RFID (read/write/clone)
18. RF24 (nRF24L01+ radio)
19. Modbus (RTU/TCP)
20. Scripting (Bus Pirate bytecode, Python over serial)

**Key Features:**
- CLI over Serial, WebSocket, or on-device keyboard
- HTTP file server for LittleFS/SD card access
- Network services (SSH, Telnet, Netcat)
- Protocol sniffers (I²C, SPI, 1-Wire, CAN, etc.)
- Bus Pirate instruction syntax
- EEPROM/Flash dump tools
- Frequency analysis for radio signals

### 2.2 Dependencies

The current C++ codebase relies on:
- **Arduino framework** (built on ESP-IDF)
- **M5Stack libraries** (for M5 device support)
- **TFT_eSPI** (display driver)
- **FastLED** (LED control)
- **ArduinoJson** (JSON parsing)
- **Third-party protocol libraries** (OneWire, MCP2515, RF24, etc.)

All of these will be replaced with Rust `no_std` equivalents.

---

## 3. Rust Project Structure

### 3.1 Cargo Workspace Layout

```
esp32-bus-pirate-rust/
├── Cargo.toml                  # Workspace manifest
├── .cargo/
│   └── config.toml            # Xtensa target configuration
├── rust-toolchain.toml        # Rust nightly + Xtensa support
├── docs/
│   ├── design.md              # This document
│   ├── protocol.md            # Protocol specification
│   └── pin-mappings.md        # Hardware pin reference
├── hal/                       # Hardware Abstraction Layer
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── board.rs           # Board-specific initialization
│       ├── pins.rs            # Pin definitions
│       └── peripherals/
│           ├── i2c.rs
│           ├── spi.rs
│           ├── uart.rs
│           ├── gpio.rs
│           ├── pwm.rs
│           ├── i2s.rs
│           └── usb.rs
├── drivers/                   # Device drivers
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── display/
│       │   ├── st7789.rs     # ST7789 display driver (wraps st7789 crate)
│       │   └── mod.rs
│       ├── touch/
│       │   ├── cst328.rs     # Custom CST328 driver
│       │   └── mod.rs
│       ├── imu/
│       │   └── qmi8658.rs    # QMI8658 IMU driver
│       ├── rtc/
│       │   └── pcf85063.rs   # PCF85063 RTC driver
│       └── audio/
│           └── pcm5101.rs    # PCM5101 audio codec
├── protocol/                  # Binary protocol layer
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── message.rs         # Message definitions
│       ├── codec.rs           # Framing and serialization
│       ├── crc.rs             # Checksum utilities
│       └── version.rs         # Protocol versioning
├── bus-modes/                 # Bus Pirate mode implementations
│   ├── Cargo.toml
│   └── src/
│       ├── lib.rs
│       ├── i2c.rs
│       ├── spi.rs
│       ├── uart.rs
│       ├── onewire.rs
│       ├── twowire.rs
│       ├── threewire.rs
│       ├── dio.rs
│       └── traits.rs          # Common traits for bus modes
├── firmware/                  # Main application binary
│   ├── Cargo.toml
│   ├── build.rs               # Build script for linking
│   ├── memory.x               # Linker script
│   └── src/
│       ├── main.rs
│       ├── app.rs             # Application state machine
│       ├── cli/
│       │   ├── parser.rs     # Command parser
│       │   └── executor.rs   # Command executor
│       ├── gui/
│       │   ├── menu.rs       # On-screen menu system
│       │   └── widgets.rs    # Display widgets
│       └── storage/
│           ├── flash.rs      # NVS/Flash config storage
│           └── sdcard.rs     # SD card filesystem
└── README.md
```

### 3.2 Crate Dependencies

**Primary Crates:**
- `esp32s3-hal` (or `esp-hal` with S3 support) - ESP32-S3 HAL
- `embedded-hal` v1.0 - Hardware abstraction traits
- `embedded-hal-async` - Async HAL traits (optional)
- `embedded-graphics` - 2D graphics library
- `st7789` - ST7789 display driver
- `embedded-io` - I/O traits for serial/network
- `heapless` - Static data structures (Vec, String, etc.)
- `postcard` - Compact binary serialization
- `crc` or `crc-any` - CRC calculation
- `defmt` or `log` - Logging (defmt preferred for size)
- `critical-section` - Critical section implementation
- `static_cell` - Static mutable singletons
- `embassy-executor` (optional) - Async executor

**Optional Crates (depending on feature flags):**
- `smoltcp` - TCP/IP stack for Wi-Fi/Ethernet
- `embedded-sdmmc` - FAT filesystem for SD card
- `littlefs2` - LittleFS filesystem
- `usbd-serial` - USB CDC serial
- `usbd-hid` - USB HID class

---

## 4. Hardware Abstraction Layer (HAL)

### 4.1 HAL Architecture

The HAL crate will provide a thin wrapper around `esp32s3-hal` with board-specific initialization:

```rust
// hal/src/board.rs
pub struct WaveshareS3Board {
    pub display_spi: SpiDevice<SPI2>,
    pub display_dc: GpioPin<41>,
    pub display_cs: GpioPin<42>,
    pub display_rst: GpioPin<39>,
    pub display_bl: GpioPin<5>,
    
    pub touch_i2c: I2c<I2C0>,
    pub touch_int: GpioPin<4>,
    pub touch_rst: GpioPin<2>,
    
    pub usb: Usb,
    pub uart0: Uart<UART0>,
    pub uart1: Uart<UART1>,
    // ... other peripherals
}

impl WaveshareS3Board {
    pub fn new() -> Self {
        // Initialize clocks, GPIO, peripherals
        // Return configured board instance
    }
}
```

### 4.2 Pin Definitions

Create a central pin mapping file:

```rust
// hal/src/pins.rs
pub mod display {
    pub const MOSI: u8 = 45;
    pub const SCLK: u8 = 40;
    pub const CS: u8 = 42;
    pub const DC: u8 = 41;
    pub const RESET: u8 = 39;
    pub const BL: u8 = 5;
}

pub mod touch {
    pub const SDA: u8 = 1;
    pub const SCL: u8 = 3;
    pub const INT: u8 = 4;
    pub const RST: u8 = 2;
}

// ... similar for other peripherals
```

### 4.3 Peripheral Abstractions

Each peripheral module (I²C, SPI, UART) will expose a safe, high-level API:

```rust
// hal/src/peripherals/i2c.rs
pub struct I2cBus<I> {
    inner: I,
    frequency: u32,
}

impl<I: embedded_hal::i2c::I2c> I2cBus<I> {
    pub fn new(i2c: I, frequency: u32) -> Self {
        Self { inner: i2c, frequency }
    }
    
    pub fn scan(&mut self) -> Result<heapless::Vec<u8, 128>, Error> {
        // Scan I²C bus for devices
    }
    
    pub fn read_register(&mut self, addr: u8, reg: u8) -> Result<u8, Error> {
        // Read single register
    }
    
    // ... more helper methods
}
```

---

## 5. Display and Touch Layer

### 5.1 Display Driver (ST7789)

Use the `st7789` crate with `embedded-graphics` for drawing:

```rust
// drivers/src/display/st7789.rs
use st7789::{ST7789, Orientation};
use embedded_graphics::prelude::*;

pub struct Display {
    driver: ST7789<SpiDevice, DcPin, RstPin>,
    backlight: BlPin,
}

impl Display {
    pub fn new(
        spi: SpiDevice,
        dc: DcPin,
        rst: RstPin,
        cs: CsPin,
        bl: BlPin,
    ) -> Result<Self, Error> {
        let mut driver = ST7789::new(spi, dc, rst, 240, 320);
        driver.init(&mut Delay)?;
        driver.set_orientation(Orientation::Portrait)?;
        
        Ok(Self { driver, backlight: bl })
    }
    
    pub fn set_brightness(&mut self, level: u8) {
        // PWM control for backlight
    }
}

impl DrawTarget for Display {
    type Color = Rgb565;
    type Error = Error;
    
    fn draw_iter<I>(&mut self, pixels: I) -> Result<(), Self::Error>
    where
        I: IntoIterator<Item = Pixel<Self::Color>>,
    {
        self.driver.draw_iter(pixels)
    }
}
```

### 5.2 Touch Controller (CST328)

Implement a custom driver for CST328:

```rust
// drivers/src/touch/cst328.rs
pub struct Cst328<I> {
    i2c: I,
    address: u8,
    int_pin: IntPin,
    rst_pin: RstPin,
}

impl<I: embedded_hal::i2c::I2c> Cst328<I> {
    const ADDR: u8 = 0x5A; // CST328 I²C address
    
    pub fn new(i2c: I, int_pin: IntPin, rst_pin: RstPin) -> Self {
        Self { i2c, address: Self::ADDR, int_pin, rst_pin }
    }
    
    pub fn init(&mut self) -> Result<(), Error> {
        // Reset and initialize touch controller
    }
    
    pub fn read_touch_event(&mut self) -> Result<Option<TouchEvent>, Error> {
        // Read touch coordinates and gesture
    }
}

#[derive(Debug, Clone, Copy)]
pub struct TouchEvent {
    pub x: u16,
    pub y: u16,
    pub event_type: TouchEventType,
}

#[derive(Debug, Clone, Copy)]
pub enum TouchEventType {
    Press,
    Release,
    Move,
}
```

### 5.3 GUI Framework

Use `embedded-graphics` for drawing UI elements:

```rust
// firmware/src/gui/menu.rs
use embedded_graphics::prelude::*;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::text::Text;

pub struct Menu {
    items: heapless::Vec<&'static str, 16>,
    selected: usize,
}

impl Menu {
    pub fn draw<D: DrawTarget<Color = Rgb565>>(&self, display: &mut D) -> Result<(), D::Error> {
        for (i, item) in self.items.iter().enumerate() {
            let y = 20 + i * 20;
            let style = if i == self.selected {
                MonoTextStyle::new(&FONT_6X10, Rgb565::BLACK)
            } else {
                MonoTextStyle::new(&FONT_6X10, Rgb565::WHITE)
            };
            
            Text::new(item, Point::new(10, y as i32), style).draw(display)?;
        }
        Ok(())
    }
}
```

---

## 6. Binary Protocol Design

### 6.1 Protocol Overview

Design a compact binary protocol for CLI communication over Serial/USB/Network. The protocol must:
- Be efficient in both size and processing
- Support versioning for backward compatibility
- Include error detection (CRC)
- Handle variable-length messages
- Be `no_std` compatible

### 6.2 Protocol Comparison

| Feature | postcard | prost-lite |
|---------|----------|------------|
| Size overhead | Minimal (~1-2 bytes) | Small (~2-4 bytes) |
| `no_std` support | Excellent | Good |
| Schema evolution | Limited | Good (Proto3) |
| Encoding speed | Very fast | Fast |
| Tooling | Rust-native | Protobuf ecosystem |
| Learning curve | Low | Medium |

**Recommendation:** Use `postcard` for simplicity and minimal overhead. Reserve Protocol Buffers for future extensions if cross-language compatibility is needed.

### 6.3 Message Framing

```
┌─────────┬─────────┬─────────┬──────────┬─────────┬─────────┐
│ START   │ VERSION │ LENGTH  │ PAYLOAD  │ CRC16   │  END    │
│ (0xAA)  │ (1 byte)│ (2 bytes│ (n bytes)│ (2 bytes│ (0x55)  │
└─────────┴─────────┴─────────┴──────────┴─────────┴─────────┘
```

- **START:** Magic byte `0xAA`
- **VERSION:** Protocol version (currently `0x01`)
- **LENGTH:** Payload length (little-endian u16)
- **PAYLOAD:** Serialized message (postcard-encoded)
- **CRC16:** CRC-16-CCITT of VERSION + LENGTH + PAYLOAD
- **END:** End marker `0x55`

### 6.4 Message Types

```rust
// protocol/src/message.rs
use serde::{Serialize, Deserialize};
use heapless::{String, Vec};

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    // Mode selection
    SetMode { mode: Mode },
    GetMode,
    
    // Bus operations
    I2cScan,
    I2cWrite { addr: u8, data: Vec<u8, 256> },
    I2cRead { addr: u8, len: u8 },
    
    SpiTransfer { data: Vec<u8, 256> },
    
    UartWrite { data: Vec<u8, 256> },
    UartRead { len: u16 },
    
    // Configuration
    SetConfig { key: String<32>, value: String<64> },
    GetConfig { key: String<32> },
    
    // File operations
    FileList { path: String<128> },
    FileRead { path: String<128> },
    FileWrite { path: String<128>, data: Vec<u8, 1024> },
    
    // Responses
    Response(Response),
    Error(ErrorCode),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Mode {
    HiZ,
    I2c,
    Spi,
    Uart,
    OneWire,
    // ... all other modes
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Response {
    Success,
    Data(Vec<u8, 1024>),
    I2cDevices(Vec<u8, 128>),
    ModeChanged(Mode),
    ConfigValue(String<64>),
    // ... more response types
}

#[derive(Debug, Serialize, Deserialize)]
pub enum ErrorCode {
    InvalidCommand,
    ProtocolError,
    BusError,
    FileNotFound,
    PermissionDenied,
    // ... more error codes
}
```

### 6.5 Codec Implementation

```rust
// protocol/src/codec.rs
use crate::message::Message;
use postcard::{from_bytes, to_vec};
use crc::{Crc, CRC_16_IBM_SDLC};
use heapless::Vec;

const START_BYTE: u8 = 0xAA;
const END_BYTE: u8 = 0x55;
const VERSION: u8 = 0x01;
const CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);

pub struct MessageCodec;

impl MessageCodec {
    pub fn encode(msg: &Message) -> Result<Vec<u8, 1024>, Error> {
        let payload = to_vec(msg).map_err(|_| Error::EncodingFailed)?;
        let len = payload.len() as u16;
        
        let mut frame = Vec::new();
        frame.push(START_BYTE).map_err(|_| Error::BufferFull)?;
        frame.push(VERSION).map_err(|_| Error::BufferFull)?;
        frame.extend_from_slice(&len.to_le_bytes()).map_err(|_| Error::BufferFull)?;
        frame.extend_from_slice(&payload).map_err(|_| Error::BufferFull)?;
        
        // Calculate CRC over VERSION + LENGTH + PAYLOAD
        let crc = CRC.checksum(&frame[1..]);
        frame.extend_from_slice(&crc.to_le_bytes()).map_err(|_| Error::BufferFull)?;
        frame.push(END_BYTE).map_err(|_| Error::BufferFull)?;
        
        Ok(frame)
    }
    
    pub fn decode(frame: &[u8]) -> Result<Message, Error> {
        if frame.len() < 7 {
            return Err(Error::FrameTooShort);
        }
        
        if frame[0] != START_BYTE || frame[frame.len() - 1] != END_BYTE {
            return Err(Error::InvalidFrame);
        }
        
        let version = frame[1];
        if version != VERSION {
            return Err(Error::UnsupportedVersion);
        }
        
        let len = u16::from_le_bytes([frame[2], frame[3]]) as usize;
        let payload_end = 4 + len;
        
        if frame.len() < payload_end + 3 {
            return Err(Error::FrameTooShort);
        }
        
        let payload = &frame[4..payload_end];
        let crc_received = u16::from_le_bytes([frame[payload_end], frame[payload_end + 1]]);
        
        // Verify CRC
        let crc_calculated = CRC.checksum(&frame[1..payload_end]);
        if crc_received != crc_calculated {
            return Err(Error::CrcMismatch);
        }
        
        from_bytes(payload).map_err(|_| Error::DecodingFailed)
    }
}
```

---

## 7. Bus Mode Implementations

### 7.1 Common Traits

Define common traits for all bus modes:

```rust
// bus-modes/src/traits.rs
pub trait BusMode {
    type Config;
    type Error;
    
    fn name(&self) -> &'static str;
    fn init(&mut self, config: Self::Config) -> Result<(), Self::Error>;
    fn deinit(&mut self) -> Result<(), Self::Error>;
}

pub trait Scanner {
    type DeviceId;
    type Error;
    
    fn scan(&mut self) -> Result<heapless::Vec<Self::DeviceId, 128>, Self::Error>;
}

pub trait Sniffer {
    type Event;
    type Error;
    
    fn start_sniff(&mut self) -> Result<(), Self::Error>;
    fn stop_sniff(&mut self) -> Result<(), Self::Error>;
    fn read_event(&mut self) -> Result<Option<Self::Event>, Self::Error>;
}
```

### 7.2 I²C Mode Implementation

```rust
// bus-modes/src/i2c.rs
use embedded_hal::i2c::I2c;
use crate::traits::{BusMode, Scanner};

pub struct I2cMode<I> {
    i2c: I,
    config: I2cConfig,
}

pub struct I2cConfig {
    pub frequency: u32,
    pub sda_pin: u8,
    pub scl_pin: u8,
}

impl<I: I2c> BusMode for I2cMode<I> {
    type Config = I2cConfig;
    type Error = I2cError;
    
    fn name(&self) -> &'static str {
        "I2C"
    }
    
    fn init(&mut self, config: Self::Config) -> Result<(), Self::Error> {
        self.config = config;
        // Configure I²C peripheral
        Ok(())
    }
    
    fn deinit(&mut self) -> Result<(), Self::Error> {
        // Release I²C peripheral
        Ok(())
    }
}

impl<I: I2c> Scanner for I2cMode<I> {
    type DeviceId = u8;
    type Error = I2cError;
    
    fn scan(&mut self) -> Result<heapless::Vec<u8, 128>, Self::Error> {
        let mut devices = heapless::Vec::new();
        
        for addr in 0x08..=0x77 {
            if self.i2c.write(addr, &[]).is_ok() {
                devices.push(addr).ok();
            }
        }
        
        Ok(devices)
    }
}

impl<I: I2c> I2cMode<I> {
    pub fn read_register(&mut self, addr: u8, reg: u8) -> Result<u8, I2cError> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(addr, &[reg], &mut buf)?;
        Ok(buf[0])
    }
    
    pub fn write_register(&mut self, addr: u8, reg: u8, value: u8) -> Result<(), I2cError> {
        self.i2c.write(addr, &[reg, value])?;
        Ok(())
    }
    
    pub fn dump_registers(&mut self, addr: u8, count: u8) -> Result<heapless::Vec<u8, 256>, I2cError> {
        let mut data = heapless::Vec::new();
        for reg in 0..count {
            data.push(self.read_register(addr, reg)?).ok();
        }
        Ok(data)
    }
}
```

### 7.3 SPI Mode Implementation

```rust
// bus-modes/src/spi.rs
use embedded_hal::spi::SpiDevice;
use crate::traits::BusMode;

pub struct SpiMode<S> {
    spi: S,
    config: SpiConfig,
}

pub struct SpiConfig {
    pub frequency: u32,
    pub mode: SpiMode,
    pub cs_pin: u8,
    pub clk_pin: u8,
    pub mosi_pin: u8,
    pub miso_pin: u8,
}

impl<S: SpiDevice> BusMode for SpiMode<S> {
    type Config = SpiConfig;
    type Error = SpiError;
    
    fn name(&self) -> &'static str {
        "SPI"
    }
    
    fn init(&mut self, config: Self::Config) -> Result<(), Self::Error> {
        self.config = config;
        Ok(())
    }
    
    fn deinit(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<S: SpiDevice> SpiMode<S> {
    pub fn transfer(&mut self, data: &mut [u8]) -> Result<(), SpiError> {
        self.spi.transfer_in_place(data)?;
        Ok(())
    }
    
    pub fn read_flash_id(&mut self) -> Result<[u8; 3], SpiError> {
        let mut cmd = [0x9F, 0x00, 0x00, 0x00];
        self.transfer(&mut cmd)?;
        Ok([cmd[1], cmd[2], cmd[3]])
    }
}
```

### 7.4 UART Mode Implementation

```rust
// bus-modes/src/uart.rs
use embedded_hal::serial::{Read, Write};
use crate::traits::BusMode;

pub struct UartMode<U> {
    uart: U,
    config: UartConfig,
}

pub struct UartConfig {
    pub baudrate: u32,
    pub data_bits: DataBits,
    pub parity: Parity,
    pub stop_bits: StopBits,
    pub tx_pin: u8,
    pub rx_pin: u8,
}

impl<U> BusMode for UartMode<U> {
    type Config = UartConfig;
    type Error = UartError;
    
    fn name(&self) -> &'static str {
        "UART"
    }
    
    fn init(&mut self, config: Self::Config) -> Result<(), Self::Error> {
        self.config = config;
        Ok(())
    }
    
    fn deinit(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<U: Read<u8> + Write<u8>> UartMode<U> {
    pub fn write_bytes(&mut self, data: &[u8]) -> Result<(), UartError> {
        for &byte in data {
            self.uart.write(byte)?;
        }
        Ok(())
    }
    
    pub fn read_available(&mut self) -> Result<heapless::Vec<u8, 256>, UartError> {
        let mut buf = heapless::Vec::new();
        while let Ok(byte) = self.uart.read() {
            buf.push(byte).ok();
            if buf.is_full() {
                break;
            }
        }
        Ok(buf)
    }
    
    pub fn bridge<U2: Read<u8> + Write<u8>>(&mut self, other: &mut U2) -> Result<(), UartError> {
        // Bidirectional bridge between two UARTs
        loop {
            if let Ok(byte) = self.uart.read() {
                other.write(byte)?;
            }
            if let Ok(byte) = other.read() {
                self.uart.write(byte)?;
            }
        }
    }
}
```

### 7.5 1-Wire Mode Implementation

```rust
// bus-modes/src/onewire.rs
use crate::traits::BusMode;

pub struct OneWireMode<P> {
    pin: P,
    config: OneWireConfig,
}

pub struct OneWireConfig {
    pub pin: u8,
}

impl<P> BusMode for OneWireMode<P> {
    type Config = OneWireConfig;
    type Error = OneWireError;
    
    fn name(&self) -> &'static str {
        "1-WIRE"
    }
    
    fn init(&mut self, config: Self::Config) -> Result<(), Self::Error> {
        self.config = config;
        Ok(())
    }
    
    fn deinit(&mut self) -> Result<(), Self::Error> {
        Ok(())
    }
}

impl<P> OneWireMode<P> {
    pub fn reset(&mut self) -> Result<bool, OneWireError> {
        // Send reset pulse and detect presence
        todo!()
    }
    
    pub fn search_devices(&mut self) -> Result<heapless::Vec<[u8; 8], 16>, OneWireError> {
        // 1-Wire ROM search algorithm
        todo!()
    }
    
    pub fn read_rom(&mut self) -> Result<[u8; 8], OneWireError> {
        // Read ROM command (0x33)
        todo!()
    }
}
```

---

## 8. Application Layer

### 8.1 Main Application Structure

```rust
// firmware/src/main.rs
#![no_std]
#![no_main]

use hal::WaveshareS3Board;
use drivers::display::Display;
use protocol::MessageCodec;

#[entry]
fn main() -> ! {
    // Initialize board
    let board = WaveshareS3Board::new();
    
    // Initialize display
    let display = Display::new(
        board.display_spi,
        board.display_dc,
        board.display_rst,
        board.display_cs,
        board.display_bl,
    ).unwrap();
    
    // Initialize touch
    let touch = Cst328::new(
        board.touch_i2c,
        board.touch_int,
        board.touch_rst,
    );
    
    // Create application state
    let mut app = Application::new(display, touch, board);
    
    // Main loop
    loop {
        app.update();
    }
}
```

### 8.2 Command Parser

```rust
// firmware/src/cli/parser.rs
use heapless::String;

pub struct CommandParser;

impl CommandParser {
    pub fn parse(line: &str) -> Result<Command, ParseError> {
        let parts: heapless::Vec<&str, 8> = line.split_whitespace().collect();
        
        match parts.first() {
            Some(&"mode") => Self::parse_mode(&parts[1..]),
            Some(&"scan") => Ok(Command::Scan),
            Some(&"sniff") => Ok(Command::Sniff),
            Some(&"help") => Ok(Command::Help),
            _ => Err(ParseError::UnknownCommand),
        }
    }
    
    fn parse_mode(args: &[&str]) -> Result<Command, ParseError> {
        match args.first() {
            Some(&"i2c") => Ok(Command::SetMode(Mode::I2c)),
            Some(&"spi") => Ok(Command::SetMode(Mode::Spi)),
            Some(&"uart") => Ok(Command::SetMode(Mode::Uart)),
            _ => Err(ParseError::InvalidMode),
        }
    }
}
```

### 8.3 Application State Machine

```rust
// firmware/src/app.rs
pub struct Application {
    display: Display,
    touch: Cst328,
    current_mode: Mode,
    state: AppState,
}

pub enum Mode {
    HiZ,
    I2c(I2cMode),
    Spi(SpiMode),
    Uart(UartMode),
    // ... other modes
}

pub enum AppState {
    Menu,
    ModeActive,
    Configuration,
}

impl Application {
    pub fn new(display: Display, touch: Cst328, board: WaveshareS3Board) -> Self {
        Self {
            display,
            touch,
            current_mode: Mode::HiZ,
            state: AppState::Menu,
        }
    }
    
    pub fn update(&mut self) {
        // Handle touch input
        if let Some(event) = self.touch.read_touch_event().ok().flatten() {
            self.handle_touch(event);
        }
        
        // Update display
        self.draw();
    }
    
    fn handle_touch(&mut self, event: TouchEvent) {
        match self.state {
            AppState::Menu => {
                // Handle menu navigation
            }
            AppState::ModeActive => {
                // Handle mode-specific input
            }
            AppState::Configuration => {
                // Handle configuration input
            }
        }
    }
    
    fn draw(&mut self) {
        // Clear display
        self.display.clear(Rgb565::BLACK).ok();
        
        // Draw UI based on current state
        match self.state {
            AppState::Menu => self.draw_menu(),
            AppState::ModeActive => self.draw_mode_screen(),
            AppState::Configuration => self.draw_config_screen(),
        }
    }
}
```

---

## 9. Memory Management

### 9.1 Memory Layout

```
┌─────────────────────────────────┐ 0x4037_0000 (IRAM top)
│        Stack (grows down)       │
├─────────────────────────────────┤
│                                 │
│        Heap (grows up)          │
│                                 │
├─────────────────────────────────┤
│        .bss (zero-init)         │
├─────────────────────────────────┤
│        .data (initialized)      │
├─────────────────────────────────┤
│        .rodata (read-only)      │
├─────────────────────────────────┤
│        .text (code)             │
└─────────────────────────────────┘ 0x4037_8000 (IRAM start)

External PSRAM: 0x3C00_0000 - 0x3C7F_FFFF (8MB)
```

### 9.2 Static Allocation Strategy

Use `static_cell` for singleton peripherals:

```rust
use static_cell::StaticCell;

static DISPLAY: StaticCell<Display> = StaticCell::new();
static I2C_BUS: StaticCell<I2cBus> = StaticCell::new();

fn init() {
    let display = Display::new(...);
    let display_ref = DISPLAY.init(display);
    
    // Now display_ref has 'static lifetime
}
```

### 9.3 PSRAM Usage

For large buffers (e.g., display framebuffer, SD card cache), use PSRAM:

```rust
#[link_section = ".psram.bss"]
static mut FRAMEBUFFER: [u8; 240 * 320 * 2] = [0; 240 * 320 * 2];
```

### 9.4 Heap Allocator (Optional)

If a heap is needed for dynamic allocation, use `esp-alloc`:

```rust
use esp_alloc::EspHeap;

#[global_allocator]
static ALLOCATOR: EspHeap = EspHeap::empty();

fn init_heap() {
    const HEAP_SIZE: usize = 64 * 1024; // 64KB heap
    static mut HEAP: [u8; HEAP_SIZE] = [0; HEAP_SIZE];
    
    unsafe {
        ALLOCATOR.init(HEAP.as_mut_ptr(), HEAP_SIZE);
    }
}
```

**However**, prefer static allocation with `heapless` wherever possible to avoid fragmentation and OOM errors.

---

## 10. Error Handling Strategy

### 10.1 Error Types

Define a central error type:

```rust
#[derive(Debug)]
pub enum Error {
    I2c(I2cError),
    Spi(SpiError),
    Uart(UartError),
    Display(DisplayError),
    Touch(TouchError),
    Protocol(ProtocolError),
    Filesystem(FsError),
    OutOfMemory,
    InvalidState,
}

impl From<I2cError> for Error {
    fn from(e: I2cError) -> Self {
        Error::I2c(e)
    }
}

// ... similar From impls for other error types
```

### 10.2 Error Propagation

Use `?` operator extensively:

```rust
fn read_sensor(&mut self) -> Result<SensorData, Error> {
    let id = self.i2c.read_register(SENSOR_ADDR, REG_ID)?;
    let temp = self.i2c.read_register(SENSOR_ADDR, REG_TEMP)?;
    
    Ok(SensorData { id, temp })
}
```

### 10.3 Panic Handler

Implement a minimal panic handler for debugging:

```rust
use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    // Log panic info via defmt or UART
    defmt::error!("Panic: {}", defmt::Debug2Format(info));
    
    loop {
        // Halt or restart
    }
}
```

### 10.4 Logging

Use `defmt` for efficient logging:

```rust
use defmt::{info, warn, error};

info!("I2C scan found {} devices", device_count);
warn!("Retrying I2C transaction...");
error!("Failed to initialize display: {}", err);
```

---

## 11. Testing Strategy

### 11.1 Unit Tests

Test pure logic and algorithms without hardware:

```rust
// protocol/src/codec.rs
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encode_decode_roundtrip() {
        let msg = Message::I2cScan;
        let encoded = MessageCodec::encode(&msg).unwrap();
        let decoded = MessageCodec::decode(&encoded).unwrap();
        
        // Note: requires Message to derive PartialEq
        assert_eq!(msg, decoded);
    }
    
    #[test]
    fn test_crc_validation() {
        let mut frame = vec![START_BYTE, VERSION, 0x01, 0x00, 0x42];
        let crc = CRC.checksum(&frame[1..]);
        frame.extend_from_slice(&crc.to_le_bytes());
        frame.push(END_BYTE);
        
        // Corrupt CRC
        frame[frame.len() - 2] ^= 0xFF;
        
        assert!(MessageCodec::decode(&frame).is_err());
    }
}
```

### 11.2 Integration Tests (on-target)

Use `defmt-test` for on-device testing:

```rust
// tests/i2c.rs
#![no_std]
#![no_main]

use defmt_test::tests;

#[tests]
mod tests {
    use super::*;
    
    #[init]
    fn init() -> TestContext {
        let board = WaveshareS3Board::new();
        TestContext { board }
    }
    
    #[test]
    fn test_i2c_scan(ctx: &mut TestContext) {
        let mut i2c_mode = I2cMode::new(ctx.board.i2c0);
        let devices = i2c_mode.scan().unwrap();
        
        // Touch controller should be present
        assert!(devices.contains(&0x5A));
    }
}
```

### 11.3 Hardware-in-the-Loop (HIL) Tests

For protocol testing, use a second device as a "golden reference":

```
┌──────────────┐         ┌──────────────┐
│   DUT        │         │  Reference   │
│  (Rust FW)   │ <─I²C─> │  (C FW)      │
│              │         │              │
└──────────────┘         └──────────────┘
       │                        │
       └────────────────────────┘
           Test coordinator
```

### 11.4 Fuzz Testing

Use `cargo-fuzz` to test protocol parsing:

```rust
// fuzz/fuzz_targets/protocol_decode.rs
#![no_main]
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let _ = MessageCodec::decode(data);
});
```

---

## 12. Migration Phases

### Phase 1: Foundation (Weeks 1-2)
- [ ] Set up Rust project structure
- [ ] Configure Cargo workspace and toolchain
- [ ] Implement HAL wrapper for Waveshare board
- [ ] Create pin mapping definitions
- [ ] Implement basic GPIO, SPI, I²C drivers
- [ ] Test display initialization (ST7789)
- [ ] Test touch controller (CST328)
- [ ] Milestone: "Hello World" on LCD with touch response

### Phase 2: Protocol Layer (Week 3)
- [ ] Define message schema
- [ ] Implement postcard-based codec
- [ ] Add CRC validation
- [ ] Implement framing logic
- [ ] Test message encode/decode
- [ ] Implement USB CDC serial
- [ ] Test protocol over serial
- [ ] Milestone: Send/receive binary messages over USB

### Phase 3: Core Bus Modes (Weeks 4-6)
- [ ] Implement I²C mode (scan, read, write)
- [ ] Implement SPI mode (transfer, flash ID)
- [ ] Implement UART mode (read, write, bridge)
- [ ] Implement 1-Wire mode (search, ROM read)
- [ ] Test each mode with real hardware
- [ ] Implement basic sniffers (I²C, SPI)
- [ ] Milestone: Basic bus operations working

### Phase 4: GUI and Menu System (Week 7)
- [ ] Design menu structure
- [ ] Implement embedded-graphics UI
- [ ] Add touch-based navigation
- [ ] Show mode status and pinout
- [ ] Display logic analyzer traces
- [ ] Milestone: Fully functional on-screen UI

### Phase 5: Advanced Features (Weeks 8-10)
- [ ] Implement remaining bus modes (2-Wire, 3-Wire, DIO)
- [ ] Add EEPROM/Flash dump tools
- [ ] Implement SD card filesystem support
- [ ] Add Wi-Fi support (if needed)
- [ ] Implement USB HID features
- [ ] Add scripting engine (bytecode interpreter)
- [ ] Milestone: Feature parity with C version

### Phase 6: Testing and Optimization (Weeks 11-12)
- [ ] Write comprehensive unit tests
- [ ] Perform integration testing
- [ ] Optimize binary size
- [ ] Optimize performance (speed, latency)
- [ ] Stress test with real devices
- [ ] Document all features
- [ ] Milestone: Production-ready firmware

---

## 13. Implementation Guidelines

### 13.1 Code Style

Follow Rust conventions:
- Use `snake_case` for variables and functions
- Use `PascalCase` for types and traits
- Keep line length ≤ 100 characters
- Use `rustfmt` for automatic formatting
- Run `clippy` for linting

### 13.2 Unsafe Code

Minimize `unsafe` usage:
- Only use `unsafe` when interfacing with hardware registers
- Document all invariants that must be upheld
- Encapsulate `unsafe` in safe abstractions
- Example:

```rust
/// # Safety
/// This function writes directly to a hardware register.
/// Caller must ensure no other code is accessing this register concurrently.
unsafe fn write_register(addr: usize, value: u32) {
    core::ptr::write_volatile(addr as *mut u32, value);
}

// Safe wrapper
pub fn set_gpio_high(pin: u8) {
    unsafe {
        write_register(GPIO_SET_REG, 1 << pin);
    }
}
```

### 13.3 Documentation

Document all public APIs:
```rust
/// Scans the I²C bus for connected devices.
///
/// This function probes all valid 7-bit addresses (0x08-0x77)
/// and returns a list of addresses that responded with an ACK.
///
/// # Returns
/// A vector of device addresses (7-bit, unshifted).
///
/// # Errors
/// Returns `I2cError::BusError` if the bus is in an invalid state.
pub fn scan(&mut self) -> Result<heapless::Vec<u8, 128>, I2cError> {
    // ...
}
```

### 13.4 Version Control

- Keep commits atomic and focused
- Write descriptive commit messages
- Reference issues in commits (e.g., "Fix I2C timeout, closes #42")
- Use feature branches for major changes

### 13.5 Continuous Integration

Set up GitHub Actions for:
- Build checks (all targets)
- Clippy lints
- Unit tests
- Binary size tracking
- Documentation generation

Example `.github/workflows/ci.yml`:
```yaml
name: CI

on: [push, pull_request]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: dtolnay/rust-toolchain@nightly
      - name: Install Xtensa toolchain
        run: |
          cargo install espup
          espup install
      - name: Build
        run: cargo build --release
      - name: Test
        run: cargo test --lib
      - name: Clippy
        run: cargo clippy -- -D warnings
```

---

## 14. References

### 14.1 Hardware Documentation

- **Waveshare ESP32-S3-Touch-LCD-2.8:**  
  https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-2.8

- **ESP32-S3 Technical Reference Manual:**  
  https://www.espressif.com/sites/default/files/documentation/esp32-s3_technical_reference_manual_en.pdf

- **ST7789VW Datasheet:**  
  https://www.displayfuture.com/Display/datasheet/controller/ST7789.pdf

- **CST328 Datasheet:**  
  (Available from Hynitron, may require NDA)

- **QMI8658C Datasheet:**  
  https://www.qst.com/download/QMI8658C_datasheet.pdf

- **PCF85063 Datasheet:**  
  https://www.nxp.com/docs/en/data-sheet/PCF85063A.pdf

### 14.2 Rust Embedded Resources

- **Embedded Rust Book:**  
  https://docs.rust-embedded.org/book/

- **esp-rs Organization:**  
  https://github.com/esp-rs

- **esp-hal Documentation:**  
  https://docs.rs/esp-hal/latest/esp_hal/

- **embedded-graphics:**  
  https://docs.rs/embedded-graphics/latest/embedded_graphics/

- **heapless:**  
  https://docs.rs/heapless/latest/heapless/

- **postcard:**  
  https://docs.rs/postcard/latest/postcard/

### 14.3 Original Project

- **ESP32 Bus Pirate (C/Arduino):**  
  https://github.com/x0f5c3/esp32-bus-pirate

- **ESP32 Bus Pirate Wiki:**  
  https://github.com/geo-tp/ESP32-Bus-Pirate/wiki

---

## Appendix A: Crate Selection Matrix

| Functionality | Primary Crate | Alternative | Notes |
|---------------|---------------|-------------|-------|
| HAL | esp-hal | esp32s3-hal | Use latest esp-hal |
| Display Driver | st7789 | mipidsi | st7789 is specific, mipidsi is generic |
| Graphics | embedded-graphics | - | De facto standard |
| Serialization | postcard | prost-lite | postcard for simplicity |
| CRC | crc | crc-any | crc is more ergonomic |
| Logging | defmt | log | defmt has lower overhead |
| USB | esp-hal USB | usb-device | esp-hal has built-in support |
| Wi-Fi | esp-wifi | - | Part of esp-rs |
| Filesystem | embedded-sdmmc | littlefs2 | For FAT32 / LittleFS |
| Executor | embassy-executor | RTIC | Optional, if async is needed |

---

## Appendix B: Pin Assignment Table

| Peripheral | Signal | GPIO | Direction | Notes |
|------------|--------|------|-----------|-------|
| **LCD** | MOSI | 45 | Output | SPI data |
| | SCLK | 40 | Output | SPI clock |
| | CS | 42 | Output | Chip select |
| | DC | 41 | Output | Data/command |
| | RESET | 39 | Output | Reset |
| | BL | 5 | Output | Backlight (PWM) |
| **Touch** | SDA | 1 | Bidir | I²C data |
| | SCL | 3 | Output | I²C clock |
| | INT | 4 | Input | Touch interrupt |
| | RST | 2 | Output | Touch reset |
| **IMU** | (shared I²C) | 1, 3 | - | QMI8658C |
| **RTC** | (shared I²C) | 1, 3 | - | PCF85063 |
| **Audio** | BCLK | TBD | Output | I²S bit clock |
| | LRCK | TBD | Output | I²S L/R clock |
| | DATA | TBD | Output | I²S data |
| **SD Card** | (shared SPI) | TBD | - | Check schematic |
| **USB** | D+ | 20 | Bidir | Native USB |
| | D- | 19 | Bidir | Native USB |

**Note:** Some pins are still TBD and need to be verified from the Waveshare schematic.

---

## Appendix C: Build Configuration

### rust-toolchain.toml
```toml
[toolchain]
channel = "nightly-2024-12-01"
components = ["rust-src", "rustfmt", "clippy"]
```

### .cargo/config.toml
```toml
[build]
target = "xtensa-esp32s3-none-elf"

[target.xtensa-esp32s3-none-elf]
runner = "espflash flash --monitor"

[unstable]
build-std = ["core", "alloc"]
```

### Cargo.toml (workspace root)
```toml
[workspace]
members = ["hal", "drivers", "protocol", "bus-modes", "firmware"]
resolver = "2"

[workspace.package]
version = "0.1.0"
authors = ["ESP32 Bus Pirate Contributors"]
edition = "2021"
license = "MIT OR Apache-2.0"

[workspace.dependencies]
esp-hal = { version = "0.15", features = ["esp32s3"] }
embedded-hal = "1.0"
embedded-graphics = "0.8"
heapless = "0.8"
postcard = { version = "1.0", default-features = false, features = ["heapless"] }
crc = { version = "3.0", default-features = false }
defmt = "0.3"
serde = { version = "1.0", default-features = false, features = ["derive"] }

[profile.release]
opt-level = "z"       # Optimize for size
lto = true            # Link-time optimization
codegen-units = 1     # Better optimization
strip = true          # Strip symbols
```

---

## Appendix D: Memory Budget

| Component | Estimated Size | Notes |
|-----------|----------------|-------|
| .text (code) | ~150KB | Depends on feature flags |
| .rodata | ~20KB | Strings, lookup tables |
| .data | ~5KB | Initialized globals |
| .bss | ~10KB | Zero-initialized globals |
| Stack (per core) | 8KB × 2 | 16KB total |
| Heap (if used) | 64KB | For dynamic allocation |
| Framebuffer (PSRAM) | 153KB | 240×320×2 bytes |
| **Total SRAM** | ~270KB | Out of 512KB available |
| **Total Flash** | ~200KB | Out of 16MB available |

Plenty of headroom for additional features.

---

## Appendix E: Glossary

- **HAL:** Hardware Abstraction Layer
- **PSRAM:** Pseudo-Static RAM (external DRAM with SRAM-like interface)
- **QSPI:** Quad SPI (4-bit SPI for higher bandwidth)
- **RMT:** Remote Control Transceiver (ESP32 peripheral for IR, WS2812, etc.)
- **CRC:** Cyclic Redundancy Check
- **NVS:** Non-Volatile Storage (key-value store in flash)
- **HIL:** Hardware-in-the-Loop
- **DUT:** Device Under Test
- **no_std:** Rust without the standard library (for embedded)
- **postcard:** Compact binary serialization format
- **defmt:** Efficient logging framework for embedded systems
- **heapless:** Data structures without heap allocation

---

**End of Design Document**

This document will be updated as implementation progresses and new requirements emerge.
