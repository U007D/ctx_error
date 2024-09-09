#![allow(clippy::needless_collect, clippy::unwrap_used)]

mod expected_location;
mod test_error;

use std::{fs::File,
          io::{Error as StdIoError, ErrorKind as StdErrorKind}};

use assert2::{assert, let_assert};
use expected_location::ExpectedLocation;

#[allow(unused_imports)]
use super::*;
use crate::r#trait::ICtxError;

fn file_not_found() -> test_error::TestResult<()> {
    use test_error::TestError;

    Err(TestError::Io(StdIoError::from(StdErrorKind::NotFound)))?;
    unreachable!()
}

fn sub_error() -> test_error::sub::TestResult<()> {
    use test_error::sub::TestError;

    Err(TestError::SubTestError)?;
    unreachable!()
}

fn parent_from_sub_error() -> test_error::TestResult<()> {
    sub_error()?;
    unreachable!()
}

#[test]
fn returns_expected_base_ctx_error() {
    use test_error::TestError;

    // Given
    // Empirically derived (distance from `ExpectedLocation::new()` to `open_non_existent_file()`'s
    // `File::open()` line, above.
    let (expected_line_offset, expected_column) = (-10, 5);
    let expected_location = ExpectedLocation::new(expected_line_offset, expected_column);
    let expected_error_kind = StdErrorKind::NotFound;

    // When
    let res = file_not_found().unwrap_err();

    // Then
    // Check `location`
    assert!(res.location().file() == expected_location.file());
    assert!(res.location().line() == expected_location.line());
    assert!(res.location().column() == expected_location.column());

    // Check `context`
    assert!(res.context().collect::<Vec<_>>().is_empty());

    // Check `error`
    let_assert!(TestError::Io(io_error) = res.error());
    assert!(io_error.kind() == expected_error_kind);
}

#[test]
fn sub_ctx_error_converts_to_expected_ctx_error() {
    // Given
    // Empirically derived (distance from `ExpectedLocation::new()` to `open_non_existent_file()`'s
    // `File::open()` line, above.
    let (expected_line_offset, expected_column) = (-10, 5);
    let expected_location = ExpectedLocation::new(expected_line_offset, expected_column);
    let expected_error_kind = StdErrorKind::NotFound;

    // When
    let res = parent_from_sub_error().unwrap_err();

    // Then
    // Check `location`
    assert!(res.location().file() == expected_location.file());
    assert!(res.location().line() == expected_location.line());
    assert!(res.location().column() == expected_location.column());

    // Check `context`
    assert!(res.context().collect::<Vec<_>>().is_empty());

    // Check `error`
    let_assert!(test_error::TestError::Sub(sub_test_error) = res.error());
    let_assert!(test_error::sub::TestError::SubTestError = sub_test_error);
}
