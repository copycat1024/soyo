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

        // resize on init
        let (w, h) = ctx.size();
        app.view.resize(w, h, ctx, &mut app.flow)?;

        // main loop
        'main: loop {
            // handle native events
            while let Some(event) = ctx.event()? {
                match event {
                    Event::Resize { w, h } => {
                        app.view.resize(w, h, ctx, &mut app.flow)?;
                    }
                    Event::Update { delta } => {
                        let delta = delta.as_millis() as u64;
                        app.view.tick(delta, &mut app.flow);
                    }
                    _ => {}
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
            app.view.compose(&app.flow);

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
}
