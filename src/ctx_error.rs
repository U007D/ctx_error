//! Macro for creating a `CtxError` and the associated error conversions.

/// Generate a local `CtxError` wrapping the supplied error type.
///
/// An optional second argument specifies a parent contextual error type. The generated
/// conversion preserves the original location, context and backtrace while converting the
/// wrapped error. Both error types must implement `core::error::Error`.
///
/// With the `std` feature, contexts accumulate in order. Without it, zero or one context is
/// stored, and the first attached context is retained. All contexts are `&'static str`.
#[macro_export]
macro_rules! ctx_error {
    ($err_type:ty) => {
        $crate::ctx_error!(__GENERATE_CTX_ERROR $err_type);
    };

    ($err_type:ty, $fq_parent_ctx_err_type:ty) => {
        $crate::ctx_error!(__GENERATE_CTX_ERROR $err_type);
        $crate::ctx_error!(__GENERATE_CTX_ERROR_FROM_IMPL $fq_parent_ctx_err_type);
    };

    (__GENERATE_CTX_ERROR $err_type:ty) => {
        /// The local error with its original location, static context and optional backtrace.
        #[derive(Debug)]
        pub struct CtxError {
            backtrace: ::core::option::Option<$crate::conditional_compilation::Backtrace>,
            context: $crate::conditional_compilation::Context,
            error: <Self as $crate::ICtxError>::Error,
            location: &'static ::core::panic::Location<'static>,
        }

        impl ::core::error::Error for CtxError {
            $crate::impl_error_provide!();

            #[inline]
            fn source(&self) -> ::core::option::Option<&(dyn ::core::error::Error + 'static)> {
                ::core::option::Option::Some(&self.error)
            }
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
            fn append_ctx(mut self, s: &'static str) -> Self {
                $crate::conditional_compilation::append_context(&mut self.context, s);
                self
            }

            #[inline]
            fn backtrace(&self) -> &::core::option::Option<$crate::conditional_compilation::Backtrace> {
                &self.backtrace
            }

            #[inline]
            fn context(&self) -> ::core::slice::Iter<'_, &'static str> {
                $crate::conditional_compilation::context_slice(&self.context).iter()
            }

            #[inline]
            fn error(&self) -> &Self::Error { &self.error }

            #[inline]
            fn location(&self) -> &::core::panic::Location<'_> { self.location }

            #[inline]
            fn parts<'a>(self) -> (
                <Self as $crate::ICtxError>::Error,
                &'a ::core::panic::Location<'a>,
                $crate::conditional_compilation::Context,
                ::core::option::Option<$crate::conditional_compilation::Backtrace>,
            ) {
                (self.error, self.location, self.context, self.backtrace)
            }
        }

        impl<TError> ::core::convert::From<TError> for CtxError
            where TError: ::core::convert::Into<<CtxError as $crate::ICtxError>::Error>,
        {
            #[inline]
            #[track_caller]
            fn from(error: TError) -> Self {
                Self {
                    backtrace: $crate::conditional_compilation::capture_backtrace(),
                    context: ::core::option::Option::None,
                    error: error.into(),
                    location: ::core::panic::Location::caller(),
                }
            }
        }

        impl<T, E> $crate::IResultExt<CtxError, T> for ::core::result::Result<T, E>
            where E: ::core::convert::Into<$err_type>,
        {
            #[inline]
            #[track_caller]
            fn append_ctx(self, s: &'static str) -> ::core::result::Result<T, CtxError> {
                // Keep conversion outside a closure so track_caller sees the original caller.
                match self {
                    ::core::result::Result::Err(error) => {
                        let ctx_error = <CtxError as ::core::convert::From<E>>::from(error);
                        ::core::result::Result::Err(
                            <CtxError as $crate::ICtxError>::append_ctx(ctx_error, s),
                        )
                    }
                    ::core::result::Result::Ok(value) => ::core::result::Result::Ok(value),
                }
            }
        }

        impl<T> $crate::IResultExt<CtxError, T> for ::core::result::Result<T, CtxError> {
            #[inline]
            #[track_caller]
            fn append_ctx(self, s: &'static str) -> Self {
                self.map_err(|ctx_error| {
                    <CtxError as $crate::ICtxError>::append_ctx(ctx_error, s)
                })
            }
        }
    };

    (__GENERATE_CTX_ERROR_FROM_IMPL $fq_parent_ctx_err_type:ty) => {
        impl ::core::convert::From<CtxError> for $fq_parent_ctx_err_type
            where <CtxError as $crate::ICtxError>::Error:
                ::core::convert::Into<<$fq_parent_ctx_err_type as $crate::ICtxError>::Error>,
        {
            #[inline]
            #[track_caller]
            fn from(ctx_error: CtxError) -> Self {
                let (error, location, context, backtrace) =
                    <CtxError as $crate::ICtxError>::parts(ctx_error);
                Self { backtrace, context, error: error.into(), location }
            }
        }
    };
}
