use soyo::{
    tui::{backend::Vt100, Color, Context, Event, Rect},
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
                    Event::Key { .. } => {
                        break 'main;
                    }
                    _ => {}
                }
            }

            let mut rect = Rect::new();
            rect.xywh(0, 0, 5, 5);
            ctx.render(rect, 1, |_, _, letter| {
                *letter.c = 'X';
                *letter.bg = Color::BLUE;
            });
            rect.xywh(2, 2, 5, 5);
            ctx.render(rect, 2, |_, _, letter| {
                *letter.c = 'O';
                *letter.bg = Color::BLUE;
            });

            ctx.draw()?;
        }
    }

    logger.print_raw();

    Ok(())
}
