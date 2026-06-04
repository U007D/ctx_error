//! This module contains the macro definition for creating a `CtxError` and all necessary related
//! type converters.

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
        // TODO:
        //   * Figure out how to make `thiserror` import hygenic (avoid conflict with caller's scope)
        //   * Use `#[no_std]`-compatible `thiserror` (NB: `thiserror-no-std` does not seem to work)
        #[derive(Debug, Error)]
        /// Generates a local `CtxError`.  A `CtxError` is a wrapper over the local error type
        /// (`$err_type`) which also includes `Location` (file, line and column number), user-
        /// context and optional `Backtrace` information.  A converter is also created for this type
        /// allowing conversion from the local (non-contextual) `Error` type to this `CtxError`.  A
        /// second converter is optionally created to convert this `CtxError` to its parent
        /// `CtxError` if the `ctx_error` macro generating this type is provided with a parent type.
        pub struct CtxError {
            backtrace: ::std::backtrace::Backtrace,
            context: Vec<String>,
            #[source]
            error: <Self as $crate::ICtxError>::Error,
            location: &'static ::core::panic::Location<'static>,
        }

        impl ::core::fmt::Display for CtxError {
            #[inline]
            fn fmt(&self, f: &mut ::core::fmt::Formatter<'_>) -> ::core::fmt::Result {
                <Self as $crate::ICtxError>::fmt(self, f)
            }
        }

        impl $crate::ICtxError for CtxError {
            type Error = $err_type;

            #[inline]
            #[track_caller]
            fn append_ctx<TString>(mut self, s: TString) -> Self
                where TString: Into<String>, {
                let s = s.into();
                self.context.push(s);
                self
            }

            #[inline]
            fn backtrace(&self) -> &::std::backtrace::Backtrace { &self.backtrace }

            #[inline]
            fn context(&self) -> ::core::slice::Iter<'_, String> {
                self.context.iter()
            }

            #[inline]
            fn error(&self) -> &Self::Error { &self.error }

            #[inline]
            fn location(&self) -> &::core::panic::Location<'_> { self.location }

            #[inline]
            fn parts<'a>(
                self)
                -> (&'a ::core::panic::Location<'a>,
                    Vec<String>,
                    ::std::backtrace::Backtrace,
                    Self::Error) {
                (self.location, self.context, self.backtrace, self.error)
            }
        }

        impl<TError> ::core::convert::From<TError> for CtxError
            where TError: ::core::convert::Into<<CtxError as $crate::ICtxError>::Error>,
        {
            #[inline]
            #[track_caller]
            fn from(error: TError) -> Self {
                Self { backtrace: ::std::backtrace::Backtrace::capture(),
                       context: Vec::new(),
                       error: error.into(),
                       location: ::core::panic::Location::caller() }
            }
        }

        impl<T, E> $crate::IResultExt<CtxError, T> for ::core::result::Result<T, E>
            where E: ::core::convert::Into<$err_type>,
        {
            #[inline]
            #[track_caller]
            fn append_ctx<TString>(self, s: TString) -> ::core::result::Result<T, CtxError>
                where TString: ::core::convert::Into<String>
            {
                // Note: conversion using closure returns wrong `Location`
                //       (`std` bug or incomplete `#[track_caller]` impl?)
                // self.map_err(|error| CtxError {
                //         location: Location::caller(),
                //         context: vec![s.into()],
                //         backtrace: Backtrace::capture(),
                //         error: error.into()
                //     })

                match self {
                    ::core::result::Result::Err(error) => ::core::result::Result::Err(CtxError {
                        backtrace: ::std::backtrace::Backtrace::capture(),
                        context: vec![s.into()],
                        error: error.into(),
                        location: ::core::panic::Location::caller(),
                    }),
                    ::core::result::Result::Ok(t) => ::core::result::Result::Ok(t),
                }
            }
        }

        impl<T> $crate::IResultExt<CtxError, T> for ::core::result::Result<T, CtxError> {
            #[inline]
            #[track_caller]
            fn append_ctx<TString>(self, s: TString) -> Self
                where TString: ::core::convert::Into<String>
            {
                self.map_err(|ctx_error| {
                    <CtxError as $crate::ICtxError>::append_ctx(ctx_error, s.into())
                })
            }
        }
    };

    // For internal use by `ctx_error!()` macro only
    (__GENERATE_CTX_ERROR_FROM_IMPL $fq_parent_ctx_err_type:ty) => {
        impl ::core::convert::From<CtxError> for $fq_parent_ctx_err_type
            where <CtxError as $crate::ICtxError>::Error:
                      ::core::convert::Into<<$fq_parent_ctx_err_type as $crate::ICtxError>::Error>,
        {
            #[inline]
            #[track_caller]
            fn from(ctx_error: CtxError) -> Self {
                let (location, context, backtrace, error) =
                    <CtxError as $crate::ICtxError>::parts(ctx_error);
                Self { backtrace,
                       context,
                       error: error.into(),
                       location }
            }
        }
    };
}
