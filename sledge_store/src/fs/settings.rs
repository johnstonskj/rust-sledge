/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::{error::Error, STORE_SCHEMA_VERSION};
use chrono::{DateTime, Utc};
use semver::Version;
use serde::{Deserialize, Serialize};
use sledge_model::commodity::CommodityId;
use std::path::{Path, PathBuf};
use tracing::{trace, warn};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub(super) const FS_STORE_SETTINGS_FILE: &str = "settings.json";

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(super) struct FsStoreSettings {
    version: Version,
    created: DateTime<Utc>,
    default_commodity: CommodityId,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn read_settings<P>(root_path: P) -> Result<FsStoreSettings, Error>
where
    P: AsRef<Path>,
{
    let file_path = root_path.as_ref().join(FS_STORE_SETTINGS_FILE);
    trace!("Reading store settings from {:?}", file_path);
    let file = std::fs::File::options().read(true).open(file_path)?;
    let settings: FsStoreSettings = serde_json::from_reader(file)?;
    Ok(settings)
}

pub(super) fn read_settings_or_default<P>(root_path: P) -> FsStoreSettings
where
    P: AsRef<Path>,
{
    match read_settings(root_path) {
        Ok(settings) => settings,
        Err(e) => {
            warn!("Error reading store settings; error: {:?}", e);
            Default::default()
        }
    }
}

pub(super) fn write_settings<P>(root_path: P, settings: &FsStoreSettings) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let file_path = root_path.as_ref().join(FS_STORE_SETTINGS_FILE);
    trace!("Writing store settings to {:?}", file_path);
    let file = std::fs::File::options()
        .write(true)
        .create(true)
        .open(file_path)?;
    serde_json::to_writer_pretty(file, settings)?;
    Ok(())
}

// ------------------------------------------------------------------------------------------------
// Private Types
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for FsStoreSettings {
    fn default() -> Self {
        Self {
            version: STORE_SCHEMA_VERSION,
            created: Default::default(),
            default_commodity: todo!(),
        }
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
