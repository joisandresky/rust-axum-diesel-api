use std::error::Error;
use std::fmt;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ServiceError {
    pub message: String,
    pub success: bool,
    pub status: u16,
}

impl fmt::Display for ServiceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Implement the Display trait for your custom_error
        write!(f, "UserError: {}", self.message)
    }
}

impl Error for ServiceError {}