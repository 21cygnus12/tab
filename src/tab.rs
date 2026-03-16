use iced::{Color, Point, Rectangle, Renderer, Theme, mouse, widget::canvas};

const TPQN: u32 = 480;

pub struct Tab {
    string_count: u8,
    measures: Vec<Measure>,
}

impl Default for Tab {
    fn default() -> Self {
        Self {
            string_count: 6,
            measures: Default::default(),
        }
    }
}

struct Measure {
    time_signature: TimeSignature,
    events: Vec<Event>,
}

struct TimeSignature {
    numerator: u8,
    denominator: u8,
}

impl TimeSignature {
    fn new(numerator: u8, denominator: u8) -> Self {
        Self {
            numerator,
            denominator,
        }
    }

    fn ticks(&self) -> Tick {
        Tick(self.numerator as u32 * TPQN * 4 / self.denominator as u32)
    }
}

struct Event {
    duration: Tick,
    kind: EventType,
}

struct Tick(u32);

enum EventType {
    Chord(Vec<Note>),
    Rest,
}

struct Note {
    fret: u8,
    string: u8,
}

impl<Message> canvas::Program<Message> for Tab {
    type State = ();

    fn draw(
        &self,
        _state: &(),
        renderer: &Renderer,
        _theme: &Theme,
        bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Vec<canvas::Geometry> {
        let mut frame = canvas::Frame::new(renderer, bounds.size());

        let top_margin = 50.0;
        let side_margin = 50.0;
        let spacing = 20.0;

        for string_num in 0..self.string_count {
            let y = top_margin + string_num as f32 * spacing;

            let line = canvas::Path::line(
                Point::new(side_margin, y),
                Point::new(bounds.width - side_margin, y),
            );

            frame.stroke(&line, canvas::Stroke::default().with_color(Color::WHITE));
        }

        vec![frame.into_geometry()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn time_signature_ticks() {
        let ts = TimeSignature::new(4, 4);

        assert_eq!(ts.ticks().0, 1920);

        let ts = TimeSignature::new(7, 8);

        assert_eq!(ts.ticks().0, 1680);
    }
}
