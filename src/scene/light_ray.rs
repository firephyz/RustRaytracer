use std::cmp::Ordering;

use crate::scene::object::Intersect;
use super::primitives::{Ray, Point, Color};

pub struct LightRay {
    ray: Ray,
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
// impl From<(f64, f64, f64, f64, f64, f64)> for Ray {
//     fn from(pos: (f64, f64, f64, f64, f64, f64)) -> Self {
//         Ray {
//             position: Point::from((pos.0, pos.1, pos.2)),
//             direction: Point::from((pos.3, pos.4, pos.5)).normalize(),
//             color: Color::from((0, 0, 0)),
//         }
//     }
// }
