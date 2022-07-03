use std::collections::VecDeque;

pub struct Dispatch<T: Copy> {
    queue: VecDeque<T>,
}

impl<T: Copy> Dispatch<T> {
    pub fn event(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    pub fn dispatch(&mut self, event: T) {
        self.queue.push_back(event);
    }
}

impl<T: Copy> Default for Dispatch<T> {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}
