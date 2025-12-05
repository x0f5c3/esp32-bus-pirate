# ESP32 Bus Pirate Binary Protocol Specification

## Version 1.0

### Overview

The ESP32 Bus Pirate uses a compact binary protocol for communication between the host (PC, script, web interface) and the device. The protocol is designed to be:

- **Compact**: Minimal overhead using binary encoding
- **Reliable**: CRC-16 checksum for error detection
- **Extensible**: Version field allows protocol evolution
- **`no_std` compatible**: Works in embedded environments

### Protocol Stack

```
┌─────────────────────────────────┐
│     Application Layer           │
│  (Commands, Responses, Errors)  │
├─────────────────────────────────┤
│     Serialization Layer         │
│    (Postcard encoding)          │
├─────────────────────────────────┤
│     Framing Layer               │
│  (START, LEN, CRC, END markers) │
├─────────────────────────────────┤
│     Transport Layer             │
│  (Serial, USB CDC, WebSocket)   │
└─────────────────────────────────┘
```

### Message Frame Format

```
┌─────────┬─────────┬─────────┬──────────┬─────────┬─────────┐
│ START   │ VERSION │ LENGTH  │ PAYLOAD  │ CRC16   │  END    │
│ (0xAA)  │ (1 byte)│ (2 bytes│ (n bytes)│ (2 bytes│ (0x55)  │
│         │         │ LE)     │          │ LE)     │         │
└─────────┴─────────┴─────────┴──────────┴─────────┴─────────┘
```

#### Field Descriptions

| Field | Size | Description |
|-------|------|-------------|
| **START** | 1 byte | Frame start marker: `0xAA` |
| **VERSION** | 1 byte | Protocol version: `0x01` |
| **LENGTH** | 2 bytes | Payload length (little-endian, max 1024) |
| **PAYLOAD** | n bytes | Postcard-encoded message |
| **CRC16** | 2 bytes | CRC-16-IBM-SDLC over VERSION+LENGTH+PAYLOAD |
| **END** | 1 byte | Frame end marker: `0x55` |

**Note:** CRC is calculated over the VERSION, LENGTH, and PAYLOAD fields (not including START and END markers).

### Message Types

#### Command Messages

Commands are sent from host to device to request operations.

##### Mode Management

- **SetMode**: Change the current bus mode
  ```rust
  SetMode { mode: Mode::I2c }
  ```
- **GetMode**: Query the current mode
  ```rust
  GetMode
  ```

##### I2C Operations

- **I2cScan**: Scan for I2C devices
  ```rust
  I2cScan
  ```
- **I2cWrite**: Write data to an I2C device
  ```rust
  I2cWrite { addr: 0x50, data: vec![0x00, 0x01, 0x02] }
  ```
- **I2cRead**: Read data from an I2C device
  ```rust
  I2cRead { addr: 0x50, len: 16 }
  ```
- **I2cReadRegister**: Read a single register
  ```rust
  I2cReadRegister { addr: 0x50, reg: 0x00 }
  ```
- **I2cWriteRegister**: Write a single register
  ```rust
  I2cWriteRegister { addr: 0x50, reg: 0x00, value: 0xFF }
  ```

##### SPI Operations

- **SpiTransfer**: Full-duplex SPI transfer
  ```rust
  SpiTransfer { data: vec![0x9F, 0x00, 0x00, 0x00] }
  ```

##### UART Operations

- **UartWrite**: Send data over UART
  ```rust
  UartWrite { data: vec![0x48, 0x65, 0x6C, 0x6C, 0x6F] }
  ```
- **UartRead**: Read data from UART
  ```rust
  UartRead { len: 256 }
  ```
- **UartConfig**: Configure UART parameters
  ```rust
  UartConfig { baudrate: 115200 }
  ```

##### Configuration

- **SetConfig**: Set a configuration key-value pair
  ```rust
  SetConfig { key: "i2c_frequency", value: "400000" }
  ```
- **GetConfig**: Get a configuration value
  ```rust
  GetConfig { key: "i2c_frequency" }
  ```

##### File Operations

- **FileList**: List files in a directory
  ```rust
  FileList { path: "/scripts" }
  ```
- **FileRead**: Read file contents
  ```rust
  FileRead { path: "/config.txt" }
  ```
- **FileWrite**: Write file contents
  ```rust
  FileWrite { path: "/data.bin", data: vec![...] }
  ```

#### Response Messages

Responses are sent from device to host in reply to commands.

- **Response::Success**: Operation completed successfully
- **Response::Data(bytes)**: Binary data payload
- **Response::I2cDevices(addrs)**: List of I2C device addresses found
- **Response::CurrentMode(mode)**: Current operating mode
- **Response::ConfigValue(value)**: Configuration value
- **Response::FileList(files)**: List of file names

#### Error Messages

- **Error(ErrorCode)**: Operation failed with error code

Error codes:
- `InvalidCommand`: Unknown or malformed command
- `ProtocolError`: Protocol violation
- `BusError`: I2C/SPI/UART communication error
- `FileNotFound`: Requested file does not exist
- `PermissionDenied`: Operation not allowed
- `Timeout`: Operation timed out
- `NotConfigured`: Bus mode not initialized
- `InvalidParameter`: Invalid parameter value

### Encoding Example

#### Command: I2C Scan

1. **Message structure (Rust)**:
   ```rust
   Message::I2cScan
   ```

2. **Postcard encoding**:
   ```
   Payload: [0x02]  // Variant index for I2cScan
   ```

3. **Frame construction**:
   ```
   START:   0xAA
   VERSION: 0x01
   LENGTH:  0x01 0x00  (little-endian: 1 byte)
   PAYLOAD: 0x02
   CRC16:   0xXX 0xXX  (calculated over [0x01, 0x01, 0x00, 0x02])
   END:     0x55
   ```

4. **Complete frame (hex)**:
   ```
   AA 01 01 00 02 [CRC_L] [CRC_H] 55
   ```

#### Response: I2C Devices Found

1. **Message structure (Rust)**:
   ```rust
   Message::Response(Response::I2cDevices(vec![0x50, 0x68, 0x76]))
   ```

2. **Postcard encoding**:
   ```
   Payload: [0x0F, 0x01, 0x03, 0x50, 0x68, 0x76]
   // 0x0F: Message::Response variant
   // 0x01: Response::I2cDevices variant
   // 0x03: Vec length
   // 0x50, 0x68, 0x76: Device addresses
   ```

3. **Complete frame**: Similar to above with 6-byte payload

### CRC Calculation

The protocol uses **CRC-16-IBM-SDLC** (also known as CRC-16-CCITT-FALSE or CRC-16-ANSI).

**Parameters:**
- Polynomial: 0x8005
- Initial value: 0xFFFF
- Reflect input: True
- Reflect output: True
- Final XOR: 0x0000

**Rust implementation**:
```rust
use crc::{Crc, CRC_16_IBM_SDLC};

const CRC: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);

fn calculate_crc(data: &[u8]) -> u16 {
    CRC.checksum(data)
}
```

### Error Handling

#### Malformed Frame

If a frame is malformed (invalid START/END, wrong CRC, unsupported version), the device should:
1. Discard the frame
2. Optionally send an `Error(ProtocolError)` response
3. Continue listening for valid frames

#### Command Errors

If a command cannot be executed (e.g., bus error, timeout), the device sends:
```rust
Message::Error(ErrorCode::BusError)
```

### Transport Layers

#### Serial / USB CDC

- Baud rate: 115200 (default), configurable
- Data bits: 8
- Parity: None
- Stop bits: 1
- Flow control: None

Frames are sent as raw byte streams over the serial port.

#### WebSocket

- URL: `ws://<device-ip>/ws`
- Frames are sent as binary WebSocket messages
- Each protocol frame = one WebSocket message

#### Network (TCP)

- Port: 5555 (default)
- Raw TCP socket
- Same binary protocol as serial

### Version Negotiation

**Current version: 1.0 (0x01)**

Future versions may support:
- Protocol feature negotiation
- Backward compatibility mode
- Extended message types

For now, only exact version match (0x01) is accepted.

### Security Considerations

- **No encryption**: Protocol is plaintext
- **No authentication**: No password or token required
- **File access**: Limited to specific directories (e.g., `/scripts`, `/logs`)
- **Protected pins**: Certain GPIO pins are hardware-protected

For secure communication, use TLS over WebSocket or SSH tunneling.

### Implementation Notes

#### Message Size Limits

- Maximum payload size: 1024 bytes
- Maximum file transfer chunk: 512 bytes
- For large files, use multiple FileWrite commands

#### Timeouts

- Command execution timeout: 5 seconds
- I2C operation timeout: 1 second
- UART read timeout: 100ms (or until buffer full)

#### Flow Control

No explicit flow control. Host should wait for response before sending next command.

For high-throughput applications (e.g., UART bridge), consider using asynchronous responses or streaming mode (future feature).

### Example Session

```
Host → Device: SetMode { mode: I2c }
Device → Host: Response(Success)

Host → Device: I2cScan
Device → Host: Response(I2cDevices([0x50, 0x68]))

Host → Device: I2cReadRegister { addr: 0x50, reg: 0x00 }
Device → Host: Response(Data([0xA5]))

Host → Device: SetMode { mode: HiZ }
Device → Host: Response(Success)
```

### References

- Postcard encoding: https://docs.rs/postcard/
- CRC-16-IBM-SDLC: https://docs.rs/crc/
- Original Bus Pirate protocol: http://dangerousprototypes.com/docs/Bus_Pirate

---

**Last updated**: 2025-12-05  
**Author**: ESP32 Bus Pirate Rust Migration Team
