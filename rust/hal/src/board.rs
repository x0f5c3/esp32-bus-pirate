//! Board initialization for Waveshare ESP32-S3-Touch-LCD-2.8
//!
//! This module provides the main board initialization and peripheral configuration
//! for the Waveshare ESP32-S3-Touch-LCD-2.8 development board.
//!
//! # Hardware Features
//!
//! - **Display**: ST7789 240x320 LCD via SPI2 @ 40MHz
//! - **Touch**: CST328 capacitive touch via I2C @ 100kHz
//! - **IMU**: QMI8658C 6-axis IMU via I2C
//! - **RTC**: PCF85063 real-time clock via I2C
//! - **SD Card**: MicroSD via SPI3
//! - **Audio**: PCM5101A DAC via I2S
//!
//! # Example
//!
//! ```no_run
//! use esp32_bus_pirate_hal::WaveshareS3Board;
//!
//! // Initialize the board with default configuration
//! let mut board = WaveshareS3Board::new();
//!
//! // Access peripherals
//! board.display_bl.set_high();  // Turn on backlight
//! ```

use esp_hal::{
    clock::ClockControl,
    gpio::{Io, Level, Output, Input},
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    spi::{master::Spi, FullDuplexMode, SpiMode},
    system::SystemControl,
    timer::timg::TimerGroup,
    Delay,
};

use crate::peripherals::{
    i2c::{I2cBus, I2cConfig},
    spi::{SpiBus2, SpiBus3, SpiConfig},
};
use crate::pins;

/// Main board structure with initialized peripherals
///
/// This structure contains all the peripherals and pins needed to interact
/// with the Waveshare ESP32-S3-Touch-LCD-2.8 board hardware.
pub struct WaveshareS3Board {
    /// Delay provider for precise timing
    pub delay: Delay,
    
    /// SPI2 bus for display @ 40MHz
    pub display_spi: Spi<'static, esp_hal::peripherals::SPI2, FullDuplexMode>,
    
    /// Display control pins
    pub display_dc: Output<'static>,   // Data/Command select
    pub display_cs: Output<'static>,   // Chip select (active low)
    pub display_rst: Output<'static>,  // Reset (active low)
    pub display_bl: Output<'static>,   // Backlight control
    
    /// I2C0 bus for touch controller, IMU, and RTC @ 100kHz
    pub i2c0: I2C<'static, esp_hal::peripherals::I2C0>,
    
    /// Touch controller pins
    pub touch_int: Input<'static>,     // Interrupt pin (active low)
    pub touch_rst: Output<'static>,    // Reset (active low)
    
    /// SPI3 bus for SD card
    pub sdcard_spi: Spi<'static, esp_hal::peripherals::SPI3, FullDuplexMode>,
    pub sdcard_cs: Output<'static>,    // SD card chip select
    
    // Timer group for general timing operations
    // pub timer_group: TimerGroup<'static, esp_hal::peripherals::TIMG0>,
}

impl WaveshareS3Board {
    /// Initialize the board with default configuration
    ///
    /// This function performs complete board initialization:
    /// - Configures system clocks to 240 MHz
    /// - Initializes SPI2 @ 40MHz for display
    /// - Initializes SPI3 @ 20MHz for SD card
    /// - Initializes I2C0 @ 100kHz for touch/IMU/RTC
    /// - Configures all GPIO pins
    /// - Creates a delay provider
    ///
    /// # Returns
    ///
    /// An initialized `WaveshareS3Board` instance ready for use
    ///
    /// # Example
    ///
    /// ```no_run
    /// let board = WaveshareS3Board::new();
    /// ```
    pub fn new() -> Self {
        let peripherals = Peripherals::take();
        let system = SystemControl::new(peripherals.SYSTEM);
        
        // Configure clocks to run at maximum speed (240 MHz)
        let clocks = ClockControl::max(system.clock_control).freeze();
        
        // Initialize GPIO
        let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
        
        // Create delay provider
        let delay = Delay::new(&clocks);
        
        // ===== Display SPI2 Setup @ 40MHz =====
        let sclk = io.pins.gpio40;
        let mosi = io.pins.gpio45;
        let miso = io.pins.gpio48; // Not used for display, but required by SPI
        let cs = Output::new(io.pins.gpio42, Level::High);
        
        let display_spi = Spi::new(peripherals.SPI2, 40.MHz(), SpiMode::Mode0, &clocks)
            .with_sck(sclk)
            .with_mosi(mosi)
            .with_miso(miso);
        
        // Display control pins
        let display_dc = Output::new(io.pins.gpio41, Level::Low);
        let display_rst = Output::new(io.pins.gpio39, Level::High);
        let display_bl = Output::new(io.pins.gpio5, Level::High); // Backlight on by default
        
        // ===== I2C0 Setup for Touch/IMU/RTC @ 100kHz =====
        let sda = io.pins.gpio1;
        let scl = io.pins.gpio3;
        
        let i2c0 = I2C::new(
            peripherals.I2C0,
            sda,
            scl,
            100.kHz(), // 100 kHz for touch controller compatibility
            &clocks,
        );
        
        // Touch controller pins
        let touch_int = Input::new(io.pins.gpio4, esp_hal::gpio::Pull::Up);
        let touch_rst = Output::new(io.pins.gpio2, Level::High);
        
        // ===== SD Card SPI3 Setup @ 20MHz =====
        let sd_sclk = io.pins.gpio14;
        let sd_mosi = io.pins.gpio17;
        let sd_miso = io.pins.gpio16;
        let sdcard_cs = Output::new(io.pins.gpio21, Level::High);
        
        let sdcard_spi = Spi::new(peripherals.SPI3, 20.MHz(), SpiMode::Mode0, &clocks)
            .with_sck(sd_sclk)
            .with_mosi(sd_mosi)
            .with_miso(sd_miso);
        
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
            sdcard_spi,
            sdcard_cs,
        }
    }
    
    /// Get a mutable reference to the delay provider
    ///
    /// This is useful for performing timed operations.
    pub fn delay_mut(&mut self) -> &mut Delay {
        &mut self.delay
    }
    
    /// Initialize the display
    ///
    /// Performs the necessary reset sequence for the ST7789 display.
    pub fn init_display(&mut self) {
        // Reset sequence: LOW for 10ms, then HIGH
        self.display_rst.set_low();
        self.delay.delay_millis(10);
        self.display_rst.set_high();
        self.delay.delay_millis(120);
    }
    
    /// Set display backlight brightness
    ///
    /// Note: This is a simple on/off control. For PWM brightness control,
    /// use the `peripherals::gpio::PwmChannel` wrapper.
    ///
    /// # Arguments
    ///
    /// * `on` - true to turn backlight on, false to turn it off
    pub fn set_backlight(&mut self, on: bool) {
        if on {
            self.display_bl.set_high();
        } else {
            self.display_bl.set_low();
        }
    }
    
    /// Initialize the touch controller
    ///
    /// Performs the necessary reset sequence for the CST328 touch controller.
    pub fn init_touch(&mut self) {
        // Reset sequence: LOW for 20ms, then HIGH
        self.touch_rst.set_low();
        self.delay.delay_millis(20);
        self.touch_rst.set_high();
        self.delay.delay_millis(50);
    }
    
    /// Check if touch interrupt is active (low = active)
    pub fn touch_interrupt_active(&self) -> bool {
        embedded_hal::digital::InputPin::is_low(&self.touch_int).unwrap_or(false)
    }
}

impl Default for WaveshareS3Board {
    fn default() -> Self {
        Self::new()
    }
}

/// Board configuration options
///
/// This structure allows customizing the board initialization.
#[derive(Debug, Clone)]
pub struct BoardConfig {
    /// Display SPI frequency (default: 40MHz)
    pub display_spi_freq: u32,
    /// I2C frequency (default: 100kHz)
    pub i2c_freq: u32,
    /// SD card SPI frequency (default: 20MHz)
    pub sdcard_spi_freq: u32,
    /// Enable backlight on startup
    pub backlight_on: bool,
}

impl Default for BoardConfig {
    fn default() -> Self {
        Self {
            display_spi_freq: 40_000_000,  // 40 MHz
            i2c_freq: 100_000,              // 100 kHz
            sdcard_spi_freq: 20_000_000,    // 20 MHz
            backlight_on: true,
        }
    }
}
