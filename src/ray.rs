use crate::sphere::Sphere;

use super::Coord;

#[derive(Debug,PartialEq, Clone, Copy)]
pub struct Ray {
    origin: Coord,
    direction: Coord
}

pub trait Intersect {
    // trait that implements intersection for any object and a ray
    fn intersect(&self, ray: &Ray) -> Option<[f32; 2]>;
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: Coord, direction: Coord) -> Self {
        Ray { origin, direction }
    }

    fn position(&self, time: f32) -> Coord {
        self.origin + self.direction * time
    }

    pub fn intersect(&self, object: &impl Intersect) -> Option<[f32; 2]> {
        object.intersect(&self)
    }

    pub fn get_origin(&self) -> Coord {
        self.origin
    }

    pub fn get_direction(&self) -> Coord {
        self.direction
    }
}


#[cfg(test)]
mod tests {
    use crate::sphere::Sphere;

    use super::*;

    #[test]
    fn test_new() {
        let o = Coord::point(1.0, 2.0, 3.0);
        let d = Coord::vec(4.0, 5.0, 6.0);
        let r = Ray::new(o, d);
        assert_eq!(r.direction, d);
        assert_eq!(r.origin, o);
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
