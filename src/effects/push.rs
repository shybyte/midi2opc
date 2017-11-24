use midi_message::MidiMessage;
use color_strip::ColorStrip;
use effects::effect::Effect;
use rainbow::get_rainbow_color;

pub struct Push {
    color_strip: ColorStrip,
}

impl Push {
    pub fn new(led_count: usize) -> Push {
        Push {
            color_strip: ColorStrip::new(led_count),
        }
    }
}


impl Effect for Push {
    fn paint(&mut self, color_strip: &mut ColorStrip) {
        color_strip.blit(&self.color_strip);
    }

    fn tick(&mut self) {}

    fn on_midi_message(&mut self, midi_message: MidiMessage) {
        if let MidiMessage::NoteOn(_, note, _) = midi_message {
            self.color_strip.insert(get_rainbow_color(note));
        }
    }
}