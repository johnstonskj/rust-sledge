/*!
Configuration store for Sledge client and server.

The configuration data is stored in JSON, TOML, or YAML files; these files are in a
folder named `sledge` identified either by the standard
`XDG_CONFIG_HOME` environment variable or by the app-specific override
`SLEDGE_CONFIG_ROOT` environment variable. The app looks for files named
`config.{ext}?`, `server.{ext}`, and `client.{ext}` and if more than one is
present the data in a later file overrides earlier loaded values.

# Example

```rust
# std::env::set_var("SLEDGE_CONFIG_ROOT", "tests/doc");
use sledge_config::get_config;

let config = get_config().unwrap();
println!("Config: {:#?}", config);
```

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
use std::{env, env::VarError, path::Path};
use tracing::{error, info, trace, trace_span};
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// This is the version expected by this loader, it will be checked during validation for
/// compatibility.
///
pub const CONFIG_FILE_VERSION: Version = Version::new(0, 1, 0);

///
/// The top-level configuration object itself.
///
#[derive(Debug, Deserialize, Clone)]
pub struct Configuration {
    version: Version,
    store: Url,
    #[serde(skip_serializing_if = "Option::is_none")]
    client: Option<ClientConfig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    server: Option<ServerConfig>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

///
/// Return the current configuration object, this will lazy load the data and return any errors
/// that occurred. The response is cached for any subsequent calls.
///
pub fn get_config() -> Result<&'static Configuration, &'static Error> {
    CONFIG.as_ref()
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

    pub fn store(&self) -> &Url {
        &self.store
    }

    pub fn client(&self) -> Option<&ClientConfig> {
        self.client.as_ref()
    }

    pub fn server(&self) -> Option<&ServerConfig> {
        self.server.as_ref()
    }

    pub(crate) fn is_valid(&self) -> bool {
        self.version == CONFIG_FILE_VERSION
            && self.server.as_ref().map(|s| s.is_valid()).unwrap_or(true)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

fn load_config() -> Result<Configuration, Error> {
    let span = trace_span!("load_config");
    let _enter = span.enter();

    let root_str = match env::var("SLEDGE_CONFIG_ROOT") {
        Ok(root_str) => {
            trace!("Configuration using environment variable");
            root_str
        }
        Err(e @ VarError::NotUnicode(_)) => {
            error!("Error, environment variable not Unicode, error: {:?}", e);
            return Err(var_error(e));
        }
        Err(_) => match xdirs::config_dir_for("sledge") {
            Some(root_path) => {
                trace!("Configuration using XDG config directory");
                root_path.to_string_lossy().to_string()
            }
            None => {
                error!("No environment variable OR local configuration directory");
                return Err(config_dir_unknown_error());
            }
        },
    };

    load_config_from(root_str)
}

fn load_config_from<P>(path: P) -> Result<Configuration, Error>
where
    P: AsRef<Path>,
{
    let root_path = path.as_ref();

    trace!("Loading configuration from directory {:?}", root_path);

    match StoredConfig::builder()
        .add_source(File::with_name(&root_path.join("config").to_string_lossy()).required(false))
        .add_source(File::with_name(&root_path.join("server").to_string_lossy()).required(false))
        .add_source(File::with_name(&root_path.join("client").to_string_lossy()).required(false))
        .add_source(Environment::with_prefix("SLEDGE"))
        .build()
    {
        Ok(config) => {
            let de_config: Result<Configuration, ConfigError> = config.try_deserialize();
            match de_config {
                Ok(de_config) => {
                    if de_config.is_valid() {
                        info!("Configuration data loaded successfully");
                        Ok(de_config)
                    } else {
                        error!("Loaded config failed validation");
                        Err(config_validation_error())
                    }
                }
                Err(e) => {
                    error!("Resulting configuration could not de-serialize");
                    Err(config_error(e))
                }
            }
        }
        Err(e) => {
            error!("StoredConfig loader resulted in error: {:?}", e);
            Err(config_error(e))
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;

#[doc(hidden)]
mod client;
pub use client::ClientConfig;

#[doc(hidden)]
mod server;
pub use server::{Binding, BindingOptions, ServerConfig, SslBinding};
