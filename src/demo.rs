use crossterm::{event::Event, style::Color};
use soyo::{
    tui::{backend::CrosstermBackend, Context, Rect},
    util::{LoggerServer, Result},
};
use std::io::stdout;

fn main() -> Result {
    // let mut binary_logger = LoggerServer::new();
    let mut backend_logger = LoggerServer::new();

    {
        let mut backend = CrosstermBackend::new(stdout());
        let mut ctx = Context::compose(&mut backend, &mut backend_logger);
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

            let mut rect = Rect::new();
            rect.xywh(0, 0, 1, 1);
            ctx.render(rect, 1, |_, _, letter| {
                *letter.c = 'X';
                *letter.fg = Color::Red;
            });
            rect.xywh(1, 1, 1, 1);
            ctx.render(rect, 1, |_, _, letter| {
                *letter.c = 'O';
                *letter.fg = Color::Blue;
            });

            ctx.draw()?;
        }
    }

    backend_logger.print_raw();

    Ok(())
}
