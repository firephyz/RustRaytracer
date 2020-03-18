use std::f64::consts::PI;

use super::primitives::{Point, Rotation};

pub struct Camera {
    pub position: Point,
    pub rotations: (f64, f64, f64),
    pub width: u32,
    pub height: u32,
    pub rwidth: f64,
    pub rheight: f64,
    pub rdepth: f64,
}



impl Camera {
    pub fn new(position: (f64, f64, f64), rotations: (f64, f64, f64), width: u32, height: u32, fov: f64) -> Self {
        let rdepth = 1.0;
        let rwidth = (PI*fov/360.0).tan() * rdepth;
        let rheight = ((width as f64) / (height as f64)) * rwidth;
        Camera {
            position: Point::from(position),
            rotations: rotations,
            width: width,
            height: height,
            rwidth: rwidth,
            rheight: rheight,
            rdepth: rdepth,
        }
    }

    // transform an offset in pixel space (origin upper left), to a vector in 3d space
    pub fn transform(&self, x: f64, y: f64) -> Point {
        // move origin to center of camera
        let x = x - (self.width as f64 / 2.0);
        let y = -(y - (self.height as f64 / 2.0));

        // convert from pixel scale to real scale
        let x = (x as f64) / (self.width as f64) * self.rwidth;
        let y = (y as f64) / (self.height as f64) * self.rheight;
        let dist_origin = (x*x + y*y).sqrt();

        // account for camera rotations
        let point = Point::from((self.rdepth, x, y)).normalize();
        let point = point.rotate(self.rotations.2, Rotation::Planar);
        let point = point.rotate(self.rotations.1, Rotation::Z);
        let point = point.rotate(self.rotations.0, Rotation::XY);

        // add camera position offset
        let point = point.add(&self.position);
        point
    }
}
