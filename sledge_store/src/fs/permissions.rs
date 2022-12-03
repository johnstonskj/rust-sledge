/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::error::Error;
use chrono::{DateTime, Utc};
use semver::Version;
use serde::{Deserialize, Serialize};
use sledge_model::audit::{Action, Resource, RoleId};
use std::{
    collections::{HashMap, HashSet},
    path::Path,
};
use tracing::{trace, warn};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub(super) const FS_STORE_PERMISSIONS_FILE: &str = "permissions.json";

#[derive(Debug, Deserialize, Serialize, Clone)]
pub(super) struct FsStorePermissions {
    version: Version,
    created: DateTime<Utc>,
    roles: HashMap<RoleId, HashMap<Resource, HashSet<Action>>>,
    users: HashMap<UserId, HashSet<RoleId>>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub(super) fn read_settings<P>(root_path: P) -> Result<FsStoreSettings, Error>
where
    P: AsRef<Path>,
{
    let file_path = root_path.as_ref().join(FS_STORE_PERMISSIONS_FILE);
    trace!("Reading store permissions from {:?}", file_path);
    let file = std::fs::File::options().read(true).open(file_path)?;
    let settings: FsStorePermissions = serde_json::from_reader(file)?;
    Ok(settings)
}

pub(super) fn read_settings_or_default<P>(root_path: P) -> FsStoreSettings
where
    P: AsRef<Path>,
{
    match read_settings(root_path) {
        Ok(settings) => settings,
        Err(e) => {
            warn!("Error reading store permissions; error: {:?}", e);
            Default::default()
        }
    }
}

pub(super) fn write_settings<P>(root_path: P, settings: &FsStoreSettings) -> Result<(), Error>
where
    P: AsRef<Path>,
{
    let file_path = root_path.as_ref().join(FS_STORE_PERMISSIONS_FILE);
    trace!("Writing store permissions to {:?}", file_path);
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------
