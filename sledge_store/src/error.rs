/*!
Provides the crate's Error and Result types as well as helper
functions.

 */

use serde_json::Error as JsonError;
use std::fmt::{Debug, Display};
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The Error type for this crate.
///
#[derive(Debug)]
pub enum Error {
    /// An error was signaled by the standard library I/O functions.
    IoError {
        source: std::io::Error,
    },
    UnknownStoreSchema {
        uri: Url,
    },
    StoreExists {
        uri: Url,
    },
    StoreDoesNotExist {
        uri: Url,
    },
    SettingsFileError {
        source: JsonError,
    },
}

///
/// A Result type that specifically uses this crate's Error.
///
pub type Result<T> = std::result::Result<Error, T>;

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

/// Construct an Error from the provided source.
#[inline]
pub fn io_error(source: std::io::Error) -> Error {
    Error::IoError { source }
}

/// Construct an Error from the provided source.
#[inline]
pub fn settings_file_error(source: JsonError) -> Error {
    Error::SettingsFileError { source }
}

/// Construct an Error from the provided path.
#[inline]
pub fn unknown_store_scheme(uri: Url) -> Error {
    Error::StoreExists { uri }
}

/// Construct an Error from the provided path.
#[inline]
pub fn store_exists(uri: Url) -> Error {
    Error::StoreExists { uri }
}

/// Construct an Error from the provided path.
#[inline]
pub fn store_does_not_exist(uri: Url) -> Error {
    Error::StoreDoesNotExist { uri }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Error::IoError { source } => format!("An I/O error occurred; source: {}", source),
                Error::UnknownStoreSchema { uri } => format!(
                    "The scheme {:?} is not supported (from connection URI <{}>)",
                    uri.scheme(),
                    uri
                ),
                Error::StoreExists { uri } => format!("A data store already exists at <{:?}>", uri),
                Error::StoreDoesNotExist { uri } =>
                    format!("A data store was not found at <{:?}>", uri),
                Error::SettingsFileError { source } => format!(
                    "Could not read or write the settings file; error: {:?}",
                    source
                ),
            }
        )
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::IoError { source } => Some(source),
            Error::SettingsFileError { source } => Some(source),
            _ => None,
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        io_error(source)
    }
}

impl From<JsonError> for Error {
    fn from(source: JsonError) -> Self {
        settings_file_error(source)
    }
}
