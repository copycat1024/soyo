use super::Flow;
use crate::{tui, view::Compose};

pub trait Model: 'static + Sized {
    type Event: 'static + Copy;
    type View: Compose;

    fn new() -> (Self, Self::View);
    fn dispatch(&self, _event: tui::Event, _view: &Self::View) -> Option<Self::Event>;
    fn reduce(&mut self, _event: Self::Event, _flow: &mut Flow);
    fn update(&self, _view: &mut Self::View);
}
