use std::fmt::Arguments;

#[derive(Default)]
pub struct Text {
    pub data: Vec<char>,
}

const FILL_CHAR: char = ' ';

impl std::ops::Index<i32> for Text {
    type Output = char;

    fn index(&self, i: i32) -> &Self::Output {
        self.data.get(i as usize).unwrap_or(&FILL_CHAR)
    }
}

impl Text {
    pub fn write_fmt(&mut self, fmt: Arguments<'_>) {
        self.data = format!("{}", fmt).chars().collect();
    }
}
