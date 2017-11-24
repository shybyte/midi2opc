static RAINBOW: [[u8; 3]; 12] = [[255, 0, 0], [255, 128, 0], [255, 255, 0], [128, 255, 0], [0, 255, 0], [0, 255, 128], [0, 255, 255], [0, 127, 255], [0, 0, 255], [128, 0, 255], [255, 0, 255], [255, 0, 128]];

use color::Color;

pub fn get_rainbow_color(note: u8) -> Color {
    let rgb = RAINBOW[note as usize % 12 as usize];
    Color::new(rgb[0], rgb[1], rgb[2])
}