use super::{Compose, ComposeHost};
use crate::{
    util::SharedPtr,
    view::{Frame, Host},
};

pub struct Composer<T: Compose> {
    pub ptr: SharedPtr<ComposeHost<T>>,
}

impl<T: Compose> Composer<T> {
    pub fn new(composer: T) -> Self {
        Self {
            ptr: SharedPtr::new(ComposeHost::new(composer)),
        }
    }

    pub fn layout(&mut self, attr: Frame) -> Frame {
        self.ptr.borrow_mut().layout(attr)
    }

    pub fn get<F, R>(&self, callback: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        callback(&self.ptr.borrow().widget)
    }

    pub fn set<F, R>(&mut self, f: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        f(&mut self.ptr.borrow_mut().widget)
    }
}

impl<T: Compose + Default> Default for Composer<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
