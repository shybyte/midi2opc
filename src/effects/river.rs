use midi_message::MidiMessage;
use color::Color;
use color_strip::ColorStrip;
use effects::effect::Effect;
use rainbow::get_rainbow_color;

#[derive(Copy, Clone, Debug, Default)]
struct Point {
    pos: f32,
    speed: f32,
    color: Color
}

impl Point {
    pub fn new(pos: f32, color: Color, speed: f32) -> Self {
        Self { pos, color, speed }
    }
}

pub struct River {
    led_count: usize,
    points: Vec<Point>,
}

impl River {
    pub fn new(led_count: usize) -> River {
        River {
            led_count,
            points: vec![]
        }
    }

    pub fn on_note(&mut self, note: u8) {
        self.points.push(Point::new(0.0, get_rainbow_color(note), 1.0))
    }
}


impl Effect for River {
    fn paint(&mut self, color_strip: &mut ColorStrip) {
        for (i, pixel) in color_strip.pixel.iter_mut().enumerate() {
            let pos = i as f32;
            let mut color = Color::black();
            for point in &self.points {
                let delta = point.pos - pos;
                let opacity = f32::max(1.0 - (delta * delta) / 5.0, 0.0);
                color += point.color.mul_with_opacity(opacity);
            }
            *pixel += color;
        }
    }

    fn tick(&mut self) {
        for point in &mut self.points {
            point.pos += point.speed / 20.0;
        }
        let max_pos = self.led_count;
        self.points.retain(|p| (p.pos as usize) < max_pos);
    }

    fn on_midi_message(&mut self, midi_message: MidiMessage) {
        if let MidiMessage::NoteOn(_, note, _) = midi_message {
            self.on_note(note)
        }
    }
}