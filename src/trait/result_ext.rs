use crate::r#trait::ICtxError;

pub trait IResultExt<E, T> {
    #[must_use]
    fn append_ctx<TString>(self, s: TString) -> Self
        where TString: Into<String>;
}
