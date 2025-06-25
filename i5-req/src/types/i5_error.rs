use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum I5RequestError {
    ValidationError,
    SerializeError(serde_json::Error),
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
