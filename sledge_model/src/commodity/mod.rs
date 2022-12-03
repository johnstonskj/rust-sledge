/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use codes_iso_4217::CurrencyCode;
use rust_decimal::Decimal;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub enum CommodityId {
    Currency(CurrencyCode),
    Security(InternationalSecuritiesId),
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
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

impl CommodityId {}

impl From<CurrencyCode> for CommodityId {
    fn from(v: CurrencyCode) -> Self {
        Self::Currency(v)
    }
}

impl From<InternationalSecuritiesId> for CommodityId {
    fn from(v: InternationalSecuritiesId) -> Self {
        Self::Security(v)
    }
}

// ------------------------------------------------------------------------------------------------
// Private Functions
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Modules
// ------------------------------------------------------------------------------------------------

#[doc(hidden)]
mod exchange;
pub use exchange::{Rate, RateRecord, RatedQuantity};

#[doc(hidden)]
mod market;
pub use market::{Market, MarketIdentifierCode};

#[doc(hidden)]
mod security;
pub use security::{InternationalSecuritiesId, NationalSecuritiesId, Security};
use serde::{Deserialize, Serialize};
