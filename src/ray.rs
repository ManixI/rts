use crate::matrix::Matrix;

use super::Coord;
use intersection::*;

pub mod intersection;

#[derive(Debug,PartialEq, Clone, Copy)]
pub struct Ray {
    origin: Coord,
    direction: Coord,
    norm_dir: Coord
}

pub trait Intersect<T> {
    // trait that implements intersection for any object and a ray
    fn intersect(&self, ray: &Ray) -> Option<[Intersection<T>; 2]>;
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: Coord, direction: Coord) -> Self {
        Ray { origin, direction: direction, norm_dir: direction.normalized() }
    }

    pub fn get_norm_direction(&self) -> Coord {
       self.norm_dir
    }

    fn position(&self, time: f32) -> Coord {
        self.origin + self.direction * time
    }

    pub fn intersect<T: Intersect<T>>(&self, object: &impl Intersect<T>) -> Option<[Intersection<T>; 2]> {
        object.intersect(&self)
    }

    pub fn get_origin(&self) -> Coord {
        self.origin
    }

    pub fn get_direction(&self) -> Coord {
        self.direction
    }

    pub fn transform(&self, mat: Matrix) -> Self {
        todo!()
    }
}

// TODO NOW: figure out normalization


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let o = Coord::point(1.0, 2.0, 3.0);
        let d = Coord::vec(4.0, 5.0, 6.0);
        let r = Ray::new(o, d);
        assert_eq!(r.direction, d);
        assert_eq!(r.origin, o);
        assert_eq!(r.norm_dir, d.normalized());
    }

    #[test]
    fn test_position() {
        let r = Ray::new(Coord::point(2.0, 3.0, 4.0), Coord::vec(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), Coord::point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Coord::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Coord::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Coord::point(4.5, 3.0, 4.0));
    }

}
