mod api;
mod logger;
mod tag;

pub use api::{debug, enable_log, flush_log, log};
pub use logger::Logger;
pub use tag::Tag;
