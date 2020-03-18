mod sphere;
pub use sphere::Sphere;

use crate::scene::primitives::{Point, Ray, Color};
use crate::scene::LightRay;

pub enum SceneObjectType {
    Sphere(Sphere),
}

pub struct SceneObject {
    pos: Point,
    typ: SceneObjectType, // optimize so only used during compilation?, type info that is
}

impl SceneObject {
    pub fn new(pos: Point, typ: SceneObjectType) -> Self {
        SceneObject {
            pos: pos,
            typ: typ,
        }
    }
}

impl Intersect for SceneObject {
    fn intersect(&self, ray: &LightRay) -> Option<(Ray, f64, Color)> {
        // can this be like a constexpr call?
        match &self.typ {
            SceneObjectType::Sphere(obj) => obj.intersect(ray),
        }
    }
}

pub trait Intersect {
    // returns normal at intersection, distance, and color picked up
    fn intersect(&self, ray: &LightRay) -> Option<(Ray, f64, Color)>;
}
