//TODO Don't render previous calculed mesh points

use std::vec::Vec;
use std::iter::Iterator;
use std::ops::Rem;
use std::fmt::Debug;

use crate::scene::LightRay;
use crate::scene::primitives::{Point, Ray, Color};
use crate::scene::camera::Camera;

pub struct RenderSquare<'b> {
    mesh: &'b RenderMesh<'b>,
    pub xl: u32,
    pub xr: u32,
    pub yh: u32,
    pub yl: u32,
}

impl<'b> RenderSquare<'b> {
    pub fn width(&self) -> u32 {
        self.xr - self.xl + 1
    }

    pub fn height(&self) -> u32 {
        self.yl - self.yh + 1
    }

    pub fn get_ray(&self) -> Ray {
        let mid_x = (self.xl as f64 + self.xr as f64) / 2.0;
        let mid_y = (self.yh as f64 + self.yl as f64) / 2.0;
        let direction = self.mesh.camera.transform(mid_x, mid_y).normalize();

        Ray::new(self.mesh.camera.position.clone(), direction)
    }
}

impl<'b> Debug for RenderSquare<'b> {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> Result<(), std::fmt::Error> {
        write!(fmt, "RenderSquare {{\n");
        write!(fmt, "\txl: {}", self.xl);
        write!(fmt, "\txr: {}\n", self.xr);
        write!(fmt, "\tyh: {}", self.yh);
        write!(fmt, "\tyl: {}\n", self.yl);
        write!(fmt, "}}");
        Ok(())
    }
}

pub struct RenderMeshIterator<'b> {
    mesh: &'b RenderMesh<'b>,
    iter_index: usize,
}

impl<'b> Iterator for RenderMeshIterator<'b> {
    type Item = RenderSquare<'b>;
    fn next(&mut self) -> Option<Self::Item> {
        let result = if (self.iter_index == (self.mesh.xvals.len() - 1) * (self.mesh.yvals.len() - 1)) {
            None
        }
        else {
            let x_index = self.iter_index.rem(self.mesh.xvals.len() - 1);
            let y_index = self.iter_index / (self.mesh.yvals.len() - 1);

            Some(RenderSquare {
                mesh: self.mesh,
                xl: self.mesh.xvals[x_index],
                xr: self.mesh.xvals[x_index + 1],
                yh: self.mesh.yvals[y_index],
                yl: self.mesh.yvals[y_index + 1],
            })
        };

        self.iter_index += 1;
        result
    }
}

pub struct RenderMesh<'b> {
    camera: &'b Camera,
    xvals: Vec<u32>,
    yvals: Vec<u32>,
    xres: u32,
    yres: u32,
}

impl<'b> RenderMesh<'b> {
    pub fn new(camera: &'b Camera) -> Self {
        let xvals = vec![0, camera.width];
        let yvals = vec![0, camera.height];
        RenderMesh {
            camera:camera,
            xvals: xvals,
            yvals: yvals,
            xres: camera.width,
            yres: camera.height,
        }
    }

    // Returns 1 when minimum resolution is reached for each dimension
    pub fn res(&self) -> u32 {
        self.xres * self.yres
    }

    pub fn refine(&mut self) {
        let (xvals, min_x_diff) = Self::refine_vector(&mut self.xvals);
        let (yvals, min_y_diff) = Self::refine_vector(&mut self.yvals);

        self.xvals = xvals;
        self.yvals = yvals;
        self.xres = min_x_diff;
        self.yres = min_y_diff;
    }

    fn refine_vector(vals: &mut Vec<u32>) -> (Vec<u32>, u32) {
        let mut new_vals = Vec::<u32>::with_capacity(2 * vals.len() - 1);

        // always guaranteed to have at least two elements by constructor
        let mut min_diff = vals[1] - vals[0];
        let prev_values = vals.iter().take(vals.len()-1);
        let next_values = vals.iter().skip(1);
        for (prev, next) in prev_values.zip(next_values) {
            // keep current mesh
            new_vals.push(*prev);

            // if theres room to refine, refine between existing mesh
            if (next - prev > 1) {
                let new_mesh_value = (next + prev) / 2;
                new_vals.push(new_mesh_value);

                min_diff = min_diff.min(new_mesh_value - prev).min(next - new_mesh_value);
            }
        }

        // don't forget last existing mesh value
        new_vals.push(vals[vals.len() - 1]);

        (new_vals, min_diff)
    }

    pub fn iter(&self) -> RenderMeshIterator {
        RenderMeshIterator {
            mesh: &self,
            iter_index: 0,
        }
    }
}

//impl Iterator for
