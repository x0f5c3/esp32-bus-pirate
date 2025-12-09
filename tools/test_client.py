#!/usr/bin/env python3
"""
ESP32 Bus Pirate Protocol Test Client

This client implements the binary protocol for communicating with the ESP32 Bus Pirate
over USB serial. It provides both CLI and interactive modes for sending commands.

Protocol Frame Format:
┌─────────┬─────────┬─────────┬──────────┬─────────┬─────────┐
│ START   │ VERSION │ LENGTH  │ PAYLOAD  │ CRC16   │  END    │
│ (0xAA)  │ (1 byte)│ (2 bytes│ (n bytes)│ (2 bytes│ (0x55)  │
└─────────┴─────────┴─────────┴──────────┴─────────┴─────────┘

Uses Postcard serialization for the payload and CRC-16-IBM-SDLC for integrity checking.
"""

import argparse
import struct
import sys
from dataclasses import dataclass
from enum import IntEnum
from typing import List, Optional, Tuple, Union
import serial
import serial.tools.list_ports

# Protocol constants
START_BYTE = 0xAA
END_BYTE = 0x55
PROTOCOL_VERSION = 0x01
MAX_MESSAGE_SIZE = 1024

# CRC-16-IBM-SDLC parameters (also known as CRC-16/X-25)
# poly=0x1021, init=0xFFFF, refin=True, refout=True, xorout=0xFFFF
CRC_POLY = 0x1021
CRC_INIT = 0xFFFF
CRC_XOROUT = 0xFFFF


class Mode(IntEnum):
    """Bus Pirate operating modes"""
    HIZ = 0
    I2C = 1
    SPI = 2
    UART = 3
    ONE_WIRE = 4
    TWO_WIRE = 5
    THREE_WIRE = 6
    DIO = 7
    INFRARED = 8
    USB = 9
    BLUETOOTH = 10
    WIFI = 11
    ETHERNET = 12
    JTAG = 13
    LED = 14
    I2S = 15
    CAN = 16
    SUB_GHZ = 17
    RFID = 18
    RF24 = 19


class ErrorCode(IntEnum):
    """Protocol error codes"""
    INVALID_COMMAND = 0
    PROTOCOL_ERROR = 1
    BUS_ERROR = 2
    FILE_NOT_FOUND = 3
    PERMISSION_DENIED = 4
    TIMEOUT = 5
    NOT_CONFIGURED = 6
    INVALID_PARAMETER = 7


def crc16_ibm_sdlc(data: bytes) -> int:
    """
    Calculate CRC-16-IBM-SDLC checksum.
    
    This matches the Rust implementation using the crc crate with CRC_16_IBM_SDLC.
    Parameters: poly=0x1021, init=0xFFFF, refin=True, refout=True, xorout=0xFFFF
    
    Args:
        data: Bytes to calculate CRC for
        
    Returns:
        16-bit CRC value
    """
    def reflect_byte(byte):
        """Reflect (reverse) bits in a byte"""
        return int('{:08b}'.format(byte)[::-1], 2)
    
    def reflect_u16(value):
        """Reflect (reverse) bits in a 16-bit value"""
        return int('{:016b}'.format(value)[::-1], 2)
    
    crc = CRC_INIT
    
    for byte in data:
        # Reflect input byte
        byte_reflected = reflect_byte(byte)
        crc ^= byte_reflected << 8
        
        for _ in range(8):
            if crc & 0x8000:
                crc = (crc << 1) ^ CRC_POLY
            else:
                crc = crc << 1
            crc &= 0xFFFF
    
    # Reflect output
    crc_reflected = reflect_u16(crc)
    
    # XOR out
    return crc_reflected ^ CRC_XOROUT


class PostcardEncoder:
    """
    Simple Postcard-compatible encoder for the protocol messages.
    
    This is a minimal implementation supporting only the types used in the protocol.
    It's not a full Postcard implementation but matches the encoding for our message types.
    """
    
    @staticmethod
    def encode_varint(value: int) -> bytes:
        """Encode an unsigned integer as a varint"""
        result = bytearray()
        while value > 0x7F:
            result.append((value & 0x7F) | 0x80)
            value >>= 7
        result.append(value & 0x7F)
        return bytes(result)
    
    @staticmethod
    def encode_string(s: str) -> bytes:
        """Encode a string (length prefix + UTF-8 bytes)"""
        utf8_bytes = s.encode('utf-8')
        return PostcardEncoder.encode_varint(len(utf8_bytes)) + utf8_bytes
    
    @staticmethod
    def encode_bytes(data: bytes) -> bytes:
        """Encode a byte array (length prefix + bytes)"""
        return PostcardEncoder.encode_varint(len(data)) + data


class MessageEncoder:
    """Encodes protocol messages into binary frames"""
    
    @staticmethod
    def encode_set_mode(mode: Mode) -> bytes:
        """Encode SetMode message"""
        # Enum variant index 0, then mode value
        payload = struct.pack('<BB', 0, mode)
        return MessageEncoder._frame(payload)
    
    @staticmethod
    def encode_get_mode() -> bytes:
        """Encode GetMode message"""
        # Enum variant index 1
        payload = struct.pack('<B', 1)
        return MessageEncoder._frame(payload)
    
    @staticmethod
    def encode_i2c_scan() -> bytes:
        """Encode I2cScan message"""
        # Enum variant index 2
        payload = struct.pack('<B', 2)
        return MessageEncoder._frame(payload)
    
    @staticmethod
    def encode_i2c_write(addr: int, data: bytes) -> bytes:
        """Encode I2cWrite message"""
        # Enum variant index 3, address, then data array
        payload = struct.pack('<BB', 3, addr)
        payload += PostcardEncoder.encode_bytes(data)
        return MessageEncoder._frame(payload)
    
    @staticmethod
    def encode_i2c_read(addr: int, length: int) -> bytes:
        """Encode I2cRead message"""
        # Enum variant index 4, address, length
        payload = struct.pack('<BBB', 4, addr, length)
        return MessageEncoder._frame(payload)
    
    @staticmethod
    def encode_i2c_read_register(addr: int, reg: int) -> bytes:
        """Encode I2cReadRegister message"""
        # Enum variant index 5, address, register
        payload = struct.pack('<BBB', 5, addr, reg)
        return MessageEncoder._frame(payload)
    
    @staticmethod
    def encode_i2c_write_register(addr: int, reg: int, value: int) -> bytes:
        """Encode I2cWriteRegister message"""
        # Enum variant index 6, address, register, value
        payload = struct.pack('<BBBB', 6, addr, reg, value)
        return MessageEncoder._frame(payload)
    
    @staticmethod
    def encode_spi_transfer(data: bytes) -> bytes:
        """Encode SpiTransfer message"""
        # Enum variant index 7, then data array
        payload = struct.pack('<B', 7)
        payload += PostcardEncoder.encode_bytes(data)
        return MessageEncoder._frame(payload)
    
    @staticmethod
    def encode_uart_write(data: bytes) -> bytes:
        """Encode UartWrite message"""
        # Enum variant index 8, then data array
        payload = struct.pack('<B', 8)
        payload += PostcardEncoder.encode_bytes(data)
        return MessageEncoder._frame(payload)
    
    @staticmethod
    def encode_uart_read(length: int) -> bytes:
        """Encode UartRead message"""
        # Enum variant index 9, length as u16
        payload = struct.pack('<BH', 9, length)
        return MessageEncoder._frame(payload)
    
    @staticmethod
    def encode_uart_config(baudrate: int) -> bytes:
        """Encode UartConfig message"""
        # Enum variant index 10, baudrate as u32
        payload = struct.pack('<BI', 10, baudrate)
        return MessageEncoder._frame(payload)
    
    @staticmethod
    def _frame(payload: bytes) -> bytes:
        """
        Frame a payload with protocol headers and CRC.
        
        Frame format:
        START + VERSION + LENGTH(2) + PAYLOAD + CRC(2) + END
        """
        length = len(payload)
        
        # Build the frame up to the CRC
        frame = bytearray()
        frame.append(START_BYTE)
        frame.append(PROTOCOL_VERSION)
        frame.extend(struct.pack('<H', length))  # Little-endian 16-bit length
        frame.extend(payload)
        
        # Calculate CRC over VERSION + LENGTH + PAYLOAD
        crc_data = bytes(frame[1:])
        crc = crc16_ibm_sdlc(crc_data)
        
        # Add CRC and END byte
        frame.extend(struct.pack('<H', crc))  # Little-endian 16-bit CRC
        frame.append(END_BYTE)
        
        return bytes(frame)


class MessageDecoder:
    """Decodes binary frames into protocol messages"""
    
    @staticmethod
    def decode(frame: bytes) -> Optional[dict]:
        """
        Decode a binary frame.
        
        Returns:
            Dictionary with decoded message or None if invalid
        """
        if len(frame) < 7:
            print(f"Error: Frame too short ({len(frame)} bytes)")
            return None
        
        # Check frame markers
        if frame[0] != START_BYTE:
            print(f"Error: Invalid start byte (0x{frame[0]:02X})")
            return None
        
        if frame[-1] != END_BYTE:
            print(f"Error: Invalid end byte (0x{frame[-1]:02X})")
            return None
        
        # Check version
        version = frame[1]
        if version != PROTOCOL_VERSION:
            print(f"Error: Unsupported version (0x{version:02X})")
            return None
        
        # Extract length
        length = struct.unpack('<H', frame[2:4])[0]
        
        # Extract payload and CRC
        payload_end = 4 + length
        if len(frame) < payload_end + 3:
            print(f"Error: Frame truncated")
            return None
        
        payload = frame[4:payload_end]
        crc_received = struct.unpack('<H', frame[payload_end:payload_end + 2])[0]
        
        # Verify CRC
        crc_data = frame[1:payload_end]
        crc_calculated = crc16_ibm_sdlc(crc_data)
        
        if crc_received != crc_calculated:
            print(f"Error: CRC mismatch (received 0x{crc_received:04X}, calculated 0x{crc_calculated:04X})")
            return None
        
        # Decode payload (simplified - just return raw for now)
        return {
            'version': version,
            'payload': payload,
            'payload_hex': payload.hex(),
        }


class BusPirateClient:
    """Client for communicating with the Bus Pirate over serial"""
    
    def __init__(self, port: str, baudrate: int = 115200):
        """
        Initialize the client.
        
        Args:
            port: Serial port path (e.g., '/dev/ttyUSB0' or 'COM3')
            baudrate: Serial baud rate
        """
        self.port = port
        self.baudrate = baudrate
        self.serial: Optional[serial.Serial] = None
    
    def connect(self) -> bool:
        """
        Connect to the Bus Pirate.
        
        Returns:
            True if connected successfully
        """
        try:
            self.serial = serial.Serial(
                port=self.port,
                baudrate=self.baudrate,
                timeout=1.0,
                write_timeout=1.0
            )
            print(f"Connected to {self.port} at {self.baudrate} baud")
            return True
        except Exception as e:
            print(f"Error connecting to {self.port}: {e}")
            return False
    
    def disconnect(self):
        """Disconnect from the Bus Pirate"""
        if self.serial and self.serial.is_open:
            self.serial.close()
            print("Disconnected")
    
    def send_message(self, message: bytes) -> bool:
        """
        Send a message to the Bus Pirate.
        
        Args:
            message: Encoded message frame
            
        Returns:
            True if sent successfully
        """
        if not self.serial or not self.serial.is_open:
            print("Error: Not connected")
            return False
        
        try:
            self.serial.write(message)
            print(f"Sent {len(message)} bytes: {message.hex()}")
            return True
        except Exception as e:
            print(f"Error sending message: {e}")
            return False
    
    def receive_message(self, timeout: float = 1.0) -> Optional[bytes]:
        """
        Receive a message from the Bus Pirate.
        
        Args:
            timeout: Receive timeout in seconds
            
        Returns:
            Received frame or None
        """
        if not self.serial or not self.serial.is_open:
            print("Error: Not connected")
            return None
        
        try:
            self.serial.timeout = timeout
            
            # Wait for start byte
            start = self.serial.read(1)
            if len(start) == 0:
                print("Timeout waiting for start byte")
                return None
            
            if start[0] != START_BYTE:
                print(f"Warning: Expected start byte, got 0x{start[0]:02X}")
                return None
            
            # Read version and length
            header = self.serial.read(3)
            if len(header) < 3:
                print("Error: Incomplete header")
                return None
            
            version = header[0]
            length = struct.unpack('<H', header[1:3])[0]
            
            # Read payload + CRC + END
            remaining = length + 3
            data = self.serial.read(remaining)
            if len(data) < remaining:
                print(f"Error: Incomplete frame (expected {remaining}, got {len(data)})")
                return None
            
            # Reconstruct full frame
            frame = start + header + data
            print(f"Received {len(frame)} bytes: {frame.hex()}")
            
            return frame
            
        except Exception as e:
            print(f"Error receiving message: {e}")
            return None
    
    def send_and_receive(self, message: bytes, timeout: float = 1.0) -> Optional[dict]:
        """
        Send a message and wait for a response.
        
        Args:
            message: Encoded message frame
            timeout: Receive timeout in seconds
            
        Returns:
            Decoded response or None
        """
        if not self.send_message(message):
            return None
        
        response_frame = self.receive_message(timeout)
        if response_frame is None:
            return None
        
        return MessageDecoder.decode(response_frame)


def list_serial_ports():
    """List available serial ports"""
    ports = serial.tools.list_ports.comports()
    if not ports:
        print("No serial ports found")
        return
    
    print("Available serial ports:")
    for port in ports:
        print(f"  {port.device}: {port.description}")


def interactive_mode(client: BusPirateClient):
    """Run interactive command-line interface"""
    print("\nESP32 Bus Pirate Interactive Mode")
    print("==================================")
    print("Commands:")
    print("  mode <mode>          - Set bus mode (i2c, spi, uart, etc.)")
    print("  getmode              - Get current mode")
    print("  i2c scan             - Scan I2C bus")
    print("  i2c write <addr> <data> - Write to I2C device")
    print("  i2c read <addr> <len>   - Read from I2C device")
    print("  quit / exit          - Exit")
    print()
    
    while True:
        try:
            cmd = input("BP> ").strip().lower()
            
            if not cmd:
                continue
            
            if cmd in ['quit', 'exit']:
                break
            
            parts = cmd.split()
            
            if parts[0] == 'mode' and len(parts) == 2:
                mode_name = parts[1].upper()
                try:
                    mode = Mode[mode_name]
                    msg = MessageEncoder.encode_set_mode(mode)
                    resp = client.send_and_receive(msg)
                    if resp:
                        print(f"Response: {resp}")
                except KeyError:
                    print(f"Unknown mode: {parts[1]}")
            
            elif parts[0] == 'getmode':
                msg = MessageEncoder.encode_get_mode()
                resp = client.send_and_receive(msg)
                if resp:
                    print(f"Response: {resp}")
            
            elif parts[0] == 'i2c':
                if len(parts) < 2:
                    print("Usage: i2c <scan|write|read>")
                    continue
                
                if parts[1] == 'scan':
                    msg = MessageEncoder.encode_i2c_scan()
                    resp = client.send_and_receive(msg)
                    if resp:
                        print(f"Response: {resp}")
                
                elif parts[1] == 'write' and len(parts) >= 3:
                    addr = int(parts[2], 0)
                    data = bytes.fromhex(''.join(parts[3:]))
                    msg = MessageEncoder.encode_i2c_write(addr, data)
                    resp = client.send_and_receive(msg)
                    if resp:
                        print(f"Response: {resp}")
                
                elif parts[1] == 'read' and len(parts) == 4:
                    addr = int(parts[2], 0)
                    length = int(parts[3])
                    msg = MessageEncoder.encode_i2c_read(addr, length)
                    resp = client.send_and_receive(msg)
                    if resp:
                        print(f"Response: {resp}")
                
                else:
                    print("Usage: i2c scan | i2c write <addr> <hex_data> | i2c read <addr> <len>")
            
            else:
                print(f"Unknown command: {cmd}")
        
        except KeyboardInterrupt:
            print("\nInterrupted")
            break
        except Exception as e:
            print(f"Error: {e}")


def main():
    """Main entry point"""
    parser = argparse.ArgumentParser(
        description='ESP32 Bus Pirate Protocol Test Client',
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog='''
Examples:
  # List available ports
  %(prog)s --list
  
  # Test CRC calculation
  %(prog)s --test-crc
  
  # Send a command
  %(prog)s --port /dev/ttyUSB0 --command getmode
  
  # Interactive mode
  %(prog)s --port /dev/ttyUSB0 --interactive
        '''
    )
    
    parser.add_argument('--list', action='store_true',
                        help='List available serial ports')
    parser.add_argument('--test-crc', action='store_true',
                        help='Test CRC calculation')
    parser.add_argument('--port', type=str,
                        help='Serial port (e.g., /dev/ttyUSB0 or COM3)')
    parser.add_argument('--baudrate', type=int, default=115200,
                        help='Serial baud rate (default: 115200)')
    parser.add_argument('--command', type=str,
                        help='Command to send (getmode, i2cscan, etc.)')
    parser.add_argument('--interactive', '-i', action='store_true',
                        help='Enter interactive mode')
    
    args = parser.parse_args()
    
    if args.list:
        list_serial_ports()
        return 0
    
    if args.test_crc:
        # Test CRC calculation
        test_data = b"Hello, World!"
        crc = crc16_ibm_sdlc(test_data)
        print(f"Test data: {test_data}")
        print(f"CRC-16-IBM-SDLC: 0x{crc:04X}")
        
        # Test encoding
        msg = MessageEncoder.encode_get_mode()
        print(f"\nGetMode message: {msg.hex()}")
        
        # Test decoding
        decoded = MessageDecoder.decode(msg)
        if decoded:
            print(f"Decoded: {decoded}")
        
        return 0
    
    if not args.port:
        print("Error: --port required (or use --list to see available ports)")
        return 1
    
    # Connect to Bus Pirate
    client = BusPirateClient(args.port, args.baudrate)
    if not client.connect():
        return 1
    
    try:
        if args.interactive:
            interactive_mode(client)
        elif args.command:
            # Execute single command
            cmd = args.command.lower()
            
            if cmd == 'getmode':
                msg = MessageEncoder.encode_get_mode()
            elif cmd == 'i2cscan':
                msg = MessageEncoder.encode_i2c_scan()
            else:
                print(f"Unknown command: {cmd}")
                return 1
            
            resp = client.send_and_receive(msg)
            if resp:
                print(f"Success: {resp}")
            else:
                print("No response or error")
                return 1
        else:
            print("Specify --command or --interactive")
            return 1
    
    finally:
        client.disconnect()
    
    return 0


if __name__ == '__main__':
    sys.exit(main())
