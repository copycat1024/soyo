use super::Frame;

#[derive(Clone, Copy)]
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
    pub fn from_size(w: i32, h: i32) -> Self {
        Self {
            frame: Frame::screen(w, h),
        }
    }
}
