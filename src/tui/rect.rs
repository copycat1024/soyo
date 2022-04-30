#[derive(Clone, Copy, Default)]
pub struct Rect {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
}

impl Rect {
    pub fn new() -> Self {
        Self {
            x: 0,
            y: 0,
            w: 0,
            h: 0,
        }
    }

    pub fn xywh(x: i32, y: i32, w: i32, h: i32) -> Self {
        Self { x, y, w, h }
    }

    pub fn center(&self, w: i32, h: i32) -> Self {
        Self {
            x: self.x + (self.w - w) / 2,
            y: self.y + (self.h - h) / 2,
            w,
            h,
        }
    }

    pub fn inside(&self, src: &Self) -> bool {
        self.x >= src.x
            && self.y >= src.y
            && self.x + self.w <= src.x + src.w
            && self.y + self.h <= src.y + src.h
    }

    pub fn inter(&self, b: &Self) -> Self {
        use std::cmp::{max, min};

        let x1 = max(self.x, b.x);
        let y1 = max(self.y, b.y);
        let x2 = min(self.x + self.w, b.x + b.w);
        let y2 = min(self.y + self.h, b.y + b.h);

        Self {
            x: x1,
            y: y1,
            w: x2.saturating_sub(x1),
            h: y2.saturating_sub(y1),
        }
    }

    pub fn iter(&self, abs: bool) -> Iter<'_> {
        Iter::new(self, abs)
    }
}

pub struct Iter<'a> {
    src: &'a Rect,
    abs: bool,
    i: i32,
}

impl<'a> Iter<'a> {
    pub fn new(src: &'a Rect, abs: bool) -> Self {
        Self { src, abs, i: 0 }
    }
}

impl<'a> Iterator for Iter<'a> {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.src.w * self.src.h > self.i {
            let x = self.i % self.src.w;
            let y = self.i / self.src.w;
            self.i += 1;
            if self.abs {
                Some((x + self.src.x, y + self.src.y))
            } else {
                Some((x, y))
            }
        } else {
            None
        }
    }
}
