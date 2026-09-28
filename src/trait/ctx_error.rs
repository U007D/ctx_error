//! Interface shared by contextual errors.

use core::{error::Error as CoreError,
           fmt::{Formatter, Result as FmtResult},
           panic::Location,
           slice};

use crate::conditional_compilation::{Backtrace, Context};
#[allow(clippy::wildcard_imports,
            reason = "Message constants are maintained together for localization.")]
use crate::shared_const::msg::*;

/// The interface implemented by every generated `CtxError`.
pub trait ICtxError: CoreError {
    /// The underlying error type.
    type Error: CoreError;

    /// Attach a static context string describing why an operation was attempted.
    ///
    /// With `std`, every call appends a context. Without `std`, only the first is retained;
    /// subsequent calls leave it unchanged. An empty string is an attached context, not absence.
    #[must_use]
    #[track_caller]
    fn append_ctx(self, s: &'static str) -> Self;

    /// Borrow the backtrace captured at error creation.
    ///
    /// With `std`, capture is controlled by the standard backtrace environment variables.
    /// Without `std`, the value is always `None`, with `Infallible` as its uninhabited inner type.
    fn backtrace(&self) -> &Option<Backtrace>;

    /// Iterate over references to the attached static strings, in attachment order.
    ///
    /// There may be no contexts. Without `std`, there is at most one.
    fn context(&self) -> slice::Iter<'_, &'static str>;

    /// Borrow the underlying error.
    #[must_use]
    fn error(&self) -> &Self::Error;

    /// Write the error, original location, contexts and available backtrace to the formatter.
    ///
    /// Formatting streams directly to the destination without allocating an intermediate string.
    #[inline]
    fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
        writeln!(f, "{} {MSG_AT} {}", self.error(), self.location())?;
        let contexts = self.context();
        if contexts.len() != 0 {
            let max_digits = usize::try_from(contexts.len()
                                                     .checked_ilog10()
                                                     .unwrap_or_default()
                                                     .saturating_add(1)).unwrap_or(1);
            writeln!(f,
                     "{MSG_CONTEXT} ({MSG_DISPLAY_ORDER}: {MSG_ERROR_ORIGINATION_SITE} -> \
                      {MSG_PROGRAM_ENTRY_POINT}):")?;
            for (index, context) in contexts.enumerate() {
                let line_number = index.saturating_add(1);
                writeln!(f, "  {line_number:>max_digits$}: {context}")?;
            }
        }
        #[cfg(feature = "std")]
        if let Some(backtrace) = self.backtrace().as_ref() {
            if backtrace.status() != std::backtrace::BacktraceStatus::Disabled {
                write!(f, "{MSG_ERROR_BACKTRACE}\n{backtrace}")?;
            }
        }
        Ok(())
    }

    /// Borrow the location where the underlying error first became contextual.
    #[must_use]
    fn location(&self) -> &Location<'_>;

    /// Consume the error and transfer its underlying error, location, context and backtrace.
    ///
    /// Context storage is `Option<Vec<&'static str>>` with `std`, and `Option<&'static str>`
    /// without it. Transferring these parts does not clone or reallocate the context storage.
    fn parts<'a>(self) -> (Self::Error, &'a Location<'a>, Context, Option<Backtrace>);
}
