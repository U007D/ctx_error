#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ExpectedLocation {
    file: &'static str,
    line: u32,
    column: u32,
}

impl ExpectedLocation {
    #[track_caller]
    pub fn new(line_offset: i32, column: u32) -> Self {
        let file = file!();
        let line = u32::try_from(i64::from(line!()).checked_add(i64::from(line_offset)).unwrap()).unwrap();

        Self {
            file,
            line,
            column,
        }
    }

    #[must_use]
    pub const fn file(&self) -> &'static str {
        self.file
    }

    #[must_use]
    pub const fn line(&self) -> u32 {
        self.line
    }

    #[must_use]
    pub const fn column(&self) -> u32 {
        self.column
    }
}
