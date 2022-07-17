use super::Frame;

pub struct Attribute {
    pub frame: Frame,
}

impl Default for Attribute {
    fn default() -> Self {
        Self {
            frame: Frame::screen(0, 0),
        }
    }
}

impl Attribute {
    pub fn resize(&mut self, w: i32, h: i32) {
        self.frame = self.frame.set_w(w).set_h(h);
    }
}
