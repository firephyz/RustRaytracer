use std::cmp::Ordering;
use std::fmt::Debug;

use crate::scene::object::Intersect;
use super::primitives::{Ray, Point, Color};

pub struct LightRay {
    pub ray: Ray,
    color: Color,
}

impl LightRay {
    pub fn new(ray: Ray, color: Color) -> LightRay {
        LightRay {
            ray: ray,
            color: color,
        }
    }

    pub fn pos(&self) -> &Point {
        &self.ray.position
    }

    pub fn dir(&self) -> &Point {
        &self.ray.direction
    }

    pub fn trace(&mut self, objects: &Vec<Box<Intersect>>) -> Color {
        const NUM_RAYS: u32 = 1; // number of reflections

        for ray_index in 0..NUM_RAYS {
            let intersection = self.find_closest_intersection(&objects);
            //println!("{:?}", &intersection);

            match intersection {
                None => {
                    self.color.mix(&Color::from((128, 128, 128)));
                    break;
                },
                Some((normal, color)) => {
                    // modify starting point of ray to compute reflection
                    self.ray = Ray::new(normal.position.clone(), self.reflect(&normal));
                    self.color.mix(&color);
                },
            }
        }

        self.color.clone()
    }

    // Returns normal to intersection and color picked up
    fn find_closest_intersection(&self, objects: &Vec<Box<Intersect>>) -> Option<(Ray, Color)> {
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
        LightRay::new(ray, Color::new())
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
