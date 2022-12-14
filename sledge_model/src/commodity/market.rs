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

use crate::party::CountryCode;

/// Market identifier codes (MIC)
///
/// ISO-10383
#[derive(Debug)]
pub struct MarketIdentifierCode(String);

#[derive(Debug)]
pub struct Market {
    mic: MarketIdentifierCode,
    name: String,
    country: CountryCode,
    city: String,
    bloomberg: Option<String>,
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
