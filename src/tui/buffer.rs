use crate::tui::Quad;

pub struct Buffer<T: Clone> {
    rect: Quad,
    data: Vec<T>,
}

impl<T: Clone> Buffer<T> {
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

    fn index(&self, x: i32, y: i32) -> Option<usize> {
        let Self { rect, .. } = self;
        let index = (y * rect.w + x) as usize;

        if x >= 0 && y >= 0 && x < rect.w && y < rect.h {
            Some(index)
        } else {
            None
        }
    }

    pub fn iter(&self, abs: bool) -> Iter<'_, T> {
        let Self { data, rect } = self;
        Iter(data.iter().zip(rect.iter(abs)))
    }

    pub fn iter_mut(&mut self, abs: bool) -> IterMut<'_, T> {
        let Self { data, rect } = self;
        IterMut(data.iter_mut().zip(rect.iter(abs)))
    }

    pub fn rect(&self) -> &Quad {
        &self.rect
    }

    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn get_mut(&mut self, x: i32, y: i32) -> Option<&mut T> {
        self.index(x, y).map(|index| &mut self.data[index])
    }
}

impl<T: Clone> Default for Buffer<T> {
    fn default() -> Self {
        Self {
            rect: Quad::new(),
            data: Vec::new(),
        }
    }
}

pub struct Iter<'a, T: Clone>(std::iter::Zip<std::slice::Iter<'a, T>, super::quad::Iter<'a>>);

impl<'a, T: Clone> Iterator for Iter<'a, T> {
    type Item = (&'a T, i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(c, (x, y))| (c, x, y))
    }
}

pub struct IterMut<'a, T: Clone>(std::iter::Zip<std::slice::IterMut<'a, T>, super::quad::Iter<'a>>);

impl<'a, T: Clone> Iterator for IterMut<'a, T> {
    type Item = (&'a mut T, i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(c, (x, y))| (c, x, y))
    }
}
