//! Ergonomic, static, contextual errors in Rust.
//!
//! In a (typically larger) Rust program which is handling errors correctly, by default Rust does not
//! capture the location (filename, line number and column number) where an `Error` was first emitted.
//! `ctx_error` addresses this issue with a minimal ergonomic hit (requires changing at most 3 lines
//! per error module).

#![feature(error_generic_member_access)]

mod error;

use std::fs::File;

use ctx_error::IResultExt;

#[termination::display]
fn main() -> error::Result<()> {
    failing_file_open().append_ctx("Opening configuration file.")?;

    Ok(())
}

fn failing_file_open() -> error::sub_domain::Result<File> {
    const NON_EXISTENT_FILE: &str = "non-existent file.txt";

    File::open(NON_EXISTENT_FILE).append_ctx(format!("Error opening \"{NON_EXISTENT_FILE}\"."))
}
