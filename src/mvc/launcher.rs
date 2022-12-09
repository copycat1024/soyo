use crate::{
    log::{enable_log, flush_log},
    tui::{backend::Vt100, Context},
    util::Result,
};
use std::io::stdout;

pub type Instance<Args = ()> = fn(args: &mut Args, ctx: &mut Context) -> Result<usize>;

pub struct Launcher<Args: 'static> {
    ctx: Context,
    code: usize,
    args: Args,
}

impl<Args: 'static> Launcher<Args> {
    pub fn new(args: Args) -> Self {
        let vt100 = Vt100::new(stdout());
        Self {
            ctx: Context::new(vt100),
            code: 0,
            args,
        }
    }

    pub fn launch(self, tags: &[u8], app_list: &[Instance<Args>]) -> Result {
        for tag in tags {
            enable_log(*tag)
        }

        let r = self.run(app_list);

        flush_log();

        r
    }

    fn run(self, app_list: &[Instance<Args>]) -> Result {
        let Self {
            mut code,
            mut ctx,
            mut args,
        } = self;

        loop {
            code = if code < app_list.len() {
                (app_list[code])(&mut args, &mut ctx)?
            } else {
                break;
            }
        }

        Ok(())
    }
}

impl<Args> Default for Launcher<Args>
where
    Args: 'static + Default,
{
    fn default() -> Self {
        Self::new(Args::default())
    }
}
