use std::ops::{Add, Sub, AddAssign, SubAssign};
use std::cmp::{max, min};
use std::i32;


#[derive(Copy, Clone, Debug, Default)]
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

    pub fn black() -> Color {
        Color::default()
    }

    pub fn mul_with_opacity(&self, opacity: f32) -> Color {
        Color {
            r: mul_f32_safe(self.r, opacity),
            g: mul_f32_safe(self.g, opacity),
            b: mul_f32_safe(self.b, opacity),
        }
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

impl AddAssign for Color {
    fn add_assign(&mut self, other: Color) {
        *self = *self + other;
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, other: Color) {
        *self = *self - other;
    }
}


fn sub_u8_safe(x: u8, y: u8) -> u8 {
    max(i32::from(x) - i32::from(y), 0) as u8
}

fn add_u8_safe(x: u8, y: u8) -> u8 {
    min(i32::from(x) + i32::from(y), 255) as u8
}

fn mul_f32_safe(x: u8, y: f32) -> u8 {
    min((f32::from(x) * y) as i32, 255) as u8
}