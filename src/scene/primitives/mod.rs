pub mod color;
pub mod point;
pub mod ray;

pub use color::Color;
pub use point::Point;
pub use ray::Ray;

pub enum Rotation {
    XY,
    Z,
    Planar, // rotation around normal
}
