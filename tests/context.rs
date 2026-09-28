//! Behavioral tests shared by the std and allocation-free configurations.
#![cfg(test)]
#![allow(clippy::arbitrary_source_item_ordering,
         reason = "Keep the test domain together and group tests by behavior.")]
#![cfg_attr(feature = "std", feature(error_generic_member_access))]
#![allow(clippy::unwrap_used,
         clippy::unwrap_in_result,
         clippy::panic,
         clippy::missing_docs_in_private_items,
         clippy::missing_inline_in_public_items,
         reason = "Tests use assertions and unwrap to report failures.")]

use core::{error::Error as CoreError, fmt, fmt::Write as _};

use ctx_error::{ICtxError, IResultExt};

mod domain {
    #[derive(Debug)]
    pub enum Error {
        Device(device::Error),
    }

    impl core::fmt::Display for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            match *self {
                Self::Device(ref error) => core::fmt::Display::fmt(error, f),
            }
        }
    }

    impl core::error::Error for Error {
        fn source(&self) -> Option<&(dyn core::error::Error + 'static)> {
            match *self {
                Self::Device(ref error) => Some(error),
            }
        }
    }

    impl From<device::Error> for Error {
        fn from(error: device::Error) -> Self { Self::Device(error) }
    }

    ctx_error::ctx_error!(Error);

    pub mod device {
        #[derive(Debug, PartialEq, Eq)]
        pub struct Error;

        impl core::fmt::Display for Error {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.write_str("device failed")
            }
        }

        impl core::error::Error for Error {}

        // Exercise conversion from a different underlying error type into the local error.
        impl From<core::fmt::Error> for Error {
            fn from(_: core::fmt::Error) -> Self { Self }
        }

        ctx_error::ctx_error!(Error, super::CtxError);
    }
}

use domain::device::{CtxError, Error};

type CallSite = (&'static str, u32, u32);

fn fail_without_context(origin: &mut Option<CallSite>) -> Result<(), CtxError> {
    // Keep this assignment immediately before the conversion; column 5 is the Err expression.
    *origin = Some((file!(), line!().saturating_add(1), 5));
    Err::<(), _>(fmt::Error)?;
    Ok(())
}

fn fail_with_context(origin: &mut Option<CallSite>) -> Result<(), CtxError> {
    // Keep this assignment immediately before append_ctx; column 30 is the method name.
    *origin = Some((file!(), line!().saturating_add(1), 30));
    Err::<(), _>(fmt::Error).append_ctx("first")
}

fn propagate_to_parent(result: Result<(), CtxError>) -> Result<(), domain::CtxError> {
    result?;
    Ok(())
}

fn assert_call_site(error: &impl ICtxError, expected: Option<CallSite>) {
    let (file, line, column) = expected.unwrap();
    assert_eq!(error.location().file(), file);
    assert_eq!(error.location().line(), line);
    assert_eq!(error.location().column(), column);
}

#[test]
fn one_hop_conversion_preserves_error_and_location_without_context() {
    let mut origin = None;
    let error = fail_without_context(&mut origin).unwrap_err();
    assert_call_site(&error, origin);
    assert_eq!(error.context().len(), 0);
    assert!(CoreError::source(&error).unwrap().is::<Error>());
    let (inner, location, contexts, _) = error.parts();
    assert_eq!(inner, Error);
    assert_eq!((location.file(), location.line(), location.column()), origin.unwrap());
    assert!(contexts.is_none());
}

#[test]
fn error_append_retains_origin_and_stores_context() {
    let error = CtxError::from(Error);
    let location = (error.location().line(), error.location().column());
    let contextual = error.append_ctx("first").append_ctx("second");
    assert_eq!((contextual.location().line(), contextual.location().column()), location);
    #[cfg(feature = "std")]
    assert!(contextual.context().copied().eq(["first", "second"]));
    #[cfg(not(feature = "std"))]
    assert!(contextual.context().copied().eq(["first"]));
}

#[test]
fn result_append_preserves_context_and_original_call_site() {
    let mut origin = None;
    let error = fail_with_context(&mut origin).append_ctx("second").unwrap_err();
    assert_call_site(&error, origin);
    assert_eq!(*error.error(), Error);
    #[cfg(feature = "std")]
    assert!(error.context().copied().eq(["first", "second"]));
    #[cfg(not(feature = "std"))]
    assert!(error.context().copied().eq(["first"]));
}

#[test]
fn result_append_can_attach_the_first_context_to_an_existing_error() {
    let error = Err::<(), _>(CtxError::from(Error)).append_ctx("first").unwrap_err();
    assert!(error.context().copied().eq(["first"]));
}

#[test]
fn successful_results_preserve_their_value() {
    let result: Result<u8, CtxError> = Ok::<_, Error>(42).append_ctx("unused");
    assert_eq!(result.unwrap(), 42);
    assert_eq!(Ok::<u8, CtxError>(42).append_ctx("unused").unwrap(), 42);
}

#[test]
fn two_hop_conversion_preserves_error_and_location_without_context() {
    let mut origin = None;
    let parent = propagate_to_parent(fail_without_context(&mut origin)).unwrap_err();
    assert_call_site(&parent, origin);
    assert_eq!(parent.context().len(), 0);
    assert!(matches!(parent.error(), domain::Error::Device(Error)));
    assert!(parent.parts().2.is_none());
}

#[test]
fn two_hop_conversion_preserves_context_storage_error_and_location() {
    let mut origin = None;
    let child = fail_with_context(&mut origin).append_ctx("second").unwrap_err();
    #[cfg(feature = "std")]
    let storage = child.context().as_slice().as_ptr();
    let parent = propagate_to_parent(Err(child)).unwrap_err();
    #[cfg(feature = "std")]
    assert_eq!(parent.context().as_slice().as_ptr(), storage);
    assert_call_site(&parent, origin);
    assert!(matches!(parent.error(), domain::Error::Device(Error)));
    #[cfg(feature = "std")]
    assert!(parent.context().copied().eq(["first", "second"]));
    #[cfg(not(feature = "std"))]
    assert!(parent.context().copied().eq(["first"]));
}

#[test]
fn empty_string_counts_as_an_attached_context() {
    let error = CtxError::from(Error).append_ctx("").append_ctx("second");
    #[cfg(feature = "std")]
    assert!(error.context().copied().eq(["", "second"]));
    #[cfg(not(feature = "std"))]
    assert!(error.context().copied().eq([""]));
}

#[test]
fn display_omits_context_header_when_none_is_attached() {
    let error = CtxError::from(Error);
    let displayed = error.to_string();
    assert!(displayed.starts_with(&format!("device failed at {}\n", error.location())));
    assert!(!displayed.contains("Context ("));
}

#[test]
fn display_lists_the_stored_contexts_in_order() {
    let error = CtxError::from(Error).append_ctx("first").append_ctx("second");
    let displayed = error.to_string();
    let header = "Context (Display order: error origination site -> program entry point):\n";
    assert!(displayed.contains(header));
    #[cfg(feature = "std")]
    assert!(displayed.contains("  1: first\n  2: second\n"));
    #[cfg(not(feature = "std"))]
    assert_eq!(displayed, format!("device failed at {}\n{header}  1: first\n", error.location()));
}

#[test]
fn display_propagates_destination_failure() {
    struct RejectWrites;
    impl fmt::Write for RejectWrites {
        fn write_str(&mut self, _: &str) -> fmt::Result { Err(fmt::Error) }
    }
    assert!(write!(RejectWrites, "{}", CtxError::from(Error).append_ctx("first")).is_err());
}

#[cfg(feature = "std")]
#[test]
fn display_aligns_context_numbers_at_ten_entries() {
    let indices: core::ops::Range<u8> = 0..10;
    let error = indices.fold(CtxError::from(Error), |error, _| error.append_ctx("context"));
    let displayed = error.to_string();
    assert!(displayed.contains("   1: context\n"));
    assert!(displayed.contains("  10: context\n"));
}

#[cfg(feature = "std")]
#[test]
fn backtrace_is_available_through_the_error_provider() {
    let error = CtxError::from(Error);
    let provided = core::error::request_ref::<std::backtrace::Backtrace>(&error).unwrap();
    assert!(core::ptr::eq(provided, error.backtrace().as_ref().unwrap()));
    assert!(error.parts().3.is_some());
}

#[cfg(not(feature = "std"))]
#[test]
fn no_std_errors_have_no_backtrace() {
    let error = CtxError::from(Error).append_ctx("first");
    assert!(error.backtrace().is_none());
    assert!(error.parts().3.is_none());
}
