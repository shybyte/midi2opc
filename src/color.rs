use std::ops::{Add, Sub};
use std::cmp::{max, min};


#[derive(Copy, Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8) -> Color {
        Color { r, g, b }
    }

    pub fn gray(v: u8) -> Color {
        Color { r: v, g: v, b: v }
    }
}

impl Sub for Color {
    type Output = Color;

    fn sub(self, other: Color) -> Self::Output {
        Color {
            r: sub_u8_safe(self.r, other.r),
            g: sub_u8_safe(self.g, other.g),
            b: sub_u8_safe(self.b, other.b),
        }
    }
}

impl Add for Color {
    type Output = Color;

    fn add(self, other: Color) -> Self::Output {
        Color {
            r: add_u8_safe(self.r, other.r),
            g: add_u8_safe(self.g, other.g),
            b: add_u8_safe(self.b, other.b),
        }
    }
}


fn sub_u8_safe(x: u8, y: u8) -> u8 {
    max(x as i32 - y as i32 , 0) as u8
}

fn add_u8_safe(x: u8, y: u8) -> u8 {
    min(x as i32 + y as i32 , 255) as u8
}