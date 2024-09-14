use std::fs::File;

use lib::{self, error::Result, r#trait::IResultExt};

fn foo() -> Result<()> {
    const FILENAME: &str = "does not exist.txt";
    File::open(FILENAME).append_ctx("whoops!")?;
    Ok(())
}

#[termination::display]
fn main() -> Result<()> {
    foo()?;

    Ok(())
}
