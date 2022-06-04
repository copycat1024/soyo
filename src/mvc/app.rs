use super::{Control, Dispatch, Flow, Model, View};
use crate::{
    tui::{Context, Event},
    util::Result,
};

pub struct App<M, V>
where
    M: Model,
    V: View,
{
    dispatch: Dispatch<M::Event>,
    model: M,
    view: V,
    control: Control<M, V>,
    flow: Flow,
}

impl<M, V> App<M, V>
where
    M: Model,
    V: View,
{
    pub fn new(control: Control<M, V>) -> Self {
        Self {
            dispatch: Dispatch::default(),
            model: M::default(),
            view: V::default(),
            control,
            flow: Flow::default(),
        }
    }

    pub fn run(&mut self, ctx: &mut Context) -> Result {
        let (w, h) = ctx.size();

        self.view.setup();
        self.resize(ctx, w, h)?;

        // main loop
        loop {
            // handle native events
            while let Some(event) = ctx.event()? {
                if let Event::Resize { w, h } = event {
                    self.resize(ctx, w, h)?;
                }

                self.dispatch(event);
            }

            // handle domain event
            while let Some(event) = self.dispatch.event() {
                self.model.reduce(event, &mut self.flow);
            }
            if self.flow.stop {
                break;
            }

            // update view
            self.update();

            if self.flow.draw {
                self.draw(ctx)?;
            }
        }

        // clean up app
        ctx.clear()
    }

    fn dispatch(&mut self, event: Event) {
        let Self {
            control,
            view,
            dispatch,
            ..
        } = self;

        control.dispatch(event, view, dispatch)
    }

    fn update(&mut self) {
        let Self {
            control,
            model,
            view,
            ..
        } = self;

        control.update(model, view);
    }

    fn draw(&mut self, ctx: &mut Context) -> Result {
        self.flow.draw = false;
        self.view.render(ctx);
        ctx.draw()
    }

    fn resize(&mut self, ctx: &mut Context, w: i32, h: i32) -> Result {
        self.flow.draw = true;
        self.view.resize(w, h);
        ctx.clear()
    }
}
