pub mod ctx_error;

use thiserror::Error;

#[expect(clippy::wildcard_imports,
             reason = "`shared_const` is designed to be wildcard imported (and is under our \
                       control).")]
use crate::shared_const::*;

pub type Result<T, E = Error> = core::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{}: {}", msg::ERR_IO, _0)]
    Io(#[from] std::io::Error),
}
