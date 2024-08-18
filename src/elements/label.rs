use crate as kon3;
use crate::resources::Resources;
use crate::values::ValueSource;
use crate::Build;
use crate::Drawer;
use crate::Element;
use crate::Event;
use crate::EventResult;
use crate::HandleEvent;
use crate::Location;
use std::hash::Hash;
use sww::shaders::mesh::Rectangle;
use sww::vec2;

// TODO: turn into `text` field
#[derive(Build)]
pub struct Label<Src>(Src);

impl<Src> HandleEvent for Label<Src> {
    fn handle_event(&mut self, _event: &Event) -> EventResult {
        Ok(())
    }
}

impl<Src: ValueSource<Value = str>> Element for Label<Src> {
    fn draw<'e>(&self, drawer: &mut Drawer<'e>, resources: &'e Resources, location: Location) {
        // FIXME
        use super::rect;
        use sww::Color;

        let hash = {
            use std::hash::DefaultHasher;
            use std::hash::Hasher;

            let mut hasher = DefaultHasher::default();
            self.0.value().hash(&mut hasher);
            (hasher.finish() % 16) as usize
        };

        let padding = 1. / hash as f32;
        let location = location.subrect(Rectangle {
            top_left: vec2(padding, 0.),
            size: vec2(1. - padding * 2., 1.),
        });
        rect(Color::new_rgba(1., 1., 1., 0.5)).draw(drawer, resources, location)
    }
}

pub const fn label<Src: Build<Built: ValueSource<Value = str>>>(
    ra_fixture_source: Src,
) -> Label<Src> {
    Label(ra_fixture_source)
}
