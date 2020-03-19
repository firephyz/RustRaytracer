use std::cmp::Ordering;
use std::fmt::Debug;

use crate::scene::{Scene, LightSource};
use crate::scene::object::Intersect;
use super::primitives::{Ray, Point, Color};

pub struct LightRay {
    pub ray: Ray,
    color: Color,
}

impl LightRay {
    pub fn new(ray: Ray) -> LightRay {
        LightRay {
            ray: ray,
            color: Color::from((0, 0, 0)),
        }
    }

    // pub fn new(ray: Ray, color: Color) -> LightRay {
    //     LightRay {
    //         ray: ray,
    //         color: color,
    //     }
    // }

    pub fn pos(&self) -> &Point {
        &self.ray.position
    }

    pub fn dir(&self) -> &Point {
        &self.ray.direction
    }

    pub fn trace(&mut self, scene: &Scene) -> Color {
        const NUM_RAYS: u32 = 1; // number of reflections

        for _ray_index in 0..NUM_RAYS {
            let intersection = self.find_closest_intersection(&scene.objects);

            match intersection {
                None => {
                    self.color.mix(&Color::from((128, 128, 128)));
                    break;
                },
                Some((normal, color)) => {
                    // modify starting point of ray to compute reflection
                    let bounce_ray = Ray::new(normal.position.clone(), self.reflect(&normal));
                    let shadow_scalar = self.compute_shadows(&normal, &scene.lights, &scene.objects);
                    let color = color.scale(shadow_scalar);

                    self.ray = bounce_ray;
                    self.color.mix(&color);
                },
            }
        }

        self.color.clone()
    }

    // computes ratio 1.0 to 0.0 of intensity of light
    // TODO does intersections interior type get optimized away since
    //   we only care about the count?
    fn compute_shadows(&self,
        normal: &Ray,
        lights: &Vec<LightSource>,
        objects: &Vec<Box<dyn Intersect>>) -> f64 {

        // compute intensities from each light source
        let intensities = lights.iter().map(|light| {
            // compute ray to light source
            let ray_dir = light.position.add(&normal.position.mult(-1.0));
            let ray_position = normal.position.add(&normal.direction.mult(1e-20));
            let ray = LightRay::new(Ray::new(ray_position, ray_dir));

            // count number of intersections
            let intersections : Vec<_> = objects.iter().filter_map(|obj| {
                obj.intersect(&ray)
            }).collect();

            if intersections.len() == 0 {
                let cos_theta = normal.direction.dot(&ray.ray.direction) / ray.ray.direction.abs();
                cos_theta.powi(2)
            }
            else {
                0.0
            }
        }).collect::<Vec<f64>>();

        intensities.iter().fold(0.0, |value, &intensity| {
            value + (intensity / intensities.len() as f64)
        })
    }

    // Returns normal to intersection and color picked up
    fn find_closest_intersection(&self, objects: &Vec<Box<dyn Intersect>>) -> Option<(Ray, Color)> {
        // remove Nones
        // TODO examine
        let intersected = objects.iter().filter_map(|obj| {
            obj.intersect(&self)
        });

        // Intersects at Point, f64 distance away with object of intersection and ray color Color
        let mut sorted = intersected.collect::<Vec<(Ray, f64, Color)>>();
        sorted.sort_by(|tup, other| {
            if tup.1 < other.1 {
                Ordering::Less
            }
            else if tup.1 > other.1 {
                Ordering::Greater
            }
            else {
                Ordering::Equal
            }
        });

        // Check if no intersection
        if sorted.len() == 0 {
            None
        }
        else {
            let closest = sorted[0].clone();
            Some((closest.0, closest.2))
        }
    }

    // Reflect self ray across normal and normalize
    fn reflect(&self, normal: &Ray) -> Point {
        Point::from((0.0, 0.0, 0.0))
    }
}

impl From<Ray> for LightRay {
    fn from(ray: Ray) -> Self {
        LightRay::new(ray)
    }
}

impl Debug for LightRay {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "({}, {}, {})\t({}, {}, {})",
            self.ray.position.x,
            self.ray.position.y,
            self.ray.position.z,
            self.ray.direction.x,
            self.ray.direction.y,
            self.ray.direction.z);
        Ok(())
    }
}
