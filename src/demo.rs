#![allow(unused_imports)]

use crossterm::event::Event;
use soyo::{
    tui::{
        backend::{CrosstermBackend, DebugBackend},
        Context,
    },
    util::Result,
};

fn main() -> Result {
    let log;
    {
        let mut ctx = Context::<CrosstermBackend>::default();
        let mut once = true;
        ctx.clear()?;

        'main: loop {
            if let Some(e) = ctx.event()? {
                match e {
                    Event::Key(_) => {
                        break 'main;
                    }
                    _ => {}
                }
            }

            if once {
                if let Some(mut cell) = ctx.item(2, 2, 1) {
                    (*cell).c = 'a';
                }

                once = false;
            }

            ctx.draw()?;
        }

        log = ctx.leak_log();
    }

    print!("{log}");

    Ok(())
}
