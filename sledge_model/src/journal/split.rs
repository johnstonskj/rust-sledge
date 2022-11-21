/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::account::AccountId;
use crate::commodity::{exchange::RatedQuantity, Quantity};
use crate::journal::transaction::TransactionId;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct Split {
    id: TransactionId,
    account: AccountId,
    quantity: Quantity,
    exchanged_from: Option<RatedQuantity>,
    description: Option<String>,
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