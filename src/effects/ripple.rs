use midi_message::MidiMessage;
use color::Color;
use color_strip::ColorStrip;
use effects::effect::Effect;

// http://www.roxlu.com/downloads/scholar/001.fluid.height_field_simulation.pdf
pub struct Ripple {
    u: Vec<f64>,
    v: Vec<f64>,
}

impl Ripple {
    pub fn new(led_count: usize) -> Ripple {
        Ripple {
            u: vec![0.0; led_count],
            v: vec![0.0; led_count],
        }
    }

    pub fn on_note(&mut self, note: u8) {
        let i = (i32::from(note)) % self.u.len() as i32;
        self.u[i as usize] = 1256.0;
    }
}


fn wrap_index(i: usize, delta: i32, l: usize) -> usize {
    ((i as i32 + delta + l as i32) % l as i32) as usize
}

impl Effect for Ripple {
    fn paint(&mut self, color_strip: &mut ColorStrip) {
        for (i, &height) in self.u.iter().enumerate() {
            let light = f64::min(height, 255.0) as u8;
            color_strip.pixel[i] += if light < 128 {
                Color::new(0, 0, 128 + light)
            } else {
                Color::new((light - 128) * 2, (light - 128) * 2, 255)
            };
        }
    }

    fn tick(&mut self) {
        let l = self.u.len();
        for i in 0..l {
            let new_v = (self.u[wrap_index(i, -1, l)] + self.u[wrap_index(i, 1, l)]) / 2.0 -
                self.u[i];
            self.v[i] = new_v * 0.999;
        }
        for i in 0..l {
            self.u[i] = f64::max(self.u[i] + self.v[i] * 1.0 - 2.0, 0.0);
        }
    }

    fn on_midi_message(&mut self, midi_message: MidiMessage) {
        if let MidiMessage::NoteOn(_, note, _) = midi_message {
            self.on_note(note)
        }
    }
}