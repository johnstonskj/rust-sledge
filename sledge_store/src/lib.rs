/*!
One-line description.

More detailed description, with

# Example

YYYYY

# Features

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

use crate::error::Error;
use chrono::{DateTime, Utc};
use error::unknown_store_scheme;
use fs::{FileSystemStore, FS_STORE_SCHEME};
use semver::Version;
use sledge_model::{
    journal::Journal,
    ledger::{Ledger, LedgerKind},
};
use std::{fmt::Display, hash::Hash, sync::Arc};
use tracing::trace_span;
use url::Url;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub const STORE_SCHEMA_VERSION: Version = Version::new(0, 1, 0);

pub trait DataStore {
    fn connect(connection_uri: &Url) -> Result<Self, Error>
    where
        Self: Sized;

    fn exists(connection_uri: &Url) -> bool
    where
        Self: Sized;

    fn create(connection_uri: &Url, content: &CreateDatastoreContents) -> Result<Self, Error>
    where
        Self: Sized;

    fn ledgers(&self) -> Result<Box<dyn EntityStore<LedgerKind, Ledger>>, Error>;

    fn journals(&self) -> Result<Box<dyn EntityStore<String, Journal>>, Error>;

    fn disconnect(self) -> Result<(), Error>;
}

pub trait Entity<I>
where
    I: Display + Eq + Hash,
{
    fn identifier(&self) -> &I;

    fn label(&self) -> &String;

    fn created(&self) -> DateTime<Utc>;
}

pub trait EntityStore<I, E>
where
    I: Display + Eq + Hash,
    E: Entity<I>,
{
    fn create(&self, entity: E) -> Result<I, Error>;

    fn create_with_id(&self, entity: E, id: I) -> Result<(), Error>;

    fn list(&self, page: Option<String>) -> Result<Vec<E>, Error>;

    fn get_by_id(&self, id: &I) -> Result<Option<&E>, Error>;

    fn update(&self, entity: E) -> Result<(), Error>;

    fn delete(&self, id: &I) -> Result<(), Error>;
}

#[derive(Debug)]
pub struct CreateDatastoreContents {
    pub ledgers: Vec<Ledger>,
    pub journals: Vec<Journal>,
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

pub fn get_current_datastore(connection_uri: &Url) -> Result<Arc<dyn DataStore>, Error> {
    let span = trace_span!("current_datastore");
    let _enter = span.enter();

    match connection_uri.scheme() {
        FS_STORE_SCHEME => Ok(Arc::new(FileSystemStore::connect(connection_uri)?)),
        _ => Err(unknown_store_scheme(connection_uri.clone())),
    }
}

pub fn create_datastore(
    connection_uri: &Url,
    initial_content: &CreateDatastoreContents,
) -> Result<Arc<dyn DataStore>, Error> {
    let span = trace_span!("create_datastore");
    let _enter = span.enter();

    match connection_uri.scheme() {
        FS_STORE_SCHEME => Ok(Arc::new(FileSystemStore::create(
            connection_uri,
            initial_content,
        )?)),
        _ => Err(unknown_store_scheme(connection_uri.clone())),
    }
}

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

impl Default for CreateDatastoreContents {
    fn default() -> Self {
        Self {
            ledgers: Default::default(),
            journals: Default::default(),
        }
    }
}

impl CreateDatastoreContents {
    pub fn personal_ledger(self) -> Self {
        self
    }

    pub fn general_ledger(self) -> Self {
        self
    }

    pub fn sales_ledger(self) -> Self {
        self
    }

    pub fn purchase_ledger(self) -> Self {
        self
    }

    pub fn combined_journal(self) -> Self {
        self
    }

    pub fn journal_per_ledger(self) -> Self {
        self
    }

    pub fn daily_journals(self) -> Self {
        self
    }
}

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;

pub mod fs;
