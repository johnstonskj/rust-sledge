/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::{FileSystemConfig, StoreConfig};
use serde::Deserialize;
use std::collections::HashSet;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct Binding {
    host: String,
    port: u16,
    ssl: Option<SslBinding>,
    options: Option<BindingOptions>,
}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct SslBinding {}

#[derive(Debug, Deserialize, Clone, PartialEq, Eq, Hash)]
pub struct BindingOptions {}

#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    bindings: HashSet<Binding>,
    store: StoreConfig,
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

impl Default for Binding {
    fn default() -> Self {
        Self {
            host: Default::default(),
            port: Default::default(),
            ssl: Default::default(),
            options: Default::default(),
        }
    }
}

impl Binding {
    pub fn host(&self) -> &String {
        &self.host
    }

    pub fn port(&self) -> u16 {
        self.port
    }

    pub fn ssl_binding(&self) -> Option<&SslBinding> {
        self.ssl.as_ref()
    }

    pub fn binding_options(&self) -> Option<&BindingOptions> {
        self.options.as_ref()
    }

    pub(crate) fn is_valid(&self) -> bool {
        !self.host.is_empty() && self.port > 0
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for SslBinding {
    fn default() -> Self {
        Self {}
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for BindingOptions {
    fn default() -> Self {
        Self {}
    }
}

// ------------------------------------------------------------------------------------------------

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bindings: Default::default(),
            store: FileSystemConfig::default().into(),
        }
    }
}

impl ServerConfig {
    pub fn bindings(&self) -> impl Iterator<Item = &Binding> {
        self.bindings.iter()
    }

    pub fn store(&self) -> &StoreConfig {
        &self.store
    }

    pub(crate) fn is_valid(&self) -> bool {
        !self.bindings.is_empty() && self.bindings.iter().all(Binding::is_valid)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
