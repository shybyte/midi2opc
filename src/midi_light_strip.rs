use opc_strip::OpcStrip;

use std::io;
use std::f32;
use color_strip::ColorStrip;
use color::Color;
use std::sync::mpsc;

use midi_message::MidiMessage;

use std::thread;
use std::time::Duration;

static RAINBOW: [[u8; 3]; 12] = [[255, 0, 0], [255, 128, 0], [255, 255, 0], [128, 255, 0], [0, 255, 0], [0, 255, 128], [0, 255, 255], [0, 127, 255], [0, 0, 255], [128, 0, 255], [255, 0, 255], [255, 0, 128]];

#[derive(Default, Clone)]
pub struct MidiLightConfig {
    pub blink: bool,
    pub flash: bool,
    pub stream: bool,
    pub led_count: usize,
    pub reversed: bool,
    pub max_note: u8
}


pub struct MidiLightStrip {
    tx_strip: mpsc::Sender<MidiMessage>,
}

impl MidiLightStrip {
    pub fn start(config: MidiLightConfig) -> io::Result<MidiLightStrip> {
        let mut opc_strip = OpcStrip::connect(30, config.reversed)?;
        let (tx_strip, rx_strip) = mpsc::channel();

        thread::spawn(move || {
            let mut my_thread = MidiLightStripThread { rx_strip, config, opc_strip };
            my_thread.run();
        });

        Ok(MidiLightStrip { tx_strip })
    }

    pub fn on_midi_message(&self, midi_message: MidiMessage) {
        self.tx_strip.send(midi_message).ok();
    }
}


pub struct MidiLightStripThread {
    opc_strip: OpcStrip,
    rx_strip: mpsc::Receiver<MidiMessage>,
    config: MidiLightConfig
}

impl MidiLightStripThread {
    pub fn run(&mut self) {
        let mut color_strip = ColorStrip::new(self.config.led_count);
        let mut blink_color = Color::gray(0);
        let half_white = Color::gray(200);

        let mut pressed_keys: Vec<u8> = vec![];
        let mut stream_i: u64 = 0;

        loop {
            let first_pixel = color_strip.pixel[0];
            color_strip.insert(first_pixel);

            if let Some(midi_message) = self.rx_strip.try_recv().ok() {
                match midi_message {
                    MidiMessage::NoteOn(_, note, _) if note < self.config.max_note => {
                        pressed_keys.push(note);
                        let color = get_rainbow_color(note);
                        if self.config.flash {
                            color_strip.pixel[0] = color;
                        }
                        if self.config.blink {
                            if self.config.flash {
                                blink_color = color + half_white;
                            } else {
                                blink_color = color;
                            }
                        }
                    }
                    MidiMessage::NoteOff(_, message_note, _) => {
                        pressed_keys.retain(|&note| note != message_note);
                    }
                    _ => {}
                }
            } else {
                if !pressed_keys.is_empty() && self.config.stream {
                    let key_index = (stream_i / 11) as usize % pressed_keys.len();
                    color_strip.pixel[0] = get_rainbow_color(pressed_keys[key_index]) -
                        Color::gray(((stream_i as f32 / 2.0).sin() * 100.0 + 120.0) as u8);
                    stream_i += 1;
                } else {
                    color_strip.pixel[0] = color_strip.pixel[0] - Color::gray(10);
                }
                blink_color = blink_color - Color::gray(40);
            }

            self.opc_strip.send(&color_strip, blink_color);
            thread::sleep(Duration::from_millis(10));
        }
    }
}


fn get_rainbow_color(note: u8) -> Color {
    let rgb = RAINBOW[note as usize % 12 as usize];
    Color::new(rgb[0], rgb[1], rgb[2])
}