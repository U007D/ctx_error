#![allow(missing_docs, clippy::missing_docs_in_private_items, reason = "Not strictly required for examples.")]
#![allow(clippy::single_call_fn, reason = "Necessary for example code.")]
#![feature(error_generic_member_access)]

mod error;

use std::fs::File;

#[termination::display]
fn main() -> error::Result<()> {
    failing_file_open()?;

    Ok(())
}

fn failing_file_open() -> error::sub_domain::Result<File> {
    const NON_EXISTENT_FILE: &str = "non-existent file.txt";

    File::open(NON_EXISTENT_FILE)?;
    unreachable!("Please ensure the filename in `NON_EXISTENT_FILE` does not exist.");
}
