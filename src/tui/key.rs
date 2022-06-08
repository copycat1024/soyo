use std::fmt::{Debug, Error, Formatter};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Key(pub char);

impl Key {
    pub const NULL: Self = Self('\u{E000}');
    pub const ENTER: Self = Self('\u{E001}');
    pub const ESC: Self = Self('\u{E002}');

    pub const UP: Self = Self('\u{E003}');
    pub const DOWN: Self = Self('\u{E004}');
    pub const LEFT: Self = Self('\u{E005}');
    pub const RIGHT: Self = Self('\u{E006}');
}

impl Debug for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if *self == Self::ENTER {
            f.write_str("ENTER")
        } else if *self == Self::ESC {
            f.write_str("ESC")
        } else if *self == Self::UP {
            f.write_str("UP")
        } else if *self == Self::DOWN {
            f.write_str("DOWN")
        } else if *self == Self::LEFT {
            f.write_str("LEFT")
        } else if *self == Self::RIGHT {
            f.write_str("RIGHT")
        } else {
            f.write_fmt(format_args!("'{}'", self.0))
        }
    }
}
