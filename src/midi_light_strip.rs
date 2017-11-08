use opc_strip::OpcStrip;

use chan;
use std::io;
use color_strip::ColorStrip;
use color::Color;

use pm::MidiMessage;

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
    tx_strip: chan::Sender<MidiMessage>,
    config: MidiLightConfig
}

impl MidiLightStrip {
    pub fn start(config: MidiLightConfig) -> io::Result<MidiLightStrip> {
        let (tx_strip, rx_strip) = chan::sync::<MidiMessage>(0);

        let thread_config = config.clone();

        thread::spawn(move || {
            let mut opc_strip_result = OpcStrip::connect(30, thread_config.reversed);
            if let Ok(ref mut led_strip) = opc_strip_result {
                let mut color_strip = ColorStrip::new(thread_config.led_count);
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
                        if thread_config.flash {
                            color_strip.pixel[0] = color;
                        }
                        if thread_config.blink  {
                            if thread_config.flash {
                                blink_color = color + half_white;
                            } else {
                                blink_color = color ;
                            }
                        }
                    },
                }

                    led_strip.send(&color_strip, blink_color);
                    thread::sleep(Duration::from_millis(10));
                }
            }
        });

        Ok(MidiLightStrip { tx_strip, config })
    }

    pub fn on_midi_message(&self, midi_message: MidiMessage) {
        match midi_message.status {
            144 | 153 => {
                if midi_message.data1 < self.config.max_note {
                    self.tx_strip.send(midi_message);
                }
            }
            _ => {}
        }
    }
}