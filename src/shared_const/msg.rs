//! Message constants for this crate.  This enables the localization technique of resourcing end-
//! user-facing messages in a separate file from the code.

mod en_us;

#[allow(clippy::wildcard_imports,
            reason = "`shared_const` items are not intended to be tracked individually.")]
pub use en_us::*;
