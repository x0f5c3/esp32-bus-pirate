//! I2C bus mode implementation

use crate::{traits::{BusMode, Scanner}, Error};
use embedded_hal::i2c::I2c;
use heapless::Vec;

/// I2C bus mode
pub struct I2cMode<I> {
    i2c: I,
    config: Option<I2cConfig>,
}

/// I2C configuration
#[derive(Debug, Clone, Copy)]
pub struct I2cConfig {
    pub frequency: u32,
}

impl<I: I2c> I2cMode<I> {
    /// Create a new I2C mode instance
    pub fn new(i2c: I) -> Self {
        Self { i2c, config: None }
    }
}

impl<I: I2c> BusMode for I2cMode<I> {
    type Config = I2cConfig;
    
    fn name(&self) -> &'static str {
        "I2C"
    }
    
    fn init(&mut self, config: Self::Config) -> Result<(), Error> {
        self.config = Some(config);
        Ok(())
    }
    
    fn deinit(&mut self) -> Result<(), Error> {
        self.config = None;
        Ok(())
    }
}

impl<I: I2c> Scanner for I2cMode<I> {
    type DeviceId = u8;
    
    fn scan(&mut self) -> Result<Vec<u8, 128>, Error> {
        let mut devices = Vec::new();
        
        // Scan all valid 7-bit addresses (0x08-0x77)
        for addr in 0x08..=0x77 {
            // Try to write zero bytes to detect device presence
            if self.i2c.write(addr, &[]).is_ok() {
                devices.push(addr).ok();
            }
        }
        
        Ok(devices)
    }
}

impl<I: I2c> I2cMode<I> {
    /// Read a register from an I2C device
    pub fn read_register(&mut self, addr: u8, reg: u8) -> Result<u8, Error> {
        let mut buf = [0u8; 1];
        self.i2c
            .write_read(addr, &[reg], &mut buf)
            .map_err(|_| Error::Communication)?;
        Ok(buf[0])
    }
    
    /// Write a register to an I2C device
    pub fn write_register(&mut self, addr: u8, reg: u8, value: u8) -> Result<(), Error> {
        self.i2c
            .write(addr, &[reg, value])
            .map_err(|_| Error::Communication)
    }
    
    /// Read multiple bytes from an I2C device
    pub fn read(&mut self, addr: u8, buffer: &mut [u8]) -> Result<(), Error> {
        self.i2c.read(addr, buffer).map_err(|_| Error::Communication)
    }
    
    /// Write multiple bytes to an I2C device
    pub fn write(&mut self, addr: u8, data: &[u8]) -> Result<(), Error> {
        self.i2c.write(addr, data).map_err(|_| Error::Communication)
    }
}
