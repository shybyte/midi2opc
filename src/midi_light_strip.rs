use opc_strip::OpcStrip;

use chan;
use std::io;
use color_strip::ColorStrip;
use color::Color;

use pm::MidiMessage;

use std::thread;
use std::time::Duration;

static RAINBOW: [[u8; 3]; 12] = [[255, 0, 0], [255, 128, 0], [255, 255, 0], [128, 255, 0], [0, 255, 0], [0, 255, 128], [0, 255, 255], [0, 127, 255], [0, 0, 255], [128, 0, 255], [255, 0, 255], [255, 0, 128]];

pub struct MidiLightStrip {
    tx_strip: chan::Sender<MidiMessage>,
    max_note: u8,
}

impl MidiLightStrip {
    pub fn start(led_count: usize, reversed: bool, max_note: u8) -> io::Result<MidiLightStrip> {
        let (tx_strip, rx_strip) = chan::sync::<MidiMessage>(0);

        thread::spawn(move || {
            let mut opc_strip_result = OpcStrip::connect(30, reversed);
            if let Ok(ref mut led_strip) = opc_strip_result {
                let mut color_strip = ColorStrip::new(led_count);
                let mut blink_color = Color::gray(0);
                let half_white = Color::gray(200);

                loop {
                    let first_pixel = color_strip.pixel[0];
                    color_strip.insert(first_pixel);

                    chan_select! {
                    default => {
                        color_strip.pixel[0] = color_strip.pixel[0] - Color::gray(10);
                        blink_color = blink_color - Color::gray(40);
                    },
                    rx_strip.recv() -> event => {
                        let rgb = RAINBOW[event.unwrap().data1 as usize %12 as usize];
                        let color = Color::new(rgb[0] , rgb[1], rgb[2]);
                        color_strip.pixel[0] = color;
                        blink_color = color + half_white;
//                        blink_color = color ;
                    },
                }

                    led_strip.send(&color_strip, blink_color);
                    thread::sleep(Duration::from_millis(10));
                }
            }
        });

        Ok(MidiLightStrip { tx_strip, max_note })
    }

    pub fn on_midi_message(&self, midi_message: MidiMessage) {
        match midi_message.status {
            144 | 153 => {
                if midi_message.data1 < self.max_note {
                    self.tx_strip.send(midi_message);
                }
            }
            _ => {}
        }
    }
}