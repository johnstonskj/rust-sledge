/*!
Provides the crate's Error and Result types as well as helper
functions.

 */

use std::fmt::{Debug, Display};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// The Error type for this crate.
///
#[derive(Debug)]
pub enum Error {
    ConfigDirUnknown,
    ConfigValidationFailed,
    /// An error was signaled by the standard library environment functions.
    VarError {
        source: std::env::VarError,
    },
    /// An error was signaled by the `config` package functions.
    ConfigError {
        source: config::ConfigError,
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
pub fn var_error(source: std::env::VarError) -> Error {
    Error::VarError { source }
}

/// Construct an Error from the provided source.
#[inline]
pub fn config_error(source: config::ConfigError) -> Error {
    Error::ConfigError { source }
}

/// Construct an Error.
#[inline]
pub fn config_dir_unknown_error() -> Error {
    Error::ConfigDirUnknown
}

/// Construct an Error.
#[inline]
pub fn config_validation_error() -> Error {
    Error::ConfigDirUnknown
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
                Self::VarError { source } =>
                    format!("An environment error occurred; source: {}", source),
                Self::ConfigError { source } => format!(
                    "An error occurred in the `config` package; source: {}",
                    source
                ),
                Self::ConfigDirUnknown => "Configuration directory unknown".to_string(),
                Self::ConfigValidationFailed =>
                    "The parsed configuration failed validation".to_string(),
            }
        )
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::VarError { source } => Some(source),
            Self::ConfigError { source } => Some(source),
            _ => None,
        }
    }
}

impl From<std::env::VarError> for Error {
    fn from(source: std::env::VarError) -> Self {
        var_error(source)
    }
}

impl From<config::ConfigError> for Error {
    fn from(source: config::ConfigError) -> Self {
        config_error(source)
    }
}
