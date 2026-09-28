//! Conditional compilation of context storage and backtrace support.
//!
//! These definitions select behavior using this crate's `std` feature. The exported
//! `ctx_error!` macro refers to them so its behavior does not depend on the caller's features.

#[cfg(not(feature = "std"))]
pub use core::convert::Infallible as Backtrace;
#[cfg(feature = "std")]
pub use std::backtrace::Backtrace;

/// Generate `core::error::Error::provide` for a contextual error with `std` enabled.
///
/// Used by [`ctx_error!`](crate::ctx_error) inside its generated `Error` implementation.
/// The method forwards requests to the wrapped `error` field, then provides the optional
/// `backtrace` field. Selection uses this crate's features, not the caller's.
#[cfg(feature = "std")]
#[macro_export]
macro_rules! impl_error_provide {
    () => {
        fn provide<'a>(&'a self, request: &mut ::core::error::Request<'a>) {
            ::core::error::Error::provide(&self.error, request);
            if let ::core::option::Option::Some(backtrace) = self.backtrace.as_ref() {
                request.provide_ref::<$crate::conditional_compilation::Backtrace>(backtrace);
            }
        }
    };
}

/// Emit no `Error::provide` method when backtraces are unavailable.
///
/// Used by [`ctx_error!`](crate::ctx_error) so the allocation-free configuration does not
/// require the unstable error-provider API.
#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! impl_error_provide {
    () => {};
}

/// Owned context storage transferred between contextual errors.
#[cfg(feature = "std")]
pub type Context = Option<Vec<&'static str>>;
/// Allocation-free storage for zero or one context.
#[cfg(not(feature = "std"))]
pub type Context = Option<&'static str>;

/// Append context, or retain the first context when allocation is disabled.
#[inline]
pub fn append_context(context: &mut Context, s: &'static str) {
    #[cfg(feature = "std")]
    context.get_or_insert_with(Vec::new).push(s);
    #[cfg(not(feature = "std"))]
    context.get_or_insert(s);
}

/// Capture a backtrace when supported; otherwise return `None`.
#[inline]
#[must_use]
#[allow(clippy::missing_const_for_fn, reason = "Backtrace capture is not const with std.")]
pub fn capture_backtrace() -> Option<Backtrace> {
    #[cfg(feature = "std")]
    {
        Some(Backtrace::capture())
    }
    #[cfg(not(feature = "std"))]
    {
        None
    }
}

/// Borrow the attached contexts without allocating.
#[inline]
#[must_use]
#[allow(clippy::missing_const_for_fn, reason = "The std implementation is not const.")]
pub fn context_slice(context: &Context) -> &[&'static str] {
    #[cfg(feature = "std")]
    {
        context.as_deref().unwrap_or(&[])
    }
    #[cfg(not(feature = "std"))]
    {
        context.as_slice()
    }
}
