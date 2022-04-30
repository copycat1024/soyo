pub(super) struct Depot {
    pub(super) data: Vec<u8>,
    pub(super) flag: Vec<bool>,
}

impl Depot {
    pub fn enable(&mut self, name: u8) {
        let n = name as usize;
        if self.flag.len() < n {
            self.flag.resize(n, false);
        }
        self.flag[n] = true;
    }

    pub fn enabled(&self, name: u8) -> bool {
        let n = name as usize;
        if self.flag.len() < n {
            false
        } else {
            self.flag[n]
        }
    }
}
