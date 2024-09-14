pub mod sub;

use core::result::Result as CoreResult;

use lib::ctx_error;
use thiserror::Error;

pub type Result<T, E = CtxError> = CoreResult<T, E>;
#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] sub::Error),
}

ctx_error!(Error);
