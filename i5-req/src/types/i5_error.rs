use std::{error::Error, fmt::Display};

#[derive(Debug)]
pub enum I5RequestError {
    ValidationError,
    SerializeError,
    RequestError,
}

impl Display for I5RequestError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ValidationError => write!(f, "I5Request not valid!"),
            Self::SerializeError => write!(f, "Faild to convert Object to String"),
            Self::RequestError => write!(f, "Failed posting Body to Interface5"),
        }
    }
}

impl Error for I5RequestError {}
