mod object;
mod primitives;
pub mod camera;
//mod render_mesh;
pub mod light_ray;

use std::convert::From;

extern crate sdl2;
use sdl2::render::{Canvas};
use sdl2::video::{Window};
use sdl2::pixels::Color as SdlColor;
use sdl2::rect::Point as SdlPoint;

use camera::Camera;
use object::{Intersect, Sphere};
use primitives::{Point, Color};
//use render_mesh::{RenderMesh, RenderSquare};
use light_ray::LightRay;

pub struct Scene {
    objects: Vec<Box<Intersect>>,
    camera: Camera,
}

impl Scene {
    pub fn new(camera: Camera) -> Scene {
        let mut objects = Vec::<Box<Intersect>>::new();
        objects.push(Box::new(Sphere::new(
            Point::from((10.0, 0.0, 0.0)),
            0.5)));
        objects.push(Box::new(Sphere::new(
            Point::from((15.0, 2.0, -3.0)),
            1.0)));

        Scene {
            objects: objects,
            camera: camera,
        }
    }

    // TODO don't copy around the x and y's
    pub fn render<'b>(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(SdlColor::RGB(0, 0, 0));
        canvas.clear();

        let pixel_colors = self.camera.pixels_iter().map(|(x, y)| {
            let mut ray = LightRay::from(self.camera.get_ray(x, y));
            let color = ray.trace(&self.objects);
            color
        }).collect::<Vec<Color>>();

        for (count, color) in pixel_colors.iter().enumerate() {
            // println!("{:?}", &color);
            canvas.set_draw_color(SdlColor::RGB(color.r, color.g, color.b));
            canvas.draw_point(SdlPoint::new(
                (count as i32) % (self.camera.width as i32),
                (count as i32) / (self.camera.width as i32))).unwrap();
        }
    }
}
