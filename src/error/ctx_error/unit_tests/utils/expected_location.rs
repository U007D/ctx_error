use std::{panic::Location, path::Path};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct ExpectedLocation {
    file: String,
    line: u32,
    column: u32,
}

impl ExpectedLocation {
    #[track_caller]
    pub fn new(line: u32, column: u32) -> Self {
        let location = Location::caller();
        let file = {
            let mut tmp_path = Path::new(location.file()).parent()
                                                         .unwrap_or_else(|| Path::new(""))
                                                         .to_str()
                                                         .unwrap_or("")
                                                         .to_string();
            tmp_path.push_str("/utils/functions.rs");
            tmp_path
        };
        Self { file, line, column }
    }

    #[must_use]
    #[track_caller]
    pub fn file(&self) -> &str { &self.file }

    #[must_use]
    #[track_caller]
    pub const fn line(&self) -> u32 { self.line }

    #[must_use]
    #[track_caller]
    pub const fn column(&self) -> u32 { self.column }
}
