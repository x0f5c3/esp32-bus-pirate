//! Board initialization for Waveshare ESP32-S3-Touch-LCD-2.8

use esp_hal::{
    delay::Delay,
    gpio::{Level, Output, OutputConfig},
    i2c::master::{Config as I2cConfig, I2c},
    peripherals::Peripherals,
    spi::master::{Config as SpiConfig, Spi},
    Blocking,
};
use esp_hal::time::Rate;

use crate::pins;

/// Main board structure with initialized peripherals
pub struct WaveshareS3Board {
    /// Delay provider
    pub delay: Delay,
    /// SPI bus for display (and possibly SD card)
    pub display_spi: Spi<'static, Blocking>,
    
    /// Display control pins
    pub display_dc: Output<'static>,
    pub display_cs: Output<'static>,
    pub display_rst: Output<'static>,
    pub display_bl: Output<'static>,
    
    /// I2C bus for touch, IMU, and RTC
    pub i2c0: I2c<'static, Blocking>,
    
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
        // Initialize with default configuration
        let peripherals = esp_hal::init(esp_hal::Config::default());
        
        // Create delay provider
        let delay = Delay::new();
        
        // ===== Display SPI Setup =====
        let sclk = peripherals.GPIO40;
        let mosi = peripherals.GPIO45;
        let miso = peripherals.GPIO48; // Not used for display, but SPI needs it
        let display_cs = Output::new(peripherals.GPIO42, Level::High, OutputConfig::default());
        
        let spi_config = SpiConfig::default().with_frequency(Rate::from_mhz(40));
        let display_spi = Spi::new(peripherals.SPI2, spi_config)
            .expect("SPI initialization failed")
            .with_sck(sclk)
            .with_mosi(mosi)
            .with_miso(miso);
        
        // Display control pins
        let display_dc = Output::new(peripherals.GPIO41, Level::Low, OutputConfig::default());
        let display_rst = Output::new(peripherals.GPIO39, Level::High, OutputConfig::default());
        let display_bl = Output::new(peripherals.GPIO5, Level::High, OutputConfig::default()); // Backlight on
        
        // ===== I2C Setup for Touch/IMU/RTC =====
        let sda = peripherals.GPIO1;
        let scl = peripherals.GPIO3;
        
        let i2c_config = I2cConfig::default().with_frequency(Rate::from_khz(100));
        let i2c0 = I2c::new(peripherals.I2C0, i2c_config)
            .expect("I2C initialization failed")
            .with_sda(sda)
            .with_scl(scl);
        
        // Touch controller pins
        let touch_int = Output::new(peripherals.GPIO4, Level::High, OutputConfig::default());
        let touch_rst = Output::new(peripherals.GPIO2, Level::High, OutputConfig::default());
        
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
