use iced::{Color, Point, Rectangle, Renderer, Theme, mouse, widget::canvas};

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
        let spacing = 20.0;

        for string_num in 0..self.string_count {
            let y = top_margin + string_num as f32 * spacing;

            let line = canvas::Path::line(Point::new(0.0, y), Point::new(bounds.width, y));

            frame.stroke(&line, canvas::Stroke::default().with_color(Color::WHITE));
        }

        vec![frame.into_geometry()]
    }
}
