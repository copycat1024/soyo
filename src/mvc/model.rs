use super::Flow;

pub trait Model: Default + 'static {
    type Event: 'static + Copy;

    fn reduce(&mut self, event: Self::Event, flow: &mut Flow);
}
