/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::StoreConfig;
use serde::Deserialize;
use std::str::FromStr;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Clone)]
pub struct ClientConfig {
    kind: ClientKind,
    locale: Option<String>,
}

#[derive(Debug, Deserialize, Clone)]
pub enum ClientKind {
    Local(StoreConfig),
    Remote(RemoteConfig),
}

#[derive(Debug, Deserialize, Clone)]
pub struct RemoteConfig {
    server_url: Url,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl From<ClientKind> for ClientConfig {
    fn from(kind: ClientKind) -> Self {
        Self {
            kind,
            locale: Default::default(),
        }
    }
}

impl ClientConfig {
    pub fn kind(&self) -> &ClientKind {
        &self.kind
    }

    pub fn locale(&self) -> Option<&String> {
        self.locale.as_ref()
    }

    pub(crate) fn is_valid(&self) -> bool {
        true
    }
}

// ------------------------------------------------------------------------------------------------

impl From<StoreConfig> for ClientKind {
    fn from(v: StoreConfig) -> Self {
        Self::Local(v)
    }
}

impl From<RemoteConfig> for ClientKind {
    fn from(v: RemoteConfig) -> Self {
        Self::Remote(v)
    }
}

impl ClientKind {
    pub fn is_local(&self) -> bool {
        matches!(self, Self::Local(_))
    }

    pub fn as_local(&self) -> Option<&StoreConfig> {
        match self {
            Self::Local(v) => Some(v),
            _ => None,
        }
    }

    pub fn is_remote(&self) -> bool {
        matches!(self, Self::Remote(_))
    }

    pub fn as_remote(&self) -> Option<&RemoteConfig> {
        match self {
            Self::Remote(v) => Some(v),
            _ => None,
        }
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for RemoteConfig {
    fn default() -> Self {
        Self {
            server_url: Url::from_str("http://localhost:8080").unwrap(),
        }
    }
}

impl From<Url> for RemoteConfig {
    fn from(v: Url) -> Self {
        Self { server_url: v }
    }
}

impl RemoteConfig {
    pub fn server_url(&self) -> &Url {
        &self.server_url
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
