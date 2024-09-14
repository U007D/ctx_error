use assert2::{assert, let_assert};

use super::*;
use crate::{error::ctx_error::unit_tests::utils::{functions::file_not_found,
                                                  test_error::TestError},
            r#trait::IResultExt};

#[test]
fn returns_expected_context() {
    // Given
    let expected_context = "Test user context for error.";
    let sut = file_not_found();

    // When
    let res = sut.append_ctx(expected_context);

    // Then
    // Extract context
    let_assert!(Err(ctx_error) = res);
    let (_location, context, _backtrace, error) = ctx_error.parts();

    // Check context
    assert!(context.into_iter().eq([expected_context]));

    // Check error type
    let_assert!(TestError::Io(_) = error);
}
