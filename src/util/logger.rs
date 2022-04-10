use std::{
    io::{Result, Write},
    str::from_utf8,
};

pub struct Logger {
    data: Vec<u8>,
}

impl Logger {
    pub fn new() -> Self {
        Self { data: Vec::new() }
    }

    pub fn print_ascii(&self) {
        let mut null_count = 0_usize;

        for c in self.data.iter() {
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
        let data = from_utf8(&self.data).expect("Cannot convert log from UTF-8");
        println!("{data}");
    }
}

impl Write for &mut Logger {
    fn write(&mut self, data: &[u8]) -> Result<usize> {
        self.data.extend(data);
        Ok(data.len())
    }

    fn flush(&mut self) -> Result<()> {
        Ok(())
    }
}
