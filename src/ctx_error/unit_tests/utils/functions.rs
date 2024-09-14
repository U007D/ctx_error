use std::{io::{Error as StdIoError, ErrorKind as StdIoErrorKind},
          sync::LazyLock};

use crate::error::ctx_error::unit_tests::utils::{test_error, test_error::TestError};

// Functions used by various unit tests to test error context and conversions
pub fn file_not_found() -> test_error::TestResult<(), TestError> {
    Err(TestError::Io(StdIoError::from(StdIoErrorKind::NotFound)))?;
    unreachable!()
}

// Functions used by various unit tests to test error context and conversions
pub fn ctx_file_not_found() -> test_error::TestResult<()> {
    use test_error::TestError;

    Err(TestError::Io(StdIoError::from(StdIoErrorKind::NotFound)))?;
    unreachable!()
}
// To avoid lots of red tests, keep the `static FNF_TEST_ERROR_LINE_NO` line below immediately
// following the `unit_tests::file_not_found()` function. Specifically, `FNF_TEST_ERROR_LINE_NUMBER`
// is computing the line number containing the `try` operator (`?`) invokation on the error, above.
pub static FNF_TEST_ERROR_LINE_NO: LazyLock<u32> =
    LazyLock::new(|| line!().checked_sub(7).unwrap());

pub fn ctx_sub_error() -> test_error::sub::TestResult<()> {
    use test_error::sub::TestError;

    Err(TestError::SubTestError)?;
    unreachable!()
}
// To avoid lots of red tests, keep the `static SUB_TEST_ERROR_LINE_NO` line below immediately
// following the `unit_tests::sub_error()` function.  Specifically, `TEST_ERROR_LINE_NUMBER` is
// computing the line number containing the `try` operator (`?`) invokation on the error, above.
pub static SUB_TEST_ERROR_LINE_NO: LazyLock<u32> =
    LazyLock::new(|| line!().checked_sub(7).unwrap());

pub fn ctx_parent_from_sub_error() -> test_error::TestResult<()> {
    ctx_sub_error()?;
    unreachable!()
}
