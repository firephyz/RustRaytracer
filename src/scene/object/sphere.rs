use crate::scene::object::{Intersect};
use crate::scene::primitives::{Point, Ray, Color};
use crate::scene::LightRay;

pub struct Sphere {
    position: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(pos: Point, radius: f64) -> Sphere {
        Sphere {
            position: pos,
            radius: radius,
        }
    }
}

impl Intersect for Sphere {
    fn intersect(&self, ray: &LightRay) -> Option<(Ray, f64, Color)> {
        let t = (ray.dir().dot(&self.position) - ray.dir().dot(&ray.pos())) / ray.dir().dot(&ray.dir());
        let point_min_dist = ray.pos().add(&ray.dir().mult(t));

        let dist_vec = point_min_dist.add(&self.position.mult(-1.0));
        let distance = dist_vec.abs();

        if distance <= self.radius && (t > 0.0) {
            let t = t - (self.radius.powi(2) - distance.powi(2)).sqrt();
            let intersection = ray.pos().add(&ray.dir().mult(t));
            let normal_dir = intersection.add(&self.position.mult(-1.0)).normalize();
            let normal = Ray::new(intersection, normal_dir);

            Some((normal, t, Color::from((255, 255, 255))))
        }
        else {
            None
        }
    }
}
