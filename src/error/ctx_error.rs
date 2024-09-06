#[cfg(test)]
mod unit_tests;

use core::{error::Error as CoreError,
           fmt::{Display, Formatter, Result as FmtResult},
           panic::Location,
           result::Result as CoreResult,
           slice};
use std::backtrace::Backtrace;

use thiserror::Error;

use crate::r#trait::{ICtxError, ResultExt};

pub type Result<T, E = CtxError> = std::result::Result<T, E>;

#[derive(Debug, Error)]
pub struct CtxError {
    location: &'static Location<'static>,
    context: Vec<String>,
    backtrace: Backtrace,
    #[source]
    error: <Self as ICtxError>::Error,
}

impl Display for CtxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { <Self as ICtxError>::fmt(self, f) }
}

impl ICtxError for CtxError {
    type Error = super::Error;

    fn append_ctx<TString>(&mut self, s: TString) -> &mut Self
        where TString: Into<String>, {
        let s = s.into();
        self.context.push(s);
        self
    }

    #[must_use]
    fn location(&self) -> &core::panic::Location<'_> { self.location }

    fn backtrace(&self) -> &Backtrace { &self.backtrace }

    fn context(&self) -> slice::Iter<'_, String> { self.context.iter() }

    #[must_use]
    fn error(&self) -> &Self::Error { &self.error }

    fn parts<'a>(self) -> (&'a core::panic::Location<'a>, Vec<String>, Backtrace, Self::Error) {
        (self.location, self.context, self.backtrace, self.error)
    }
}

impl<T, E, TCtxError> ResultExt<T, E, TCtxError> for CoreResult<T, E>
    where E: CoreError + Into<TCtxError>,
          TCtxError: ICtxError,
{
    fn append_ctx<TString>(self, s: TString) -> CoreResult<T, TCtxError>
        where TString: Into<String>, {
        self.map_err(|error| {
                let mut ctx_error = error.into();
                ctx_error.append_ctx(s);
                ctx_error
            })
    }
}

impl<TError> From<TError> for CtxError where TError: Into<super::Error>, {
    #[track_caller]
    fn from(error: TError) -> Self {
        Self { location: Location::caller(),
               context: Vec::new(),
               backtrace: Backtrace::capture(),
               error: error.into() }
    }
}
