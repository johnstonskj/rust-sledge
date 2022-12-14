/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::audit::Signature;
use chrono::{DateTime, Duration, Utc};
use codes_iso_4217::CurrencyCode;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Journal {
    name: String,
    created: DateTime<Utc>,
    read_only_after: Option<Duration>,
    transactions: Vec<Transaction>,
    currency: CurrencyCode,
    signature: Option<Signature>,
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
mod split;
pub use split::Split;

#[doc(hidden)]
mod transaction;
pub use transaction::{Action, ActionId, Transaction, TransactionId};

#[doc(hidden)]
mod reconcile;
pub use reconcile::Reconciled;
