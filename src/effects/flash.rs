use midi_message::MidiMessage;
use color::Color;
use color_strip::ColorStrip;
use effects::effect::Effect;
use rainbow::get_rainbow_color;

pub struct Flash {
    color_strip: ColorStrip
}

impl Flash {
    pub fn new(led_count: usize) -> Flash {
        Flash {
            color_strip: ColorStrip::new(led_count)
        }
    }
}


impl Effect for Flash {
    fn paint(&mut self, color_strip: &mut ColorStrip) {
        color_strip.blit(&self.color_strip);
    }

    fn tick(&mut self) {
        let darkened_first_pixel = self.color_strip.pixel[0] - Color::gray(10);
        self.color_strip.insert(darkened_first_pixel);
    }

    fn on_midi_message(&mut self, midi_message: MidiMessage) {
        if let MidiMessage::NoteOn(_, note, _) = midi_message {
            self.color_strip.pixel[0] = get_rainbow_color(note);
        }
    }
}