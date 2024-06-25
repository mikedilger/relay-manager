use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    Http(http::Error),
    Join(tokio::task::JoinError),
    Json(serde_json::Error),
    Reqwest(reqwest::Error),
    Timeout(tokio::time::error::Elapsed),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Error::Http(e) => write!(f, "Http: {e}"),
            Error::Join(e) => write!(f, "Tokio join: {e}"),
            Error::Json(e) => write!(f, "JSON: {e}"),
            Error::Reqwest(e) => write!(f, "Http: {e}"),
            Error::Timeout(e) => write!(f, "Timeout: {e}"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            Error::Http(inner) => Some(inner),
            Error::Join(inner) => Some(inner),
            Error::Json(inner) => Some(inner),
            Error::Reqwest(inner) => Some(inner),
            Error::Timeout(inner) => Some(inner),
            //_ => None,
        }
    }
}

impl From<http::Error> for Error {
    fn from(e: http::Error) -> Error {
        Error::Http(e)
    }
}

impl From<tokio::task::JoinError> for Error {
    fn from(e: tokio::task::JoinError) -> Error {
        Error::Join(e)
    }
}

impl From<serde_json::Error> for Error {
    fn from(e: serde_json::Error) -> Error {
        Error::Json(e)
    }
}

impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Error {
        Error::Reqwest(e)
    }
}

impl From<tokio::time::error::Elapsed> for Error {
    fn from(e: tokio::time::error::Elapsed) -> Error {
        Error::Timeout(e)
    }
}
