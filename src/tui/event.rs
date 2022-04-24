use crate::tui::Key;
use std::{
    fmt::{Debug, Error, Formatter},
    time::Duration,
};

#[derive(Clone, Copy)]
pub enum Event {
    Update { delta: Duration },
    Resize { w: i32, h: i32 },
    Key { key: Key },
}

impl Debug for Event {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        match self {
            Self::Update { delta } => f.debug_struct("Update").field("delta", delta).finish(),
            Self::Resize { w, h } => f
                .debug_struct("Resize")
                .field("w", w)
                .field("h", h)
                .finish(),
            Self::Key { key } => f.debug_struct("Key").field("key", key).finish(),
        }
    }
}
