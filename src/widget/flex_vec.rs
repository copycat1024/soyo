use std::fmt::Arguments;

pub struct FlexVec<T> {
    data: Vec<T>,
    default: T,
}

impl<T> FlexVec<T> {
    pub fn new(default: T) -> Self {
        Self {
            data: Vec::new(),
            default,
        }
    }

    pub fn len(&self) -> i32 {
        self.data.len() as i32
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
}

impl<T> std::ops::Index<i32> for FlexVec<T> {
    type Output = T;

    fn index(&self, i: i32) -> &Self::Output {
        self.data.get(i as usize).unwrap_or(&self.default)
    }
}

impl<T> Extend<T> for FlexVec<T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        self.data.extend(iter)
    }
}

impl FlexVec<char> {
    pub fn write_fmt(&mut self, fmt: Arguments<'_>) {
        self.data = format!("{}", fmt).chars().collect();
    }
}
