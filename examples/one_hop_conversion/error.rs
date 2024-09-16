use ctx_error::ctx_error;
use core::result::Result as CoreResult;

use thiserror::Error;

pub type Result<T, E = CtxError> = CoreResult<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
}

ctx_error!(Error);
