#![no_std]
#![no_main]

//! Basic HAL Example
//!
//! This example demonstrates the basic usage of the ESP32 Bus Pirate HAL
//! for the Waveshare ESP32-S3-Touch-LCD-2.8 board.
//!
//! ## Hardware
//!
//! - Waveshare ESP32-S3-Touch-LCD-2.8 board
//!
//! ## What it does
//!
//! 1. Initializes the board
//! 2. Blinks the backlight
//! 3. Scans the I2C bus for devices
//! 4. Prints results to UART

use esp_backtrace as _;
use esp_hal::{entry, prelude::*};
use esp_println::println;

use esp32_bus_pirate_hal::{
    WaveshareS3Board,
    peripherals::i2c::I2cExt,
};

#[entry]
fn main() -> ! {
    println!("ESP32 Bus Pirate HAL - Basic Example");
    println!("====================================\n");

    // Initialize the board
    println!("Initializing board...");
    let mut board = WaveshareS3Board::new();
    println!("Board initialized!\n");

    // Initialize display and touch
    println!("Initializing display...");
    board.init_display();
    println!("Display initialized!\n");

    println!("Initializing touch controller...");
    board.init_touch();
    println!("Touch controller initialized!\n");

    // Blink backlight a few times
    println!("Blinking backlight...");
    for i in 0..5 {
        println!("  Blink {}/5", i + 1);
        board.set_backlight(true);
        board.delay.delay_millis(200);
        board.set_backlight(false);
        board.delay.delay_millis(200);
    }
    board.set_backlight(true);
    println!("Backlight on!\n");

    // Scan I2C bus
    println!("Scanning I2C bus...");
    let devices = board.i2c0.scan();
    
    if devices.is_empty() {
        println!("No I2C devices found");
    } else {
        println!("Found {} I2C device(s):", devices.len());
        for addr in devices {
            println!("  - 0x{:02X}", addr);
            
            // Print known device names
            match addr {
                0x5A => println!("    (CST328 Touch Controller)"),
                0x6B => println!("    (QMI8658C IMU)"),
                0x51 => println!("    (PCF85063 RTC)"),
                _ => println!("    (Unknown device)"),
            }
        }
    }
    println!();

    // Check touch interrupt
    println!("Touch interrupt status: {}", 
        if board.touch_interrupt_active() { "ACTIVE" } else { "INACTIVE" });
    println!();

    println!("Example complete! Entering idle loop...");
    
    // Main loop
    loop {
        // In a real application, this would be the main event loop
        board.delay.delay_millis(1000);
    }
}
