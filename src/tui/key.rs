use std::fmt::{Debug, Error, Formatter};

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Key(pub char);

impl Key {
    pub const NULL: Self = Self('\u{E000}');
    pub const ENTER: Self = Self('\u{E001}');
    pub const ESC: Self = Self('\u{E002}');
}

impl Debug for Key {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if *self == Self::ENTER {
            f.write_str("ENTER")
        } else if *self == Self::ESC {
            f.write_str("ESC")
        } else {
            f.write_fmt(format_args!("'{}'", self.0))
        }
    }
}
