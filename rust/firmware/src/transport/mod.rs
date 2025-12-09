//! Transport layer module
//!
//! This module provides transport implementations for the protocol,
//! allowing communication over different physical interfaces.

pub mod usb_cdc;

pub use usb_cdc::UsbCdcTransport;

/// Transport trait for sending and receiving protocol messages
pub trait Transport {
    /// Send a message frame
    fn send(&mut self, frame: &[u8]) -> Result<(), TransportError>;
    
    /// Receive a message frame
    /// Returns None if no complete frame is available
    fn receive(&mut self) -> Result<Option<&[u8]>, TransportError>;
    
    /// Check if the transport is connected
    fn is_connected(&self) -> bool;
}

/// Transport error types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransportError {
    /// Buffer overflow
    BufferFull,
    /// Transport disconnected
    Disconnected,
    /// I/O error
    IoError,
    /// Timeout
    Timeout,
}
