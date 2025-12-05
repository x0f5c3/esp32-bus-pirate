//! UART bus mode implementation

use crate::{traits::BusMode, Error};

/// UART bus mode
pub struct UartMode<U> {
    uart: U,
    config: Option<UartConfig>,
}

/// UART configuration
#[derive(Debug, Clone, Copy)]
pub struct UartConfig {
    pub baudrate: u32,
}

impl<U> UartMode<U> {
    /// Create a new UART mode instance
    pub fn new(uart: U) -> Self {
        Self { uart, config: None }
    }
}

impl<U> BusMode for UartMode<U> {
    type Config = UartConfig;
    
    fn name(&self) -> &'static str {
        "UART"
    }
    
    fn init(&mut self, config: Self::Config) -> Result<(), Error> {
        self.config = Some(config);
        Ok(())
    }
    
    fn deinit(&mut self) -> Result<(), Error> {
        self.config = None;
        Ok(())
    }
}

// Note: Full UART implementation requires embedded-io traits
// which we'll add when we implement the firmware
