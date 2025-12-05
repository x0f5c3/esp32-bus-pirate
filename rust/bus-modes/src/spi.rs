//! SPI bus mode implementation

use crate::{traits::BusMode, Error};
use embedded_hal::spi::SpiDevice;

/// SPI bus mode
pub struct SpiMode<S> {
    spi: S,
    config: Option<SpiConfig>,
}

/// SPI configuration
#[derive(Debug, Clone, Copy)]
pub struct SpiConfig {
    pub frequency: u32,
}

impl<S: SpiDevice> SpiMode<S> {
    /// Create a new SPI mode instance
    pub fn new(spi: S) -> Self {
        Self { spi, config: None }
    }
    
    /// Transfer data (full duplex)
    pub fn transfer(&mut self, data: &mut [u8]) -> Result<(), Error> {
        self.spi
            .transfer_in_place(data)
            .map_err(|_| Error::Communication)
    }
    
    /// Read Flash ID (common SPI Flash command)
    pub fn read_flash_id(&mut self) -> Result<[u8; 3], Error> {
        let mut cmd = [0x9F, 0x00, 0x00, 0x00];
        self.transfer(&mut cmd)?;
        Ok([cmd[1], cmd[2], cmd[3]])
    }
}

impl<S: SpiDevice> BusMode for SpiMode<S> {
    type Config = SpiConfig;
    
    fn name(&self) -> &'static str {
        "SPI"
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
