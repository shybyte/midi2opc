use midi_message::MidiMessage;
use color::Color;
use color_strip::ColorStrip;
use effects::effect::Effect;
use rainbow::get_rainbow_color;

pub struct Blink {
    blink_color: Color,
    add_color: Color
}

impl Blink {
    pub fn new() -> Blink {
        Self::new_with_add_color(Color::black())
    }

    pub fn new_with_add_color(add_color: Color) -> Blink {
        Blink {
            blink_color: Color::black(),
            add_color
        }
    }

    pub fn on_note(&mut self, note: u8) {
        self.blink_color = get_rainbow_color(note) + self.add_color;
    }
}


impl Effect for Blink {
    fn paint(&mut self, color_strip: &mut ColorStrip) {
        for p in &mut color_strip.pixel {
            *p += self.blink_color;
        }
    }

    fn tick(&mut self) {
        self.blink_color -= Color::gray(40);
    }

    fn on_midi_message(&mut self, midi_message: MidiMessage) {
        if let MidiMessage::NoteOn(_, note, _) = midi_message {
            self.on_note(note);
        }
    }
}