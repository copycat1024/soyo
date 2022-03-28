use super::Rect;
use std::ops::{Index, IndexMut};

pub struct Buffer<T: Clone> {
    rect: Rect,
    data: Vec<T>,
}

impl<T: Clone> Buffer<T> {
    pub fn new() -> Self {
        Self {
            rect: Rect::new(),
            data: Vec::new(),
        }
    }

    pub fn resize(&mut self, w: i32, h: i32, val: T) -> bool {
        let Self { data, rect } = self;

        if rect.h != h || rect.w != w {
            rect.w = w;
            rect.h = h;
            *data = vec![val; (rect.w * rect.h) as usize];
            true
        } else {
            false
        }
    }

    fn index(&self, (x, y): (i32, i32)) -> usize {
        let Self { rect, .. } = self;

        assert!(x >= 0);
        assert!(y >= 0);
        assert!(x < rect.w);
        assert!(y < rect.h);

        (y * rect.w + x) as usize
    }

    pub fn iter(&self, abs: bool) -> Iter<'_, T> {
        let Self { data, rect } = self;
        Iter(data.iter().zip(rect.iter(abs)))
    }

    pub fn iter_mut(&mut self, abs: bool) -> IterMut<'_, T> {
        let Self { data, rect } = self;
        IterMut(data.iter_mut().zip(rect.iter(abs)))
    }

    pub fn rect(&self) -> &Rect {
        &self.rect
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }
}

impl<T: Clone> Index<(i32, i32)> for Buffer<T> {
    type Output = T;

    fn index(&self, coor: (i32, i32)) -> &T {
        let Self { data, rect } = self;
        &data[self.index(coor)]
    }
}

impl<T: Clone> IndexMut<(i32, i32)> for Buffer<T> {
    fn index_mut(&mut self, coor: (i32, i32)) -> &mut T {
        let i = self.index(coor);
        let Self { data, rect } = self;
        &mut data[i]
    }
}

pub struct Iter<'a, T: Clone>(std::iter::Zip<std::slice::Iter<'a, T>, super::rect::Iter<'a>>);

impl<'a, T: Clone> Iterator for Iter<'a, T> {
    type Item = (&'a T, i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(c, (x, y))| (c, x, y))
    }
}

pub struct IterMut<'a, T: Clone>(std::iter::Zip<std::slice::IterMut<'a, T>, super::rect::Iter<'a>>);

impl<'a, T: Clone> Iterator for IterMut<'a, T> {
    type Item = (&'a mut T, i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(c, (x, y))| (c, x, y))
    }
}
