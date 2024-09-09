use core::error::Error as CoreError;

pub trait IResultExt<T, E, TCtxError>
    where E: CoreError + Into<TCtxError>, {
    fn append_ctx<TString>(self, s: TString) -> Result<T, TCtxError>
        where TString: Into<String>;
}
