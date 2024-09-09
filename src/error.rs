mod ctx_error;

use core::result::Result as CoreResult;

use thiserror::Error;

#[allow(clippy::wildcard_imports,
            reason = "`shared_const` is designed to be wildcard imported (and is under our \
                      control).")]
use crate::{ctx_error, shared_const::msg::*};
pub type Result<T, E = CtxError> = CoreResult<T, E>;

#[derive(Debug, Error)]
pub enum Error {
    #[error("{ERR_IO}: {_0}")]
    Io(#[from] std::io::Error),
}

ctx_error!(Error);

// impl<T, E, TCtxError> IResultExt<T, E, TCtxError> for CoreResult<T, E>
//     where E: CoreError + Into<TCtxError>,
//           TCtxError: ICtxError,
// {
//     fn append_ctx<TString>(self, s: TString) -> CoreResult<T, TCtxError>
//         where TString: Into<String>, {
//         self.map_err(|error| {
//                 let mut ctx_error = error.into();
//                 ctx_error.append_ctx(s);
//                 ctx_error
//             })
//     }
// }
