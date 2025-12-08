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

use esp_hal::gpio::{GpioPin, Input, Output, PullUp, PullDown, Floating};
use esp_hal::ledc::{Ledc, LowSpeed, LSGlobalClkSource, channel, timer};
use esp_hal::peripherals::LEDC;
use esp_hal::clock::Clocks;

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
    channel: channel::Channel<'d, LowSpeed, 0>,
    config: PwmConfig,
    max_duty: u32,
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
    pub fn new<P: esp_hal::gpio::OutputPin>(
        ledc: &'d mut Ledc<'d>,
        timer: &'d timer::Timer<'d, LowSpeed, 0>,
        pin: P,
        config: PwmConfig,
    ) -> Self {
        // Get max duty from timer resolution
        let max_duty = timer.get_max_duty();
        
        // Create channel and bind to pin
        let channel = ledc.get_channel(channel::Number::Channel0, pin);
        
        // Configure channel with timer
        channel.configure(channel::config::Config {
            timer,
            duty_pct: config.duty_percent,
            pin_config: channel::config::PinConfig::PushPull,
        }).ok();

        Self {
            channel,
            config,
            max_duty,
        }
    }

    /// Set the duty cycle percentage (0-100)
    ///
    /// 0% = off, 100% = full on
    pub fn set_duty_percent(&mut self, percent: u8) {
        let percent = percent.min(100);
        self.config.duty_percent = percent;
        
        let duty = (self.max_duty as u64 * percent as u64 / 100) as u32;
        self.channel.set_duty(duty);
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
    pub type InputFloating<'d, const PIN: u8> = Input<'d, GpioPin<PIN>>;

    /// Input mode with pull-up
    pub type InputPullUp<'d, const PIN: u8> = Input<'d, GpioPin<PIN>>;

    /// Input mode with pull-down
    pub type InputPullDown<'d, const PIN: u8> = Input<'d, GpioPin<PIN>>;

    /// Output mode (push-pull)
    pub type OutputPushPull<'d, const PIN: u8> = Output<'d, GpioPin<PIN>>;
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
    pub fn input_floating<'d, const PIN: u8>(
        pin: GpioPin<PIN>,
    ) -> Input<'d, GpioPin<PIN>> {
        Input::new(pin, esp_hal::gpio::Pull::None)
    }

    /// Configure a pin as input with pull-up
    pub fn input_pull_up<'d, const PIN: u8>(
        pin: GpioPin<PIN>,
    ) -> Input<'d, GpioPin<PIN>> {
        Input::new(pin, esp_hal::gpio::Pull::Up)
    }

    /// Configure a pin as input with pull-down
    pub fn input_pull_down<'d, const PIN: u8>(
        pin: GpioPin<PIN>,
    ) -> Input<'d, GpioPin<PIN>> {
        Input::new(pin, esp_hal::gpio::Pull::Down)
    }

    /// Configure a pin as output
    pub fn output<'d, const PIN: u8>(
        pin: GpioPin<PIN>,
        initial_level: esp_hal::gpio::Level,
    ) -> Output<'d, GpioPin<PIN>> {
        Output::new(pin, initial_level)
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

impl<'d, const PIN: u8> GpioExt for Output<'d, GpioPin<PIN>> {
    fn set_high(&mut self) {
        embedded_hal::digital::OutputPin::set_high(self).ok();
    }

    fn set_low(&mut self) {
        embedded_hal::digital::OutputPin::set_low(self).ok();
    }

    fn toggle(&mut self) {
        embedded_hal::digital::OutputPin::toggle(self).ok();
    }

    fn is_high(&self) -> bool {
        embedded_hal::digital::OutputPin::is_set_high(self).unwrap_or(false)
    }

    fn is_low(&self) -> bool {
        embedded_hal::digital::OutputPin::is_set_low(self).unwrap_or(false)
    }
}

impl<'d, const PIN: u8> GpioExt for Input<'d, GpioPin<PIN>> {
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
        embedded_hal::digital::InputPin::is_high(self).unwrap_or(false)
    }

    fn is_low(&self) -> bool {
        embedded_hal::digital::InputPin::is_low(self).unwrap_or(false)
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
