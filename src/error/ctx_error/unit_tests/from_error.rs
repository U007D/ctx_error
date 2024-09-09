use assert2::let_assert;

use super::*;

#[test]
fn returns_expected_ctx_error() {
    use crate::error::ctx_error::unit_tests::test_error::TestError;

    // Given
    // Empirically derived difference between this module's `ExpectedLocation::new()` statement's
    // line number and `unit_tests::open_non_existent_file()`'s
    // `Err(TestError::Io(StdIoError::from(StdErrorKind::NotFound)))?;` line number (line 19).
    let (expected_line_offset, expected_column) = (5, 5);
    let expected_location = ExpectedLocation::new(expected_line_offset, expected_column);
    let expected_error_kind = StdErrorKind::NotFound;

    // When
    let res = file_not_found().unwrap_err();

    // Then
    // Check `location`
    assert2::assert!(res.location().file() == expected_location.file());
    assert2::assert!(res.location().line() == expected_location.line());
    assert2::assert!(res.location().column() == expected_location.column());

    // Check `context`
    assert2::assert!(res.context().collect::<Vec<_>>().is_empty());

    // Check `error`
    let_assert!(TestError::Io(io_error) = res.error());
    assert2::assert!(io_error.kind() == expected_error_kind);
}
