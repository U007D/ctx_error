use crate::shared_const::*;
use thiserror::Error;

pub type TestResult<T, E = TestError> = core::result::Result<T, E>;

#[derive(Debug, Error)]
pub enum TestError {
    #[error("TestError::Io: {}: {:?}: ", msg::ERR_IO, 0)]
    Io(#[from] std::io::Error),
}