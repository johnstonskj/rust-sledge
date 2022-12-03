/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::{
    commodity::{CommodityId, Quantity},
    party::PartyId,
};
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

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
    Liability,
    Equity,  // Capital
    Income,  // Operating Revenue
    Expense, // Operating Expenses
}

#[derive(Debug)]
pub struct Account {
    id: AccountId,
    created: DateTime<Utc>,
    is_active: bool,
    parent_id: Option<String>,
    kind: AccountKind,
    description: String,
    commodity: CommodityId,
    is_recording: bool,
}

pub enum AccountRepresents {
    BankAccount,
    BrokerageAccount,
    CreditCard,
    Customer,
    Equipment,
    Loan,
    Supplier,
    Salary,
    Tax,
    UtilityService,
}

#[derive(Debug)]
pub struct BankAccount {
    institution: PartyId,
    account_number: String,
    interest_apr: Option<Decimal>,
}

#[derive(Debug)]
pub struct BrokerageAccount {
    institution: PartyId,
    account_number: String,
}

#[derive(Debug)]
pub struct CreditCard {
    institution: PartyId,
    account_number: String,
    close_month: u8,
    close_day: u8,
    interest_apr: Decimal,
    annual_fee: Quantity,
}

#[derive(Debug)]
pub struct Customer {
    party: PartyId,
}

#[derive(Debug)]
pub struct Equipment {}

#[derive(Debug)]
pub struct Loan {
    institution: PartyId,
    account_number: String,
    interest_apr: Option<Decimal>,
    term_in_months: u8,
    //    against:
}

#[derive(Debug)]
pub struct Supplier {
    party: PartyId,
}

#[derive(Debug)]
pub struct Salary {}

#[derive(Debug)]
pub struct Tax {}

#[derive(Debug)]
pub struct UtilityService {
    provider: PartyId,
    account_number: String,
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
