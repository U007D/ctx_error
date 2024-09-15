#![feature(error_generic_member_access)]

mod error;

use std::fs::File;

use lib::r#trait::IResultExt;

#[termination::display]
fn main() -> error::Result<()> {
    failing_file_open().append_ctx("Opening configuration file.")?;

    Ok(())
}

fn failing_file_open() -> error::sub::Result<File> {
    const NON_EXISTENT_FILE: &str = "non-existent file.txt";

    // TODO: manually converting using `Into` or `From` returns error in
    //       `/rustc/library/core/src/ops/function.rs:250:5`; find a fix
    File::open(NON_EXISTENT_FILE).map_err(error::sub::CtxError::from)
                                 .append_ctx(format!("Error opening \"{NON_EXISTENT_FILE}\"."))
}
