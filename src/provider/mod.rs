use library::GlobalLibrary;
use std::error;
use reqwest;
use std::fmt;

pub trait Provider {
    fn sync(&mut self, library: GlobalLibrary) -> Result<(), SyncError>;
}

#[derive(Debug)]
pub enum SyncError {
    ConfigurationError,
    HttpError(reqwest::Error)
}

impl fmt::Display for SyncError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            SyncError::ConfigurationError => write!(f, "Configuration Error"),
            SyncError::HttpError(ref err) => write!(f, "HTTP Error: {}", err)
        }
    }
}

impl error::Error for SyncError {
    fn description(&self) -> &str {
        match *self {
            SyncError::ConfigurationError => "invalid provider configuration",
            SyncError::HttpError(ref err) => err.description()
        }
    }

    fn cause(&self) -> Option<&error::Error> {
        match *self {
            SyncError::HttpError(ref error) => Some(error),
            _ => None
        }
    }
}