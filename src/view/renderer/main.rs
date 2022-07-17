use super::{Render, RenderHost};
use crate::{util::SharedPtr, view::Attribute};

pub struct Renderer<T: Render> {
    pub ptr: SharedPtr<RenderHost<T>>,
}

impl<T: Render> Renderer<T> {
    pub fn new(composer: T) -> Self {
        Self {
            ptr: SharedPtr::new(RenderHost::new(composer)),
        }
    }

    pub fn compose<F>(&mut self, callback: F)
    where
        F: Fn(&mut Attribute),
    {
        callback(&mut self.ptr.borrow_mut().attr)
    }

    pub fn view<F, R>(&mut self, callback: F) -> R
    where
        F: FnOnce(&mut T) -> R,
    {
        callback(&mut self.ptr.borrow_mut().widget)
    }
}

impl<T: Render + Default> Default for Renderer<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
