#![allow(clippy::needless_collect, clippy::unwrap_used)]

mod expected_location;
mod test_error;
mod from_error;
mod from_ctx_error;

use std::io::{Error as StdIoError, ErrorKind as StdErrorKind};

use expected_location::ExpectedLocation;

#[allow(unused_imports)]
use super::*;
use crate::r#trait::ICtxError;

// Functions used by various unit tests to test error context and conversions
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
