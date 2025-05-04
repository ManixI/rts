use std::{fmt::{self, Debug}, rc::Rc};

use crate::renderable::Renderable;

use super::Ray;

#[allow(dead_code)]
pub trait Intersect<T> {
    // trait that implements intersection for any object and a ray
    fn intersect(&self, ray: &Ray) -> Option<[Intersection; 2]>;
}


#[derive(Clone)]
pub struct Intersection {
    t: f32,
    object: Rc<dyn Renderable>
}

#[allow(dead_code)]
impl Intersection {
    pub fn new(t: f32, object: Rc<dyn Renderable>) -> Self {
        Self { t, object }
    }

    pub fn get_time(&self) -> f32 {
        self.t
    }

    pub fn get_object(&self) -> &dyn Renderable {
        self.object.as_ref()
    }

    pub fn get_object_pointer(&self) -> Rc<dyn Renderable> {
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

impl Debug for Intersection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Point")
         .field("time", &self.get_time())
         .field("object pos", &self.get_object().get_pos())
         .field("object material", &self.get_object().get_material())
         .field("object transformation", &self.get_object().get_transformation())
         .finish()
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::{coord::Coord, ray::{Intersect, Ray}, renderable::Renderable, sphere::Sphere};
    use super::Intersection;

    fn compare(a: &dyn Renderable, b: &dyn Renderable) {
        assert_eq!(a.get_material(), b.get_material());
        assert_eq!(a.get_pos(), b.get_pos());
        assert_eq!(a.get_transformation(), b.get_transformation());
        assert_eq!(a.get_type(), b.get_type());
    }

    fn compare_intersection(a: &Intersection, b: &Intersection) {
        assert_eq!(a.get_time(), b.get_time());
        compare(a.get_object(), b.get_object());
    }

    #[test]
    fn test_creation() {
        let s = Rc::new(Sphere::default());
        let intersection = Intersection::new(3.5, s.clone());
        assert_eq!(intersection.t, 3.5);
        //compare(intersection.object, s);
        assert_eq!(intersection.object.get_material(), s.get_material());
        assert_eq!(intersection.object.get_pos(), s.get_pos());
        assert_eq!(intersection.object.get_transformation(), s.get_transformation());
    }

    #[test]
    fn test_create_2() {
        let r = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = s.intersect(&r);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        compare(xs[0].get_object(), &s);
        //assert_eq!(xs[0].get_object(), &s);
        assert_eq!(xs[0].get_time(), 4.0);
        compare(xs[1].get_object(), &s);
        //assert_eq!(xs[1].get_object(), &s);
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
        compare_intersection(&data[0], &test);
        compare_intersection(&data[2], &test);
        //assert_eq!(data[0], test);
        //assert_eq!(data[2], test);
        let test = Intersection::new(6.0, Rc::new(s));
        compare_intersection(&data[1], &test);
        compare_intersection(&data[3], &test);
        //assert_eq!(data[1], test);
        //assert_eq!(data[3], test);
    }

    #[test]
    fn test_detect_hit() {
        let s = Rc::new(Sphere::default());
        let i1 = Intersection::new(1.0, s.clone());
        let i2 = Intersection::new(2.0, s.clone());
        let data = vec![i1.clone(), i2];
        compare_intersection(Intersection::find_hit(&data).unwrap(), &i1);
        //assert_eq!(Intersection::find_hit(&data).unwrap(), &i1);

        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(1.0, s.clone());
        let data = vec![i1, i2.clone()];
        compare_intersection(Intersection::find_hit(&data).unwrap(), &i2);
        //assert_eq!(Intersection::find_hit(&data).unwrap(), &i2);

        let i1 = Intersection::new(-1.0, s.clone());
        let i2 = Intersection::new(-2.0, s.clone());
        let data = vec![i1, i2];
        assert!(Intersection::find_hit(&data).is_none());
    
        let i1 = Intersection::new(5.0, s.clone());
        let i2 = Intersection::new(7.0, s.clone());
        let i3 = Intersection::new(-3.0, s.clone());
        let i4 = Intersection::new(2.0, s.clone());
        let data = vec![i1, i2, i3, i4.clone()];
        compare_intersection(Intersection::find_hit(&data).unwrap(), &i4);
        //assert_eq!(Intersection::find_hit(&data).unwrap(), &i4);
    }
}