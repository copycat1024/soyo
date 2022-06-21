use super::{Dispatch, Model, View};
use crate::tui::Event;

pub struct Control<M, V>
where
    M: Model,
    V: View,
{
    dispatch: fn(Event, &V, &mut Dispatch<M::Event>),
    update: fn(&M, &mut V),
}

impl<M, V> Control<M, V>
where
    M: Model,
    V: View,
{
    pub const fn new(
        dispatch: fn(Event, &V, &mut Dispatch<M::Event>),
        update: fn(&M, &mut V),
    ) -> Self {
        Self { dispatch, update }
    }

    pub fn dispatch(&self, event: Event, view: &V, dispatch: &mut Dispatch<M::Event>) {
        (self.dispatch)(event, view, dispatch);
    }

    pub fn update(&self, model: &M, view: &mut V) {
        (self.update)(model, view);
    }
}
