use std::cell::Cell;
use std::ops::{Deref, DerefMut};

pub struct Hot<T> {
    hot: Cell<bool>,
    val: T,
}

impl<T> Hot<T> {
    pub fn new(val: T) -> Self {
        Self {
            hot: Cell::new(true),
            val,
        }
    }

    pub fn check(&self) -> bool {
        let hot = self.hot.get();
        self.hot.set(false);
        hot
    }

    pub fn get_ref(&mut self)->HotRef<T>{
        HotRef(self)
    }
}

impl<T> Hot<T>
where
    T: PartialEq,
{
    pub fn update(&mut self, src: T) {
        if self.val != src {
            self.val = src;
            self.hot.set(true);
        }
    }
}

impl<T> Deref for Hot<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.val
    }
}

impl<T> DerefMut for Hot<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.hot.set(true);
        &mut self.val
    }
}

impl<T: Copy> From<T> for Hot<T> {
    fn from(v: T) -> Hot<T> {
        Hot::new(v)
    }
}

impl<T: Clone> Clone for Hot<T> {
    fn clone(&self) -> Self {
        Hot::new(self.val.clone())
    }
}

pub struct HotRef<'a, T>(&'a mut Hot<T>);

impl<'a, T> Deref for HotRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.0.val
    }
}

impl<'a, T> DerefMut for HotRef<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.hot.set(true);
        &mut self.0.val
    }
}
