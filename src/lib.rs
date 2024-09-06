#![feature(error_generic_member_access)]

// Safety-critical application lints
pub mod error;
pub mod shared_const;
pub mod r#trait;

pub use error::{Error, Result};
