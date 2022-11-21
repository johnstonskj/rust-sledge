/*!
One-line description.

More detailed description, with

# Example

YYYYY

*/

use crate::commodity::currency::CurrencyId;
use crate::commodity::market::MarketId;

// ------------------------------------------------------------------------------------------------
// Public Macros
// ------------------------------------------------------------------------------------------------

// ------------------------------------------------------------------------------------------------
// Public Types
// ------------------------------------------------------------------------------------------------

///
/// International Securities Identification Number (ISIN)
///
/// (ISO-6166)[https://www.isin.org]
///
/// * A two-letter country code, drawn from a list (ISO 6166) prepared by the
///   International Organization for Standardization (ISO). This code is
///   assigned according to the location of a company's head office. A special
///   code, 'XS' is used for international securities cleared through
///   pan-European clearing systems like Euroclear and CEDEL. Depository receipt
///   ISIN usage is unique in that the country code for the security is that of
///   the receipt issuer, not that of the underlying security.
/// * A nine-digit numeric identifier, called the National Securities
///   Identifying Number (NSIN), and assigned by each country's or region's . If
///   a national number is composed of less than nine digits, it is padded with
///   leading zeros to become a NSIN. The numeric identifier has no intrinsic
///   meaning it is essentially a serial number.
/// * A single check-digit. The digit is calculated based upon the preceding
///   11 characters/digits and uses a sum modulo 10 algorithm and helps ensure
///   against counterfeit numbers.
///   * Characters are assumed to be ASCII and each character is converted to it's
///     corresponding code.
///
/// # Example
///
/// `"US-049580485-1"`
///
/// ```scheme
/// (define (isin-check-digit country nsin)
///   (modulo
///     (+
///       (flatten
///         (map char->integer (string->list country))
///         (map string->integer (string->list nsin))))
///     10))
/// ```
///
#[derive(Debug)]
pub struct InternationalSecuritiesId(String);

///
/// National Securities Identifying Number (NSIN)
///
#[derive(Debug)]
pub struct NationalSecuritiesId(String);

#[derive(Debug)]
pub struct Security {
    market: MarketId,
    symbol: Option<String>,
    isin: Option<InternationalSecuritiesId>,
    name: Option<String>,
    base_currency: Option<CurrencyId>,
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
