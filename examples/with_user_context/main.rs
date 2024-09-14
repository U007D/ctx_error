#![feature(error_generic_member_access)]

mod error;

use std::fs::File;

use lib::r#trait::IResultExt;

#[termination::display]
fn main() -> error::Result<()> {
    failing_file_open().append_ctx("Opening configuration file.")?;

    Ok(())
}

fn failing_file_open() -> error::sub_error::Result<File> {
    const NON_EXISTENT_FILE: &str = "non-existent file.txt";

    let res = File::open(NON_EXISTENT_FILE);
    // <Result::<File, std::io::Error> as IResultExt<error::sub_error::CtxError, File,
    // std::io::Error>>::append_ctx(res, format!("Error opening \"{NON_EXISTENT_FILE}\"."))
}
