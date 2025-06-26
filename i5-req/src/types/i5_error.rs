use std::{error::Error, fmt::Display};

/// An error type representing possible failures when handling Interface5 requests.
///
/// This enum encapsulates typical error scenarios that can occur when:
/// - Validating request data
/// - Serializing the request body to a JSON String
/// - Sending the HTTP request to the target interface
///
/// # Variants
///
/// - [`ValidationError`]: The i5Request failed validation checks.
/// - [`SerializeError`]: JSON serialization failed (typically from `serde_json::to_string`).
/// - [`RequestError`]: Sending the HTTP request via `reqwest` failed.
///
#[derive(Debug)]
pub enum I5RequestError {
    /// The i5Request Object validation.
    ValidationError,

    /// Serialization of the request i5Request Struct into JSON failed.
    ///
    /// Contains the original [`serde_json::Error`].
    SerializeError(serde_json::Error),

    /// Sending the HTTP request failed.
    ///
    /// Contains the original [`reqwest::Error`].
    RequestError(reqwest::Error),
}

impl Display for I5RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidationError => write!(f, "I5Request not valid!"),
            Self::SerializeError(err) => write!(f, "Faild to convert Object to String: {}", err),
            Self::RequestError(err) => {
                write!(f, "Failed posting Body to Interface5: {}", err)
            }
        }
    }
}

impl Error for I5RequestError {}
