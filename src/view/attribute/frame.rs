use crate::tui::Quad;

#[derive(Clone, Copy)]
pub struct Frame {
    pub x: i32,
    pub y: i32,
    pub w: i32,
    pub h: i32,
    pub z: i32,
}

impl Frame {
    pub fn screen(w: i32, h: i32) -> Self {
        Self {
            x: 0,
            y: 0,
            w,
            h,
            z: 0,
        }
    }

    pub fn rise_z(self) -> Self {
        self.set_z(self.z + 1)
    }

    pub fn center(self, w: i32, h: i32) -> Self {
        Self {
            x: self.x + (self.w - w) / 2,
            y: self.y + (self.h - h) / 2,
            w,
            h,
            z: self.z,
        }
    }
}

impl Frame {
    pub fn set_x(self, x: i32) -> Self {
        Self {
            x,
            y: self.y,
            w: self.w,
            h: self.h,
            z: self.z,
        }
    }

    pub fn set_y(self, y: i32) -> Self {
        Self {
            x: self.x,
            y,
            w: self.w,
            h: self.h,
            z: self.z,
        }
    }

    pub fn set_w(self, w: i32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            w,
            h: self.h,
            z: self.z,
        }
    }

    pub fn set_h(self, h: i32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            w: self.w,
            h,
            z: self.z,
        }
    }

    pub fn set_z(self, z: i32) -> Self {
        Self {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
            z,
        }
    }

    pub fn offset(self, dx: i32, dy: i32, dw: i32, dh: i32, dz: i32) -> Self {
        Self {
            x: self.x + dx,
            y: self.y + dy,
            w: self.w + dw,
            h: self.h + dh,
            z: self.z + dz,
        }
    }

    pub fn quad(&self) -> Quad {
        Quad {
            x: self.x,
            y: self.y,
            w: self.w,
            h: self.h,
        }
    }

    pub fn z_value(&self) -> i32 {
        self.z
    }
}
