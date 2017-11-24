use opc_strip::OpcStrip;

use std::io;
use color_strip::ColorStrip;
use effects::effect::Effect;
use effects::ripple::Ripple;
use effects::flash::Flash;
use effects::blink::Blink;
use effects::stream::Stream;
use effects::push::Push;
use effects::river::River;
use effects::fish::Fish;
use effects::stream_center::StreamCenter;
use color::Color;
use std::sync::mpsc;

use midi_message::MidiMessage;

use std::thread;
use std::time::Duration;

enum MidiLightMessage {
    MidiMessage(MidiMessage),
    Reconfigure(MidiLightPatch),
    Stop
}


#[derive(Default, Clone)]
pub struct MidiLightConfig {
    pub led_count: usize,
    pub reversed: bool,
    pub patch: MidiLightPatch
}


pub struct MidiLightStrip {
    tx_strip: mpsc::Sender<MidiLightMessage>,
}

impl MidiLightStrip {
    pub fn start(config: MidiLightConfig) -> io::Result<MidiLightStrip> {
        let opc_strip = OpcStrip::connect(config.led_count, config.reversed)?;
        let (tx_strip, rx_strip) = mpsc::channel();

        thread::spawn(move || {
            let mut my_thread = MidiLightStripThread::new(opc_strip, rx_strip, config);
            my_thread.run();
        });

        Ok(MidiLightStrip { tx_strip })
    }

    pub fn on_midi_message(&self, midi_message: MidiMessage) {
        self.tx_strip.send(MidiLightMessage::MidiMessage(midi_message)).ok();
    }

    pub fn reconfigure(&self, midi_light_patch: &MidiLightPatch) {
        self.tx_strip.send(MidiLightMessage::Reconfigure(midi_light_patch.clone())).ok();
    }

    pub fn on_raw_midi_message(&self, status_and_channel: u8, data1: u8, data2: u8) {
        self.on_midi_message(MidiMessage::new(status_and_channel, data1, data2));
    }

    pub fn stop(&self) {
        self.tx_strip.send(MidiLightMessage::Stop).ok();
    }
}


pub struct MidiLightStripThread {
    opc_strip: OpcStrip,
    rx_strip: mpsc::Receiver<MidiLightMessage>,
    effects: Vec<Box<Effect>>,
    config: MidiLightConfig
}

impl MidiLightStripThread {
    fn new(opc_strip: OpcStrip,
           rx_strip: mpsc::Receiver<MidiLightMessage>,
           config: MidiLightConfig) -> MidiLightStripThread {
        let mut result = MidiLightStripThread { rx_strip, config, opc_strip, effects: vec![] };
        result.init();
        result
    }

    fn init(&mut self) {
        self.effects.clear();
        let midi_light_patch = &self.config.patch;

        if midi_light_patch.push {
            self.effects.push(Box::new(Push::new(self.config.led_count)));
        }
        if midi_light_patch.stream {
            self.effects.push(Box::new(Stream::new(self.config.led_count)));
        }
        if midi_light_patch.stream_center {
            self.effects.push(Box::new(StreamCenter::new(self.config.led_count)));
        }
        if midi_light_patch.flash {
            self.effects.push(Box::new(Flash::new(self.config.led_count)));
        }
        if midi_light_patch.ripples {
            self.effects.push(Box::new(Ripple::new(self.config.led_count)));
        }
        if midi_light_patch.river {
            self.effects.push(Box::new(River::new(self.config.led_count)));
        }

        if midi_light_patch.fish {
            self.effects.push(Box::new(Fish::new(self.config.led_count)));
        }

        if midi_light_patch.blink {
            if midi_light_patch.flash || midi_light_patch.stream {
                self.effects.push(Box::new(Blink::new_with_add_color(Color::gray(200))));
            } else {
                self.effects.push(Box::new(Blink::new()));
            }
        }
    }


    pub fn run(&mut self) {
        let mut color_strip = ColorStrip::new(self.config.led_count);

        loop {
            if let Ok(midi_light_message) = self.rx_strip.try_recv() {
                match midi_light_message {
                    MidiLightMessage::MidiMessage(midi_message) => {
                        let forward_message = match midi_message {
                            MidiMessage::NoteOn(_, note, _) if note >= self.config.patch.max_note => false,
                            _ => true
                        };
                        if forward_message {
                            for effect in &mut self.effects {
                                effect.on_midi_message(midi_message);
                            }
                        }
                    }
                    MidiLightMessage::Reconfigure(midi_light_patch) => {
                        self.config.patch = midi_light_patch;
                        self.init();
                    }
                    MidiLightMessage::Stop => {
                        break;
                    }
                }
            }

            color_strip.clear(Color::black());

            for effect in &mut self.effects {
                effect.paint(&mut color_strip);
                effect.tick();
            }

            self.opc_strip.send(&color_strip);

            thread::sleep(Duration::from_millis(10));
        }
    }
}


#[derive(Default, Clone)]
pub struct MidiLightPatch {
    pub fish: bool,
    pub river: bool,
    pub blink: bool,
    pub flash: bool,
    pub stream: bool,
    pub stream_center: bool,
    pub ripples: bool,
    pub push: bool,
    pub max_note: u8
}