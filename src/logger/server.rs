use crate::logger::{Client, Depot};
use std::{
    cell::{Ref, RefCell},
    rc::Rc,
};

pub struct Server {
    depot: Rc<RefCell<Depot>>,
}

impl Server {
    pub fn client<T: Into<u8>>(&self, tag: T) -> Client {
        Client {
            tag: tag.into(),
            depot: Rc::downgrade(&self.depot),
        }
    }

    pub fn enable<T: Into<u8>>(&mut self, tag: T) {
        self.depot.borrow_mut().enable(tag.into())
    }

    pub fn print_raw(&self) {
        let data_vec = self.get_data();
        let data_str = std::str::from_utf8(&data_vec.data).expect("Cannot convert log from UTF-8");
        println!("{data_str}");
    }

    fn get_data(&self) -> Ref<Depot> {
        self.depot.borrow()
    }
}

impl Default for Server {
    fn default() -> Self {
        Self {
            depot: Rc::new(RefCell::new(Depot {
                data: Vec::new(),
                flag: Vec::new(),
            })),
        }
    }
}
