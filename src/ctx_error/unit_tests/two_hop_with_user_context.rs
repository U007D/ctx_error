#![allow(clippy::unwrap_used, missing_docs)]

mod error;

use std::{cell::OnceCell, fs::File, io::ErrorKind, sync::Mutex};

use assert2::{assert, let_assert};
use error::{Error, Result};

use crate::{ICtxError, IResultExt};

const USER_CONTEXT_1: &str = "User context 1 message";
const USER_CONTEXT_2: &str = "User context 2 message";

static ERROR_LINE: Mutex<OnceCell<u32>> = Mutex::new(OnceCell::new());

#[test]
fn returns_root_ctx_error_and_user_context() {
    // Given
    let ((expected_file, expected_column), expected_user_context) =
        ((file!(), 35), vec![String::from(USER_CONTEXT_1), String::from(USER_CONTEXT_2)].into_iter());
    let expected_sub_error_kind = ErrorKind::NotFound;

    // When
    let res = returns_sub_domain_error(expected_sub_error_kind);

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
    let_assert!(Error::SubDomain(error::sub_domain::Error::Io(_)) = ctx_error.error());
}

fn returns_sub_domain_error(expected_sub_error_kind: ErrorKind) -> Result<File> {
    let sub_domain_ctx_error = failing_file_open().unwrap_err();

    // Check error
    let_assert!(error::sub_domain::Error::Io(std_io_error) = sub_domain_ctx_error.error());
    assert!(std_io_error.kind() == expected_sub_error_kind);

    Err(sub_domain_ctx_error).append_ctx(USER_CONTEXT_2)?;

    unreachable!();
}

fn failing_file_open() -> error::sub_domain::Result<File> {
    const NON_EXISTENT_FILE: &str = "non-existent file.txt";

    // Keep `ERROR_LINE` immediately preceding `File::open(NON_EXISTENT_FILE)?;`
    ERROR_LINE.lock().unwrap().set(line!().checked_add(1).unwrap()).unwrap();
    File::open(NON_EXISTENT_FILE).append_ctx(USER_CONTEXT_1)
}
