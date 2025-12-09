//! Comprehensive MessageCodec tests

use esp32_bus_pirate_protocol::{
    codec::MessageCodec, message::*, Error, ErrorCode, Mode, Response, END_BYTE,
    MAX_MESSAGE_SIZE, PROTOCOL_VERSION, START_BYTE,
};
use heapless::{String, Vec};

// ===== Basic Encoding/Decoding Tests =====

#[test]
fn test_encode_decode_set_mode_i2c() {
    let msg = Message::SetMode { mode: Mode::I2c };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_set_mode_spi() {
    let msg = Message::SetMode { mode: Mode::Spi };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_set_mode_uart() {
    let msg = Message::SetMode { mode: Mode::Uart };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_get_mode() {
    let msg = Message::GetMode;
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_i2c_scan() {
    let msg = Message::I2cScan;
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_i2c_write() {
    let mut data = Vec::new();
    data.extend_from_slice(&[0x01, 0x02, 0x03]).unwrap();
    
    let msg = Message::I2cWrite { addr: 0x50, data };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_i2c_read() {
    let msg = Message::I2cRead {
        addr: 0x50,
        len: 16,
    };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_i2c_read_register() {
    let msg = Message::I2cReadRegister {
        addr: 0x50,
        reg: 0x10,
    };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_i2c_write_register() {
    let msg = Message::I2cWriteRegister {
        addr: 0x50,
        reg: 0x10,
        value: 0xAB,
    };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_spi_transfer() {
    let mut data = Vec::new();
    data.extend_from_slice(&[0xFF, 0x00, 0xAA, 0x55]).unwrap();
    
    let msg = Message::SpiTransfer { data };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_uart_write() {
    let mut data = Vec::new();
    data.extend_from_slice(b"Hello, UART!").unwrap();
    
    let msg = Message::UartWrite { data };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_uart_read() {
    let msg = Message::UartRead { len: 100 };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_uart_config() {
    let msg = Message::UartConfig { baudrate: 115200 };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_set_config() {
    let msg = Message::SetConfig {
        key: String::try_from("timeout").unwrap(),
        value: String::try_from("1000").unwrap(),
    };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_get_config() {
    let msg = Message::GetConfig {
        key: String::try_from("timeout").unwrap(),
    };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_file_list() {
    let msg = Message::FileList {
        path: String::try_from("/data").unwrap(),
    };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_file_read() {
    let msg = Message::FileRead {
        path: String::try_from("/data/config.txt").unwrap(),
    };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_file_write() {
    let mut data = Vec::new();
    data.extend_from_slice(b"File content here").unwrap();
    
    let msg = Message::FileWrite {
        path: String::try_from("/data/test.txt").unwrap(),
        data,
    };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_response_success() {
    let msg = Message::Response(Response::Success);
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_response_data() {
    let mut data = Vec::new();
    data.extend_from_slice(&[1, 2, 3, 4, 5]).unwrap();
    
    let msg = Message::Response(Response::Data(data));
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_response_i2c_devices() {
    let mut devices = Vec::new();
    devices.extend_from_slice(&[0x50, 0x51, 0x68]).unwrap();
    
    let msg = Message::Response(Response::I2cDevices(devices));
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_response_current_mode() {
    let msg = Message::Response(Response::CurrentMode(Mode::I2c));
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_response_config_value() {
    let msg = Message::Response(Response::ConfigValue(String::try_from("12345").unwrap()));
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_response_file_list() {
    let mut files = Vec::new();
    files.push(String::try_from("file1.txt").unwrap()).unwrap();
    files.push(String::try_from("file2.bin").unwrap()).unwrap();
    
    let msg = Message::Response(Response::FileList(files));
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_error_invalid_command() {
    let msg = Message::Error(ErrorCode::InvalidCommand);
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_error_protocol_error() {
    let msg = Message::Error(ErrorCode::ProtocolError);
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_encode_decode_error_bus_error() {
    let msg = Message::Error(ErrorCode::BusError);
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

// ===== All Mode Types =====

#[test]
fn test_encode_decode_all_modes() {
    let modes = vec![
        Mode::HiZ,
        Mode::I2c,
        Mode::Spi,
        Mode::Uart,
        Mode::OneWire,
        Mode::TwoWire,
        Mode::ThreeWire,
        Mode::Dio,
        Mode::Infrared,
        Mode::Usb,
        Mode::Bluetooth,
        Mode::Wifi,
        Mode::Ethernet,
        Mode::Jtag,
        Mode::Led,
        Mode::I2s,
        Mode::Can,
        Mode::SubGhz,
        Mode::Rfid,
        Mode::Rf24,
    ];
    
    for mode in modes {
        let msg = Message::SetMode { mode };
        let encoded = MessageCodec::encode(&msg).unwrap();
        let decoded = MessageCodec::decode(&encoded).unwrap();
        assert_eq!(msg, decoded);
    }
}

// ===== Frame Structure Tests =====

#[test]
fn test_frame_has_correct_start_byte() {
    let msg = Message::GetMode;
    let frame = MessageCodec::encode(&msg).unwrap();
    assert_eq!(frame[0], START_BYTE);
}

#[test]
fn test_frame_has_correct_end_byte() {
    let msg = Message::GetMode;
    let frame = MessageCodec::encode(&msg).unwrap();
    assert_eq!(frame[frame.len() - 1], END_BYTE);
}

#[test]
fn test_frame_has_correct_version() {
    let msg = Message::GetMode;
    let frame = MessageCodec::encode(&msg).unwrap();
    assert_eq!(frame[1], PROTOCOL_VERSION);
}

#[test]
fn test_frame_minimum_size() {
    let msg = Message::GetMode;
    let frame = MessageCodec::encode(&msg).unwrap();
    // START + VERSION + LEN(2) + PAYLOAD(>=0) + CRC(2) + END >= 7
    assert!(frame.len() >= 7);
}

// ===== CRC Validation Tests =====

#[test]
fn test_crc_detects_corrupted_payload() {
    // Use a message with a larger payload to ensure we can corrupt it
    let mut data = Vec::new();
    data.extend_from_slice(&[0x01, 0x02, 0x03, 0x04, 0x05]).unwrap();
    let msg = Message::I2cWrite { addr: 0x50, data };
    let mut frame = MessageCodec::encode(&msg).unwrap();
    
    // Frame structure: START + VERSION + LEN(2) + PAYLOAD + CRC(2) + END
    // Corrupt a byte in the middle of the payload (after length bytes)
    let payload_start = 4; // After START, VERSION, LEN(2)
    if frame.len() > payload_start + 2 {
        frame[payload_start + 1] ^= 0xFF;
    }
    
    let result = MessageCodec::decode(&frame);
    assert!(matches!(result, Err(Error::CrcMismatch)));
}

#[test]
fn test_crc_detects_corrupted_length() {
    let msg = Message::GetMode;
    let mut frame = MessageCodec::encode(&msg).unwrap();
    
    // Corrupt the length field
    frame[2] ^= 0xFF;
    
    let result = MessageCodec::decode(&frame);
    assert!(matches!(result, Err(Error::CrcMismatch) | Err(Error::FrameTooShort)));
}

#[test]
fn test_crc_detects_corrupted_version() {
    let msg = Message::GetMode;
    let mut frame = MessageCodec::encode(&msg).unwrap();
    
    // Corrupt the version byte (but keep it non-matching)
    frame[1] = 0xFF;
    
    let result = MessageCodec::decode(&frame);
    assert!(matches!(result, Err(Error::UnsupportedVersion)));
}

#[test]
fn test_crc_is_consistent() {
    let msg = Message::I2cScan;
    let frame1 = MessageCodec::encode(&msg).unwrap();
    let frame2 = MessageCodec::encode(&msg).unwrap();
    
    // Same message should produce same encoding
    assert_eq!(frame1, frame2);
}

// ===== Error Handling Tests =====

#[test]
fn test_decode_frame_too_short() {
    let frame = [START_BYTE, PROTOCOL_VERSION];
    let result = MessageCodec::decode(&frame);
    assert!(matches!(result, Err(Error::FrameTooShort)));
}

#[test]
fn test_decode_empty_frame() {
    let frame = [];
    let result = MessageCodec::decode(&frame);
    assert!(matches!(result, Err(Error::FrameTooShort)));
}

#[test]
fn test_decode_invalid_start_byte() {
    let msg = Message::GetMode;
    let mut frame = MessageCodec::encode(&msg).unwrap();
    frame[0] = 0x00;
    
    let result = MessageCodec::decode(&frame);
    assert!(matches!(result, Err(Error::InvalidFrame)));
}

#[test]
fn test_decode_invalid_end_byte() {
    let msg = Message::GetMode;
    let mut frame = MessageCodec::encode(&msg).unwrap();
    let len = frame.len();
    frame[len - 1] = 0x00;
    
    let result = MessageCodec::decode(&frame);
    assert!(matches!(result, Err(Error::InvalidFrame)));
}

#[test]
fn test_decode_unsupported_version() {
    let msg = Message::GetMode;
    let mut frame = MessageCodec::encode(&msg).unwrap();
    frame[1] = 0x99; // Invalid version
    
    let result = MessageCodec::decode(&frame);
    assert!(matches!(result, Err(Error::UnsupportedVersion)));
}

#[test]
fn test_decode_truncated_frame() {
    let msg = Message::I2cScan;
    let frame = MessageCodec::encode(&msg).unwrap();
    
    // Try decoding with truncated frame
    let truncated = &frame[..frame.len() - 2];
    let result = MessageCodec::decode(truncated);
    assert!(matches!(
        result,
        Err(Error::FrameTooShort) | Err(Error::InvalidFrame)
    ));
}

// ===== Edge Cases =====

#[test]
fn test_empty_data_payload() {
    let data = Vec::new();
    let msg = Message::I2cWrite { addr: 0x50, data };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_max_i2c_write_data() {
    let mut data = Vec::new();
    for i in 0..=255u8 {
        data.push(i).unwrap();
    }
    
    let msg = Message::I2cWrite { addr: 0x50, data };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_max_file_write_data() {
    let mut data = Vec::new();
    for i in 0..512 {
        data.push((i % 256) as u8).unwrap();
    }
    
    let msg = Message::FileWrite {
        path: String::try_from("/test.bin").unwrap(),
        data,
    };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_long_path_name() {
    let path = String::try_from("/very/long/path/to/some/file/that/is/nested/deeply/in/filesystem/hierarchy.txt").unwrap();
    let msg = Message::FileRead { path };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_max_uart_read_length() {
    let msg = Message::UartRead { len: u16::MAX };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

#[test]
fn test_zero_uart_read_length() {
    let msg = Message::UartRead { len: 0 };
    let encoded = MessageCodec::encode(&msg).unwrap();
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

// ===== Version Compatibility Tests =====

#[test]
fn test_current_version_is_accepted() {
    let msg = Message::GetMode;
    let frame = MessageCodec::encode(&msg).unwrap();
    assert_eq!(frame[1], PROTOCOL_VERSION);
    
    let decoded = MessageCodec::decode(&frame);
    assert!(decoded.is_ok());
}

// ===== All Error Codes Test =====

#[test]
fn test_encode_decode_all_error_codes() {
    let error_codes = vec![
        ErrorCode::InvalidCommand,
        ErrorCode::ProtocolError,
        ErrorCode::BusError,
        ErrorCode::FileNotFound,
        ErrorCode::PermissionDenied,
        ErrorCode::Timeout,
        ErrorCode::NotConfigured,
        ErrorCode::InvalidParameter,
    ];
    
    for error_code in error_codes {
        let msg = Message::Error(error_code);
        let encoded = MessageCodec::encode(&msg).unwrap();
        let decoded = MessageCodec::decode(&encoded).unwrap();
        assert_eq!(msg, decoded);
    }
}

// ===== Stress Tests =====

#[test]
fn test_rapid_encoding_decoding() {
    for _ in 0..100 {
        let msg = Message::I2cScan;
        let encoded = MessageCodec::encode(&msg).unwrap();
        let decoded = MessageCodec::decode(&encoded).unwrap();
        assert_eq!(msg, decoded);
    }
}

#[test]
fn test_multiple_messages_independent() {
    let msg1 = Message::SetMode { mode: Mode::I2c };
    let msg2 = Message::I2cScan;
    let msg3 = Message::GetMode;
    
    let enc1 = MessageCodec::encode(&msg1).unwrap();
    let enc2 = MessageCodec::encode(&msg2).unwrap();
    let enc3 = MessageCodec::encode(&msg3).unwrap();
    
    let dec1 = MessageCodec::decode(&enc1).unwrap();
    let dec2 = MessageCodec::decode(&enc2).unwrap();
    let dec3 = MessageCodec::decode(&enc3).unwrap();
    
    assert_eq!(msg1, dec1);
    assert_eq!(msg2, dec2);
    assert_eq!(msg3, dec3);
}

#[test]
fn test_frame_size_is_reasonable() {
    let msg = Message::GetMode;
    let frame = MessageCodec::encode(&msg).unwrap();
    
    // Frame should be much smaller than MAX_MESSAGE_SIZE for simple messages
    assert!(frame.len() < 50);
}

#[test]
fn test_large_response_data() {
    let mut data = Vec::new();
    for i in 0..512 {
        data.push((i % 256) as u8).unwrap();
    }
    
    let msg = Message::Response(Response::Data(data));
    let encoded = MessageCodec::encode(&msg).unwrap();
    assert!(encoded.len() < MAX_MESSAGE_SIZE);
    
    let decoded = MessageCodec::decode(&encoded).unwrap();
    assert_eq!(msg, decoded);
}

// ===== Python Client Compatibility Tests =====

#[test]
fn test_getmode_encoding_matches_python() {
    use std::string::String as StdString;
    
    let msg = Message::GetMode;
    let encoded = MessageCodec::encode(&msg).unwrap();
    
    // Print for verification with Python
    let hex_string = encoded.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<StdString>();
    
    println!("Rust GetMode encoding: {}", hex_string);
    println!("Python GetMode encoding: aa0101000130ab55");
    
    // Verify the encoding matches Python client
    let expected_hex = "aa0101000130ab55";
    assert_eq!(hex_string, expected_hex, 
        "Rust and Python encodings must match for interoperability");
}
