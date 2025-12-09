//! Message encoding and decoding with CRC validation

use crate::{message::Message, Error, END_BYTE, MAX_MESSAGE_SIZE, START_BYTE};
use crc::{Crc, CRC_16_IBM_SDLC};
use heapless::Vec;
use postcard::{from_bytes, to_slice};

const CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);

/// Message codec for encoding and decoding protocol messages
pub struct MessageCodec;

impl MessageCodec {
    /// Encode a message into a framed byte stream
    ///
    /// Frame format:
    /// ```text
    /// ┌─────────┬─────────┬─────────┬──────────┬─────────┬─────────┐
    /// │ START   │ VERSION │ LENGTH  │ PAYLOAD  │ CRC16   │  END    │
    /// │ (0xAA)  │ (1 byte)│ (2 bytes│ (n bytes)│ (2 bytes│ (0x55)  │
    /// └─────────┴─────────┴─────────┴──────────┴─────────┴─────────┘
    /// ```
    pub fn encode(msg: &Message) -> Result<Vec<u8, MAX_MESSAGE_SIZE>, Error> {
        // Serialize message with postcard
        let mut payload_buf = [0u8; MAX_MESSAGE_SIZE];
        let payload_slice = to_slice(msg, &mut payload_buf)
            .map_err(|_| Error::EncodingFailed)?;
        
        let len = payload_slice.len() as u16;
        
        // Build frame
        let mut frame = Vec::new();
        frame.push(START_BYTE).map_err(|_| Error::BufferFull)?;
        frame
            .push(crate::version::PROTOCOL_VERSION)
            .map_err(|_| Error::BufferFull)?;
        frame
            .extend_from_slice(&len.to_le_bytes())
            .map_err(|_| Error::BufferFull)?;
        frame
            .extend_from_slice(payload_slice)
            .map_err(|_| Error::BufferFull)?;
        
        // Calculate CRC over VERSION + LENGTH + PAYLOAD
        let crc_data = &frame[1..];
        let crc_value = CRC.checksum(crc_data);
        
        frame
            .extend_from_slice(&crc_value.to_le_bytes())
            .map_err(|_| Error::BufferFull)?;
        frame.push(END_BYTE).map_err(|_| Error::BufferFull)?;
        
        Ok(frame)
    }
    
    /// Decode a framed byte stream into a message
    pub fn decode(frame: &[u8]) -> Result<Message, Error> {
        // Minimum frame: START + VERSION + LEN(2) + CRC(2) + END = 7 bytes
        if frame.len() < 7 {
            return Err(Error::FrameTooShort);
        }
        
        // Check frame markers
        if frame[0] != START_BYTE || frame[frame.len() - 1] != END_BYTE {
            return Err(Error::InvalidFrame);
        }
        
        // Check version
        let version = frame[1];
        if version != crate::version::PROTOCOL_VERSION {
            return Err(Error::UnsupportedVersion);
        }
        
        // Extract length
        let len = u16::from_le_bytes([frame[2], frame[3]]) as usize;
        let payload_end = 4 + len;
        
        if frame.len() < payload_end + 3 {
            return Err(Error::FrameTooShort);
        }
        
        // Extract payload and CRC
        let payload = &frame[4..payload_end];
        let crc_received = u16::from_le_bytes([frame[payload_end], frame[payload_end + 1]]);
        
        // Verify CRC
        let crc_data = &frame[1..payload_end];
        let crc_calculated = CRC.checksum(crc_data);
        
        if crc_received != crc_calculated {
            return Err(Error::CrcMismatch);
        }
        
        // Deserialize payload
        from_bytes(payload).map_err(|_| Error::DecodingFailed)
    }
}
