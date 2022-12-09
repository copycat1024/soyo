use std::collections::VecDeque;

pub struct Dispatch<T> {
    queue: VecDeque<T>,
}

impl<T> Dispatch<T> {
    pub fn event(&mut self) -> Option<T> {
        self.queue.pop_front()
    }

    pub fn dispatch(&mut self, event: T) {
        self.queue.push_back(event);
    }
}

impl<T> Default for Dispatch<T> {
    fn default() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }
}
