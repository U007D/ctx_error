#![feature(error_generic_member_access)]

mod error;

use std::fs::File;

#[termination::display]
fn main() -> error::Result<()> {
    failing_file_open()?;

    Ok(())
}

fn failing_file_open() -> error::sub::Result<File> {
    const NON_EXISTENT_FILE: &str = "non-existent file.txt";

    File::open(NON_EXISTENT_FILE)?;
    unreachable!("Please ensure the filename in `NON_EXISTENT_FILE` does not exist.");
}
