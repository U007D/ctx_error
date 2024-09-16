pub mod sub_domain;

use core::result::Result as CoreResult;

use ctx_error::ctx_error;
use thiserror::Error;

pub type Result<T, E = CtxError> = CoreResult<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SubDomain(#[from] sub_domain::Error),
}

ctx_error!(Error);
