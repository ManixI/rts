use std::ops::Mul;

use crate::{matrix::Matrix, renderable::{Intersection, Renderable}};

use super::Coord;

#[derive(Debug,PartialEq, Clone, Copy)]
pub struct Ray {
    origin: Coord,
    direction: Coord,
    norm_dir: Coord
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: Coord, direction: Coord) -> Self {
        // TODO: is there a better way then calcing the norm for every new ray?
        let direction = direction.to_vec();
        Ray { origin, direction: direction, norm_dir: direction.normalized() }
    }

    pub fn get_norm_direction(&self) -> Coord {
       self.norm_dir
    }

    pub fn position(&self, time: f32) -> Coord {
        self.origin + self.direction * time
    }

    pub fn intersect(&self, object: &dyn Renderable) -> Option<[Intersection; 2]> {
        object.intersect(&self)
    }

    pub fn get_origin(&self) -> Coord {
        self.origin
    }

    pub fn get_direction(&self) -> Coord {
        self.direction
    }

    pub fn transform(&self, mat: Matrix) -> Self {
        Self::new(
            mat.clone() * self.origin,
            mat * self.direction
        )
    }
}



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

    #[test]
    fn test_transform() {
        let r = Ray::new(Coord::point(1.0, 2.0, 3.0), Coord::vec(0.0, 1.0, 0.0));
        let m = Matrix::translation(3.0, 4.0, 5.0);
        let new = r.transform(m);
        //println!("{:?}\n", new);
        assert_eq!(new.get_origin(), Coord::point(4.0, 6.0, 8.0));
        assert_eq!(new.get_direction(), Coord::vec(0.0, 1.0, 0.0));

        let m = Matrix::scaling(2.0, 3.0, 4.0);
        let new = r.transform(m);
        //println!("{:?}\n", new);
        assert_eq!(new.get_origin(), Coord::point(2.0, 6.0, 12.0));
        assert_eq!(new.get_direction(), Coord::vec(0.0, 3.0, 0.0));
    }

    #[test]
    fn test_is_vec() {
        let r = Ray::new(Coord::point(0.0, 0.0, 0.0), Coord::point(0.0, 1.0, 1.0));
        assert!(r.get_direction().is_vec());
    }
}
