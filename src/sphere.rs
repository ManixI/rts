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

//const EPSILON: f32 = 0.02;
impl Intersect for Sphere {
    // this is the geometric solution
    fn intersect(&self, ray: &Ray) -> Option<[f32; 2]> {
        // ref: https://discussions.unity.com/t/how-do-i-find-the-closest-point-on-a-line/588895/3
        let dir = ray.get_direction();//.normalized();
        let v = self.origin - ray.get_origin();
        let d = v.dot(dir);
        //if d < 0 {
        //    return None;
        //}
        println!("{} {}", d, self.radius);
        let nearest = ray.get_origin() + dir * d;
        //let dist = nearest.len();
        // better to square radius for comparison then sqrt the dist as dist isn't actually needed
        let dist = nearest.get_x().powi(2) + nearest.get_y().powi(2) + nearest.get_z().powi(2);
        //println!("dist: {:?}", dist);
        if dist > self.radius.powi(2) {
            //println!{"None"}
            return None;
        }
        // assume nearest point is exactly radius far away
        let mut c = 0.0;
        // if not, calculate actual distance
        if dist != self.radius.powi(2) {
            let a = self.radius;
            let b = dist;
            c = (a.powi(2) + b.powi(2)).sqrt();
            //println!("{} {}", a, b);
        }
        
        let mut out: [f32; 2] = [0.0; 2];
        
        let vec = nearest - (dir*c) - ray.get_origin();
        out[0] = dir.scalar_multiple(&vec).unwrap();

        let vec = nearest + (dir*c)- ray.get_origin();
        out[1] = dir.scalar_multiple(&vec).unwrap();

        //println!("t: {:?}\n", out);
        Some(out)
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_sphere_intersection_no_position() {
        let r = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());
    
        let r = Ray::new(Coord::point(0.0, 1.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());

        let r = Ray::new(Coord::point(0.0, 2.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_none());

        let r = Ray::new(Coord::point(0.0, 0.0, 0.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());

        let r = Ray::new(Coord::point(0.0, 0.0, 5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());

        let r = Ray::new(Coord::point(0.0, 1.0, -5.0), Coord::vec(0.0, -0.1, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());
    }

    #[test]
    fn test_sphere_intersection() {
        let r = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    
        let r = Ray::new(Coord::point(0.0, 1.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);

        let r = Ray::new(Coord::point(0.0, 2.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_none());

        let r = Ray::new(Coord::point(0.0, 0.0, 0.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);

        let r = Ray::new(Coord::point(0.0, 0.0, 5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }
}