use super::{Dispatch, Model};
use crate::{
    tui::Event,
    view::{Compose, NodeRef},
};

pub struct Control<M, C>
where
    M: Model,
    C: Compose,
{
    init: fn() -> (M, C),
    dispatch: fn(Event, &NodeRef<C>, &mut Dispatch<M::Event>),
    update: fn(&M, &mut NodeRef<C>),
}

impl<M, C> Control<M, C>
where
    M: Model,
    C: Compose,
{
    pub const fn new(
        init: fn() -> (M, C),
        dispatch: fn(Event, &NodeRef<C>, &mut Dispatch<M::Event>),
        update: fn(&M, &mut NodeRef<C>),
    ) -> Self {
        Self {
            init,
            dispatch,
            update,
        }
    }

    pub fn init(&self) -> (M, C) {
        (self.init)()
    }

    pub fn dispatch(&self, event: Event, view: &NodeRef<C>, dispatch: &mut Dispatch<M::Event>) {
        (self.dispatch)(event, view, dispatch);
    }

    pub fn update(&self, model: &M, view: &mut NodeRef<C>) {
        (self.update)(model, view);
    }
}
