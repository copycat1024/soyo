#[derive(Default)]
pub struct Flow {
    pub stop: bool,
    pub draw: bool,
    pub clear: bool,
    pub code: usize,
}

impl Flow {
    pub fn exit(&mut self, code: usize) {
        self.stop = true;
        self.code = code;
    }
}
