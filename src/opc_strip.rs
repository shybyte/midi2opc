use std::net::TcpStream;

use std::io::prelude::*;
use std::io;
use color_strip::ColorStrip;
use color::Color;


pub struct OpcStrip {
    stream: TcpStream,
    data: Vec<u8>,
    led_count: usize,
    reversed: bool
}

impl OpcStrip {
    pub fn connect(led_count: usize, reversed: bool) -> io::Result<OpcStrip> {
        let mut data: Vec<u8> = vec![0; 4 + (led_count * 3) as usize];
        data[3] = led_count as u8 * 3;
        TcpStream::connect("127.0.0.1:7890").map(|stream| OpcStrip { stream, data, led_count, reversed })
    }

    pub fn send(&mut self, color_strip: &ColorStrip, add_color: Color) {
        for (i, color) in color_strip.pixel[0..self.led_count].iter().enumerate() {
            let final_color = *color + add_color;
            let mut j = if self.reversed { self.data.len() - i * 3 - 3 } else { i * 3 + 4 };
            self.data[j] = final_color.r;
            j += 1;
            self.data[j] = final_color.g;
            j += 1;
            self.data[j] = final_color.b;
        }
        self.stream.write(&self.data[..]).ok();
    }
}

