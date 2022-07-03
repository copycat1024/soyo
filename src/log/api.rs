use super::{tag, Logger};
use std::{lazy::SyncLazy, sync::Mutex};

pub static DEPOT: SyncLazy<Mutex<String>> = SyncLazy::new(|| Mutex::new(String::new()));
pub static TAG: SyncLazy<Mutex<Vec<bool>>> = SyncLazy::new(|| Mutex::new(Vec::new()));

pub fn log<T: Into<u8>>(tag: T) -> Logger {
    Logger {
        tag: Into::<u8>::into(tag),
    }
}

pub fn debug() -> Logger {
    log(tag::DEBUG)
}

pub fn enable_log<T: Into<u8>>(tag: T) {
    let tag = Into::<u8>::into(tag) as usize;
    if let Ok(mut tag_list) = TAG.lock() {
        if tag_list.len() < tag + 1 {
            tag_list.resize(tag + 1, false);
        }
        tag_list[tag] = true;
    }
}

pub fn flush_log() {
    if let Ok(depot) = DEPOT.lock() {
        println!("{}", depot);
    }
}
