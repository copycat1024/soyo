use std::cell::Cell;
use std::ops::{Deref, DerefMut};

#[derive(Clone)]
pub struct Hot<T: Copy + PartialEq> {
    old: Cell<T>,
    new: T,
}

impl<T: Copy + PartialEq> Hot<T> {
    pub fn new(val: T) -> Self {
        Self {
            old: Cell::new(val),
            new: val,
        }
    }

    pub fn hot(&self) -> bool {
        self.old.get() != self.new
    }
}

impl<T: Copy + PartialEq> Deref for Hot<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        if self.old.get() != self.new {
            self.old.set(self.new);
        }
        &self.new
    }
}

impl<T: Copy + PartialEq> DerefMut for Hot<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.new
    }
}

impl<T: Copy + PartialEq> From<T> for Hot<T> {
    fn from(v: T) -> Hot<T> {
        Hot::new(v)
    }
}
