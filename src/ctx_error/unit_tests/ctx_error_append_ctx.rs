use assert2::assert;

use super::*;
use crate::error::ctx_error::unit_tests::utils::functions::ctx_file_not_found;

#[test]
fn returns_expected_context() {
    // Given
    let expected_context = "Test user context for error.";
    let mut sut = ctx_file_not_found().unwrap_err();

    // When
    let res = sut.append_ctx(expected_context);

    // Then
    // Check `context`
    assert!(res.context().eq([expected_context]));
}
