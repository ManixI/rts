use core::f32;

use crate::ray::{Intersect, Ray};

use super::Coord;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Sphere {
    origin: Coord,
    radius: f32,
}

#[allow(dead_code)]
impl Sphere {
    /// a sphere at position (0, 0, 0) with a radius of 1
    pub fn default() -> Self {
        Self { origin: Coord::point(0.0, 0.0, 0.0), radius: 1.0 }
    }
}

const EPSILON: f32 = 0.02;
impl Intersect for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<f32> {
        let a = self.origin - ray.get_origin();
        println!("a: {:?}", a);
        println!("|a|: {:?}", a.len());
        if a.len() == 0.0 {
            println!();
            return vec![0.0, 0.0];
        }

        let b = ((a.cross(ray.get_direction()).len())/(a.len() * ray.get_direction().len()));
        let b_degrees = b.asin() * (180.0/f32::consts::PI);
        println!("B: {:?}", b_degrees);
        if b_degrees == f32::NAN {
            println!();
            return Vec::<f32>::with_capacity(0);
        }
        // ray goes though center of origin
        if b == 0.0 {
            println!();
            // TODO: calc area of sphere
            return vec![a.len(), a.len()];
        }
        let b = b * (a.len() / (f32::consts::PI/2.0-b.asin()).sin());
        println!("b: {:?}", b);
        if b - self.radius > EPSILON {
            println!();
            return Vec::<f32>::with_capacity(0);
        }
        println!();
        vec![0.0, 0.0]
        //todo!()
    }
}