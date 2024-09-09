pub mod sub;

use thiserror::Error;

use crate::{ctx_error, shared_const::msg::*};

pub type TestResult<T, E = CtxError> = core::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum TestError {
    #[error("TestError::Io: {ERR_IO}: {_0:?}")]
    Io(#[from] std::io::Error),
    #[error("TestError::Sub: {_0:?}")]
    Sub(#[from] sub::TestError),
}

ctx_error!(TestError);
