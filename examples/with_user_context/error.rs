pub mod sub_error;

use core::result::Result as CoreResult;

use lib::error::ctx_error;
use thiserror::Error;

pub type Result<T, E = CtxError> = CoreResult<T, E>;
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] sub_error::Error),
}

ctx_error!(Error);
