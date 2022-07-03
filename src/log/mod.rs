mod api;
mod logger;
pub mod tag;

pub use api::{debug, enable_log, flush_log, log};
pub use logger::Logger;
