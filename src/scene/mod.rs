mod object;
mod primitives;
pub mod camera;
mod render_mesh;
pub mod light_ray;

use std::convert::From;

extern crate sdl2;
use sdl2::render::{Canvas};
use sdl2::video::{Window};
use sdl2::pixels::Color as SdlColor;
use sdl2::rect::Rect;

use camera::Camera;
use object::{Intersect, Sphere};
use primitives::{Point, Color};
use render_mesh::{RenderMesh, RenderSquare};
use light_ray::LightRay;

pub struct Scene {
    objects: Vec<Box<Intersect>>,
    camera: Camera,
}

impl Scene {
    pub fn new(camera: Camera) -> Scene {
        let mut objects = Vec::<Box<Intersect>>::new();
        objects.push(Box::new(Sphere::new(
            Point::from((5.0, 0.0, 0.0)),
            2.5)));

        Scene {
            objects: objects,
            camera: camera,
        }
    }

    pub fn render<'b>(&self, canvas: &mut Canvas<Window>) {
        canvas.set_draw_color(SdlColor::RGB(0, 0, 0));
        canvas.clear();

        let mut render_mesh = RenderMesh::new(&self.camera);
        while (render_mesh.res() != 1) {
            let square_colors : Vec<(RenderSquare, Color)> = render_mesh.iter().map(|square| {
                let mut ray = LightRay::from(square.get_ray());
                let color = ray.trace(&self.objects);
                (square, color)
            }).collect();

            for (square, color) in square_colors {
                canvas.set_draw_color(SdlColor::RGB(color.r, color.g, color.b));
                canvas.draw_rect(Rect::new(
                    square.xl as i32,
                    square.yh as i32,
                    square.width(),
                    square.height()
                )).unwrap();
            }

            canvas.present();

            render_mesh.refine();
        }
    }
}
