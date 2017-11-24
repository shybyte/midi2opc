use color::Color;

#[derive(Clone, Debug)]
pub struct ColorStrip {
    pub pixel: Vec<Color>
}

impl ColorStrip {
    pub fn new(size: usize) -> ColorStrip {
        ColorStrip { pixel: vec![Color::gray(0); size] }
    }

    pub fn insert(&mut self, c: Color) {
        self.pixel.insert(0, c);
        self.pixel.pop();
    }

    pub fn clear(&mut self, color: Color) {
        for p in &mut self.pixel {
            *p = color;
        }
    }

    pub fn blit(&mut self, color_strip: &ColorStrip) {
        for (target, &src) in self.pixel.iter_mut().zip(color_strip.pixel.iter()) {
            *target = src;
        }
    }

    pub fn add(&mut self, color_strip: &ColorStrip) {
        for (target, &src) in self.pixel.iter_mut().zip(color_strip.pixel.iter()) {
            *target += src;
        }
    }
}