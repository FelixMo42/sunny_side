use crate::renderer::Screen;
use crate::Spot;

pub trait Pain<Event> {
    fn update(&mut self, event: Event, screen: &mut Screen) -> std::io::Result<Spot>;
}
