use std::convert::From;

use super::Rotation;

#[derive(Clone)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn abs(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let abs = self.abs();
        Point::from((self.x / abs, self.y / abs, self.z / abs))
    }

    pub fn rotate(&self, theta: f64, axis: Rotation) -> Self {
        match axis {
            Rotation::XY => Point {
                x: theta.cos() * self.x - theta.sin() * self.y,
                y: theta.sin() * self.x + theta.cos() * self.y,
                z: self.z,
            },
            Rotation::Z => Point {
                x: theta.cos() * self.x - theta.sin() * self.z,
                y: self.y,
                z: theta.sin() * self.x + theta.cos() * self.z,
            },
            Rotation::Planar => Point {
                x: self.x,
                y: theta.cos() * self.y - theta.sin() * self.z,
                z: theta.sin() * self.y + theta.cos() * self.z,
            },
        }
    }

    pub fn add(&self, point: &Point) -> Point {
        Point {
            x: self.x + point.x,
            y: self.y + point.y,
            z: self.z + point.z,
        }
    }

    pub fn mult(&self, value: f64) -> Point {
        Point {
            x: self.x * value,
            y: self.y * value,
            z: self.z * value,
        }
    }

    pub fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }
}

impl From<(f64, f64, f64)> for Point {
    fn from(pos: (f64, f64, f64)) -> Self {
        Point {
            x: pos.0,
            y: pos.1,
            z: pos.2,
        }
    }
}
