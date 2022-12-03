/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use chrono::{DateTime, Utc};
use codes_iso_4217::CurrencyCode;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug, PartialEq)]
pub enum LedgerKind {
    General,
    Sales,
    Purchase,
    Other(String),
}

#[derive(Debug)]
pub struct Ledger {
    kind: LedgerKind,
    created: DateTime<Utc>,
    description: String,
    currency: CurrencyCode,
    book: Vec<Account>,
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
mod account;
pub use account::{Account, AccountId, AccountKind};
