//! Module defining a `Result` extension trait, which allows the programmer to call `append_ctx()`
//! on a `CoreResult` and receive a `CtxError` with the user-`context` field set.

/// Extension trait designed to add a method to all `core::result::Result<T, E>` where 1. `E` is
/// convertible into a local error type and 2) that error type can be converted to a `CtxError`.
pub trait IResultExt<TDstError, T> {
    /// `append_ctx` makes `CtxError`s feel transparent to the developer, allowing ergonomic
    /// attachment of context with no boilerplate.
    fn append_ctx<TString>(self, s: TString) -> Result<T, TDstError>
        where TString: Into<String>;
}
