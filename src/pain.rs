use crate::renderer::Screen;
use crate::Spot;

pub trait Pain<Event> {
    fn update(&mut self, screen: &mut Screen, event: Event) -> std::io::Result<Spot>;
}
