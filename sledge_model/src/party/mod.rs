/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

// use ...

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// ICD from ISO/IEC 6523-1:1998
/// 4 digit numeric
#[derive(Debug)]
pub struct InternationalCodeDesignator(String);

#[derive(Debug)]
pub enum PartyId {
    LegalEntity(LegalEntityId),
    Business(DunsNumber),
    Person(PersonId),
}

#[derive(Debug)]
pub struct PersonId(String);

#[derive(Debug)]
pub struct Party {
    id: PartyId,
    active: bool,
    name: String,
    notes: String,
    mailing_address: Address,
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

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
mod location;
pub use location::{Address, CountryCode, GeoLocation, GlobalLocationNumber, Location};

#[doc(hidden)]
mod supplier;

#[doc(hidden)]
mod customer;

#[doc(hidden)]
mod organization;
pub use organization::{DunsNumber, Isic, LegalEntityId};

#[doc(hidden)]
mod employee;

#[doc(hidden)]
mod person;
