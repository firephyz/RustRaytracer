mod object;
pub mod primitives;
//mod render_mesh;
mod light_ray;
mod light_source;

use std::convert::From;
use std::rc::Rc;
use std::cell::RefCell;

extern crate sdl2;
use sdl2::render::{Canvas};
use sdl2::video::{Window};
use sdl2::pixels::Color as SdlColor;
use sdl2::rect::Point as SdlPoint;

use crate::camera::Camera;
use object::{Intersect, Sphere};
use primitives::{Point, Color};
use light_ray::LightRay;
pub use light_source::LightSource;

pub struct Scene {
    pub lights: Vec<LightSource>,
    pub objects: Vec<Box<dyn Intersect>>,
    pub camera: Rc<RefCell<Camera>>,
}

impl Scene {
    pub fn new(camera: Rc<RefCell<Camera>>) -> Scene {
        let mut lights = Vec::<LightSource>::new();
        let mut objects = Vec::<Box<dyn Intersect>>::new();

        lights.push(LightSource::new(Point::from((10.0, -5.0, 3.5))));

        objects.push(Box::new(Sphere::new(
            Point::from((10.0, 0.0, -1.0)),
            Color::from((255, 255, 100)),
            1.0)));
        objects.push(Box::new(Sphere::new(
            Point::from((9.5, 0.0, 1.0)),
            Color::from((100, 100, 255)),
            0.25)));

        Scene {
            lights: lights,
            objects: objects,
            camera: camera,
        }
    }

    // TODO don't copy around the x and y's
    pub fn render(&self, canvas: &mut Canvas<Window>) {
        let camera = (*self.camera).borrow();
        let pixel_colors = camera.pixels_iter().map(|(x, y)| {
            let mut ray = LightRay::from(camera.get_ray(x, y));
            let color = ray.trace(&self);
            color
        }).collect::<Vec<Color>>();

        for (count, color) in pixel_colors.iter().enumerate() {
            // println!("{:?}", &color);
            canvas.set_draw_color(SdlColor::RGB(color.r, color.g, color.b));
            canvas.draw_point(SdlPoint::new(
                (count as i32) % (camera.width as i32),
                (count as i32) / (camera.width as i32))).unwrap();
        }
    }
}
