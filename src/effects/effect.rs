use midi_message::MidiMessage;
use color_strip::ColorStrip;

pub trait Effect {
    fn paint(&mut self, color_strip: &mut ColorStrip);
    fn on_midi_message(&mut self, midi_message: MidiMessage);
    fn tick(&mut self);
}

