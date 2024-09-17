pub mod sub_domain;

use core::result::Result as CoreResult;

use thiserror::Error;

use crate::ctx_error;

pub type Result<T, E = CtxError> = CoreResult<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error(transparent)]
    SubDomain(#[from] sub_domain::Error),
}

ctx_error!(Error);
