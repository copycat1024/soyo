use std::{fmt::Arguments, lazy::SyncLazy, sync::Mutex};

pub static DEPOT: SyncLazy<Mutex<String>> = SyncLazy::new(|| Mutex::new(String::new()));
pub static TAG: SyncLazy<Mutex<Vec<bool>>> = SyncLazy::new(|| Mutex::new(Vec::new()));

pub fn log<T: Into<u8>>(tag: T) -> Logger {
    Logger {
        tag: Into::<u8>::into(tag),
    }
}

pub fn activate_logger<T: Into<u8>>(tag: T) {
    let tag = Into::<u8>::into(tag) as usize;
    if let Ok(mut tag_list) = TAG.lock() {
        if tag_list.len() < tag + 1 {
            tag_list.resize(tag + 1, false);
        }
        tag_list[tag] = true;
    }
}

pub fn flush_logger() {
    if let Ok(depot) = DEPOT.lock() {
        println!("{}", depot);
    }
}

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
