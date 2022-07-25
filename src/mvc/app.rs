use super::{Control, Dispatch, Flow, Model, View};
use crate::{
    tui::{Context, Event},
    util::Result,
    view::Compose,
};

pub struct App<M, C>
where
    M: Model,
    C: Compose,
{
    flow: Flow,
    dispatch: Dispatch<M::Event>,
    model: M,
    view: View<C>,
    control: Control<M, C>,
}

impl<M, C> App<M, C>
where
    M: Model,
    C: Compose,
{
    pub fn new(control: Control<M, C>) -> Self {
        let (model, composer) = control.init();
        Self {
            dispatch: Dispatch::default(),
            model,
            view: View::new(composer),
            control,
            flow: Flow::default(),
        }
    }

    pub fn run(control: Control<M, C>, ctx: &mut Context) -> Result<usize> {
        let mut app = Self::new(control);
        let (w, h) = ctx.size();

        app.resize(ctx, w, h)?;

        // main loop
        'main: loop {
            // handle native events
            while let Some(event) = ctx.event()? {
                if let Event::Resize { w, h } = event {
                    app.resize(ctx, w, h)?;
                }

                app.dispatch(event);
            }

            // handle domain event
            while let Some(event) = app.dispatch.event() {
                app.model.reduce(event, &mut app.flow);
                if app.flow.stop {
                    break 'main;
                }
            }

            // update view
            app.update();

            // compose view
            app.view.compose();

            // draw
            app.view.draw(ctx, &mut app.flow)?;
        }

        // clean up app
        ctx.clear()?;

        Ok(app.flow.code)
    }

    fn dispatch(&mut self, event: Event) {
        let Self {
            control,
            view,
            dispatch,
            ..
        } = self;

        control.dispatch(event, view.node(), dispatch)
    }

    fn update(&mut self) {
        let Self {
            control,
            model,
            view,
            ..
        } = self;

        control.update(model, view.node_mut());
    }

    fn resize(&mut self, ctx: &mut Context, w: i32, h: i32) -> Result {
        self.flow.draw = true;
        self.view.resize(w, h);
        ctx.clear()
    }
}
