use midi_message::MidiMessage;
use color::Color;
use color_strip::ColorStrip;
use effects::effect::Effect;
use rainbow::get_rainbow_color;

pub struct StreamCenter {
    color_strip: ColorStrip,
    pressed_keys: Vec<u8>,
    i: u64,
    i2: u64
}

impl StreamCenter {
    pub fn new(led_count: usize) -> StreamCenter {
        StreamCenter {
            color_strip: ColorStrip::new(led_count / 2),
            pressed_keys: vec![],
            i: 0,
            i2: 0
        }
    }
}


impl Effect for StreamCenter {
    fn paint(&mut self, color_strip: &mut ColorStrip) {
        let self_length = self.color_strip.pixel.len();
        for (i, pixel) in color_strip.pixel.iter_mut().enumerate() {
            if i < self_length {
                *pixel += self.color_strip.pixel[i];
            } else {
                *pixel += self.color_strip.pixel[self_length - (i - self_length) - 1];
            }
        }
    }

    fn tick(&mut self) {
        self.i2 = self.i2 + 1;
        if self.i2 % 5 != 0 {
            return;
        }
        if !self.pressed_keys.is_empty() {
            let key_index = (self.i / 11) as usize % self.pressed_keys.len();
            self.color_strip.insert(get_rainbow_color(self.pressed_keys[key_index]));
            self.i += 1;
        } else {
            let darkened_first_pixel = self.color_strip.pixel[0] - Color::gray(10);
            self.color_strip.insert(darkened_first_pixel);
        }
    }

    fn on_midi_message(&mut self, midi_message: MidiMessage) {
        match midi_message {
            MidiMessage::NoteOn(_, note, _) => {
                self.pressed_keys.push(note);
                self.color_strip.pixel[0] = get_rainbow_color(note);
            }
            MidiMessage::NoteOff(_, message_note, _) => {
                self.pressed_keys.retain(|&note| note != message_note);
            }
            _ => {}
        }
    }
}