use core::{error::Error,
           fmt::{Formatter, Result as FmtResult},
           slice};
use std::backtrace::{Backtrace, BacktraceStatus};

use bool_ext::BoolExt;

#[allow(clippy::wildcard_imports)]
use crate::shared_const::msg::*;

pub trait ICtxError: Error {
    type Error: Error;

    #[must_use]
    #[track_caller]
    fn append_ctx<TString>(self, s: TString) -> Self
        where TString: Into<String>;

    #[must_use]
    fn location(&self) -> &core::panic::Location<'_>;

    fn backtrace(&self) -> &Backtrace;

    fn context(&self) -> slice::Iter<'_, String>;

    #[must_use]
    fn error(&self) -> &Self::Error;

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
                                          #[expect(clippy::arithmetic_side_effects,
                                                   reason = "`clippy::arithmetic_side_effects` \
                                                             lint should be `level = deny`.  \
                                                             String concatenation is sound \
                                                             (modulo OOM error), since string \
                                                             concatenation is not arithmetic \
                                                             and cannot overflow.")]
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

    fn parts<'a>(self) -> (&'a core::panic::Location<'a>, Vec<String>, Backtrace, Self::Error);
}
