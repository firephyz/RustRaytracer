use std::convert::From;

use super::Point;

#[derive(Clone, Debug)]
pub struct Ray {
    pub position: Point,
    pub direction: Point,
}

impl Ray {
    pub fn new(position: Point, direction: Point) -> Self {
        Ray {
            position: position,
            direction: direction.normalize(),
        }
    }
}

impl From<(f64, f64, f64, f64, f64, f64)> for Ray {
    fn from(pos: (f64, f64, f64, f64, f64, f64)) -> Self {
        Ray::new(Point::from((pos.0, pos.1, pos.2)),
            Point::from((pos.3, pos.4, pos.5)))
    }
}
