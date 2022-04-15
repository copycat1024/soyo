use std::{
    cell::{RefCell, RefMut},
    io::{Result, Write},
    rc::{Rc, Weak},
};

pub struct LoggerServer {
    data: Rc<RefCell<Vec<u8>>>,
}

impl LoggerServer {
    pub fn client(&self) -> LoggerClient {
        LoggerClient {
            data: Rc::downgrade(&self.data),
        }
    }

    pub fn print_ascii(&self) {
        let mut null_count = 0_usize;

        for c in self.data.borrow().iter() {
            if *c == 0 {
                null_count += 1;
            } else {
                if null_count > 0 {
                    println!("\\0 [{null_count}]");
                    null_count = 0;
                }
                print!("{c:02x}");
                if c.is_ascii_graphic() {
                    let c = char::from(*c);
                    print!(" {c}");
                }
                println!();
            }
        }

        if null_count > 0 {
            println!("\\0 [{null_count}]");
        }
    }

    pub fn print_raw(&self) {
        let data_vec = &self.data.borrow();
        let data_str = std::str::from_utf8(data_vec).expect("Cannot convert log from UTF-8");
        println!("{data_str}");
    }
}

impl Default for LoggerServer {
    fn default() -> Self {
        Self {
            data: Rc::new(RefCell::new(Vec::new())),
        }
    }
}

pub struct LoggerClient {
    data: Weak<RefCell<Vec<u8>>>,
}

impl Default for LoggerClient {
    fn default() -> Self {
        Self { data: Weak::new() }
    }
}

impl Write for LoggerClient {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        if let Some(data) = self.data.upgrade() {
            data.borrow_mut().write(buf)
        } else {
            Ok(buf.len())
        }
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
