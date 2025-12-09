//! Pin definitions for Waveshare ESP32-S3-Touch-LCD-2.8 board
//!
//! Based on Waveshare documentation:
//! https://www.waveshare.com/wiki/ESP32-S3-Touch-LCD-2.8

/// LCD (ST7789) pin assignments
pub mod display {
    pub const MOSI: u8 = 45;
    pub const SCLK: u8 = 40;
    pub const CS: u8 = 42;
    pub const DC: u8 = 41;
    pub const RESET: u8 = 39;
    pub const BACKLIGHT: u8 = 5;
}

/// Touch controller (CST328) pin assignments
pub mod touch {
    pub const SDA: u8 = 1;
    pub const SCL: u8 = 3;
    pub const INT: u8 = 4;
    pub const RST: u8 = 2;
    
    /// I2C address for CST328
    pub const I2C_ADDR: u8 = 0x5A;
}

/// IMU (QMI8658C) configuration
/// Uses shared I2C bus with touch controller
pub mod imu {
    pub use super::touch::{SDA, SCL};
    
    /// I2C address for QMI8658C
    pub const I2C_ADDR: u8 = 0x6B;
}

/// RTC (PCF85063) configuration
/// Uses shared I2C bus with touch controller
pub mod rtc {
    pub use super::touch::{SDA, SCL};
    
    /// I2C address for PCF85063
    pub const I2C_ADDR: u8 = 0x51;
}

/// Audio codec (PCM5101A) pin assignments
/// Note: Actual pins TBD - need to verify from schematic
pub mod audio {
    pub const BCLK: u8 = 0; // TBD
    pub const LRCK: u8 = 0; // TBD
    pub const DATA: u8 = 0; // TBD
}

/// SD card pin assignments
/// Note: Shares SPI3 bus (different from display SPI2)
pub mod sdcard {
    pub const MISO: u8 = 16;  // SD_D0
    pub const MOSI: u8 = 17;  // SD_CMD
    pub const SCLK: u8 = 14;  // SD_SCK
    pub const CS: u8 = 21;    // SD_D3
}

/// USB native peripheral
pub mod usb {
    pub const DP: u8 = 20;
    pub const DM: u8 = 19;
}

/// User-assignable GPIO pins for Bus Pirate modes
/// These are the pins available for protocol operations
pub mod bus {
    /// GPIO pins available for I2C, SPI, UART, etc.
    /// Excludes pins used by on-board peripherals
    pub const AVAILABLE_PINS: &[u8] = &[
        6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 
        21, // May be used for status LED
        // GPIO 19-20 are USB
        // GPIO 1-5, 39-42, 45 are used by display/touch
    ];
}
