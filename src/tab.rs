#[derive(Default)]
pub struct Tab {
    measures: Vec<Measure>,
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
