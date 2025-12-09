// Test runner for protocol crate (std environment)
use esp32_bus_pirate_protocol::{codec::MessageCodec, message::*, *};

#[test]
fn test_protocol_constants() {
    assert_eq!(START_BYTE, 0xAA);
    assert_eq!(END_BYTE, 0x55);
    assert_eq!(MAX_MESSAGE_SIZE, 1024);
}

#[test]
fn test_error_types() {
    let err = Error::InvalidFrame;
    assert!(matches!(err, Error::InvalidFrame));
}

#[test]
fn test_message_encoding_all_variants() {
    let test_cases = vec![
        Message::SetMode { mode: Mode::I2c },
        Message::GetMode,
        Message::I2cScan,
        Message::UartConfig { baudrate: 115200 },
        Message::Response(Response::Success),
    ];

    for msg in test_cases {
        let encoded = MessageCodec::encode(&msg).expect("encoding should succeed");
        let decoded = MessageCodec::decode(&encoded).expect("decoding should succeed");
        assert_eq!(msg, decoded, "roundtrip failed for {:?}", msg);
    }
}

#[test]
fn test_mode_variants() {
    let modes = vec![
        Mode::HiZ,
        Mode::I2c,
        Mode::Spi,
        Mode::Uart,
        Mode::OneWire,
        Mode::Dio,
    ];

    for mode in modes {
        let msg = Message::SetMode { mode };
        let encoded = MessageCodec::encode(&msg).unwrap();
        assert!(encoded.len() > 0);
    }
}

#[test]
fn test_error_code_variants() {
    let errors = vec![
        ErrorCode::InvalidCommand,
        ErrorCode::ProtocolError,
        ErrorCode::BusError,
        ErrorCode::FileNotFound,
        ErrorCode::Timeout,
    ];

    for error in errors {
        let msg = Message::Error(error);
        let encoded = MessageCodec::encode(&msg).unwrap();
        let decoded = MessageCodec::decode(&encoded).unwrap();
        assert_eq!(msg, decoded);
    }
}

#[test]
fn test_large_payload() {
    use heapless::Vec;
    
    let mut data = Vec::<u8, 256>::new();
    for i in 0..200 {
        data.push(i as u8).unwrap();
    }

    let msg = Message::I2cWrite { addr: 0x50, data };
    let encoded = MessageCodec::encode(&msg).expect("encoding should succeed");
    let decoded = MessageCodec::decode(&encoded).expect("decoding should succeed");
    assert_eq!(msg, decoded);
}

#[test]
fn test_frame_structure() {
    let msg = Message::GetMode;
    let frame = MessageCodec::encode(&msg).unwrap();

    // Check frame markers
    assert_eq!(frame[0], START_BYTE, "Start byte mismatch");
    assert_eq!(frame[frame.len() - 1], END_BYTE, "End byte mismatch");

    // Check version
    assert_eq!(frame[1], PROTOCOL_VERSION, "Version mismatch");

    // Frame should have minimum size
    assert!(frame.len() >= 7, "Frame too short");
}

#[test]
fn test_invalid_start_byte() {
    let mut frame = vec![0xFF, 0x01, 0x00, 0x00, 0x00, 0x00, END_BYTE];
    let result = MessageCodec::decode(&frame);
    assert!(matches!(result, Err(Error::InvalidFrame)));
}

#[test]
fn test_invalid_end_byte() {
    let msg = Message::GetMode;
    let mut frame = MessageCodec::encode(&msg).unwrap();
    let len = frame.len();
    frame[len - 1] = 0xFF; // Corrupt end byte

    let result = MessageCodec::decode(&frame);
    assert!(matches!(result, Err(Error::InvalidFrame)));
}

#[test]
fn test_wrong_version() {
    let mut frame = vec![START_BYTE, 99, 0x00, 0x00, 0x00, 0x00, END_BYTE];
    let result = MessageCodec::decode(&frame);
    assert!(matches!(result, Err(Error::UnsupportedVersion)));
}

#[test]
fn test_corrupted_crc() {
    let msg = Message::GetMode;
    let mut frame = MessageCodec::encode(&msg).unwrap();
    
    // Corrupt the CRC bytes (second and third from end)
    let len = frame.len();
    frame[len - 3] ^= 0xFF;

    let result = MessageCodec::decode(&frame);
    assert!(matches!(result, Err(Error::CrcMismatch)));
}

#[test]
fn test_truncated_frame() {
    let msg = Message::GetMode;
    let frame = MessageCodec::encode(&msg).unwrap();
    
    // Take only first 5 bytes (incomplete frame)
    let truncated = &frame[..5];
    
    let result = MessageCodec::decode(truncated);
    assert!(matches!(result, Err(Error::FrameTooShort)));
}
