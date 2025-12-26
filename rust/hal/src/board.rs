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
    delay::Delay,
    gpio::{Input, Level, Output, OutputConfig, Pull},
    i2c::master::{Config as EspI2cConfig, I2c},
    otg_fs::Usb,
    spi::master::{Config as EspSpiConfig, Spi},
    Blocking,
};
use esp_hal::time::Rate;

/// Main board structure with initialized peripherals
///
/// This structure contains all the peripherals and pins needed to interact
/// with the Waveshare ESP32-S3-Touch-LCD-2.8 board hardware.
pub struct WaveshareS3Board {
    /// Delay provider for precise timing
    pub delay: Delay,
    /// SPI bus for display (and possibly SD card)
    pub display_spi: Spi<'static, Blocking>,
    
    /// Display control pins
    pub display_dc: Output<'static>,   // Data/Command select
    pub display_cs: Output<'static>,   // Chip select (active low)
    pub display_rst: Output<'static>,  // Reset (active low)
    pub display_bl: Output<'static>,   // Backlight control
    
    /// I2C bus for touch, IMU, and RTC
    pub i2c0: I2c<'static, Blocking>,
    
    /// Touch controller pins
    pub touch_int: Input<'static>,     // Interrupt pin (active low)
    pub touch_rst: Output<'static>,    // Reset (active low)
    
    /// SPI3 bus for SD card
    pub sdcard_spi: Spi<'static, Blocking>,
    pub sdcard_cs: Output<'static>,    // SD card chip select

    /// USB OTG peripheral
    pub usb: Usb<'static>,
    
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
        let peripherals = esp_hal::init(esp_hal::Config::default());

        let delay = Delay::new();

        // ===== Display SPI Setup =====
        let display_cs = Output::new(peripherals.GPIO42, Level::High, OutputConfig::default());
        let display_cfg = EspSpiConfig::default()
            .with_frequency(Rate::from_mhz(40))
            .with_mode(esp_hal::spi::Mode::_0);
        let display_spi = Spi::new(peripherals.SPI2, display_cfg)
            .expect("SPI initialization failed")
            .with_sck(peripherals.GPIO40)
            .with_mosi(peripherals.GPIO45)
            .with_miso(peripherals.GPIO48);

        let display_dc = Output::new(peripherals.GPIO41, Level::Low, OutputConfig::default());
        let display_rst = Output::new(peripherals.GPIO39, Level::High, OutputConfig::default());
        let display_bl = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default()); // Backlight on

        // ===== I2C Setup for Touch/IMU/RTC =====
        let i2c_config = EspI2cConfig::default().with_frequency(Rate::from_khz(100));
        let i2c0 = I2c::new(peripherals.I2C0, i2c_config)
            .expect("I2C initialization failed")
            .with_sda(peripherals.GPIO1)
            .with_scl(peripherals.GPIO3);

        let touch_int = Input::new(
            peripherals.GPIO4,
            esp_hal::gpio::InputConfig::default().with_pull(Pull::Up),
        );
        let touch_rst = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());

        // ===== SD Card SPI3 Setup @ 20MHz =====
        let sd_cfg = EspSpiConfig::default()
            .with_frequency(Rate::from_mhz(20))
            .with_mode(esp_hal::spi::Mode::_0);
        let sdcard_spi = Spi::new(peripherals.SPI3, sd_cfg)
            .expect("SPI3 initialization failed")
            .with_sck(peripherals.GPIO14)
            .with_mosi(peripherals.GPIO17)
            .with_miso(peripherals.GPIO16);
        let sdcard_cs = Output::new(peripherals.GPIO21, Level::High, OutputConfig::default());

        // ===== USB OTG =====
        let usb = Usb::new(peripherals.USB0, peripherals.GPIO20, peripherals.GPIO19);

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
            usb,
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
        self.touch_int.is_low()
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
