# ESP32 Bus Pirate Test Tools

This directory contains test and development tools for the ESP32 Bus Pirate.

## test_client.py

Python test client for communicating with the ESP32 Bus Pirate over USB serial using the binary protocol.

### Features

- **Message encoding/decoding** - Implements the complete binary protocol with Postcard serialization
- **CRC validation** - CRC-16-IBM-SDLC checksums matching the Rust implementation
- **CLI interface** - Send individual commands from the command line
- **Interactive mode** - REPL-style interface for exploring the Bus Pirate
- **Port discovery** - Lists available serial ports

### Requirements

```bash
pip install pyserial
```

### Usage

#### List available serial ports

```bash
./test_client.py --list
```

#### Test CRC calculation and encoding

```bash
./test_client.py --test-crc
```

Expected output:
```
Test data: b'Hello, World!'
CRC-16-IBM-SDLC: 0x9BD5

GetMode message: aa0101000130ab55
Decoded: {'version': 1, 'payload': b'\x01', 'payload_hex': '01'}
```

#### Send a single command

```bash
# Get current mode
./test_client.py --port /dev/ttyUSB0 --command getmode

# Scan I2C bus
./test_client.py --port /dev/ttyUSB0 --command i2cscan
```

#### Interactive mode

```bash
./test_client.py --port /dev/ttyUSB0 --interactive
```

Interactive commands:
- `mode <mode>` - Set bus mode (i2c, spi, uart, hiz, etc.)
- `getmode` - Get current mode
- `i2c scan` - Scan I2C bus for devices
- `i2c write <addr> <hex_data>` - Write data to I2C device
- `i2c read <addr> <len>` - Read from I2C device
- `quit` / `exit` - Exit interactive mode

Example session:
```
BP> mode i2c
Response: {...}

BP> i2c scan
Response: {...}

BP> i2c write 0x50 00 10
Response: {...}

BP> i2c read 0x50 4
Response: {...}

BP> quit
```

### Protocol Format

The binary protocol uses this frame format:

```
┌─────────┬─────────┬─────────┬──────────┬─────────┬─────────┐
│ START   │ VERSION │ LENGTH  │ PAYLOAD  │ CRC16   │  END    │
│ (0xAA)  │ (1 byte)│ (2 bytes│ (n bytes)│ (2 bytes│ (0x55)  │
└─────────┴─────────┴─────────┴──────────┴─────────┴─────────┘
```

- **START**: Always `0xAA`
- **VERSION**: Protocol version (currently `0x01`)
- **LENGTH**: Payload length in bytes (little-endian u16)
- **PAYLOAD**: Postcard-serialized message
- **CRC16**: CRC-16-IBM-SDLC over VERSION + LENGTH + PAYLOAD (little-endian u16)
- **END**: Always `0x55`

### Supported Message Types

#### Mode Management
- `SetMode { mode }` - Set operating mode
- `GetMode` - Get current mode

#### I²C Operations
- `I2cScan` - Scan for devices
- `I2cWrite { addr, data }` - Write data
- `I2cRead { addr, len }` - Read data
- `I2cReadRegister { addr, reg }` - Read register
- `I2cWriteRegister { addr, reg, value }` - Write register

#### SPI Operations
- `SpiTransfer { data }` - Transfer data

#### UART Operations
- `UartWrite { data }` - Write data
- `UartRead { len }` - Read data
- `UartConfig { baudrate }` - Configure UART

#### File Operations
- `FileList { path }` - List files
- `FileRead { path }` - Read file
- `FileWrite { path, data }` - Write file

#### Configuration
- `SetConfig { key, value }` - Set configuration
- `GetConfig { key }` - Get configuration

### Implementation Notes

The Python client implements a minimal Postcard encoder/decoder sufficient for the Bus Pirate protocol. It's not a full Postcard implementation but handles:

- Varint encoding
- String encoding (length prefix + UTF-8)
- Byte array encoding (length prefix + bytes)
- Enum variants with payloads

The CRC implementation uses CRC-16-IBM-SDLC parameters:
- Polynomial: `0x1021`
- Initial value: `0xFFFF`
- Reflect input: `True`
- Reflect output: `True`
- XOR output: `0xFFFF`

This exactly matches the Rust `crc` crate's `CRC_16_IBM_SDLC` algorithm.

### Testing

The encoding/decoding is verified against the Rust implementation in the test suite. See `rust/protocol/tests/codec_tests.rs` for the compatibility test:

```rust
#[test]
fn test_getmode_encoding_matches_python() {
    // Verifies Rust and Python produce identical encodings
}
```

### Troubleshooting

**Permission denied on Linux**

Add your user to the `dialout` group:
```bash
sudo usermod -a -G dialout $USER
# Log out and back in for changes to take effect
```

**No module named 'serial'**

Install pyserial:
```bash
pip install pyserial
```

**Device not found**

Check available ports with `--list` and verify your device is connected.

## Future Tools

- `flash_firmware.sh` - Script to flash firmware to ESP32-S3
- `benchmark.py` - Performance benchmarking tool
- `capture.py` - Logic analyzer capture tool
- `protocol_fuzzer.py` - Fuzz testing for the protocol
