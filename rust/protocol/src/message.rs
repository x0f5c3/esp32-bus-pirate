//! Protocol message definitions

use heapless::{String, Vec};
use serde::{Deserialize, Serialize};

/// Main message enum for all protocol operations
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Message {
    // ===== Mode Management =====
    /// Set the current bus mode
    SetMode { mode: Mode },
    /// Get the current bus mode
    GetMode,
    
    // ===== I2C Operations =====
    /// Scan I2C bus for devices
    I2cScan,
    /// Write data to I2C device
    I2cWrite { addr: u8, data: Vec<u8, 256> },
    /// Read data from I2C device
    I2cRead { addr: u8, len: u8 },
    /// Read register from I2C device
    I2cReadRegister { addr: u8, reg: u8 },
    /// Write register to I2C device
    I2cWriteRegister { addr: u8, reg: u8, value: u8 },
    
    // ===== SPI Operations =====
    /// Transfer data over SPI
    SpiTransfer { data: Vec<u8, 256> },
    
    // ===== UART Operations =====
    /// Write data to UART
    UartWrite { data: Vec<u8, 256> },
    /// Read data from UART
    UartRead { len: u16 },
    /// Configure UART parameters
    UartConfig { baudrate: u32 },
    
    // ===== Configuration =====
    /// Set configuration value
    SetConfig { key: String<32>, value: String<64> },
    /// Get configuration value
    GetConfig { key: String<32> },
    
    // ===== File Operations =====
    /// List files in directory
    FileList { path: String<128> },
    /// Read file contents
    FileRead { path: String<128> },
    /// Write file contents
    FileWrite { path: String<128>, data: Vec<u8, 512> },
    
    // ===== Responses =====
    /// Response message
    Response(Response),
    /// Error response
    Error(ErrorCode),
}

/// Bus Pirate operating modes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Mode {
    /// High impedance (default)
    HiZ,
    /// I2C mode
    I2c,
    /// SPI mode
    Spi,
    /// UART mode
    Uart,
    /// 1-Wire mode
    OneWire,
    /// 2-Wire mode
    TwoWire,
    /// 3-Wire mode
    ThreeWire,
    /// Digital I/O mode
    Dio,
    /// Infrared mode
    Infrared,
    /// USB mode
    Usb,
    /// Bluetooth mode
    Bluetooth,
    /// Wi-Fi mode
    Wifi,
    /// Ethernet mode
    Ethernet,
    /// JTAG mode
    Jtag,
    /// LED mode
    Led,
    /// I2S audio mode
    I2s,
    /// CAN bus mode
    Can,
    /// Sub-GHz radio mode
    SubGhz,
    /// RFID mode
    Rfid,
    /// RF24 radio mode
    Rf24,
}

/// Response types
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Response {
    /// Simple success
    Success,
    /// Data payload
    Data(Vec<u8, 512>),
    /// List of I2C device addresses
    I2cDevices(Vec<u8, 128>),
    /// Current mode
    CurrentMode(Mode),
    /// Configuration value
    ConfigValue(String<64>),
    /// File list
    FileList(Vec<String<64>, 32>),
}

/// Error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ErrorCode {
    /// Invalid command
    InvalidCommand,
    /// Protocol error
    ProtocolError,
    /// Bus error (I2C, SPI, etc.)
    BusError,
    /// File not found
    FileNotFound,
    /// Permission denied
    PermissionDenied,
    /// Operation timeout
    Timeout,
    /// Device not configured
    NotConfigured,
    /// Invalid parameter
    InvalidParameter,
}
