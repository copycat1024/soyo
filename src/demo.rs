use soyo::{
    log::{enable_log, flush_log, tag},
    tui::{backend::Vt100, Color, Context, Event, Quad},
    util::Result,
};
use std::io::stdout;

fn main() -> Result {
    enable_log(tag::EVENT);

    {
        let backend = Vt100::new(stdout());
        let mut ctx = Context::new(backend);
        ctx.clear()?;

        'main: loop {
            if let Some(Event::Key { .. }) = ctx.event()? {
                break 'main;
            }

            let mut rect = Quad::xywh(0, 0, 5, 5);
            ctx.render(rect, 1, |_, letter| {
                *letter.c = 'X';
                *letter.bg = Color::BLUE;
            });
            rect = Quad::xywh(2, 2, 5, 5);
            ctx.render(rect, 2, |_, letter| {
                *letter.c = 'O';
                *letter.bg = Color::BLUE;
            });

            ctx.draw()?;
        }
    }

    flush_log();

    Ok(())
}
