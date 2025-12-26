//! UART peripheral wrapper for ESP32-S3
//!
//! This module provides safe abstractions over the ESP32-S3 UART peripheral,
//! implementing the `embedded-hal` serial traits.
//!
//! # Example
//!
//! ```no_run
//! use esp32_bus_pirate_hal::peripherals::uart::{UartConfig, Parity, StopBits};
//!
//! let config = UartConfig::default()
//!     .with_baudrate(115200)
//!     .with_parity(Parity::None)
//!     .with_stop_bits(StopBits::One);
//! ```

use esp_hal::{
    Blocking,
    uart::{Uart, UartTx, UartRx, DataBits as EspDataBits, Parity as EspParity, StopBits as EspStopBits, Config as EspUartConfig},
};
use embedded_io::{Read, Write, ErrorType as IoErrorType};

/// UART parity configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parity {
    /// No parity bit
    None,
    /// Even parity
    Even,
    /// Odd parity
    Odd,
}

impl From<Parity> for EspParity {
    fn from(parity: Parity) -> Self {
        match parity {
            Parity::None => EspParity::None,
            Parity::Even => EspParity::Even,
            Parity::Odd => EspParity::Odd,
        }
    }
}

/// UART stop bits configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StopBits {
    /// 1 stop bit
    One,
    /// 2 stop bits
    Two,
}

impl From<StopBits> for EspStopBits {
    fn from(stop_bits: StopBits) -> Self {
        match stop_bits {
            StopBits::One => EspStopBits::_1,
            StopBits::Two => EspStopBits::_2,
        }
    }
}

/// UART data bits configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DataBits {
    /// 5 data bits
    Five,
    /// 6 data bits
    Six,
    /// 7 data bits
    Seven,
    /// 8 data bits
    Eight,
}

impl From<DataBits> for EspDataBits {
    fn from(data_bits: DataBits) -> Self {
        match data_bits {
            DataBits::Five => EspDataBits::_5,
            DataBits::Six => EspDataBits::_6,
            DataBits::Seven => EspDataBits::_7,
            DataBits::Eight => EspDataBits::_8,
        }
    }
}

/// UART configuration
#[derive(Debug, Clone, Copy)]
pub struct UartConfig {
    /// Baud rate in bits per second
    pub baudrate: u32,
    /// Parity checking mode
    pub parity: Parity,
    /// Number of stop bits
    pub stop_bits: StopBits,
    /// Number of data bits
    pub data_bits: DataBits,
}

impl Default for UartConfig {
    fn default() -> Self {
        Self {
            baudrate: 115200,
            parity: Parity::None,
            stop_bits: StopBits::One,
            data_bits: DataBits::Eight,
        }
    }
}

impl UartConfig {
    /// Create a new UART configuration with the given baud rate
    pub fn new(baudrate: u32) -> Self {
        Self {
            baudrate,
            parity: Parity::None,
            stop_bits: StopBits::One,
            data_bits: DataBits::Eight,
        }
    }

    /// Set the baud rate
    pub fn with_baudrate(mut self, baudrate: u32) -> Self {
        self.baudrate = baudrate;
        self
    }

    /// Set the parity mode
    pub fn with_parity(mut self, parity: Parity) -> Self {
        self.parity = parity;
        self
    }

    /// Set the stop bits
    pub fn with_stop_bits(mut self, stop_bits: StopBits) -> Self {
        self.stop_bits = stop_bits;
        self
    }

    /// Set the data bits
    pub fn with_data_bits(mut self, data_bits: DataBits) -> Self {
        self.data_bits = data_bits;
        self
    }

    /// Convert to esp-hal UART config
    pub fn to_esp_config(&self) -> EspUartConfig {
        EspUartConfig::default()
            .with_baudrate(self.baudrate)
            .with_data_bits(self.data_bits.into())
            .with_parity(self.parity.into())
            .with_stop_bits(self.stop_bits.into())
    }
}

/// Custom UART error type
#[derive(Debug, Clone, Copy)]
pub enum UartErrorWrapper {
    /// Buffer overrun
    Overrun,
    /// Parity check failed
    Parity,
    /// Framing error
    FrameFormat,
    /// Noise detected
    Noise,
    /// Buffer full
    BufferFull,
    /// Other hardware error
    Other,
}

impl embedded_io::Error for UartErrorWrapper {
    fn kind(&self) -> embedded_io::ErrorKind {
        embedded_io::ErrorKind::Other
    }
}

/// UART0 peripheral wrapper
///
/// This wrapper provides a safe interface to the ESP32-S3 UART0 peripheral.
pub struct UartBus0<'d> {
    uart: Uart<'d, Blocking>,
    config: UartConfig,
}

impl<'d> UartBus0<'d> {
    /// Create a new UART0 wrapper
    ///
    /// # Arguments
    ///
    /// * `uart` - The ESP-HAL UART peripheral
    /// * `config` - Configuration for the UART
    pub fn new(uart: Uart<'d, Blocking>, config: UartConfig) -> Self {
        Self { uart, config }
    }

    /// Get the current configuration
    pub fn config(&self) -> &UartConfig {
        &self.config
    }

    /// Split the UART into transmit and receive halves
    pub fn split(self) -> (UartTx<'d, Blocking>, UartRx<'d, Blocking>) {
        let (rx, tx) = self.uart.split();
        (tx, rx)
    }

    /// Write a byte to the UART (blocking)
    pub fn write_byte(&mut self, byte: u8) -> Result<(), UartErrorWrapper> {
        let buf = [byte];
        self.uart.write(&buf).map_err(|_| UartErrorWrapper::Other)?;
        Ok(())
    }

    /// Read a byte from the UART (blocking)
    pub fn read_byte(&mut self) -> Result<u8, UartErrorWrapper> {
        let mut buf = [0u8; 1];
        self.uart.read(&mut buf).map_err(|_| UartErrorWrapper::Other)?;
        Ok(buf[0])
    }

    /// Flush the UART transmit buffer
    pub fn flush_tx(&mut self) -> Result<(), UartErrorWrapper> {
        self.uart.flush().map_err(|_| UartErrorWrapper::Other)
    }
}

impl<'d> IoErrorType for UartBus0<'d> {
    type Error = UartErrorWrapper;
}

impl<'d> Write for UartBus0<'d> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.uart.write(buf).map_err(|_| UartErrorWrapper::Other)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush_tx()
    }
}

impl<'d> Read for UartBus0<'d> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.uart.read(buf).map_err(|_| UartErrorWrapper::Other)
    }
}

/// UART1 peripheral wrapper
///
/// This wrapper provides a safe interface to the ESP32-S3 UART1 peripheral.
pub struct UartBus1<'d> {
    uart: Uart<'d, Blocking>,
    config: UartConfig,
}

impl<'d> UartBus1<'d> {
    /// Create a new UART1 wrapper
    ///
    /// # Arguments
    ///
    /// * `uart` - The ESP-HAL UART peripheral
    /// * `config` - Configuration for the UART
    pub fn new(uart: Uart<'d, Blocking>, config: UartConfig) -> Self {
        Self { uart, config }
    }

    /// Get the current configuration
    pub fn config(&self) -> &UartConfig {
        &self.config
    }

    /// Split the UART into transmit and receive halves
    pub fn split(self) -> (UartTx<'d, Blocking>, UartRx<'d, Blocking>) {
        let (rx, tx) = self.uart.split();
        (tx, rx)
    }

    /// Write a byte to the UART (blocking)
    pub fn write_byte(&mut self, byte: u8) -> Result<(), UartErrorWrapper> {
        let buf = [byte];
        self.uart.write(&buf).map_err(|_| UartErrorWrapper::Other)?;
        Ok(())
    }

    /// Read a byte from the UART (blocking)
    pub fn read_byte(&mut self) -> Result<u8, UartErrorWrapper> {
        let mut buf = [0u8; 1];
        self.uart.read(&mut buf).map_err(|_| UartErrorWrapper::Other)?;
        Ok(buf[0])
    }

    /// Flush the UART transmit buffer
    pub fn flush_tx(&mut self) -> Result<(), UartErrorWrapper> {
        self.uart.flush().map_err(|_| UartErrorWrapper::Other)
    }
}

impl<'d> IoErrorType for UartBus1<'d> {
    type Error = UartErrorWrapper;
}

impl<'d> Write for UartBus1<'d> {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Self::Error> {
        self.uart.write(buf).map_err(|_| UartErrorWrapper::Other)?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> Result<(), Self::Error> {
        self.flush_tx()
    }
}

impl<'d> Read for UartBus1<'d> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Self::Error> {
        self.uart.read(buf).map_err(|_| UartErrorWrapper::Other)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_uart_config_default() {
        let config = UartConfig::default();
        assert_eq!(config.baudrate, 115200);
        assert_eq!(config.parity, Parity::None);
        assert_eq!(config.stop_bits, StopBits::One);
        assert_eq!(config.data_bits, DataBits::Eight);
    }

    #[test]
    fn test_uart_config_builder() {
        let config = UartConfig::default()
            .with_baudrate(9600)
            .with_parity(Parity::Even)
            .with_stop_bits(StopBits::Two)
            .with_data_bits(DataBits::Seven);
        assert_eq!(config.baudrate, 9600);
        assert_eq!(config.parity, Parity::Even);
        assert_eq!(config.stop_bits, StopBits::Two);
        assert_eq!(config.data_bits, DataBits::Seven);
    }

    #[test]
    fn test_parity_conversion() {
        assert_eq!(EspParity::from(Parity::None), EspParity::ParityNone);
        assert_eq!(EspParity::from(Parity::Even), EspParity::ParityEven);
        assert_eq!(EspParity::from(Parity::Odd), EspParity::ParityOdd);
    }

    #[test]
    fn test_stop_bits_conversion() {
        assert_eq!(EspStopBits::from(StopBits::One), EspStopBits::STOP1);
        assert_eq!(EspStopBits::from(StopBits::Two), EspStopBits::STOP2);
    }
}
