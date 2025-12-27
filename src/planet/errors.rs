//! Error types for Luna4
//!
//! This module defines the error types used throughout the Luna4 implementation.
//! All errors are defined as variants of the `Luna4Error` enum for consistency
//! and easy error handling.

use thiserror::Error;

/// Luna4-specific errors
///
/// This enum represents all possible error conditions that can occur
/// during Luna4 planet creation, operation, or resource management.
/// Each variant includes a descriptive error message.
#[derive(Error, Debug)]
pub(crate) enum Luna4Error {
    /// Failed to create the planet wrapper
    #[error("Failed to create planet: {0}")]
    PlanetCreation(String),
    
    /// Invalid energy cell configuration
    #[error("Invalid energy configuration: {0}")]
    EnergyError(String),
    
    /// Resource generation error
    #[error("Resource generation failed: {0}")]
    ResourceError(String),
    
    /// Phase timing error
    #[error("Lunar phase timing error: {0}")]
    PhaseError(String),
    
    /// General operational error
    #[error("Operational error: {0}")]
    OperationalError(String),
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_display() {
        let error = Luna4Error::EnergyError("test error".to_string());
        assert_eq!(format!("{}", error), "Invalid energy configuration: test error");
        
        let error = Luna4Error::PlanetCreation("creation failed".to_string());
        assert_eq!(format!("{}", error), "Failed to create planet: creation failed");
    }
    
    #[test]
    fn test_error_debug() {
        let error = Luna4Error::ResourceError("cannot generate".to_string());
        // Should not panic when formatting for debug
        let _ = format!("{:?}", error);
    }
}