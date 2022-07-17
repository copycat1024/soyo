use crate::{
    log::{enable_log, flush_log},
    tui::{backend::Vt100, Context},
    util::Result,
};
use std::io::stdout;

pub struct AppItem {
    pub name: &'static str,
    pub runtime: fn(ctx: &mut Context) -> Result<usize>,
}

impl AppItem {
    pub const fn new(name: &'static str, runtime: fn(ctx: &mut Context) -> Result<usize>) -> Self {
        Self { name, runtime }
    }
}

pub fn launch(tags: &[u8], app_list: &[AppItem]) -> Result {
    for tag in tags {
        enable_log(*tag)
    }

    {
        // create context
        let vt100 = Vt100::new(stdout());
        let mut ctx = Context::new(vt100);
        let mut code: usize = 0;

        loop {
            code = if code < app_list.len() {
                (app_list[code].runtime)(&mut ctx)?
            } else {
                break;
            }
        }
    }

    flush_log();

    Ok(())
}
