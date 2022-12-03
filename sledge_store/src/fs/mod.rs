/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use self::settings::{
    read_settings_or_default, write_settings, FsStoreSettings, FS_STORE_SETTINGS_FILE,
};
use crate::error::{store_does_not_exist, store_exists, Error};
use crate::fs::journal::create_journals;
use crate::fs::ledger::create_ledgers;
use crate::CreateDatastoreContents;
use crate::DataStore;
use sledge_model::{
    journal::Journal,
    ledger::{Ledger, LedgerKind},
};
use std::fs;
use std::path::PathBuf;
use tracing::{error, trace};
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub(super) const FS_STORE_SCHEME: &str = "fstore";

pub(super) struct FileSystemStore {
    from_uri: Url,
    root_path: PathBuf,
    settings: FsStoreSettings,
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

impl DataStore for FileSystemStore {
    fn connect(connection_uri: &Url) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if Self::exists(connection_uri) {
            trace!("Opening existing store @ <{}>", connection_uri);
            let root_path = PathBuf::from(connection_uri.path());
            let store = FileSystemStore {
                from_uri: connection_uri.clone(),
                root_path: root_path.clone(),
                settings: read_settings_or_default(&root_path),
            };
            Ok(store)
        } else {
            error!(
                "Could not find the required content for a store @ <{}>",
                connection_uri
            );
            Err(store_does_not_exist(connection_uri.clone()))
        }
    }

    fn exists(connection_uri: &Url) -> bool
    where
        Self: Sized,
    {
        let root_path = PathBuf::from(connection_uri.path());
        [FS_STORE_SETTINGS_FILE, "journals/", "ledgers/"]
            .iter()
            .all(|f| {
                let mut target = root_path.clone();
                target.push(f);
                if f.ends_with('/') {
                    target.is_dir()
                } else {
                    target.is_file()
                }
            })
    }

    fn create(connection_uri: &Url, content: &CreateDatastoreContents) -> Result<Self, Error>
    where
        Self: Sized,
    {
        if !Self::exists(connection_uri) {
            let root_path = PathBuf::from(connection_uri.path());
            fs::create_dir_all(&root_path)?;

            trace!("Write store settings file");
            write_settings(&root_path, &Default::default())?;

            trace!("Creating {} journals", content.journals.len());
            let mut path = root_path.clone();
            path.push("journals");
            fs::create_dir_all(&path)?;
            if !content.journals.is_empty() {
                create_journals(&root_path, &content.journals)?;
            }

            create_ledgers(&root_path, &content.ledgers)?;

            trace!("Store @ <{}> created", connection_uri);
            let store = FileSystemStore {
                from_uri: connection_uri.clone(),
                root_path: root_path.clone(),
                settings: read_settings_or_default(&root_path),
            };
            Ok(store)
        } else {
            error!(
                "Could not create a store, one already exists @ <{}>",
                connection_uri
            );
            Err(store_exists(connection_uri.clone()))
        }
    }

    fn ledgers(&self) -> Result<Box<dyn crate::EntityStore<LedgerKind, Ledger>>, Error> {
        todo!()
    }

    fn journals(&self) -> Result<Box<dyn crate::EntityStore<String, Journal>>, Error> {
        todo!()
    }

    fn disconnect(self) -> Result<(), Error> {
        Ok(())
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod customer;

pub mod journal;

pub mod ledger;

pub mod permissions;

pub mod settings;
