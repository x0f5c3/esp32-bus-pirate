#![no_std]

//! Bus Pirate mode implementations
//!
//! This crate implements all the protocol modes supported by the Bus Pirate:
//! I2C, SPI, UART, 1-Wire, 2-Wire, 3-Wire, DIO, etc.

pub mod traits;
pub mod i2c;
pub mod spi;
pub mod uart;
// pub mod onewire;
// pub mod twowire;
// pub mod threewire;
// pub mod dio;

pub use traits::{BusMode, Scanner, Sniffer};

/// Common error type for bus operations
#[derive(Debug)]
pub enum Error {
    /// Communication error
    Communication,
    /// Device not found or not responding
    NoDevice,
    /// Operation timeout
    Timeout,
    /// Invalid configuration
    InvalidConfig,
    /// Bus already in use
    Busy,
}
