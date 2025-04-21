use std::rc::Rc;

use super::Intersect;

pub struct Intersection<T> {
    t: f32,
    object: Rc<T>
}

#[allow(dead_code)]
impl<T: Intersect<T>> Intersection <T> {
    pub fn new(t: f32, object: Rc<T>) -> Self {
        Self { t, object }
    }

    pub fn get_time(&self) -> f32 {
        self.t
    }

    pub fn get_object(&self) -> &T {
        self.object.as_ref()
    }

    pub fn get_object_pointer(&self) -> Rc<T> {
        self.object.clone()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::{coord::Coord, ray::{Intersect, Ray}, sphere::Sphere};
    use super::Intersection;

    #[test]
    fn test_creation() {
        let s = Rc::new(Sphere::default());
        let intersection = Intersection::new(3.5, s.clone());
        assert_eq!(intersection.t, 3.5);
        assert_eq!(intersection.object, s);
    }

    #[test]
    fn test_create_2() {
        let r = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        assert_eq!(xs[0].get_object(), &s);
        assert_eq!(xs[0].get_time(), 4.0);
        assert_eq!(xs[1].get_object(), &s);
        assert_eq!(xs[1].get_time(), 6.0);
    }
}