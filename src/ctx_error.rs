#[cfg(test)]
mod unit_tests;

#[macro_export]
/// This macro accepts an error type (`$err_type`) and optionally a "fully-qualified" parent
/// contextual error type (`$fq_parent_ctx_err_type`).
///
/// The macro constructs a `CtxError` contextual error type and a `From` impl from the specified
/// error type (`$err_type`) to this `CtxError`.
///
/// When defining a `CtxError` in a subdomain, a second parameter specifying the parent `CtxError`
/// may be provided.  This will construct the `CtxError` and `From` impl as per above *plus* another
/// `From` impl to construct a parent `CtxError` from a `CtxError`.
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

        use $crate::{ICtxError, IResultExt};

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

        impl<TError> From<TError> for CtxError where TError: Into<<CtxError as ICtxError>::Error>, {
            #[track_caller]
            fn from(error: TError) -> Self {
                Self { location: Location::caller(),
                       context: Vec::new(),
                       backtrace: Backtrace::capture(),
                       error: error.into() }
            }
        }

        impl<T, E> IResultExt<CtxError, T> for Result<T, E> where E: Into<$err_type> {
            #[track_caller]
            fn append_ctx<TString>(self, s: TString) -> Result<T, CtxError> where TString: Into<String> {
                // Note: conversion using closure returns wrong `Location`
                //       (`std` bug or incomplete `#[track_caller]` impl?)
                // self.map_err(|error| CtxError {
                //         location: Location::caller(),
                //         context: vec![s.into()],
                //         backtrace: Backtrace::capture(),
                //         error: error.into()
                //     })

                match self {
                    Err(error) => Err(CtxError {
                        location: Location::caller(),
                        context: vec![s.into()],
                        backtrace: Backtrace::capture(),
                        error: error.into()
                    }),
                    Ok(t) => Ok(t),
                }
            }
        }

        impl<T> IResultExt<CtxError, T> for Result<T, CtxError> {
            #[track_caller]
            fn append_ctx<TString>(self, s: TString) -> Self where TString: Into<String> {
                self.map_err(|ctx_error| ctx_error.append_ctx(s.into()))
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
