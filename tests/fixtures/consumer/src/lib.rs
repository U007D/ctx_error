//! Exercise the exported macro in a no_std consumer with a renamed dependency.
#![no_std]
#![deny(warnings)]
#![cfg_attr(feature = "use-std", feature(error_generic_member_access))]

use errors::{ICtxError, IResultExt};

#[derive(Debug)]
pub struct Error(device::Error);

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Display::fmt(&self.0, f)
    }
}
impl core::error::Error for Error {}
impl From<device::Error> for Error {
    fn from(error: device::Error) -> Self { Self(error) }
}
errors::ctx_error!(Error);

mod device {
    #[derive(Debug)]
    pub struct Error;

    impl core::fmt::Display for Error {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            f.write_str("device failed")
        }
    }
    impl core::error::Error for Error {}
    errors::ctx_error!(Error, super::CtxError);
}

pub fn without_context() -> CtxError { device::CtxError::from(device::Error).into() }

pub fn with_context() -> Result<(), CtxError> {
    let result: Result<(), device::CtxError> = Err(device::Error).append_ctx("first");
    result.append_ctx("second")?;
    Ok(())
}

pub fn render(destination: &mut impl core::fmt::Write) -> core::fmt::Result {
    match with_context() {
        Err(error) => write!(destination, "{error}"),
        Ok(()) => Ok(()),
    }
}

pub fn context_count(error: &CtxError) -> usize { error.context().len() }

#[cfg(test)]
mod tests {
    #[test]
    fn dependency_features_select_context_capacity() {
        assert_eq!(super::context_count(&super::without_context()), 0);
        let error = super::with_context().unwrap_err();
        let expected = if cfg!(feature = "use-std") { 2 } else { 1 };
        assert_eq!(super::context_count(&error), expected);
    }
}
