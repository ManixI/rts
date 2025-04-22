use std::rc::Rc;

use super::Intersect;

#[derive(Debug, PartialEq, Clone)]
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

    pub fn aggregate_intersections(data: Vec<Option<[Self; 2]>>) -> Vec<Self> {
        let mut out = Vec::<Self>::with_capacity(data.len() * 2);
        for val in data {
            if val.is_none() {
                continue;
            }
            let val = val.unwrap();
            for inter in val {
                out.push(inter);
            }
        }
        out
    }

    pub fn find_hit(data: &Vec<Self>) -> Option<&Self> {
        let mut out = None;
        for val in data {
            if val.get_time() >= 0.0 {
                if out.is_none() {
                    out = Some(val);
                } else if out.unwrap().get_time() > val.get_time() {
                    out = Some(val);
                }
            }
        }
        out
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

    #[test]
    fn test_intersection_aggregation() {
        let s = Sphere::default();
        let ray = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let mut intersections = Vec::new();
        intersections.push(s.intersect(&ray));
        intersections.push(s.intersect(&ray));
        let ray = Ray::new(Coord::point(5.0, 5.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        intersections.push(s.intersect(&ray));
        let data = Intersection::aggregate_intersections(intersections);
        assert_eq!(data.len(), 4);
        let test = Intersection::new(4.0, Rc::new(s.clone()));
        assert_eq!(data[0], test);
        assert_eq!(data[2], test);
        let test = Intersection::new(6.0, Rc::new(s));
        assert_eq!(data[1], test);
        assert_eq!(data[3], test);
    }

    #[test]
    fn test_detect_hit() {
        let s = Rc::new(Sphere::default());
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s.clone());
        let data = vec![i1.clone(), i2];
        assert_eq!(Intersection::find_hit(&data).unwrap(), &i1);

        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(1.0, s.clone());
        let data = vec![i1, i2.clone()];
        assert_eq!(Intersection::find_hit(&data).unwrap(), &i2);

        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(-2.0, s.clone());
        let data = vec![i1, i2];
        assert!(Intersection::find_hit(&data).is_none());
    
        let i1 = Intersection::new(5.0, s.clone());
        let i2 = Intersection::new(7.0, s.clone());
        let i3 = Intersection::new(-3.0, s.clone());
        let i4 = Intersection::new(2.0, s.clone());
        let data = vec![i1, i2, i3, i4.clone()];
        assert_eq!(Intersection::find_hit(&data).unwrap(), &i4);
    }
}