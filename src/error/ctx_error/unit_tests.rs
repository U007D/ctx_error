#![allow(clippy::unwrap_used)]

mod expected_location;
mod test_error;

use std::{fs::File, io::ErrorKind};

use assert2::{assert, let_assert};
use expected_location::ExpectedLocation;
use test_error::{TestError, TestResult};

#[allow(unused_imports)]
use super::*;

fn try_open_non_existent_file() -> TestResult<()> {
    const FILENAME: &str = "does not exist.txt";
    File::open(FILENAME)?;
    Ok(())
}

#[test]
fn returns_expected_ctx_error() {
    // Given
    const FILENAME: &str = "does not exist.txt";

    let expected_loc = ExpectedLocation::new(10, 5);
    let expected_err = ErrorKind::NotFound;

    // When
    let res = try_open_non_existent_file().unwrap_err();

    // Then
    let_assert!(CtxError { location, context, backtrace, error } = res);
    let_assert!(StdIoError { } = )
    assert!(error.kind() == expected_error_kind);

    // And
}
