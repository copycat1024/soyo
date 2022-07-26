use super::{Render, RenderHost};
use crate::{
    util::SharedPtr,
    view::{Frame, Host},
};

pub struct Renderer<T: Render> {
    pub ptr: SharedPtr<RenderHost<T>>,
}

impl<T: Render> Renderer<T> {
    pub fn new(composer: T) -> Self {
        Self {
            ptr: SharedPtr::new(RenderHost::new(composer)),
        }
    }

    pub fn layout(&mut self, frame: Frame) -> Frame {
        self.ptr.borrow_mut().layout(frame)
    }

    pub fn get<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&T) -> R,
    {
        f(&self.ptr.borrow().widget)
    }

    pub fn set<F>(&mut self, f: F)
    where
        F: FnOnce(&mut T),
    {
        f(&mut self.ptr.borrow_mut().widget)
    }
}

impl<T: Render + Default> Default for Renderer<T> {
    fn default() -> Self {
        Self::new(T::default())
    }
}
