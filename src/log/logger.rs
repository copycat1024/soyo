use super::api::{DEPOT, TAG};
use std::fmt::Arguments;

#[derive(Default)]
pub struct Logger {
    pub tag: u8,
}

impl Logger {
    pub fn write_fmt(&mut self, fmt: Arguments<'_>) {
        use std::fmt::Write;
        if self.is_active() {
            if let Ok(mut depot) = DEPOT.lock() {
                write!(depot, "{}", fmt).ok();
            }
        }
    }

    fn is_active(&self) -> bool {
        let tag = self.tag as usize;
        if let Ok(tag_list) = TAG.lock() {
            if tag_list.len() < tag + 1 {
                false
            } else {
                tag_list[tag]
            }
        } else {
            false
        }
    }
}
