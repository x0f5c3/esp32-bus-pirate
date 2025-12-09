#![no_std]

//! Hardware Abstraction Layer for Waveshare ESP32-S3-Touch-LCD-2.8 board
//!
//! This crate provides board-specific initialization and pin mappings
//! for the Waveshare ESP32-S3-Touch-LCD-2.8 development board.

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