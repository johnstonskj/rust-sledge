/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use chrono::{DateTime, Utc};
use rust_decimal::Decimal;
use sledge_model::{commodity::CommodityId, Service};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct PriceQuote {
    quoted_price: Decimal,
    received_date: DateTime<Utc>,
    source: String,
}

#[derive(Debug)]
pub struct PriceHistory {
    prices: Vec<Decimal>,
    start_date: DateTime<Utc>,
    range: HistoryRange,
    received_date: DateTime<Utc>,
    source: String,
}

#[derive(Debug)]
pub enum HistoryRange {
    Hour,
    Day,
    Week,
    Month,
    Year,
}

pub trait CommodityPriceService: Service {
    fn get_price(&self, commodity: CommodityId) -> Result<PriceQuote, Self::Error>;
}

pub trait CommodityPriceHistoryService: Service {
    fn get_historical_price(
        &self,
        commodity: CommodityId,
        as_of: DateTime<Utc>,
    ) -> Result<PriceQuote, Self::Error>;

    fn get_price_trailing_history(
        &self,
        commodity: CommodityId,
        trailing: HistoryRange,
    ) -> Result<PriceHistory, Self::Error>;
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
