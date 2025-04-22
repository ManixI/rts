use core::f32;
use std::rc::Rc;
use crate::matrix::Matrix;
use crate::ray::{Intersect, Ray};
use crate::ray::intersection::*;
use super::Coord;


#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    //origin: Coord,
    radius: f32,
    transformation: Matrix,
}

#[allow(dead_code)]
impl Sphere {
    /// a sphere at position (0, 0, 0) with a radius of 1
    pub fn default() -> Self {
        Self { radius: 1.0, transformation: Matrix::identity(4) }
    }

    pub fn new(origin: Coord, radius: f32) -> Self {
        assert!(origin.is_point());
        Self {radius, transformation: Matrix::from_point(&origin)}
    }

    pub fn set_transformation(&mut self, mat: Matrix) {
        self.transformation = mat;
    }

    // TODO: remove clone here
    pub fn apply_transformation(&mut self, mat: Matrix) {
        self.transformation = self.transformation.clone() * mat;
    }

    pub fn get_origin(&self) -> Coord {
        self.transformation.to_point()
    }

    pub fn get_transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    pub fn geometric_intersect(&self, ray: &Ray) -> Option<[f32; 2]> {
        // ref: https://discussions.unity.com/t/how-do-i-find-the-closest-point-on-a-line/588895/3
        let dir = ray.get_direction();//.normalized();
        let v = self.get_origin() - ray.get_origin();
        let d = v.dot(dir);
        
        //println!("{} {}", d, self.radius);
        let nearest = ray.get_origin() + dir * d;
        
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

        if out[0] > out[1] {
            let tmp = out[0];
            out[0] = out[1];
            out[1] = tmp;
        }
        //println!("t: {:?}\n", out);
        Some(out)
    }

    pub fn analytical_intersect(&self, ray: &Ray) -> Option<[f32; 2]> {
        let l = ray.get_origin() - self.get_origin();
        //let a = ray.get_direction().dot(ray.get_direction());
        let b = 2.0 * ray.get_norm_direction().dot(l);
        let c = l.dot(l) - self.radius.powi(2);
        quadratic_formula_helper(b, c) 
    }
}

/// assumes `a` value is 1 (ie ray direction is normalized)
fn quadratic_formula_helper(b: f32, c: f32) -> Option<[f32; 2]> {
    let disc = b.powi(2) - 4.0 * c;
    if disc < 0.0 {
        return None;
    } else if disc == 0.0 {
        let out = -0.5 * b;
        return Some([out, out]);
    }
    let quot = if b > 0.0 {-0.5 * (b + disc.sqrt())} else {-0.5 * (b - disc.sqrt())};
    let mut out = [
        quot,
        c/quot
    ];
    if out[0] > out[1] {
        let tmp = out[0];
        out[0] = out[1];
        out[1] = tmp;
    }
    Some(out)
}

//const EPSILON: f32 = 0.02;
impl Intersect<Self> for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<[Intersection<Self>; 2]> {
        //self.geometric_intersect(ray) 
        let data = self.analytical_intersect(ray);
        if data.is_none() {
            return None;
        }
        let data = data.unwrap();
        let t = Rc::new(self.clone());
        Some([Intersection::new(data[0], t.clone()), Intersection::new(data[1], t)])
    }
}

mod tests {
    use super::*;

    #[test]
    fn test_sphere_creation() {
        let s = Sphere::default();
        assert_eq!(s.radius, 1.0);
        assert_eq!(s.transformation, Matrix::identity(4));

        let s = Sphere::new(Coord::point(0.0, 0.0, 0.0), 2.0);
        assert_eq!(s.radius, 2.0);
        assert_eq!(s.transformation, Matrix::identity(4));
    }

    #[test]
    fn test_set_transformation() {
        let mut s = Sphere::default();
        let mat = Matrix::translation(2.0, 3.0, 4.0);
        s.set_transformation(mat.clone());
        assert_eq!(s.get_transformation(), mat);
        assert_eq!(s.get_origin(), mat.to_point());
    }

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
        let xs = [xs[0].get_time(), xs[1].get_time()];
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    
        let r = Ray::new(Coord::point(0.0, 1.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        let xs = [xs[0].get_time(), xs[1].get_time()];
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
        let xs = [xs[0].get_time(), xs[1].get_time()];
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);

        let r = Ray::new(Coord::point(0.0, 0.0, 5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        let xs = [xs[0].get_time(), xs[1].get_time()];
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);
    }
}