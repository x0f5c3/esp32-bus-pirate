# Waveshare ESP32-S3-Touch-LCD-2.8

## Overview

The **Waveshare ESP32-S3-Touch-LCD-2.8** is a compact development board featuring the ESP32-S3 microcontroller with a built-in 2.8" capacitive touch display. It's the primary hardware target for the ESP32 Bus Pirate project.

![Waveshare ESP32-S3-Touch-LCD-2.8](https://www.waveshare.com/w/upload/thumb/5/50/ESP32-S3-Touch-LCD-2.8-1.jpg/400px-ESP32-S3-Touch-LCD-2.8-1.jpg)

**Purchase Links:**
- [Waveshare Official Store](https://www.waveshare.com/esp32-s3-touch-lcd-2.8.htm)
- [Amazon](https://www.amazon.com/s?k=waveshare+esp32-s3+touch+lcd+2.8)
- [AliExpress](https://www.aliexpress.com/w/wholesale-waveshare-esp32-s3-touch-lcd-2.8.html)

**Price:** ~$15-20 USD

## Specifications

### Microcontroller: ESP32-S3

- **CPU:** Xtensa LX7 dual-core @ 240 MHz
- **RAM:** 512 KB SRAM
- **PSRAM:** 8 MB QSPI PSRAM  
- **Flash:** 16 MB
- **Wi-Fi:** 802.11 b/g/n (2.4 GHz)
- **Bluetooth:** Bluetooth LE 5.0
- **USB:** Native USB-OTG (Full Speed)

### Display: ST7789VW

- **Size:** 2.8 inches
- **Resolution:** 240×320 pixels (RGB565)
- **Controller:** ST7789VW
- **Interface:** SPI
- **Viewing Angle:** 170°
- **Backlight:** LED, PWM dimmable

### Touch: CST328

- **Type:** Capacitive touch
- **Points:** Up to 5 simultaneous touches
- **Interface:** I²C
- **Controller:** CST328 (Hynitron)

### Sensors

**IMU: QMI8658C**
- 6-axis: 3-axis accelerometer + 3-axis gyroscope
- Interface: I²C (shared with touch)
- Applications: Orientation detection, gesture control

**RTC: PCF85063**
- Real-Time Clock with battery backup
- Interface: I²C (shared bus)
- Accuracy: ±3 ppm

### Audio

**Codec: PCM5101A**
- 32-bit stereo DAC
- Interface: I²S
- Sample Rate: Up to 384 kHz
- Built-in speaker connector

### Storage

- **MicroSD Card Slot:** SDIO/SPI interface
- **Internal Flash:** 16 MB for firmware and data

### Power

- **USB-C:** 5V power and programming
- **Battery Connector:** JST-PH 2.0mm for 3.7V LiPo
- **Charging:** Built-in charging circuit (TP4056-based)
- **Consumption:** 
  - Active: 120-250 mA (depending on Wi-Fi/BT usage)
  - Deep Sleep: ~0.5 mA

### Physical

- **Dimensions:** 58mm × 43mm
- **Weight:** ~25g
- **Mounting Holes:** 4× M3
- **Operating Temperature:** -20°C to +70°C

## Pin Mappings

### Display (ST7789) - SPI2

| Signal | GPIO | Direction | Notes |
|--------|------|-----------|-------|
| MOSI | 45 | Output | SPI Data Out |
| SCLK | 40 | Output | SPI Clock |
| CS | 42 | Output | Chip Select (active low) |
| DC | 41 | Output | Data/Command |
| RESET | 39 | Output | Hardware Reset |
| Backlight | 5 | Output | PWM control |

**SPI Configuration:**
- Frequency: 40 MHz
- Mode: Mode 0 (CPOL=0, CPHA=0)
- Bit Order: MSB first

### Touch Controller (CST328) - I²C0

| Signal | GPIO | Direction | Notes |
|--------|------|-----------|-------|
| SDA | 1 | Bidirectional | I²C Data |
| SCL | 3 | Output | I²C Clock |
| INT | 4 | Input | Touch Interrupt |
| RST | 2 | Output | Hardware Reset |

**I²C Configuration:**
- Address: 0x5A (7-bit)
- Frequency: 100 kHz

### Shared I²C Bus Devices

The following devices share I²C0 with the touch controller:

- **QMI8658C IMU:** Address 0x6B
- **PCF85063 RTC:** Address 0x51

### Available GPIO for Bus Pirate Modes

The following GPIOs are available for protocol operations:

| GPIO | Notes |
|------|-------|
| 6-18 | General purpose |
| 21 | Optional status LED |

**Reserved/In Use:**
- GPIO 1-5: Display and touch
- GPIO 19-20: USB D-/D+
- GPIO 39-45: Display SPI

## Block Diagram

```mermaid
graph TB
    subgraph ESP32-S3
        CPU[Xtensa LX7<br/>Dual Core<br/>240MHz]
        RAM[512KB SRAM<br/>8MB PSRAM]
        FLASH[16MB Flash]
        WIFI[Wi-Fi 2.4GHz]
        BT[Bluetooth LE 5.0]
        USB[USB OTG]
        
        SPI2[SPI2 Peripheral]
        I2C0[I2C0 Peripheral]
        I2S0[I2S0 Peripheral]
        SDMMC[SDMMC Peripheral]
    end
    
    subgraph Peripherals
        LCD[ST7789<br/>2.8" LCD<br/>240x320]
        TOUCH[CST328<br/>Capacitive Touch]
        IMU[QMI8658C<br/>6-axis IMU]
        RTC[PCF85063<br/>RTC]
        AUDIO[PCM5101A<br/>Audio DAC]
        SD[MicroSD Card]
        BATTERY[LiPo Battery<br/>JST Connector]
    end
    
    CPU --> SPI2
    CPU --> I2C0
    CPU --> I2S0
    CPU --> SDMMC
    
    SPI2 --> LCD
    SPI2 --> SD
    I2C0 --> TOUCH
    I2C0 --> IMU
    I2C0 --> RTC
    I2S0 --> AUDIO
    
    BATTERY --> CPU
    USB --> CPU
    
    style LCD fill:#e1f5ff
    style TOUCH fill:#ffe1f5
    style ESP32-S3 fill:#f5f5dc
```

## Power Modes

### Normal Operation
- **CPU:** 240 MHz (both cores)
- **Peripherals:** All active
- **Consumption:** 120-180 mA (display on), 200-250 mA (Wi-Fi active)

### Light Sleep
- **CPU:** Clock gated
- **RTC:** Active
- **Wake:** GPIO, timer, or touch
- **Consumption:** 3-5 mA

### Deep Sleep
- **CPU:** Powered down
- **RTC:** Active (minimal)
- **Wake:** GPIO or timer only
- **Consumption:** ~0.5 mA

## Getting Started

1. **Connect USB-C** to your computer
2. **Install drivers** if needed (usually automatic on Windows 10+/Linux/macOS)
3. **Flash firmware** using [Quick Start Guide](../getting-started/quick-start.md)
4. **Connect battery** (optional) for portable operation

## Pinout Diagram

```
                    ╔══════════════════════════════════╗
                    ║  Waveshare ESP32-S3-Touch-2.8   ║
                    ║                                  ║
          ┌─────────║         [2.8" Display]          ║─────────┐
          │ GPIO6   ║                                  ║  GPIO7  │
          │ GPIO8   ║                                  ║  GPIO9  │
          │ GPIO10  ║                                  ║  GPIO11 │
          │ GPIO12  ║                                  ║  GPIO13 │
          │ GPIO14  ║      [Touch Sensitive Area]     ║  GPIO15 │
          │ GPIO16  ║                                  ║  GPIO17 │
          │ GPIO18  ║                                  ║  GPIO21 │
          │ 3V3     ║                                  ║  GND    │
          │ GND     ║                                  ║  5V     │
          └─────────║      USB-C    [BOOT]  [RST]     ║─────────┘
                    ║       ▼         □      □         ║
                    ╚══════════════════════════════════╝
                              Battery JST ○○
```

## Comparison with Other Boards

| Feature | Waveshare S3 LCD 2.8 | M5Stack Core2 | LILYGO T-Display-S3 |
|---------|---------------------|---------------|---------------------|
| MCU | ESP32-S3 | ESP32 | ESP32-S3 |
| Display | 2.8" 240×320 | 2.0" 320×240 | 1.9" 170×320 |
| Touch | Capacitive (CST328) | Capacitive (FT6336) | No |
| IMU | QMI8658C | MPU6886 | No |
| Battery | JST (optional) | Built-in | JST (optional) |
| Price | ~$15-20 | ~$50-60 | ~$12-15 |

## Resources

- [Official Wiki](https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-2.8)
- [Schematic PDF](https://www.waveshare.com/w/upload/0/04/ESP32-S3-Touch-LCD-2.8_Schematic.pdf)
- [Example Code (C)](https://github.com/Waveshare/ESP32-S3-Touch-LCD-2.8)
- [ESP32-S3 Datasheet](https://www.espressif.com/sites/default/files/documentation/esp32-s3_datasheet_en.pdf)

## See Also

- [Battery Operation](./battery.md)
- [Quick Start Guide](../getting-started/quick-start.md)
- [Development Guide](../development/rust-migration.md)
