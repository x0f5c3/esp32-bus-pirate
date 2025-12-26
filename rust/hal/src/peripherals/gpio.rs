//! GPIO utilities for ESP32-S3
//!
//! This module provides utilities for GPIO operations including:
//! - PWM for backlight control
//! - Interrupt support for touch controller
//! - Pull-up/pull-down configuration
//!
//! # Example
//!
//! ```no_run
//! use esp32_bus_pirate_hal::peripherals::gpio::PwmConfig;
//!
//! let config = PwmConfig::default()
//!     .with_frequency(1000) // 1kHz
//!     .with_duty_percent(50); // 50% duty cycle
//! ```

use esp_hal::gpio::{Input, InputConfig, Output, OutputConfig, Pull};
use esp_hal::ledc::{channel, LowSpeed};
use esp_hal::ledc::channel::ChannelIFace;

/// PWM configuration
#[derive(Debug, Clone, Copy)]
pub struct PwmConfig {
    /// PWM frequency in Hz
    pub frequency: u32,
    /// Duty cycle as percentage (0-100)
    pub duty_percent: u8,
}

impl Default for PwmConfig {
    fn default() -> Self {
        Self {
            frequency: 5000, // 5kHz default
            duty_percent: 100, // Full brightness
        }
    }
}

impl PwmConfig {
    /// Create a new PWM configuration
    pub fn new(frequency: u32, duty_percent: u8) -> Self {
        Self {
            frequency,
            duty_percent: duty_percent.min(100),
        }
    }

    /// Set the PWM frequency
    pub fn with_frequency(mut self, frequency: u32) -> Self {
        self.frequency = frequency;
        self
    }

    /// Set the duty cycle percentage (0-100)
    pub fn with_duty_percent(mut self, duty_percent: u8) -> Self {
        self.duty_percent = duty_percent.min(100);
        self
    }
}

/// PWM channel wrapper
///
/// This wrapper provides a simple interface for PWM control,
/// primarily used for backlight brightness control.
pub struct PwmChannel<'d> {
    channel: channel::Channel<'d, LowSpeed>,
    config: PwmConfig,
}

impl<'d> PwmChannel<'d> {
    /// Create a new PWM channel
    ///
    /// # Arguments
    ///
    /// * `ledc` - The LEDC peripheral
    /// * `timer` - The LEDC timer to use for this channel
    /// * `pin` - The GPIO pin to use for PWM output
    /// * `config` - PWM configuration
    pub fn new(
        channel: channel::Channel<'d, LowSpeed>,
        config: PwmConfig,
    ) -> Self {
        let _ = channel.set_duty(config.duty_percent);
        Self { channel, config }
    }

    /// Set the duty cycle percentage (0-100)
    ///
    /// 0% = off, 100% = full on
    pub fn set_duty_percent(&mut self, percent: u8) {
        let percent = percent.min(100);
        self.config.duty_percent = percent;
        let _ = self.channel.set_duty(percent);
    }

    /// Get the current duty cycle percentage
    pub fn duty_percent(&self) -> u8 {
        self.config.duty_percent
    }

    /// Turn the PWM output on (100% duty)
    pub fn on(&mut self) {
        self.set_duty_percent(100);
    }

    /// Turn the PWM output off (0% duty)
    pub fn off(&mut self) {
        self.set_duty_percent(0);
    }

    /// Get the current configuration
    pub fn config(&self) -> &PwmConfig {
        &self.config
    }
}

/// GPIO pin modes
pub mod pin_mode {
    use super::*;

    /// Input mode with floating (no pull-up/pull-down)
    pub type InputFloating<'d> = Input<'d>;

    /// Input mode with pull-up
    pub type InputPullUp<'d> = Input<'d>;

    /// Input mode with pull-down
    pub type InputPullDown<'d> = Input<'d>;

    /// Output mode (push-pull)
    pub type OutputPushPull<'d> = Output<'d>;
}

/// GPIO interrupt configuration
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InterruptMode {
    /// Trigger on rising edge
    RisingEdge,
    /// Trigger on falling edge
    FallingEdge,
    /// Trigger on both edges
    AnyEdge,
    /// Trigger on low level
    LowLevel,
    /// Trigger on high level
    HighLevel,
}

/// GPIO utilities
pub struct GpioUtils;

impl GpioUtils {
    /// Configure a pin as input with floating mode
    pub fn input_floating<'d>(
        pin: impl esp_hal::gpio::InputPin + 'd,
    ) -> Input<'d> {
        Input::new(pin, InputConfig::default().with_pull(Pull::None))
    }

    /// Configure a pin as input with pull-up
    pub fn input_pull_up<'d>(
        pin: impl esp_hal::gpio::InputPin + 'd,
    ) -> Input<'d> {
        Input::new(pin, InputConfig::default().with_pull(Pull::Up))
    }

    /// Configure a pin as input with pull-down
    pub fn input_pull_down<'d>(
        pin: impl esp_hal::gpio::InputPin + 'd,
    ) -> Input<'d> {
        Input::new(pin, InputConfig::default().with_pull(Pull::Down))
    }

    /// Configure a pin as output
    pub fn output<'d>(
        pin: impl esp_hal::gpio::OutputPin + 'd,
        initial_level: esp_hal::gpio::Level,
    ) -> Output<'d> {
        Output::new(pin, initial_level, OutputConfig::default())
    }
}

/// Extension trait for convenient GPIO operations
pub trait GpioExt {
    /// Set the pin high
    fn set_high(&mut self);

    /// Set the pin low
    fn set_low(&mut self);

    /// Toggle the pin state
    fn toggle(&mut self);

    /// Get the current pin state
    fn is_high(&self) -> bool;

    /// Get the current pin state (inverted)
    fn is_low(&self) -> bool;
}

impl<'d> GpioExt for Output<'d> {
    fn set_high(&mut self) {
        let _ = Output::set_high(self);
    }

    fn set_low(&mut self) {
        let _ = Output::set_low(self);
    }

    fn toggle(&mut self) {
        let _ = Output::toggle(self);
    }

    fn is_high(&self) -> bool {
        Output::is_set_high(self)
    }

    fn is_low(&self) -> bool {
        Output::is_set_low(self)
    }
}

impl<'d> GpioExt for Input<'d> {
    fn set_high(&mut self) {
        // Input pins cannot be set
    }

    fn set_low(&mut self) {
        // Input pins cannot be set
    }

    fn toggle(&mut self) {
        // Input pins cannot be toggled
    }

    fn is_high(&self) -> bool {
        Input::is_high(self)
    }

    fn is_low(&self) -> bool {
        Input::is_low(self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pwm_config_default() {
        let config = PwmConfig::default();
        assert_eq!(config.frequency, 5000);
        assert_eq!(config.duty_percent, 100);
    }

    #[test]
    fn test_pwm_config_builder() {
        let config = PwmConfig::default()
            .with_frequency(1000)
            .with_duty_percent(50);
        assert_eq!(config.frequency, 1000);
        assert_eq!(config.duty_percent, 50);
    }

    #[test]
    fn test_pwm_config_duty_clamping() {
        let config = PwmConfig::new(1000, 150);
        assert_eq!(config.duty_percent, 100);
    }
}
