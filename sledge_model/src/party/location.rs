/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use rust_decimal::Decimal;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

/// ISO-3166 2-character country code
#[derive(Debug)]
pub struct CountryCode(String);

/// GS1/GLN
/// 13 digit number, with structure.
#[derive(Debug)]
pub struct GlobalLocationNumber(String);

#[derive(Debug)]
pub enum Location {
    Address(Address),
    Gln(GlobalLocationNumber),
    Geo(GeoLocation),
}

/// https://en.wikipedia.org/wiki/Address
#[derive(Debug)]
pub struct Address {
    po_box: Option<String>,
    street_number: String,
    street_name: String,
    unit_number: Option<String>,
    neighborhood_or_district: Option<String>,
    city_or_town: String,
    province_or_state: String,
    postal_code: String,
    country: CountryCode,
    gln: Option<GlobalLocationNumber>,
    geo: Option<GeoLocation>,
}

#[derive(Debug)]
pub struct GeoLocation {
    lat: Decimal,                 // `-90.0..=90.0` (WGS-84)
    long: Decimal,                // `-180.0..=180.0` (WGS-84)
    uncertainty: Option<Decimal>, // radius in m
    altitude: Option<Decimal>,    // in m
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
