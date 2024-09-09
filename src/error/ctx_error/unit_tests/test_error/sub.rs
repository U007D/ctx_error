use thiserror::Error;

use crate::error::ctx_error;

pub type TestResult<T, E = CtxError> = core::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum TestError {
    #[error("sub::TestError::SubTestError")]
    SubTestError,
}

ctx_error!(TestError, super::CtxError);
