#![allow(clippy::unwrap_used, missing_docs)]

mod error;

use std::{cell::OnceCell, fs::File, io::ErrorKind, sync::Mutex};

use assert2::{assert, let_assert};
use error::{Error, Result};

use crate::ICtxError;

static ERROR_LINE: Mutex<OnceCell<u32>> = Mutex::new(OnceCell::new());

#[test]
fn returns_root_ctx_error() {
    // Given
    let ((expected_file, expected_column), expected_user_context, expected_error) =
        ((file!(), 5), Vec::<String>::new().into_iter(), ErrorKind::NotFound);
    let sut = failing_file_open;

    // When
    let res = sut();

    // Then
    let_assert!(Err(ctx_error) = res);

    // Check location
    let location = ctx_error.location();
    assert!(location.file() == expected_file);
    assert!(location.line() == *ERROR_LINE.lock().unwrap().get().unwrap());
    assert!(location.column() == expected_column);

    // Check user context
    assert!(ctx_error.context().cloned().eq(expected_user_context));

    // Check error
    let_assert!(Error::Io(std_io_error) = ctx_error.error());
    assert!(std_io_error.kind() == expected_error);
}

fn failing_file_open() -> Result<File> {
    const NON_EXISTENT_FILE: &str = "non-existent file.txt";

    // Keep `ERROR_LINE` immediately preceding `File::open(NON_EXISTENT_FILE)?;`
    ERROR_LINE.lock().unwrap().set(line!().checked_add(1).unwrap()).unwrap();
    File::open(NON_EXISTENT_FILE)?;
    unreachable!("Please ensure the filename in `NON_EXISTENT_FILE` does not exist.");
}
