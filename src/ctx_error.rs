#[cfg(test)]
mod unit_tests;

use std::backtrace::Backtrace;
use std::panic::Location;
use crate::{error::CtxError, r#trait::ICtxError};

#[macro_export]
macro_rules! ctx_error {
    ($err_type:ty) => {
        $crate::ctx_error!(__GENERATE_CTX_ERROR $err_type);
    };

    ($err_type:ty, $fq_parent_ctx_err_type:ty) => {
        $crate::ctx_error!(__GENERATE_CTX_ERROR $err_type);
        $crate::ctx_error!(__GENERATE_CTX_ERROR_FROM_IMPL $fq_parent_ctx_err_type);
    };

    // For internal use by `ctx_error!()` macro only
    (__GENERATE_CTX_ERROR $err_type:ty) => {
        use core::{fmt::{Display, Formatter, Result as FmtResult},
                   panic::Location,
                   slice};
        use std::backtrace::Backtrace;

        use $crate::r#trait::{ICtxError, IResultExt};

        // TODO:
        //   * Figure out how to make `thiserror` import hygenic (avoid conflict with caller's scope)
        //   * Use `#[no_std]`-compatible `thiserror` (NB: `thiserror-no-std` does not seem to work)
        #[derive(Debug, Error)]
        pub struct CtxError {
            pub(super) location: &'static Location<'static>,
            pub(super) context: Vec<String>,
            pub(super) backtrace: Backtrace,
            #[source]
            pub(super) error: <Self as ICtxError>::Error,
        }

        impl Display for CtxError {
            fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult { <Self as ICtxError>::fmt(self, f) }
        }

        impl ICtxError for CtxError {
            type Error = $err_type;

            #[track_caller]
            fn append_ctx<TString>(mut self, s: TString) -> Self
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

            fn parts<'a>(
                self)
                -> (&'a core::panic::Location<'a>, Vec<String>, Backtrace, Self::Error) {
                (self.location, self.context, self.backtrace, self.error)
            }
        }

        // impl<TError> From<TError> for CtxError where TError: Into<$err_type>, {
        //     #[track_caller]
        //     fn from(error: TError) -> Self {
        //         Self { location: Location::caller(),
        //                context: Vec::new(),
        //                backtrace: Backtrace::capture(),
        //                error: error.into() }
        //     }
        // }
        impl<TError> From<TError> for CtxError where TError: Into<<CtxError as ICtxError>::Error>, {
            #[track_caller]
            fn from(error: TError) -> Self {
                Self { location: Location::caller(),
                    context: Vec::new(),
                    backtrace: Backtrace::capture(),
                    error: error.into() }
            }
        }

        impl<T> IResultExt<CtxError, T> for Result<T, CtxError> {
            #[track_caller]
            fn append_ctx<TString>(self, s: TString) -> Self where TString: Into<String> {
                match self {
                    Err(mut ctx_error) => Err(ctx_error.append_ctx(s)),
                    _ => self,
                }
            }
        }
    };

    // For internal use by `ctx_error!()` macro only
    (__GENERATE_CTX_ERROR_FROM_IMPL $fq_parent_ctx_err_type:ty) => {
        impl From<CtxError> for $fq_parent_ctx_err_type
            where <CtxError as ICtxError>::Error:
                      Into<<$fq_parent_ctx_err_type as ICtxError>::Error>,
        {
            #[track_caller]
            fn from(ctx_error: CtxError) -> Self {
                let (location, context, backtrace, error) = ctx_error.parts();
                Self { location,
                       context,
                       backtrace,
                       error: error.into() }
            }
        }
    };
}
