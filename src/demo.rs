use crossterm::{event::Event, style::Color};
use soyo::{
    tui::{backend::Vt100, Context, Rect},
    util::{LoggerServer, Result},
};
use std::io::stdout;

fn main() -> Result {
    let mut logger = LoggerServer::default();

    {
        let backend = Vt100::new(stdout());
        let mut ctx = Context::compose(backend, Some(&mut logger));
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
            rect.xywh(0, 0, 5, 5);
            ctx.render(rect, 1, |_, _, letter| {
                *letter.c = 'X';
                *letter.bg = Color::Blue;
            });
            rect.xywh(2, 2, 5, 5);
            ctx.render(rect, 2, |_, _, letter| {
                *letter.c = 'O';
                *letter.bg = Color::Blue;
            });

            ctx.draw()?;
        }
    }

    logger.print_raw();

    Ok(())
}
