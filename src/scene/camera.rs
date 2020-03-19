use std::f64::consts::PI;

use super::primitives::{Point, Ray, Rotation};

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
        let rheight = ((height as f64) / (width as f64)) * rwidth;
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
    pub fn transform(&self, x: u32, y: u32) -> Point {
        // move origin to center of camera
        let x =  (x as i32) - (self.width / 2) as i32;
        let y = -(y as i32 - (self.height / 2) as i32);

        // convert from pixel scale to real scale
        let x = (x as f64) / (self.width as f64) * self.rwidth;
        let y = (y as f64) / (self.height as f64) * self.rheight;

        // account for camera rotations
        let point = Point::from((self.rdepth, x, y)).normalize();
        let point = point.rotate(self.rotations.2, Rotation::Planar);
        let point = point.rotate(self.rotations.1, Rotation::Z);
        let point = point.rotate(self.rotations.0, Rotation::XY);

        // add camera position offset
        let point = point.add(&self.position);
        point
    }

    pub fn pixels_iter(&self) -> CameraPixelIterator {
        CameraPixelIterator::new(&self)
    }

    pub fn get_ray(&self, x: u32, y: u32) -> Ray {
        let direction = self.transform(x, y);
        Ray::new(self.position.clone(), direction)
    }
}

pub struct CameraPixelIterator<'a> {
    camera: &'a Camera,
    index: u32,
}

impl<'a> CameraPixelIterator<'a> {
    pub fn new(camera: &Camera) -> CameraPixelIterator {
        CameraPixelIterator {
            camera: camera,
            index: 0,
        }
    }
}

impl<'a> Iterator for CameraPixelIterator<'a> {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {

        if self.index == (self.camera.width * self.camera.height - 1) {
            None
        }
        else {
            self.index += 1;
            Some((self.index % self.camera.width, self.index / self.camera.width))
        }
    }
}
