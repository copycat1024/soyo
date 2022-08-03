#![feature(once_cell)]
#![feature(const_mut_refs)]
#![feature(coerce_unsized)]
#![feature(unsize)]
#![feature(let_chains)]
#![warn(clippy::all)]

pub mod log;
pub mod mvc;
pub mod tui;
pub mod util;
pub mod view;
pub mod widget;

pub use log::debug;
