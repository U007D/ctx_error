use std::io::ErrorKind as StdIoErrorKind;

use assert2::{assert, let_assert};

use super::*;
use crate::error::ctx_error::unit_tests::utils::{expected_location::ExpectedLocation,
                                                 functions::{ctx_file_not_found,
                                                             FNF_TEST_ERROR_LINE_NO}};

#[test]
fn returns_expected_ctx_error() {
    use crate::error::ctx_error::unit_tests::utils::test_error::TestError;

    // Given
    // `TEST_ERROR_LINE_NO` is referencing `unit_tests::open_non_existent_file()`'s
    // `Err(TestError::Io(StdIoError::from(StdIoErrorKind::NotFound)))?;` line.
    let (expected_line_offset, expected_column) = (*FNF_TEST_ERROR_LINE_NO, 5);
    let expected_location = ExpectedLocation::new(expected_line_offset, expected_column);
    let expected_error_kind = StdIoErrorKind::NotFound;

    // When
    let res = ctx_file_not_found().unwrap_err();

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
