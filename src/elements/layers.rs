use crate::drawer::DrawPass;
use crate::shared;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::InvalidateCache;
use crate::Location;

pub struct Layers<Es> {
    elements: Es,
}

impl<R, A: Element<R>, B: Element<R>> Element<R> for Layers<(A, B)> {
    fn draw(&self, pass: &mut DrawPass, resources: &R, location: Location) {
        let (a, b) = &self.elements;
        a.draw(pass, resources, location);
        b.draw(pass, resources, location);
    }
}

impl<R, A: Element<R>, B: Element<R>, C: Element<R>> Element<R> for Layers<(A, B, C)> {
    fn draw(&self, pass: &mut DrawPass, resources: &R, location: Location) {
        let (a, b, c) = &self.elements;
        a.draw(pass, resources, location);
        b.draw(pass, resources, location);
        c.draw(pass, resources, location);
    }
}

impl<Es: HandleEvent> HandleEvent for Layers<Es> {
    fn handle_event(&self, event: &Event) -> EventResult {
        self.elements.handle_event(event)
    }
}

impl<Es: InvalidateCache> InvalidateCache for Layers<Es> {
    fn invalidate_cache(&self, addr: shared::Addr) -> bool {
        self.elements.invalidate_cache(addr)
    }
}

pub const fn layers<Es>(ra_fixture_elements: Es) -> Layers<Es> {
    Layers {
        elements: ra_fixture_elements,
    }
}
