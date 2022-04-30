use crate::logger::Depot;
use std::{
    cell::RefCell,
    io::{Result, Write},
    rc::Weak,
};

pub struct Client {
    pub(super) depot: Weak<RefCell<Depot>>,
    pub(super) tag: u8,
}

impl Write for Client {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if let Some(depot) = &mut self.depot.upgrade() {
            let mut depot = depot.borrow_mut();
            if depot.enabled(self.tag) {
                depot.data.write(buf)
            } else {
                Ok(0)
            }
        } else {
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}

impl Default for Client {
    fn default() -> Self {
        Self {
            depot: Weak::new(),
            tag: 0,
        }
    }
}
