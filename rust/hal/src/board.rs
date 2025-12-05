//! Board initialization for Waveshare ESP32-S3-Touch-LCD-2.8

use esp_hal::{
    clock::ClockControl,
    gpio::{Io, Level, Output},
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    spi::{master::Spi, FullDuplexMode, SpiMode},
    system::SystemControl,
    timer::timg::TimerGroup,
    Delay,
};

use crate::pins;

/// Main board structure with initialized peripherals
pub struct WaveshareS3Board {
    /// Delay provider
    pub delay: Delay,
    
    /// SPI bus for display (and possibly SD card)
    pub display_spi: Spi<'static, esp_hal::peripherals::SPI2, FullDuplexMode>,
    
    /// Display control pins
    pub display_dc: Output<'static>,
    pub display_cs: Output<'static>,
    pub display_rst: Output<'static>,
    pub display_bl: Output<'static>,
    
    /// I2C bus for touch, IMU, and RTC
    pub i2c0: I2C<'static, esp_hal::peripherals::I2C0>,
    
    /// Touch controller pins
    pub touch_int: Output<'static>, // Could be input with interrupt
    pub touch_rst: Output<'static>,
    
    // Additional peripherals can be added here as needed
    // pub uart0: Uart<'static, esp_hal::peripherals::UART0>,
    // pub usb: UsbBus,
}

impl WaveshareS3Board {
    /// Initialize the board with default configuration
    ///
    /// This function:
    /// - Initializes system clocks
    /// - Configures SPI for the display
    /// - Configures I2C for touch/IMU/RTC
    /// - Sets up GPIO pins
    ///
    /// # Returns
    /// An initialized `WaveshareS3Board` instance
    pub fn new() -> Self {
        let peripherals = Peripherals::take();
        let system = SystemControl::new(peripherals.SYSTEM);
        
        // Configure clocks to run at maximum speed (240 MHz)
        let clocks = ClockControl::max(system.clock_control).freeze();
        
        // Initialize GPIO
        let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
        
        // Create delay provider
        let delay = Delay::new(&clocks);
        
        // ===== Display SPI Setup =====
        let sclk = io.pins.gpio40;
        let mosi = io.pins.gpio45;
        let miso = io.pins.gpio48; // Not used for display, but SPI needs it
        let cs = Output::new(io.pins.gpio42, Level::High);
        
        let display_spi = Spi::new(peripherals.SPI2, 40.MHz(), SpiMode::Mode0, &clocks)
            .with_sck(sclk)
            .with_mosi(mosi)
            .with_miso(miso);
        
        // Display control pins
        let display_dc = Output::new(io.pins.gpio41, Level::Low);
        let display_rst = Output::new(io.pins.gpio39, Level::High);
        let display_bl = Output::new(io.pins.gpio5, Level::High); // Backlight on
        
        // ===== I2C Setup for Touch/IMU/RTC =====
        let sda = io.pins.gpio1;
        let scl = io.pins.gpio3;
        
        let i2c0 = I2C::new(
            peripherals.I2C0,
            sda,
            scl,
            100.kHz(), // 100 kHz for touch controller
            &clocks,
        );
        
        // Touch controller pins
        let touch_int = Output::new(io.pins.gpio4, Level::High);
        let touch_rst = Output::new(io.pins.gpio2, Level::High);
        
        Self {
            delay,
            display_spi,
            display_dc,
            display_cs,
            display_rst,
            display_bl,
            i2c0,
            touch_int,
            touch_rst,
        }
    }
    
    /// Get a reference to the delay provider
    pub fn delay_mut(&mut self) -> &mut Delay {
        &mut self.delay
    }
}

impl Default for WaveshareS3Board {
    fn default() -> Self {
        Self::new()
    }
}
