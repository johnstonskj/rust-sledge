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

use sledge_model::identity::Labeled;

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

pub trait DataStore {
    type Error;

    fn exists(&self) -> bool;

    fn init(&self) -> Result<(), Self::Error>;

    fn connect(&self) -> Result<(), Self::Error>;

    fn ledgers(&self) -> Result<Box<dyn EntityStore>, Self::Error>;

    fn journals(&self) -> Result<Box<dyn EntityStore>, Self::Error>;

    fn disconnect(&self) -> Result<(), Self::Error>;
}

pub trait EntityStore {
    type Identifier;
    type Entity;
    type Summary: Labeled;
    type Error;

    fn create(&self, entity: Self::Entity) -> Result<Self::Identifier, Self::Error>;
    fn create_with_id(&self, entity: Self::Entity, id: Self::Identifier)
        -> Result<(), Self::Error>;

    fn list(&self, page: Option<String>) -> Result<Vec<(Self::Summary, String)>, Self::Error>;

    fn get_by_id(&self, id: &Self::Identifier) -> Result<Option<&Self::Entity>>;

    fn update(&self, entity: Self::Entity) -> Result<(), Self::Error>;

    fn delete(&self, id: &Self::Identifier) -> Result<(), Self::Error>;
}

// ------------------------------------------------------------------------------------------------
// Public Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Implementations
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

pub mod error;

//#[cfg(feature = "fs-store")]
pub mod fs;
