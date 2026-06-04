//! Module containing `CtxError`'s trait definition.

use core::{error::Error as CoreError,
           fmt::{Formatter, Result as FmtResult},
           panic::Location,
           slice};
use std::backtrace::{Backtrace, BacktraceStatus};

use bool_ext::BoolExt;

#[allow(clippy::wildcard_imports,
            reason = "`shared_const` items are not intended to be tracked individually.")]
use crate::shared_const::msg::*;

/// This trait defines the interface for all `CtxError`s.
pub trait ICtxError: CoreError {
    /// Specifies the type of `core::error::Error` that the implementing `CtxError` will wrap.
    type Error: CoreError;

    #[must_use]
    #[track_caller]
    /// This method allows the caller to provide additional user-specified context to a `CtxError`.
    /// This could help the user understand in what context (e.g. *why*) the code was attempting to
    /// utilize a resource (e.g. read a file) when a failure occurred.
    fn append_ctx<TString>(self, s: TString) -> Self
        where TString: Into<String>;

    /// Provide (a reference to) a `Backtrace` (call stack dump) from where this error was emitted
    /// up to the program entry point.  Note that the `RUST_BACKTRACE` environment variable must
    /// be set to `1` for the backtrace to be captured (`RUST_BACKTRACE=1 my_app` is one way to
    /// enable `Backtrace`s).
    fn backtrace(&self) -> &Backtrace;

    /// Provide an iterator of (references to) `String`s representing all the supplied user-context.
    /// The order of iteration starts from context provided closest to point of error emission.
    fn context(&self) -> slice::Iter<'_, String>;

    /// Provide a (reference to) the inner `CoreError` that `CtxError` is wrapping.
    #[must_use]
    fn error(&self) -> &Self::Error;

    #[inline]
    /// A pretty-printing default `Display` implementation that all instances of `CtxError` can call
    /// to display `Error` information, `Location` information, user-context and `Backtrace`
    /// information.
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        let max_digits = self.context()
                             .len()
                             .saturating_sub(1)
                             .checked_ilog(10)
                             .and_then(|log| usize::try_from(log.saturating_add(1)).ok())
                             .unwrap_or_default();
        let context =
            self.context().enumerate().fold(String::new(), |mut acc, (index, context)| {
                                          (index == 0).and_do(|| {
                                              acc = format!("{MSG_CONTEXT} ({MSG_DISPLAY_ORDER}: \
                                                             {MSG_ERROR_ORIGINATION_SITE} -> \
                                                             {MSG_PROGRAM_ENTRY_POINT}):\n");
                                          });
                                          let line_number = index.saturating_add(1);
                                          let prefix = format!("  {line_number: >max_digits$}: ");
                                          acc.push_str(&(prefix + context + "\n"));
                                          acc
                                      });
        #[expect(clippy::arithmetic_side_effects,
                 reason = "`clippy::arithmetic_side_effects` lint should be `level = deny`.  \
                           String concatenation is sound (modulo OOM error), since string \
                           concatenation is not arithmetic and cannot overflow.")]
        let backtrace =
            (self.backtrace().status() != BacktraceStatus::Disabled).map_or(String::new(), || {
                String::from(MSG_ERROR_BACKTRACE) + "\n" + &self.backtrace().to_string()
            });
        write!(f, "{} {MSG_AT} {}\n{context}{backtrace}", self.error(), self.location())
    }

    /// Provide (a reference to) the `Location` (filename, line number and column number) where this
    /// error was first emitted.
    #[must_use]
    fn location(&self) -> &Location<'_>;

    /// Split this `CtxError` into its key constituent parts: `Location`, user-context
    /// (`Vec<String`>), `Backtrace` and the wrapped inner `Error` itself.
    fn parts<'a>(self) -> (&'a Location<'a>, Vec<String>, Backtrace, Self::Error);
}
