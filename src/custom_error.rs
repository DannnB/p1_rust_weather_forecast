/// `custom_error.rs` defines a custom error type for the application.
///
/// ## Enum
///
/// - `CustomError`: An enumeration of possible error types that can occur in the application. 
///   It includes variants for `reqwest::Error` and `serde_json::Error`.
///
/// ## Implementations
///
/// - `fmt::Display for CustomError`: Provides a human-readable representation of errors.
/// - `Error for CustomError`: Implements the standard error trait for `CustomError`.
/// - `From<reqwest::Error> for CustomError`: Conversion from `reqwest::Error` to `CustomError`.
/// - `From<serde_json::Error> for CustomError`: Conversion from `serde_json::Error` to `CustomError`.
///
/// ## Dependencies
///
/// This file requires the `reqwest` and `serde_json` crates to handle errors related to HTTP requests
/// and JSON parsing, respectively, along with standard error handling traits from `std`.

use std::{error::Error, fmt};
use reqwest; // Make sure reqwest is included in your Cargo.toml
use serde_json; // Make sure serde_json is included in your Cargo.toml

#[derive(Debug)]
pub enum CustomError {  // Made public with `pub`
    Reqwest(reqwest::Error),
    SerdeJson(serde_json::Error),
}

impl fmt::Display for CustomError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            CustomError::Reqwest(ref err) => write!(f, "Reqwest error: {}", err),
            CustomError::SerdeJson(ref err) => write!(f, "Serde JSON error: {}", err),
        }
    }
}

impl Error for CustomError {}

impl From<reqwest::Error> for CustomError {
    fn from(err: reqwest::Error) -> CustomError {
        CustomError::Reqwest(err)
    }
}

impl From<serde_json::Error> for CustomError {
    fn from(err: serde_json::Error) -> CustomError {
        CustomError::SerdeJson(err)
    }
}