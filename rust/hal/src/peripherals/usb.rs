//! USB OTG wrapper for ESP32-S3
//!
//! Provides a thin, `no_std` safe wrapper around the ESP32-S3 USB FS peripheral.
//! This exposes the `Usb` type that can be used with the `usb-device` stack to
//! build a CDC-ACM transport (task 3 requirement).
//!
//! The Waveshare board routes the native USB pins as:
//! - D+: GPIO20
//! - D-: GPIO19
//!
//! # Example
//!
//! ```no_run
//! use esp_hal::otg_fs::UsbBus;
//! use esp32_bus_pirate_hal::peripherals::usb::UsbDriver;
//!
//! let usb = UsbDriver::new(peripherals.USB0, peripherals.GPIO20, peripherals.GPIO19);
//! let bus = UsbBus::new(usb.inner());
//! ```

use esp_hal::otg_fs::Usb as HalUsb;

/// Convenient wrapper around the ESP HAL USB peripheral.
pub struct UsbDriver<'d> {
    usb: HalUsb<'d>,
}

impl<'d> UsbDriver<'d> {
    /// Create a new USB driver using the native OTG FS peripheral.
    pub fn new(
        usb0: esp_hal::peripherals::USB0<'d>,
        dp: impl esp_hal::otg_fs::UsbDp + 'd,
        dm: impl esp_hal::otg_fs::UsbDm + 'd,
    ) -> Self {
        Self {
            usb: HalUsb::new(usb0, dp, dm),
        }
    }

    /// Access the underlying HAL USB handle.
    pub fn inner(self) -> HalUsb<'d> {
        self.usb
    }
}
