//! Protocol version management

/// Current protocol version
pub const PROTOCOL_VERSION: u8 = 0x01;

/// Check if a version is compatible with the current version
pub fn is_compatible(version: u8) -> bool {
    // For now, only exact version match is supported
    // Future versions can implement backward compatibility
    version == PROTOCOL_VERSION
}
