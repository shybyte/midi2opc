extern crate midi_message;
extern crate rand;

pub mod color;
pub mod color_strip;
pub mod opc_strip;
pub mod midi_light_strip;
pub mod rainbow;

mod effects {
    pub mod effect;
    pub mod ripple;
    pub mod flash;
    pub mod blink;
    pub mod stream;
    pub mod stream_center;
    pub mod push;
    pub mod river;
    pub mod fish;
}