use soyo::{
    log::{enable_log, flush_log, Tag},
    tui::{backend::Vt100, Color, Context, Event, Rect},
    util::Result,
};
use std::io::stdout;

fn main() -> Result {
    enable_log(Tag::Event);

    {
        let backend = Vt100::new(stdout());
        let mut ctx = Context::new(backend);
        ctx.clear()?;

        'main: loop {
            if let Some(Event::Key { .. }) = ctx.event()? {
                break 'main;
            }

            let mut rect = Rect::xywh(0, 0, 5, 5);
            ctx.render(rect, 1, |_, _, letter| {
                *letter.c = 'X';
                *letter.bg = Color::BLUE;
            });
            rect = Rect::xywh(2, 2, 5, 5);
            ctx.render(rect, 2, |_, _, letter| {
                *letter.c = 'O';
                *letter.bg = Color::BLUE;
            });

            ctx.draw()?;
        }
    }

    flush_log();

    Ok(())
}
