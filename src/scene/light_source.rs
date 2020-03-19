use crate::scene::primitives::Point;

pub struct LightSource {
// TODO implement intensity
//    pub intensity: f64
    pub position: Point,
}

impl LightSource {
    pub fn new(position: Point) -> Self {
        LightSource {
            position: position,
        }
    }
}
