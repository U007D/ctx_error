#![allow(missing_docs, clippy::missing_docs_in_private_items, reason = "Not strictly required for examples.")]
#![allow(clippy::single_call_fn, reason = "Necessary for example code.")]
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
