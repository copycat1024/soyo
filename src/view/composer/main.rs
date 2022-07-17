use super::{Compose, ComposeHost};
use crate::util::SharedPtr;

pub struct Composer<T: Compose> {
    pub ptr: SharedPtr<ComposeHost<T>>,
}

impl<T: Compose> Composer<T> {
    pub fn new(composer: T) -> Self {
        Self {
            ptr: SharedPtr::new(ComposeHost::new(composer)),
        }
    }

    pub fn call_mut<F>(&mut self, f: F)
    where
        F: Fn(&mut T),
    {
        f(&mut self.ptr.borrow_mut().widget)
    }
}
