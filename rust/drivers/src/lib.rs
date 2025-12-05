#![no_std]

//! Device drivers for Waveshare ESP32-S3-Touch-LCD-2.8 peripherals
//!
//! This crate provides drivers for:
//! - ST7789 display controller
//! - CST328 capacitive touch controller
//! - QMI8658C IMU (accelerometer + gyroscope)
//! - PCF85063 RTC
//! - PCM5101A audio codec

pub mod display;
pub mod touch;
pub mod imu;
pub mod rtc;
pub mod audio;

pub use display::Display;
pub use touch::Cst328;

/// Common error type for all drivers
#[derive(Debug)]
pub enum Error {
    /// Communication error (I2C, SPI)
    Communication,
    /// Invalid configuration or parameters
    InvalidConfig,
    /// Device not responding
    NoDevice,
    /// Operation timeout
    Timeout,
}

impl From<embedded_hal::spi::ErrorKind> for Error {
    fn from(_: embedded_hal::spi::ErrorKind) -> Self {
        Error::Communication
    }
}

impl From<embedded_hal::i2c::ErrorKind> for Error {
    fn from(_: embedded_hal::i2c::ErrorKind) -> Self {
        Error::Communication
    }
}
