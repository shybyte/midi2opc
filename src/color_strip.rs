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
}