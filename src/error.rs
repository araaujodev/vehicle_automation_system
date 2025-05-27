//! Error handling module
//!
//! Defines the error types used throughout the system.

use std::fmt;
use std::error::Error;
use std::io;

/// Enum representing possible system errors
#[derive(Debug)]
pub enum SystemError {
    /// Error related to invalid commands
    InvalidCommand(String),
    /// Error related to vehicle operations
    VehicleOperation(String),
    /// Input/Output error
    IoError(io::Error),
}

impl fmt::Display for SystemError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            SystemError::InvalidCommand(msg) => write!(f, "Invalid command: {}", msg),
            SystemError::VehicleOperation(msg) => write!(f, "Vehicle operation error: {}", msg),
            SystemError::IoError(e) => write!(f, "I/O error: {}", e),
        }
    }
}

impl Error for SystemError {}

impl From<io::Error> for SystemError {
    fn from(error: io::Error) -> Self {
        SystemError::IoError(error)
    }
}