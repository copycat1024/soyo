#[derive(Clone, Copy)]
pub enum Tag {
    Event,
    Frame,
    Backend,
    Custom,
}

impl From<Tag> for u8 {
    fn from(tag: Tag) -> Self {
        tag as u8
    }
}
