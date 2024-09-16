//! `en_us` (U.S. English) module for this crate's `const` messages.  This enables the localization
//! technique of resourcing end-user-facing messages in a separate file from the code.
#![allow(missing_docs,
          reason = "These string consts used to output messages to the user, per this module's \
                    documentation.")]
pub const ERR_IO: &str = "Error performing I/O";

pub const MSG_AT: &str = "at";
pub const MSG_BUT_EXPECTED: &str = "but expected";
pub const MSG_CONTEXT: &str = "Context";
pub const MSG_ERROR_BACKTRACE: &str = "Error Backtrace";
pub const MSG_ERROR_ORIGINATION_SITE: &str = "error origination site";
pub const MSG_DISPLAY_ORDER: &str = "Display order";
pub const MSG_PROGRAM_ENTRY_POINT: &str = "program entry point";
pub const MSG_RECEIVED: &str = "Received";
pub const MSG_TO: &str = "to";
pub const MSG_UNKNOWN_APP_NAME: &str = "<unknown application name>";
pub const MSG_USAGE: &str = "Usage";
