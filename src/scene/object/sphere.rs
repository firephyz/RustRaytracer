use crate::scene::object::{Intersect, SceneObject, SceneObjectType};
use crate::scene::primitives::{Point, Ray, Color};
use crate::scene::LightRay;

pub struct Sphere {
    radius: f64,
}

impl Sphere {
    pub fn new(pos: Point, radius: f64) -> SceneObject {
        SceneObject {
            pos: pos,
            typ: SceneObjectType::Sphere(Sphere {
                radius: radius,
            }),
        }
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &LightRay) -> Option<(Ray, f64, Color)> {
        Some((
            Ray::from((0.0, 0.0, 0.0, 0.0, 0.0, 0.0)),
            0.0,
            Color::from((0, 0, 0))))
    }
}
