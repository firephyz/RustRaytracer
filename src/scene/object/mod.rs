mod sphere;
pub use sphere::Sphere;

use crate::scene::primitives::{Ray, Color};
use crate::scene::LightRay;

pub trait Intersect {
    // returns normal at intersection, distance, and color picked up
    fn intersect(&self, ray: &LightRay) -> Option<(Ray, f64, Color)>;
}
