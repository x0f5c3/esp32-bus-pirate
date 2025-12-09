//! I2C peripheral wrapper for ESP32-S3
//!
//! This module provides safe abstractions over the ESP32-S3 I2C peripheral,
//! implementing the `embedded-hal` I2C traits.
//!
//! # Example
//!
//! ```no_run
//! use esp32_bus_pirate_hal::peripherals::i2c::I2cConfig;
//! use fugit::HertzU32;
//!
//! let config = I2cConfig::default()
//!     .with_frequency(HertzU32::kHz(100));
//! ```

use esp_hal::i2c::I2C;
use esp_hal::peripherals::I2C0;
use embedded_hal::i2c::{Error as I2cError, ErrorKind, ErrorType, I2c, Operation, SevenBitAddress, TenBitAddress};

/// I2C configuration
#[derive(Debug, Clone, Copy)]
pub struct I2cConfig {
    /// I2C bus frequency in Hz (typically 100kHz for standard mode, 400kHz for fast mode)
    pub frequency: fugit::HertzU32,
    /// Timeout for I2C operations in milliseconds
    pub timeout_ms: u32,
}

impl Default for I2cConfig {
    fn default() -> Self {
        Self {
            frequency: fugit::HertzU32::kHz(100), // 100kHz standard mode
            timeout_ms: 1000, // 1 second timeout
        }
    }
}

impl I2cConfig {
    /// Create a new I2C configuration
    pub fn new(frequency: fugit::HertzU32) -> Self {
        Self {
            frequency,
            timeout_ms: 1000,
        }
    }

    /// Set the I2C frequency
    pub fn with_frequency(mut self, frequency: fugit::HertzU32) -> Self {
        self.frequency = frequency;
        self
    }

    /// Set the operation timeout
    pub fn with_timeout_ms(mut self, timeout_ms: u32) -> Self {
        self.timeout_ms = timeout_ms;
        self
    }
}

/// Custom I2C error type
#[derive(Debug, Clone, Copy)]
pub enum I2cErrorWrapper {
    /// Bus error (arbitration lost, bus collision, etc.)
    Bus,
    /// No acknowledgment received
    NoAcknowledge,
    /// Operation timed out
    Timeout,
    /// Other hardware error
    Other,
}

impl I2cError for I2cErrorWrapper {
    fn kind(&self) -> ErrorKind {
        match self {
            I2cErrorWrapper::Bus => ErrorKind::Bus,
            I2cErrorWrapper::NoAcknowledge => ErrorKind::NoAcknowledge(embedded_hal::i2c::NoAcknowledgeSource::Unknown),
            I2cErrorWrapper::Timeout => ErrorKind::Other,
            I2cErrorWrapper::Other => ErrorKind::Other,
        }
    }
}

/// I2C peripheral wrapper
///
/// This wrapper provides a safe interface to the ESP32-S3 I2C peripheral
/// and implements the `embedded-hal` I2C traits.
pub struct I2cBus<'d> {
    i2c: I2C<'d, I2C0>,
    config: I2cConfig,
}

impl<'d> I2cBus<'d> {
    /// Create a new I2C bus wrapper
    ///
    /// # Arguments
    ///
    /// * `i2c` - The ESP-HAL I2C peripheral
    /// * `config` - Configuration for the I2C bus
    pub fn new(i2c: I2C<'d, I2C0>, config: I2cConfig) -> Self {
        Self { i2c, config }
    }

    /// Get the current configuration
    pub fn config(&self) -> &I2cConfig {
        &self.config
    }

    /// Get a mutable reference to the underlying I2C peripheral
    pub fn inner_mut(&mut self) -> &mut I2C<'d, I2C0> {
        &mut self.i2c
    }
}

impl<'d> ErrorType for I2cBus<'d> {
    type Error = I2cErrorWrapper;
}

impl<'d> I2c<SevenBitAddress> for I2cBus<'d> {
    fn transaction(
        &mut self,
        address: SevenBitAddress,
        operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        for op in operations {
            match op {
                Operation::Read(buf) => {
                    self.i2c
                        .read(address, buf)
                        .map_err(|_| I2cErrorWrapper::Other)?;
                }
                Operation::Write(buf) => {
                    self.i2c
                        .write(address, buf)
                        .map_err(|_| I2cErrorWrapper::Other)?;
                }
            }
        }
        Ok(())
    }
}

impl<'d> I2c<TenBitAddress> for I2cBus<'d> {
    fn transaction(
        &mut self,
        _address: TenBitAddress,
        _operations: &mut [Operation<'_>],
    ) -> Result<(), Self::Error> {
        // ESP32-S3 I2C hardware doesn't natively support 10-bit addressing
        // It would need to be implemented in software by using the 10-bit
        // addressing scheme manually. For now, we return an error.
        Err(I2cErrorWrapper::Other)
    }
}

/// Extension trait for convenient I2C operations
pub trait I2cExt {
    /// Scan the I2C bus for devices
    ///
    /// Returns a list of addresses that responded to a write operation.
    /// This is useful for discovering I2C devices on the bus.
    fn scan(&mut self) -> heapless::Vec<u8, 128>;

    /// Read a single byte from a device register
    fn read_register(&mut self, address: u8, register: u8) -> Result<u8, I2cErrorWrapper>;

    /// Write a single byte to a device register
    fn write_register(&mut self, address: u8, register: u8, value: u8) -> Result<(), I2cErrorWrapper>;

    /// Read multiple bytes from a device register
    fn read_registers(&mut self, address: u8, register: u8, buffer: &mut [u8]) -> Result<(), I2cErrorWrapper>;

    /// Write multiple bytes to a device register
    fn write_registers(&mut self, address: u8, register: u8, data: &[u8]) -> Result<(), I2cErrorWrapper>;
}

impl<'d> I2cExt for I2cBus<'d> {
    fn scan(&mut self) -> heapless::Vec<u8, 128> {
        let mut devices = heapless::Vec::new();
        
        // Scan addresses 0x08 to 0x77 (valid 7-bit I2C addresses)
        for addr in 0x08..=0x77 {
            // Try to write zero bytes to check if device responds
            if self.i2c.write(addr, &[]).is_ok() {
                let _ = devices.push(addr);
            }
        }
        
        devices
    }

    fn read_register(&mut self, address: u8, register: u8) -> Result<u8, I2cErrorWrapper> {
        let mut buf = [0u8; 1];
        self.i2c
            .write_read(address, &[register], &mut buf)
            .map_err(|_| I2cErrorWrapper::Other)?;
        Ok(buf[0])
    }

    fn write_register(&mut self, address: u8, register: u8, value: u8) -> Result<(), I2cErrorWrapper> {
        self.i2c
            .write(address, &[register, value])
            .map_err(|_| I2cErrorWrapper::Other)
    }

    fn read_registers(&mut self, address: u8, register: u8, buffer: &mut [u8]) -> Result<(), I2cErrorWrapper> {
        self.i2c
            .write_read(address, &[register], buffer)
            .map_err(|_| I2cErrorWrapper::Other)
    }

    fn write_registers(&mut self, address: u8, register: u8, data: &[u8]) -> Result<(), I2cErrorWrapper> {
        // Create a buffer with register address followed by data
        let mut buf = heapless::Vec::<u8, 256>::new();
        if buf.push(register).is_err() {
            return Err(I2cErrorWrapper::Other);
        }
        for &byte in data {
            if buf.push(byte).is_err() {
                return Err(I2cErrorWrapper::Other);
            }
        }
        
        self.i2c
            .write(address, &buf)
            .map_err(|_| I2cErrorWrapper::Other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i2c_config_default() {
        let config = I2cConfig::default();
        assert_eq!(config.frequency.to_Hz(), 100_000);
        assert_eq!(config.timeout_ms, 1000);
    }

    #[test]
    fn test_i2c_config_builder() {
        let config = I2cConfig::default()
            .with_frequency(fugit::HertzU32::kHz(400))
            .with_timeout_ms(500);
        assert_eq!(config.frequency.to_Hz(), 400_000);
        assert_eq!(config.timeout_ms, 500);
    }
}
