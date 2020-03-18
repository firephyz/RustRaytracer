use std::convert::From;

#[derive(Clone, Debug)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color {

    pub fn new() -> Self {
        Color {r: 0, g: 0, b: 0}
    }

    pub fn mix(&mut self, color: &Color) {
        // Float functions not const???
//        const MAX_ABS : f64 = (3.0 * (255 as f64).powi(2)).sqrt();
        const MAX_ABS : f64 = 441.6729560;

        let mut new_r = (self.r + color.r) as f64;
        let mut new_g = (self.g + color.g) as f64;
        let mut new_b = (self.b + color.b) as f64;

        let abs = (new_r.powi(2) + new_g.powi(2) + new_b.powi(2)).sqrt();
        new_r = new_r * abs / MAX_ABS;
        new_g = new_g * abs / MAX_ABS;
        new_b = new_b * abs / MAX_ABS;

        self.r = new_r.floor() as u8;
        self.g = new_g.floor() as u8;
        self.b = new_b.floor() as u8;
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from(pos: (u8, u8, u8)) -> Self {
        Color {
            r: pos.0,
            g: pos.1,
            b: pos.2,
        }
    }
}
