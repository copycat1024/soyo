use super::{Dispatch, Flow, Model, View};
use crate::{
    tui::{Context, Event},
    util::Result,
};

pub struct App<M: Model> {
    flow: Flow,
    dispatch: Dispatch<M::Event>,
    model: M,
    view: View<M::View>,
}

impl<M: Model> App<M> {
    pub fn new() -> Self {
        let (model, composer) = M::new();
        Self {
            dispatch: Dispatch::default(),
            model,
            view: View::new(composer),
            flow: Flow::default(),
        }
    }

    pub fn run(ctx: &mut Context) -> Result<usize> {
        let mut app = Self::new();

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
            view,
            dispatch,
            model,
            ..
        } = self;

        if let Some(event) = view.node().get(|view| model.dispatch(event, view)) {
            dispatch.dispatch(event);
        }
    }

    fn update(&mut self) {
        let Self { model, view, .. } = self;
        view.node_mut().set(|view| model.update(view));
    }
}
