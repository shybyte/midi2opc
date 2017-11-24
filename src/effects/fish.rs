use midi_message::MidiMessage;
use color::Color;
use color_strip::ColorStrip;
use effects::effect::Effect;
use rainbow::get_rainbow_color;
use std::f64;
use rand::random;

#[derive(Copy, Clone, Debug, Default)]
struct Point {
    pos: f64,
    speed: f64,
    finished: bool,
    color: Color
}

impl Point {
    pub fn new(pos: f64, color: Color, speed: f64) -> Self {
        Self { pos, color, speed, finished: false }
    }
}

pub struct Fish {
    led_count: usize,
    points: Vec<Point>,
    lonelyness: Vec<f64>,
}

impl Fish {
    pub fn new(led_count: usize) -> Fish {
        Fish {
            led_count,
            points: vec![],
            lonelyness: vec![0.0; led_count]
        }
    }

    pub fn on_note(&mut self, note: u8) {
        let lonely_pos = if self.points.is_empty() {
            random::<f64>() * (self.led_count as f64)
        } else {
            let lonely = (1..self.led_count - 1).max_by_key(|&i| {
                (self.lonelyness[i] * 1_000.0) as i64
            }).unwrap_or(0) as f64;
            lonely
        };
        let random_speed = random::<f64>() - 0.5;
        self.points.push(Point::new(lonely_pos.floor(), get_rainbow_color(note), random_speed))
    }
}


impl Effect for Fish {
    fn paint(&mut self, color_strip: &mut ColorStrip) {
        for (i, pixel) in color_strip.pixel.iter_mut().enumerate() {
            let pos = i as f64;
            let mut color = Color::black();
            self.lonelyness[i] = self.led_count as f64;
            for point in &self.points {
                let delta = point.pos - pos;
                let opacity = f64::max(1.0 - (delta * delta) / 5.0, 0.0);
                self.lonelyness[i] = f64::min(self.lonelyness[i], delta.abs());
                color += point.color.mul_with_opacity(opacity as f32);
            }
            *pixel += color;
        }
    }

    fn tick(&mut self) {
        if { self.points.is_empty() } {
            return;
        }
        self.points.sort_by_key(|p| (p.pos * 100.0) as i32);
        if self.points.len() > 1 {
            let max_i = self.points.len() - 1;
            for i in 0..self.points.len() {
                let point = self.points[i];
                let mut speed_delta = 0.0;
                let delta_to_prev = if i > 0 { (self.points[i - 1].pos - point.pos).abs() } else { f64::MAX };
                let delta_to_next = if i < max_i { (self.points[i + 1].pos - point.pos).abs() } else { f64::MAX };

                if delta_to_next < 1.0 || delta_to_prev < 1.0 {
                    self.points[i].finished = true;
                } else if delta_to_prev < delta_to_next {
                    speed_delta = -0.5
                } else {
                    speed_delta = 0.5
                }
                self.points[i].speed = (self.points[i].speed + speed_delta) / 2.0;
            }
        }

        for point in &mut self.points {
            point.pos += point.speed / 20.0;
        }
        let led_count = self.led_count;
        self.points.retain(|p| (p.pos as usize) > 0 && (p.pos as usize) < led_count && !p.finished);
    }

    fn on_midi_message(&mut self, midi_message: MidiMessage) {
        if let MidiMessage::NoteOn(_, note, _) = midi_message {
            self.on_note(note)
        }
    }
}