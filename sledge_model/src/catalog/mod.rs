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

use rust_decimal::Decimal;

pub struct ProductCategory {}

pub struct Product {}

pub enum UnitPackageKind {
    Each,
    Carton,
    Crate,
    Pallet,
    Container,
}

pub struct UnitPackage {
    kind: UnitPackageKind,
    quantity: u64,
}

pub enum BulkPackageKind {
    Length(UnitCode),
    Weight(UnitCode),
    Volume(UnitCode),
}

pub struct BulkPackage {
    kind: BulkPackageKind,
    quantity: Decimal,
}

pub struct PackageHierarchy {}

pub struct Service {}

pub struct RateCard {}

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
mod unit;
pub use unit::UnitCode;
