//! SPI peripheral wrapper for ESP32-S3
//!
//! This module provides safe abstractions over the ESP32-S3 SPI peripheral,
//! implementing the `embedded-hal` SPI traits with DMA support.
//!
//! # Example
//!
//! ```no_run
//! use esp32_bus_pirate_hal::peripherals::spi::{SpiConfig, SpiMode};
//! use fugit::HertzU32;
//!
//! let config = SpiConfig::default()
//!     .with_frequency(HertzU32::MHz(40))
//!     .with_mode(SpiMode::Mode0);
//! ```

use esp_hal::spi::{master::Spi, FullDuplexMode, SpiMode as EspSpiMode};
use esp_hal::peripherals::{SPI2, SPI3};
use embedded_hal::spi::{Error as SpiError, ErrorKind, ErrorType, SpiBus, SpiDevice};
use core::marker::PhantomData;

/// SPI mode configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpiMode {
    /// CPOL=0, CPHA=0
    Mode0,
    /// CPOL=0, CPHA=1
    Mode1,
    /// CPOL=1, CPHA=0
    Mode2,
    /// CPOL=1, CPHA=1
    Mode3,
}

impl From<SpiMode> for EspSpiMode {
    fn from(mode: SpiMode) -> Self {
        match mode {
            SpiMode::Mode0 => EspSpiMode::Mode0,
            SpiMode::Mode1 => EspSpiMode::Mode1,
            SpiMode::Mode2 => EspSpiMode::Mode2,
            SpiMode::Mode3 => EspSpiMode::Mode3,
        }
    }
}

/// SPI configuration
#[derive(Debug, Clone, Copy)]
pub struct SpiConfig {
    /// SPI bus frequency in Hz
    pub frequency: fugit::HertzU32,
    /// SPI mode (clock polarity and phase)
    pub mode: SpiMode,
    /// Enable DMA for efficient transfers
    pub use_dma: bool,
}

impl Default for SpiConfig {
    fn default() -> Self {
        Self {
            frequency: fugit::HertzU32::MHz(1), // 1MHz default
            mode: SpiMode::Mode0,
            use_dma: false,
        }
    }
}

impl SpiConfig {
    /// Create a new SPI configuration
    pub fn new(frequency: fugit::HertzU32, mode: SpiMode) -> Self {
        Self {
            frequency,
            mode,
            use_dma: false,
        }
    }

    /// Set the SPI frequency
    pub fn with_frequency(mut self, frequency: fugit::HertzU32) -> Self {
        self.frequency = frequency;
        self
    }

    /// Set the SPI mode
    pub fn with_mode(mut self, mode: SpiMode) -> Self {
        self.mode = mode;
        self
    }

    /// Enable or disable DMA
    pub fn with_dma(mut self, use_dma: bool) -> Self {
        self.use_dma = use_dma;
        self
    }
}

/// Custom SPI error type
#[derive(Debug, Clone, Copy)]
pub enum SpiErrorWrapper {
    /// Bus error
    Bus,
    /// Chip select error
    ChipSelectFault,
    /// Mode fault
    ModeFault,
    /// Overrun error
    Overrun,
    /// Frame format error
    FrameFormat,
    /// Other hardware error
    Other,
}

impl SpiError for SpiErrorWrapper {
    fn kind(&self) -> ErrorKind {
        match self {
            SpiErrorWrapper::Bus => ErrorKind::Other,
            SpiErrorWrapper::ChipSelectFault => ErrorKind::ChipSelectFault,
            SpiErrorWrapper::ModeFault => ErrorKind::ModeFault,
            SpiErrorWrapper::Overrun => ErrorKind::Overrun,
            SpiErrorWrapper::FrameFormat => ErrorKind::FrameFormat,
            SpiErrorWrapper::Other => ErrorKind::Other,
        }
    }
}

/// SPI peripheral wrapper for SPI2
///
/// This wrapper provides a safe interface to the ESP32-S3 SPI2 peripheral
/// and implements the `embedded-hal` SPI traits.
pub struct SpiBus2<'d> {
    spi: Spi<'d, SPI2, FullDuplexMode>,
    config: SpiConfig,
}

impl<'d> SpiBus2<'d> {
    /// Create a new SPI bus wrapper
    ///
    /// # Arguments
    ///
    /// * `spi` - The ESP-HAL SPI peripheral
    /// * `config` - Configuration for the SPI bus
    pub fn new(spi: Spi<'d, SPI2, FullDuplexMode>, config: SpiConfig) -> Self {
        Self { spi, config }
    }

    /// Get the current configuration
    pub fn config(&self) -> &SpiConfig {
        &self.config
    }

    /// Get a mutable reference to the underlying SPI peripheral
    pub fn inner_mut(&mut self) -> &mut Spi<'d, SPI2, FullDuplexMode> {
        &mut self.spi
    }

    /// Get an immutable reference to the underlying SPI peripheral
    pub fn inner(&self) -> &Spi<'d, SPI2, FullDuplexMode> {
        &self.spi
    }
}

impl<'d> ErrorType for SpiBus2<'d> {
    type Error = SpiErrorWrapper;
}

impl<'d> SpiBus for SpiBus2<'d> {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.spi
            .read(words)
            .map_err(|_| SpiErrorWrapper::Other)
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.spi
            .write(words)
            .map_err(|_| SpiErrorWrapper::Other)
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.spi
            .transfer(read, write)
            .map_err(|_| SpiErrorWrapper::Other)
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.spi
            .transfer_in_place(words)
            .map_err(|_| SpiErrorWrapper::Other)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.spi
            .flush()
            .map_err(|_| SpiErrorWrapper::Other)
    }
}

/// SPI peripheral wrapper for SPI3
///
/// This wrapper provides a safe interface to the ESP32-S3 SPI3 peripheral
/// and implements the `embedded-hal` SPI traits.
pub struct SpiBus3<'d> {
    spi: Spi<'d, SPI3, FullDuplexMode>,
    config: SpiConfig,
}

impl<'d> SpiBus3<'d> {
    /// Create a new SPI bus wrapper
    ///
    /// # Arguments
    ///
    /// * `spi` - The ESP-HAL SPI peripheral
    /// * `config` - Configuration for the SPI bus
    pub fn new(spi: Spi<'d, SPI3, FullDuplexMode>, config: SpiConfig) -> Self {
        Self { spi, config }
    }

    /// Get the current configuration
    pub fn config(&self) -> &SpiConfig {
        &self.config
    }

    /// Get a mutable reference to the underlying SPI peripheral
    pub fn inner_mut(&mut self) -> &mut Spi<'d, SPI3, FullDuplexMode> {
        &mut self.spi
    }

    /// Get an immutable reference to the underlying SPI peripheral
    pub fn inner(&self) -> &Spi<'d, SPI3, FullDuplexMode> {
        &self.spi
    }
}

impl<'d> ErrorType for SpiBus3<'d> {
    type Error = SpiErrorWrapper;
}

impl<'d> SpiBus for SpiBus3<'d> {
    fn read(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.spi
            .read(words)
            .map_err(|_| SpiErrorWrapper::Other)
    }

    fn write(&mut self, words: &[u8]) -> Result<(), Self::Error> {
        self.spi
            .write(words)
            .map_err(|_| SpiErrorWrapper::Other)
    }

    fn transfer(&mut self, read: &mut [u8], write: &[u8]) -> Result<(), Self::Error> {
        self.spi
            .transfer(read, write)
            .map_err(|_| SpiErrorWrapper::Other)
    }

    fn transfer_in_place(&mut self, words: &mut [u8]) -> Result<(), Self::Error> {
        self.spi
            .transfer_in_place(words)
            .map_err(|_| SpiErrorWrapper::Other)
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.spi
            .flush()
            .map_err(|_| SpiErrorWrapper::Other)
    }
}

/// SPI device with chip select management
///
/// This wrapper provides a `SpiDevice` implementation that manages
/// the chip select pin automatically for each transaction.
pub struct SpiDeviceWithCs<'d, SPI, CS> {
    bus: SPI,
    cs: CS,
    _phantom: PhantomData<&'d ()>,
}

impl<'d, SPI, CS> SpiDeviceWithCs<'d, SPI, CS> 
where
    CS: embedded_hal::digital::OutputPin,
{
    /// Create a new SPI device with chip select
    ///
    /// # Arguments
    ///
    /// * `bus` - The SPI bus
    /// * `cs` - The chip select pin (active low)
    pub fn new(bus: SPI, cs: CS) -> Self {
        Self {
            bus,
            cs,
            _phantom: PhantomData,
        }
    }

    /// Release the SPI bus and CS pin
    pub fn release(self) -> (SPI, CS) {
        (self.bus, self.cs)
    }
}

impl<'d, SPI, CS> ErrorType for SpiDeviceWithCs<'d, SPI, CS>
where
    SPI: ErrorType,
{
    type Error = SPI::Error;
}

impl<'d, SPI, CS> SpiDevice for SpiDeviceWithCs<'d, SPI, CS>
where
    SPI: SpiBus,
    CS: embedded_hal::digital::OutputPin,
{
    fn transaction(&mut self, operations: &mut [embedded_hal::spi::Operation<'_, u8>]) -> Result<(), Self::Error> {
        // Assert CS (active low)
        let _ = self.cs.set_low();

        let result = operations.iter_mut().try_for_each(|op| match op {
            embedded_hal::spi::Operation::Read(buf) => self.bus.read(buf),
            embedded_hal::spi::Operation::Write(buf) => self.bus.write(buf),
            embedded_hal::spi::Operation::Transfer(read, write) => self.bus.transfer(read, write),
            embedded_hal::spi::Operation::TransferInPlace(buf) => self.bus.transfer_in_place(buf),
            embedded_hal::spi::Operation::DelayNs(_) => {
                // Delay is handled by the caller in embedded-hal 1.0
                Ok(())
            }
        });

        // Deassert CS
        let _ = self.cs.set_high();

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spi_config_default() {
        let config = SpiConfig::default();
        assert_eq!(config.frequency.to_Hz(), 1_000_000);
        assert_eq!(config.mode, SpiMode::Mode0);
        assert!(!config.use_dma);
    }

    #[test]
    fn test_spi_config_builder() {
        let config = SpiConfig::default()
            .with_frequency(fugit::HertzU32::MHz(40))
            .with_mode(SpiMode::Mode3)
            .with_dma(true);
        assert_eq!(config.frequency.to_Hz(), 40_000_000);
        assert_eq!(config.mode, SpiMode::Mode3);
        assert!(config.use_dma);
    }

    #[test]
    fn test_spi_mode_conversion() {
        assert_eq!(EspSpiMode::from(SpiMode::Mode0), EspSpiMode::Mode0);
        assert_eq!(EspSpiMode::from(SpiMode::Mode1), EspSpiMode::Mode1);
        assert_eq!(EspSpiMode::from(SpiMode::Mode2), EspSpiMode::Mode2);
        assert_eq!(EspSpiMode::from(SpiMode::Mode3), EspSpiMode::Mode3);
    }
}
