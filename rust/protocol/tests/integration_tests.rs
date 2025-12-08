//! Integration tests for the protocol
//! 
//! These tests verify complete request/response cycles and error recovery scenarios.

use esp32_bus_pirate_protocol::{
    codec::MessageCodec, message::*, ErrorCode, Mode, Response,
};
use heapless::{String, Vec};

// ===== Request/Response Cycle Tests =====

#[test]
fn test_mode_change_cycle() {
    // Request: Set mode to I2C
    let request = Message::SetMode { mode: Mode::I2c };
    let encoded_request = MessageCodec::encode(&request).unwrap();
    let decoded_request = MessageCodec::decode(&encoded_request).unwrap();
    assert_eq!(request, decoded_request);
    
    // Response: Success
    let response = Message::Response(Response::Success);
    let encoded_response = MessageCodec::encode(&response).unwrap();
    let decoded_response = MessageCodec::decode(&encoded_response).unwrap();
    assert_eq!(response, decoded_response);
    
    // Request: Get current mode
    let request = Message::GetMode;
    let encoded_request = MessageCodec::encode(&request).unwrap();
    let decoded_request = MessageCodec::decode(&encoded_request).unwrap();
    assert_eq!(request, decoded_request);
    
    // Response: Current mode is I2C
    let response = Message::Response(Response::CurrentMode(Mode::I2c));
    let encoded_response = MessageCodec::encode(&response).unwrap();
    let decoded_response = MessageCodec::decode(&encoded_response).unwrap();
    assert_eq!(response, decoded_response);
}

#[test]
fn test_i2c_scan_cycle() {
    // Request: Scan I2C bus
    let request = Message::I2cScan;
    let encoded_request = MessageCodec::encode(&request).unwrap();
    let decoded_request = MessageCodec::decode(&encoded_request).unwrap();
    assert_eq!(request, decoded_request);
    
    // Response: Found devices at 0x50 and 0x68
    let mut devices = Vec::new();
    devices.extend_from_slice(&[0x50, 0x68]).unwrap();
    let response = Message::Response(Response::I2cDevices(devices));
    let encoded_response = MessageCodec::encode(&response).unwrap();
    let decoded_response = MessageCodec::decode(&encoded_response).unwrap();
    assert_eq!(response, decoded_response);
}

#[test]
fn test_i2c_write_read_cycle() {
    // Request: Write data to I2C device
    let mut write_data = Vec::new();
    write_data.extend_from_slice(&[0x00, 0x10]).unwrap();
    let request = Message::I2cWrite {
        addr: 0x50,
        data: write_data,
    };
    let encoded_request = MessageCodec::encode(&request).unwrap();
    let decoded_request = MessageCodec::decode(&encoded_request).unwrap();
    assert_eq!(request, decoded_request);
    
    // Response: Success
    let response = Message::Response(Response::Success);
    let encoded_response = MessageCodec::encode(&response).unwrap();
    let decoded_response = MessageCodec::decode(&encoded_response).unwrap();
    assert_eq!(response, decoded_response);
    
    // Request: Read from I2C device
    let request = Message::I2cRead { addr: 0x50, len: 4 };
    let encoded_request = MessageCodec::encode(&request).unwrap();
    let decoded_request = MessageCodec::decode(&encoded_request).unwrap();
    assert_eq!(request, decoded_request);
    
    // Response: Data read back
    let mut read_data = Vec::new();
    read_data.extend_from_slice(&[0xAA, 0xBB, 0xCC, 0xDD]).unwrap();
    let response = Message::Response(Response::Data(read_data));
    let encoded_response = MessageCodec::encode(&response).unwrap();
    let decoded_response = MessageCodec::decode(&encoded_response).unwrap();
    assert_eq!(response, decoded_response);
}

#[test]
fn test_uart_config_and_transfer_cycle() {
    // Request: Configure UART
    let request = Message::UartConfig { baudrate: 9600 };
    let encoded_request = MessageCodec::encode(&request).unwrap();
    let decoded_request = MessageCodec::decode(&encoded_request).unwrap();
    assert_eq!(request, decoded_request);
    
    // Response: Success
    let response = Message::Response(Response::Success);
    let encoded_response = MessageCodec::encode(&response).unwrap();
    let decoded_response = MessageCodec::decode(&encoded_response).unwrap();
    assert_eq!(response, decoded_response);
    
    // Request: Write UART data
    let mut data = Vec::new();
    data.extend_from_slice(b"AT\r\n").unwrap();
    let request = Message::UartWrite { data };
    let encoded_request = MessageCodec::encode(&request).unwrap();
    let decoded_request = MessageCodec::decode(&encoded_request).unwrap();
    assert_eq!(request, decoded_request);
    
    // Response: Success
    let response = Message::Response(Response::Success);
    let encoded_response = MessageCodec::encode(&response).unwrap();
    let decoded_response = MessageCodec::decode(&encoded_response).unwrap();
    assert_eq!(response, decoded_response);
}

#[test]
fn test_file_operations_cycle() {
    // Request: List files
    let request = Message::FileList {
        path: String::try_from("/data").unwrap(),
    };
    let encoded_request = MessageCodec::encode(&request).unwrap();
    let decoded_request = MessageCodec::decode(&encoded_request).unwrap();
    assert_eq!(request, decoded_request);
    
    // Response: File list
    let mut files = Vec::new();
    files.push(String::try_from("config.txt").unwrap()).unwrap();
    files.push(String::try_from("log.bin").unwrap()).unwrap();
    let response = Message::Response(Response::FileList(files));
    let encoded_response = MessageCodec::encode(&response).unwrap();
    let decoded_response = MessageCodec::decode(&encoded_response).unwrap();
    assert_eq!(response, decoded_response);
    
    // Request: Read file
    let request = Message::FileRead {
        path: String::try_from("/data/config.txt").unwrap(),
    };
    let encoded_request = MessageCodec::encode(&request).unwrap();
    let decoded_request = MessageCodec::decode(&encoded_request).unwrap();
    assert_eq!(request, decoded_request);
    
    // Response: File contents
    let mut data = Vec::new();
    data.extend_from_slice(b"setting=value\n").unwrap();
    let response = Message::Response(Response::Data(data));
    let encoded_response = MessageCodec::encode(&response).unwrap();
    let decoded_response = MessageCodec::decode(&encoded_response).unwrap();
    assert_eq!(response, decoded_response);
}

#[test]
fn test_config_get_set_cycle() {
    // Request: Get config
    let request = Message::GetConfig {
        key: String::try_from("timeout").unwrap(),
    };
    let encoded_request = MessageCodec::encode(&request).unwrap();
    let decoded_request = MessageCodec::decode(&encoded_request).unwrap();
    assert_eq!(request, decoded_request);
    
    // Response: Config value
    let response = Message::Response(Response::ConfigValue(
        String::try_from("1000").unwrap(),
    ));
    let encoded_response = MessageCodec::encode(&response).unwrap();
    let decoded_response = MessageCodec::decode(&encoded_response).unwrap();
    assert_eq!(response, decoded_response);
    
    // Request: Set config
    let request = Message::SetConfig {
        key: String::try_from("timeout").unwrap(),
        value: String::try_from("2000").unwrap(),
    };
    let encoded_request = MessageCodec::encode(&request).unwrap();
    let decoded_request = MessageCodec::decode(&encoded_request).unwrap();
    assert_eq!(request, decoded_request);
    
    // Response: Success
    let response = Message::Response(Response::Success);
    let encoded_response = MessageCodec::encode(&response).unwrap();
    let decoded_response = MessageCodec::decode(&encoded_response).unwrap();
    assert_eq!(response, decoded_response);
}

// ===== Error Recovery Tests =====

#[test]
fn test_error_response_cycle() {
    // Request: Invalid operation
    let request = Message::I2cRead { addr: 0xFF, len: 1 };
    let encoded_request = MessageCodec::encode(&request).unwrap();
    let decoded_request = MessageCodec::decode(&encoded_request).unwrap();
    assert_eq!(request, decoded_request);
    
    // Response: Bus error
    let response = Message::Error(ErrorCode::BusError);
    let encoded_response = MessageCodec::encode(&response).unwrap();
    let decoded_response = MessageCodec::decode(&encoded_response).unwrap();
    assert_eq!(response, decoded_response);
}

#[test]
fn test_multiple_error_types() {
    let errors = vec![
        ErrorCode::InvalidCommand,
        ErrorCode::ProtocolError,
        ErrorCode::BusError,
        ErrorCode::FileNotFound,
        ErrorCode::PermissionDenied,
        ErrorCode::Timeout,
        ErrorCode::NotConfigured,
        ErrorCode::InvalidParameter,
    ];
    
    for error_code in errors {
        let error_msg = Message::Error(error_code);
        let encoded = MessageCodec::encode(&error_msg).unwrap();
        let decoded = MessageCodec::decode(&encoded).unwrap();
        assert_eq!(error_msg, decoded);
    }
}

#[test]
fn test_recovery_after_corrupted_frame() {
    // First valid message
    let msg1 = Message::GetMode;
    let frame1 = MessageCodec::encode(&msg1).unwrap();
    let decoded1 = MessageCodec::decode(&frame1).unwrap();
    assert_eq!(msg1, decoded1);
    
    // Corrupted frame
    let msg2 = Message::I2cScan;
    let mut frame2 = MessageCodec::encode(&msg2).unwrap();
    frame2[5] ^= 0xFF; // Corrupt it
    let result2 = MessageCodec::decode(&frame2);
    assert!(result2.is_err());
    
    // Next valid message should still work
    let msg3 = Message::SetMode { mode: Mode::Spi };
    let frame3 = MessageCodec::encode(&msg3).unwrap();
    let decoded3 = MessageCodec::decode(&frame3).unwrap();
    assert_eq!(msg3, decoded3);
}

// ===== Concurrent Operations Tests =====

#[test]
fn test_interleaved_messages() {
    // Simulate multiple independent messages that could be processed concurrently
    let messages = vec![
        Message::GetMode,
        Message::I2cScan,
        Message::SetMode { mode: Mode::Uart },
        Message::UartConfig { baudrate: 115200 },
        Message::Response(Response::Success),
        Message::Error(ErrorCode::Timeout),
    ];
    
    // Encode all messages
    let mut encoded_messages: heapless::Vec<heapless::Vec<u8, 1024>, 10> = Vec::new();
    for msg in &messages {
        encoded_messages.push(MessageCodec::encode(msg).unwrap()).unwrap();
    }
    
    // Decode all messages and verify
    for (i, encoded) in encoded_messages.iter().enumerate() {
        let decoded = MessageCodec::decode(encoded).unwrap();
        assert_eq!(messages[i], decoded);
    }
}

#[test]
fn test_rapid_request_response() {
    // Simulate rapid back-and-forth communication
    for _ in 0..50 {
        // Request
        let request = Message::I2cScan;
        let enc_req = MessageCodec::encode(&request).unwrap();
        let dec_req = MessageCodec::decode(&enc_req).unwrap();
        assert_eq!(request, dec_req);
        
        // Response
        let mut devices = Vec::new();
        devices.extend_from_slice(&[0x50]).unwrap();
        let response = Message::Response(Response::I2cDevices(devices));
        let enc_resp = MessageCodec::encode(&response).unwrap();
        let dec_resp = MessageCodec::decode(&enc_resp).unwrap();
        assert_eq!(response, dec_resp);
    }
}

// ===== Complex Scenarios =====

#[test]
fn test_full_i2c_workflow() {
    // 1. Set mode to I2C
    let msg = Message::SetMode { mode: Mode::I2c };
    let enc = MessageCodec::encode(&msg).unwrap();
    let dec = MessageCodec::decode(&enc).unwrap();
    assert_eq!(msg, dec);
    
    let resp = Message::Response(Response::Success);
    let enc = MessageCodec::encode(&resp).unwrap();
    let dec = MessageCodec::decode(&enc).unwrap();
    assert_eq!(resp, dec);
    
    // 2. Scan for devices
    let msg = Message::I2cScan;
    let enc = MessageCodec::encode(&msg).unwrap();
    let dec = MessageCodec::decode(&enc).unwrap();
    assert_eq!(msg, dec);
    
    let mut devices = Vec::new();
    devices.extend_from_slice(&[0x50, 0x68]).unwrap();
    let resp = Message::Response(Response::I2cDevices(devices));
    let enc = MessageCodec::encode(&resp).unwrap();
    let dec = MessageCodec::decode(&enc).unwrap();
    assert_eq!(resp, dec);
    
    // 3. Write to register
    let msg = Message::I2cWriteRegister {
        addr: 0x50,
        reg: 0x00,
        value: 0xAB,
    };
    let enc = MessageCodec::encode(&msg).unwrap();
    let dec = MessageCodec::decode(&enc).unwrap();
    assert_eq!(msg, dec);
    
    let resp = Message::Response(Response::Success);
    let enc = MessageCodec::encode(&resp).unwrap();
    let dec = MessageCodec::decode(&enc).unwrap();
    assert_eq!(resp, dec);
    
    // 4. Read from register
    let msg = Message::I2cReadRegister {
        addr: 0x50,
        reg: 0x00,
    };
    let enc = MessageCodec::encode(&msg).unwrap();
    let dec = MessageCodec::decode(&enc).unwrap();
    assert_eq!(msg, dec);
    
    let mut data = Vec::new();
    data.push(0xAB).unwrap();
    let resp = Message::Response(Response::Data(data));
    let enc = MessageCodec::encode(&resp).unwrap();
    let dec = MessageCodec::decode(&enc).unwrap();
    assert_eq!(resp, dec);
}

#[test]
fn test_mode_switching() {
    let modes = vec![Mode::I2c, Mode::Spi, Mode::Uart, Mode::HiZ];
    
    for mode in modes {
        // Set mode
        let msg = Message::SetMode { mode };
        let enc = MessageCodec::encode(&msg).unwrap();
        let dec = MessageCodec::decode(&enc).unwrap();
        assert_eq!(msg, dec);
        
        // Confirm with response
        let resp = Message::Response(Response::Success);
        let enc = MessageCodec::encode(&resp).unwrap();
        let dec = MessageCodec::decode(&enc).unwrap();
        assert_eq!(resp, dec);
        
        // Verify mode
        let msg = Message::GetMode;
        let enc = MessageCodec::encode(&msg).unwrap();
        let dec = MessageCodec::decode(&enc).unwrap();
        assert_eq!(msg, dec);
        
        let resp = Message::Response(Response::CurrentMode(mode));
        let enc = MessageCodec::encode(&resp).unwrap();
        let dec = MessageCodec::decode(&enc).unwrap();
        assert_eq!(resp, dec);
    }
}

#[test]
fn test_large_data_transfer() {
    // Test transferring maximum sized payloads
    let mut large_data = Vec::new();
    for i in 0..512 {
        large_data.push((i % 256) as u8).unwrap();
    }
    
    let msg = Message::Response(Response::Data(large_data));
    let enc = MessageCodec::encode(&msg).unwrap();
    let dec = MessageCodec::decode(&enc).unwrap();
    assert_eq!(msg, dec);
}

#[test]
fn test_error_then_recovery() {
    // Send a command that will fail
    let msg = Message::I2cRead { addr: 0x00, len: 1 };
    let enc = MessageCodec::encode(&msg).unwrap();
    let dec = MessageCodec::decode(&enc).unwrap();
    assert_eq!(msg, dec);
    
    // Get error response
    let resp = Message::Error(ErrorCode::BusError);
    let enc = MessageCodec::encode(&resp).unwrap();
    let dec = MessageCodec::decode(&enc).unwrap();
    assert_eq!(resp, dec);
    
    // Try again with valid address
    let msg = Message::I2cRead { addr: 0x50, len: 1 };
    let enc = MessageCodec::encode(&msg).unwrap();
    let dec = MessageCodec::decode(&enc).unwrap();
    assert_eq!(msg, dec);
    
    // Get success response
    let mut data = Vec::new();
    data.push(0xFF).unwrap();
    let resp = Message::Response(Response::Data(data));
    let enc = MessageCodec::encode(&resp).unwrap();
    let dec = MessageCodec::decode(&enc).unwrap();
    assert_eq!(resp, dec);
}
