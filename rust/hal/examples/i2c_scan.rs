#![no_std]
#![no_main]

//! I2C Scanner Example
//!
//! This example demonstrates I2C bus scanning and device detection
//! using the ESP32 Bus Pirate HAL.
//!
//! ## Hardware
//!
//! - Waveshare ESP32-S3-Touch-LCD-2.8 board
//!
//! ## What it does
//!
//! Continuously scans the I2C bus and reports all detected devices.

use esp_backtrace as _;
use esp_hal::{entry, prelude::*};
use esp_println::println;

use esp32_bus_pirate_hal::{
    WaveshareS3Board,
    peripherals::i2c::I2cExt,
};

#[entry]
fn main() -> ! {
    println!("ESP32 Bus Pirate HAL - I2C Scanner Example");
    println!("==========================================\n");

    // Initialize the board
    let mut board = WaveshareS3Board::new();
    println!("Board initialized!\n");

    // Known I2C addresses on the board
    const KNOWN_DEVICES: &[(u8, &str)] = &[
        (0x5A, "CST328 Touch Controller"),
        (0x6B, "QMI8658C IMU"),
        (0x51, "PCF85063 RTC"),
    ];

    println!("Starting I2C bus scan...\n");
    println!("Known devices on this board:");
    for (addr, name) in KNOWN_DEVICES {
        println!("  0x{:02X} - {}", addr, name);
    }
    println!();

    let mut scan_count = 0;
    
    loop {
        scan_count += 1;
        println!("=== Scan #{} ===", scan_count);
        
        // Scan the I2C bus
        let devices = board.i2c0.scan();
        
        if devices.is_empty() {
            println!("No I2C devices found!");
        } else {
            println!("Found {} device(s):", devices.len());
            
            for addr in devices {
                print!("  0x{:02X}", addr);
                
                // Check if it's a known device
                if let Some((_, name)) = KNOWN_DEVICES.iter().find(|(a, _)| *a == addr) {
                    print!(" - {}", name);
                } else {
                    print!(" - Unknown device");
                }
                println!();
            }
        }
        
        println!();
        
        // Wait before next scan
        board.delay.delay_millis(2000);
    }
}
