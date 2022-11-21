/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::commodity::CommodityId;
use chrono::{DateTime, Utc};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct AccountId(String);

#[derive(Debug)]
pub enum AccountKind {
    Asset,
    AccountsReceivable,
    Bank,
    Equity,

    Liability,
    AccountsPayable,
    Credit,

    Income,
    Expense,
}

#[derive(Debug)]
pub struct Account {
    id: AccountId,
    created: DateTime<Utc>,
    is_active: bool,
    parent_id: Option<String>,
    kind: AccountKind,
    commodity: CommodityId,
    description: String,
    is_recording: bool,
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
