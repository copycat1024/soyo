use std::{
    cell::{Ref, RefCell, RefMut},
    marker::Unsize,
    ops::CoerceUnsized,
    rc::{Rc, Weak},
};

pub struct SharedPtr<T>
where
    T: 'static + ?Sized,
{
    ptr: Rc<RefCell<T>>,
}

impl<T> SharedPtr<T>
where
    T: 'static,
{
    pub fn new(ptr: T) -> Self {
        Self {
            ptr: Rc::new(RefCell::new(ptr)),
        }
    }
}

impl<T> SharedPtr<T>
where
    T: 'static + ?Sized,
{
    pub fn downgrade(&self) -> WeakPtr<T> {
        WeakPtr {
            ptr: Rc::downgrade(&self.ptr),
        }
    }

    pub fn borrow(&self) -> Ref<'_, T> {
        self.ptr.borrow()
    }

    pub fn borrow_mut(&mut self) -> RefMut<'_, T> {
        self.ptr.borrow_mut()
    }
}

impl<T> Clone for SharedPtr<T>
where
    T: 'static,
{
    fn clone(&self) -> Self {
        Self {
            ptr: self.ptr.clone(),
        }
    }
}

impl<T> Default for SharedPtr<T>
where
    T: 'static + Default,
{
    fn default() -> Self {
        Self::new(T::default())
    }
}

impl<T, U> CoerceUnsized<SharedPtr<U>> for SharedPtr<T>
where
    T: Unsize<U> + ?Sized,
    U: ?Sized,
{
}

pub struct WeakPtr<T>
where
    T: 'static + ?Sized,
{
    ptr: Weak<RefCell<T>>,
}

impl<T> WeakPtr<T>
where
    T: 'static + ?Sized,
{
    pub fn upgrade(&self) -> Option<SharedPtr<T>> {
        self.ptr.upgrade().map(|ptr| SharedPtr { ptr })
    }

    pub fn update<F, R>(&mut self, callback: F) -> Option<R>
    where
        F: FnOnce(&mut T) -> R,
    {
        self.ptr.upgrade().map(|ptr| {
            let item = &mut *ptr.borrow_mut();
            callback(item)
        })
    }
}

impl<T> Default for WeakPtr<T>
where
    T: 'static,
{
    fn default() -> Self {
        Self { ptr: Weak::new() }
    }
}
