use std::error::Error as StdError;
use std::panic::Location;

#[derive(Debug)]
pub struct Error {
    pub inner: InnerError,
    location: &'static Location<'static>,
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        Some(&self.inner)
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}, {}", self.inner, self.location)
    }
}

#[derive(Debug)]
pub enum InnerError {
    FromUtf8(std::string::FromUtf8Error),
    Http(http::Error),
    InvalidUri(http::uri::InvalidUri),
    InvalidUriParts(http::uri::InvalidUriParts),
    Json(serde_json::Error),
    MissingScheme,
    NostrTypes(nostr_types::Error),
    Reqwest(reqwest::Error),
    ServerError(u16, String),
    Timeout(tokio::time::error::Elapsed),
    UnrecognizedResponse(serde_json::Error),
}

impl std::fmt::Display for InnerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        match self {
            InnerError::FromUtf8(e) => write!(f, "From UTF-8: {e}"),
            InnerError::Http(e) => write!(f, "Http: {e}"),
            InnerError::InvalidUri(e) => write!(f, "Http: {e}"),
            InnerError::InvalidUriParts(e) => write!(f, "Http: {e}"),
            InnerError::Json(e) => write!(f, "JSON: {e}"),
            InnerError::MissingScheme => write!(f, "Missing scheme"),
            InnerError::NostrTypes(e) => write!(f, "Nostr types: {e}"),
            InnerError::Reqwest(e) => write!(f, "Http: {e}"),
            InnerError::ServerError(st, s) => {
                write!(f, "Server reports error status={}: {}", st, s)
            }
            InnerError::Timeout(e) => write!(f, "Timeout: {e}"),
            InnerError::UnrecognizedResponse(e) => write!(f, "Unrecognized response: {e}"),
        }
    }
}

impl StdError for InnerError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            InnerError::FromUtf8(inner) => Some(inner),
            InnerError::Http(inner) => Some(inner),
            InnerError::InvalidUri(inner) => Some(inner),
            InnerError::InvalidUriParts(inner) => Some(inner),
            InnerError::Json(inner) => Some(inner),
            InnerError::NostrTypes(inner) => Some(inner),
            InnerError::Reqwest(inner) => Some(inner),
            InnerError::Timeout(inner) => Some(inner),
            InnerError::UnrecognizedResponse(inner) => Some(inner),
            _ => None,
        }
    }
}

#[allow(clippy::from_over_into)]
impl Into<Error> for InnerError {
    #[track_caller]
    fn into(self) -> Error {
        Error {
            inner: self,
            location: std::panic::Location::caller(),
        }
    }
}

impl From<std::string::FromUtf8Error> for Error {
    #[track_caller]
    fn from(e: std::string::FromUtf8Error) -> Error {
        Error {
            inner: InnerError::FromUtf8(e),
            location: std::panic::Location::caller(),
        }
    }
}

impl From<http::Error> for Error {
    #[track_caller]
    fn from(e: http::Error) -> Error {
        Error {
            inner: InnerError::Http(e),
            location: std::panic::Location::caller(),
        }
    }
}

impl From<http::uri::InvalidUri> for Error {
    #[track_caller]
    fn from(e: http::uri::InvalidUri) -> Error {
        Error {
            inner: InnerError::InvalidUri(e),
            location: std::panic::Location::caller(),
        }
    }
}

impl From<http::uri::InvalidUriParts> for Error {
    #[track_caller]
    fn from(e: http::uri::InvalidUriParts) -> Error {
        Error {
            inner: InnerError::InvalidUriParts(e),
            location: std::panic::Location::caller(),
        }
    }
}

impl From<nostr_types::Error> for Error {
    #[track_caller]
    fn from(e: nostr_types::Error) -> Error {
        Error {
            inner: InnerError::NostrTypes(e),
            location: std::panic::Location::caller(),
        }
    }
}

impl From<serde_json::Error> for Error {
    #[track_caller]
    fn from(e: serde_json::Error) -> Error {
        Error {
            inner: InnerError::Json(e),
            location: std::panic::Location::caller(),
        }
    }
}

impl From<reqwest::Error> for Error {
    #[track_caller]
    fn from(e: reqwest::Error) -> Error {
        Error {
            inner: InnerError::Reqwest(e),
            location: std::panic::Location::caller(),
        }
    }
}

impl From<tokio::time::error::Elapsed> for Error {
    #[track_caller]
    fn from(e: tokio::time::error::Elapsed) -> Error {
        Error {
            inner: InnerError::Timeout(e),
            location: std::panic::Location::caller(),
        }
    }
}
