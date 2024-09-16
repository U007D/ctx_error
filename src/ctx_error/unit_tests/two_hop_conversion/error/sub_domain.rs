use core::result::Result as CoreResult;

use crate::ctx_error;
use thiserror::Error;

pub type Result<T, E = CtxError> = CoreResult<T, E>;
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

ctx_error!(Error, super::CtxError);
