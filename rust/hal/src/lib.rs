#![cfg_attr(not(test), no_std)]
#![doc = include_str!("../README.md")]

//! # ESP32 Bus Pirate HAL
//!
//! Hardware Abstraction Layer for the Waveshare ESP32-S3-Touch-LCD-2.8 board.
//!
//! This crate provides safe, `no_std` abstractions over ESP32-S3 peripherals
//! with full `embedded-hal` trait implementations for maximum portability.
//!
//! ## Quick Start
//!
//! ```no_run
//! use esp32_bus_pirate_hal::WaveshareS3Board;
//!
//! // Initialize the board
//! let mut board = WaveshareS3Board::new();
//!
//! // Initialize peripherals
//! board.init_display();
//! board.init_touch();
//! board.set_backlight(true);
//! ```
//!
//! ## Modules
//!
//! - [`board`]: Board initialization and peripheral management
//! - [`pins`]: Pin definitions for all on-board peripherals
//! - [`peripherals`]: Safe peripheral wrappers (I2C, SPI, UART, GPIO)
//!
//! ## Features
//!
//! - Complete board initialization at 240 MHz
//! - I2C @ 100kHz for touch, IMU, and RTC
//! - SPI2 @ 40MHz for display
//! - SPI3 @ 20MHz for SD card
//! - UART with configurable parameters
//! - GPIO with PWM and interrupt support

pub mod board;
pub mod pins;
pub mod peripherals;

pub use board::WaveshareS3Board;
pub use pins::*;

// Re-export commonly used types
pub use esp_hal::{
    gpio::{Input, Level, Output, OutputConfig, Pull},
    i2c::master::I2c,
    peripherals::Peripherals,
    spi::master::{Config as SpiConfig, Spi},
    time::Duration,
    timer::timg::TimerGroup,
    uart::Uart,
};
pub use fugit::ExtU64;
