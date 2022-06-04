use crate::tui::Quad;

pub trait ComposeFn: Fn(Quad, i32) -> (Quad, i32) + 'static {}
impl<F> ComposeFn for F where F: Fn(Quad, i32) -> (Quad, i32) + 'static {}

pub struct Composer {
    func: Box<dyn ComposeFn>,
}

impl Composer {
    pub fn compose(&self, quad: Quad, z: i32) -> (Quad, i32) {
        (self.func)(quad, z)
    }

    pub fn set<F>(&mut self, func: F)
    where
        F: Fn(Quad, i32) -> (Quad, i32) + 'static,
    {
        self.func = Box::new(func);
    }
}

impl Default for Composer {
    fn default() -> Self {
        Self {
            func: Box::new(|quad, z| (quad, z + 1)),
        }
    }
}
