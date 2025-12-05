//! Common traits for bus modes

use crate::Error;
use heapless::Vec;

/// Common interface for all bus modes
pub trait BusMode {
    /// Configuration type for this mode
    type Config;
    
    /// Get the name of this bus mode
    fn name(&self) -> &'static str;
    
    /// Initialize the bus mode with given configuration
    fn init(&mut self, config: Self::Config) -> Result<(), Error>;
    
    /// Deinitialize and release resources
    fn deinit(&mut self) -> Result<(), Error>;
}

/// Trait for bus modes that support device scanning
pub trait Scanner {
    /// Type representing a device identifier
    type DeviceId;
    
    /// Scan the bus for connected devices
    fn scan(&mut self) -> Result<Vec<Self::DeviceId, 128>, Error>;
}

/// Trait for bus modes that support traffic sniffing
pub trait Sniffer {
    /// Type representing a captured event
    type Event;
    
    /// Start sniffing bus traffic
    fn start_sniff(&mut self) -> Result<(), Error>;
    
    /// Stop sniffing bus traffic
    fn stop_sniff(&mut self) -> Result<(), Error>;
    
    /// Read a captured event (non-blocking)
    fn read_event(&mut self) -> Result<Option<Self::Event>, Error>;
}
