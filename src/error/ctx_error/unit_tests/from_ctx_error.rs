use assert2::let_assert;
use super::*;

#[test]
fn sub_ctx_error_converts_to_expected_ctx_error() {
    // Given
    // Empirically derived difference between this module's `ExpectedLocation::new()` statement's
    // line number and `unit_tests::open_non_existent_file()`'s
    // `Err(TestError::Io(StdIoError::from(StdErrorKind::NotFound)))?;` line number (line 19).
    let (expected_line_offset, expected_column) = (15, 5);
    let expected_location = ExpectedLocation::new(expected_line_offset, expected_column);

    // When
    let res = parent_from_sub_error().unwrap_err();

    // Then
    // Check `location`
    assert2::assert!(res.location().file() == expected_location.file());
    assert2::assert!(res.location().line() == expected_location.line());
    assert2::assert!(res.location().column() == expected_location.column());

    // Check `context`
    assert2::assert!(res.context().collect::<Vec<_>>().is_empty());

    // Check `error`
    let_assert!(test_error::TestError::Sub(sub_test_error) = res.error());
    let_assert!(test_error::sub::TestError::SubTestError = sub_test_error);
}
