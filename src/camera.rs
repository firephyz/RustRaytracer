use std::f64::consts::PI;

use crate::scene::primitives::{Point, Ray, Rotation};

#[derive(Debug)]
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
        let rdepth = 0.05;
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
        //let point = point.add(&self.position);
        point
    }

    pub fn pixels_iter(&self) -> CameraPixelIterator {
        CameraPixelIterator::new(&self)
    }

    pub fn get_ray(&self, x: u32, y: u32) -> Ray {
        let direction = self.transform(x, y);
        Ray::new(self.position.clone(), direction)
    }

    pub fn move_rotate(&mut self, yaw: i32, pitch: i32, roll: i32) {
        let yaw = 0.005 * (yaw as f64);
        let pitch = 0.005 * (pitch as f64);
        self.rotations.0 += yaw * self.rotations.2.cos() - pitch * self.rotations.2.sin();
        self.rotations.1 += yaw * self.rotations.2.sin() + pitch * self.rotations.2.cos();
        self.rotations.2 += 0.005 * (roll as f64);
    }

    pub fn move_translate(&mut self, fb: i32, lr: i32) {
        let normal = Point::from((1.0, 0.0, 0.0));
        let normal = normal.rotate(self.rotations.2, Rotation::Planar);
        let normal = normal.rotate(self.rotations.1, Rotation::Z);
        let normal = normal.rotate(self.rotations.0, Rotation::XY);
        let left_axis = Point::from((0.0, 1.0, 0.0));
        let left_axis = left_axis.rotate(self.rotations.2, Rotation::Planar);
        let left_axis = left_axis.rotate(self.rotations.1, Rotation::Z);
        let left_axis = left_axis.rotate(self.rotations.0, Rotation::XY);

        self.position.x += 0.01 * (fb as f64) * normal.x;
        self.position.y += 0.01 * (fb as f64) * normal.y;
        self.position.z += 0.01 * (fb as f64) * normal.z;

        self.position.x += 0.01 * (lr as f64) * left_axis.x;
        self.position.y += 0.01 * (lr as f64) * left_axis.y;
        self.position.z += 0.01 * (lr as f64) * left_axis.z;

        //println!("{:?}", self.position);
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

        if self.index == (self.camera.width * self.camera.height) {
            None
        }
        else {
            let result = Some((self.index % self.camera.width, self.index / self.camera.width));
            self.index += 1;
            result
        }
    }
}
