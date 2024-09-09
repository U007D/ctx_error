use std::panic::Location;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub struct ExpectedLocation {
    file: &'static str,
    line: u32,
    column: u32,
}

impl ExpectedLocation {
    #[track_caller]
    pub fn new(line_offset: i32, column: u32) -> Self {
        let location = Location::caller();
        let file = location.file();
        let line = u32::try_from(i64::from(location.line()).checked_add(i64::from(line_offset)).unwrap()).unwrap();

        Self {
            file,
            line,
            column,
        }
    }

    #[must_use]
    #[track_caller]
    pub const fn file(&self) -> &'static str {
        self.file
    }

    #[must_use]
    #[track_caller]
    pub const fn line(&self) -> u32 {
        self.line
    }

    #[must_use]
    #[track_caller]
    pub const fn column(&self) -> u32 {
        self.column
    }
}
