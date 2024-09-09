use std::fs::File;

use lib::{self, error::Result};

fn foo() -> Result<()> {
    const FILENAME: &str = "does not exist.txt";
    File::open(FILENAME)?;
    Ok(())
}

#[termination::display]
fn main() -> Result<()> {
    foo()?;

    // const FILENAME: &str = "does not exist.txt";
    // File::open(FILENAME)?;
    Ok(())
}
