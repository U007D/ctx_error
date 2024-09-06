pub mod msg;

pub const EXPECTED_ARG_COUNT: usize = 2;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum CliArg {
    NoProgramNameArg,
    HasProgramNameArg,
}
