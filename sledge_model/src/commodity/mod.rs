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

#[derive(Debug)]
pub enum CommodityId {
    Currency(CurrencyId),
    Security(InternationalSecuritiesId),
}

#[derive(Debug)]
pub struct Quantity {
    commodity: CommodityId,
    quantity: Decimal,
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
mod currency;
pub use currency::{Currency, CurrencyId};

#[doc(hidden)]
mod exchange;
pub use exchange::{Rate, RateRecord, RatedQuantity};

#[doc(hidden)]
mod market;
pub use market::{Market, MarketIdentifierCode};

#[doc(hidden)]
mod security;
pub use security::{InternationalSecuritiesId, NationalSecuritiesId, Security};
