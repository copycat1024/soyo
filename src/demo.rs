#![allow(unused_imports)]

use crossterm::event::Event;
use soyo::{
    tui::{backend::CrosstermBackend, Backend, Context},
    util::{Logger, Result},
};
use std::io::stdout;

fn main() -> Result {
    let mut binary_logger = Logger::new();
    let mut backend_logger = Logger::new();

    {
        let mut backend = CrosstermBackend::new(stdout(), &mut backend_logger);
        let mut ctx = Context::compose(&mut backend);
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
                draw(&mut ctx, 1, 1, 'a');
                draw(&mut ctx, 1, 2, 'a');

                draw(&mut ctx, 2, 1, 'b');
                draw(&mut ctx, 2, 2, 'b');
                draw(&mut ctx, 2, 4, 'b');

                draw(&mut ctx, 4, 3, 'c');
                draw(&mut ctx, 4, 4, 'c');
                draw(&mut ctx, 4, 6, 'c');
                draw(&mut ctx, 4, 7, 'c');

                once = false;
            }

            ctx.draw()?;
        }
    }

    backend_logger.print_raw();

    Ok(())
}

fn draw<B: Backend>(ctx: &mut Context<B>, x: i32, y: i32, c: char) {
    if let Some(mut cell) = ctx.item(x, y, 1) {
        (*cell).c = c;
    }
}
