/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use chrono::{DateTime, Utc};

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct KeyIdentifier(String);

#[derive(Debug)]
pub struct Signature {
    identity: KeyIdentifier,
    signed_on: DateTime<Utc>,
    binary: Vec<u8>,
}

pub trait Signed {
    type Error;

    fn is_signed(&self) -> bool;
    fn sign(&self) -> Result<Signature, Self::Error>;
    fn signature(&self) -> Option<Signature>;
}

// ------------------------------------------------------------------------------------------------

#[derive(Debug)]
pub struct AuditRecordId {
    record_kind: String,
    version: u64,
    identifier: String,
}

#[derive(Debug)]
pub struct AuditRecord {
    id: AuditRecordId,
    modified: DateTime<Utc>,
    modified_by: String, // TODO: what should this be?
    stable_representation: String,
}

pub trait Audited {
    type Error;

    fn audit_identifier(&self) -> AuditRecordId;
    fn create_stable_representation(&self) -> Result<String, Self::Error>;
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
