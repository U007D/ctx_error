#[cfg(test)]
mod unit_tests;

#[macro_export]
macro_rules! ctx_error {
    ($err_type:ty) => {
        use core::{fmt::{Display, Formatter, Result as FmtResult},
                   panic::Location,
                   slice};
        use std::backtrace::Backtrace;

        use $crate::r#trait::ICtxError;

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
            type Error = $err_type;

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

            fn parts<'a>(
                self)
                -> (&'a core::panic::Location<'a>, Vec<String>, Backtrace, Self::Error) {
                (self.location, self.context, self.backtrace, self.error)
            }
        }

        impl<TError> From<TError> for CtxError where TError: Into<$err_type>, {
            #[track_caller]
            fn from(error: TError) -> Self {
                Self { location: Location::caller(),
                       context: Vec::new(),
                       backtrace: Backtrace::capture(),
                       error: error.into() }
            }
        }
    };
}
