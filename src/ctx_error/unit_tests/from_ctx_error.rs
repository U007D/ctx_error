use assert2::{assert, let_assert};

use super::*;
use crate::error::ctx_error::unit_tests::utils::{expected_location::ExpectedLocation,
                                                 functions::{ctx_parent_from_sub_error,
                                                             SUB_TEST_ERROR_LINE_NO},
                                                 test_error};

#[test]
fn sub_ctx_error_converts_to_expected_ctx_error() {
    // Given
    // `TEST_ERROR_LINE_NO` is referencing `unit_tests::open_non_existent_file()`'s
    // `Err(TestError::Io(StdIoError::from(StdIoErrorKind::NotFound)))?;` line.
    let (expected_line, expected_column) = (*SUB_TEST_ERROR_LINE_NO, 5);
    let expected_location = ExpectedLocation::new(expected_line, expected_column);

    // When
    let res = ctx_parent_from_sub_error().unwrap_err();

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
