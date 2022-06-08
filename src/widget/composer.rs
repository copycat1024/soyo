use super::Layer;

pub trait ComposeFn: Fn(Layer) -> Layer + 'static {}
impl<F> ComposeFn for F where F: Fn(Layer) -> Layer + 'static {}

pub struct Composer {
    func: Box<dyn ComposeFn>,
}

impl Composer {
    pub fn compose(&self, layer: Layer) -> Layer {
        (self.func)(layer)
    }

    pub fn set<F>(&mut self, func: F)
    where
        F: Fn(Layer) -> Layer + 'static,
    {
        self.func = Box::new(func);
    }
}

impl Default for Composer {
    fn default() -> Self {
        Self {
            func: Box::new(|layer| layer.rise_z()),
        }
    }
}
