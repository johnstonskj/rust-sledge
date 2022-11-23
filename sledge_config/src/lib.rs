/*!
Configuration store for Sledge client and server.

More detailed description, with

# Example

YYYYY


*/

#![warn(
    unknown_lints,
    // ---------- Stylistic
    absolute_paths_not_starting_with_crate,
    elided_lifetimes_in_paths,
    explicit_outlives_requirements,
    macro_use_extern_crate,
    nonstandard_style, /* group */
    noop_method_call,
    rust_2018_idioms,
    single_use_lifetimes,
    trivial_casts,
    trivial_numeric_casts,
    // ---------- Future
    future_incompatible, /* group */
    rust_2021_compatibility, /* group */
    // ---------- Public
    missing_debug_implementations,
    // missing_docs,
    unreachable_pub,
    // ---------- Unsafe
    unsafe_code,
    unsafe_op_in_unsafe_fn,
    // ---------- Unused
    unused, /* group */
)]
#![deny(
    // ---------- Public
    exported_private_dependencies,
    private_in_public,
    // ---------- Deprecated
    anonymous_parameters,
    bare_trait_objects,
    ellipsis_inclusive_range_patterns,
    // ---------- Unsafe
    deref_nullptr,
    drop_bounds,
    dyn_drop,
)]

use crate::error::{
    config_dir_unknown_error, config_error, config_validation_error, var_error, Error,
};
use config::{Config as StoredConfig, ConfigError, Environment, File};
use lazy_static::lazy_static;
use semver::Version;
use serde::Deserialize;
use std::{env, env::VarError};
use tracing::{error, trace, trace_span};

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const CONFIG_FILE_VERSION: Version = Version::new(0, 1, 0);

#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    version: Version,
    #[serde(skip_serializing_if = "Option::is_none")]
    client: Option<ClientConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server: Option<ServerConfig>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn get_config() -> Result<&'static Configuration, &'static Error> {
    CONFIG.as_ref()
}

#[allow(dead_code)]
pub(crate) fn expand_config_string<S>(s: S) -> Option<String>
where
    S: AsRef<str>,
{
    match shellexpand::full(&s) {
        Ok(s) => Some(s.to_string()),
        Err(e) => {
            error!("Could not expand string {:?}; error: `{}`", s.as_ref(), e);
            None
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

lazy_static! {
    static ref CONFIG: Result<Configuration, Error> = load_config();
}

impl Configuration {
    pub fn schema_version(&self) -> &Version {
        &self.version
    }

    pub fn client(&self) -> Option<&ClientConfig> {
        self.client.as_ref()
    }

    pub fn server(&self) -> Option<&ServerConfig> {
        self.server.as_ref()
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.client
            .as_ref()
            .map(|c| c.is_valid())
            .unwrap_or_default()
            || self
                .server
                .as_ref()
                .map(|s| s.is_valid())
                .unwrap_or_default()
    }
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

fn load_config() -> Result<Configuration, Error> {
    let span = trace_span!("load_config");
    let _enter = span.enter();

    let root_str = match env::var("SLEDGE_CONFIG_ROOT") {
        Ok(root_str) => {
            trace!("Configuration using environment variable");
            root_str
        }
        Err(e @ VarError::NotUnicode(_)) => return Err(var_error(e)),
        Err(_) => match xdirs::config_dir_for("sledge") {
            Some(root_path) => {
                trace!("Configuration using XDG config directory");
                root_path.to_string_lossy().to_string()
            }
            None => return Err(config_dir_unknown_error()),
        },
    };

    match StoredConfig::builder()
        .add_source(File::with_name(&format!("{}/server", root_str)).required(false))
        .add_source(File::with_name(&format!("{}/client", root_str)).required(false))
        .add_source(Environment::with_prefix("SLEDGE"))
        .build()
    {
        Ok(config) => {
            let de_config: Result<Configuration, ConfigError> = config.try_deserialize();
            match de_config {
                Ok(de_config) => {
                    if de_config.is_valid() {
                        Ok(de_config)
                    } else {
                        Err(config_validation_error())
                    }
                }
                Err(e) => Err(config_error(e)),
            }
        }
        Err(e) => Err(config_error(e)),
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;

#[doc(hidden)]
mod client;
pub use client::{ClientConfig, ClientKind, RemoteConfig};

#[doc(hidden)]
mod server;
pub use server::{Binding, BindingOptions, ServerConfig, SslBinding};

#[doc(hidden)]
mod store;
pub use store::{FileSystemConfig, SqlConfig, StoreConfig};
