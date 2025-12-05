#![no_std]

//! Binary protocol implementation for ESP32 Bus Pirate
//!
//! This crate defines the message format and codec for communicating
//! with the Bus Pirate firmware over serial, USB, or network.

pub mod message;
pub mod codec;
pub mod version;

pub use codec::MessageCodec;
pub use message::{Message, Mode, Response, ErrorCode};
pub use version::PROTOCOL_VERSION;

/// Protocol framing constants
pub const START_BYTE: u8 = 0xAA;
pub const END_BYTE: u8 = 0x55;

/// Maximum message size (1KB)
pub const MAX_MESSAGE_SIZE: usize = 1024;

/// Error types for protocol operations
#[derive(Debug)]
pub enum Error {
    /// Frame too short to be valid
    FrameTooShort,
    /// Invalid frame markers
    InvalidFrame,
    /// Unsupported protocol version
    UnsupportedVersion,
    /// CRC mismatch
    CrcMismatch,
    /// Encoding failed
    EncodingFailed,
    /// Decoding failed
    DecodingFailed,
    /// Buffer too small
    BufferFull,
}
