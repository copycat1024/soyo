use super::Flow;
use crate::{tui, view::Compose};

pub trait Model<Args>: 'static + Sized {
    type Event: 'static + Copy;
    type Trigger: 'static;
    type View: Compose;

    fn new(args: &Args) -> (Self, Self::View);
    fn dispatch(&self, _event: tui::Event, _view: &Self::View) -> Option<Self::Event>;
    fn reduce(&mut self, _event: Self::Event, _flow: &mut Flow) -> Vec<Self::Trigger>;
    fn trigger(&self, _view: &mut Self::View, trigger: Self::Trigger);
    fn update(&self, _view: &mut Self::View);
}
