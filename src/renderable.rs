use std::{fmt::Debug, rc::Rc};
use crate::{coord::Coord, material::Material, matrix::Matrix, ray::Ray};

#[derive(PartialEq, Debug)]
pub enum RenderableType {
    Sphere,
    Plane
}

pub trait RenderableBase {
    fn get_material(&self) -> Material;

    fn set_material(&mut self, mat: Material);

    fn get_pos(&self) -> Coord;

    fn get_transformation(&self) -> Matrix;

    fn set_transformation(&mut self, transform: Matrix);

    fn apply_transformation(&mut self, transform: Matrix);

    fn get_type(&self) -> RenderableType;

    fn clone_rc(&self) -> Rc<dyn Renderable>;

    fn clone_dyn(&self) -> Box<dyn Renderable>;
}

/**
 * macro to auto-define getters and setters for a renderable
 */
#[macro_export]
macro_rules! impl_renderable_base {
    ($type:ty, $variant:expr) => {
        impl crate::renderable::RenderableBase for $type {
            // TODO: should return reference not actual material? 
            fn get_material(&self) -> Material { self.material }
            fn set_material(&mut self, mat: Material) { self.material = mat; }
            fn get_pos(&self) -> Coord { self.transformation.to_point() }
            fn get_transformation(&self) -> Matrix { self.transformation.clone() }
            fn set_transformation(&mut self, transform: Matrix) { self.transformation = transform }
            fn apply_transformation(&mut self, transform: Matrix) { self.transformation = self.get_transformation() * transform }
            fn get_type(&self) -> RenderableType { $variant }
            fn clone_rc(&self) -> Rc<dyn Renderable> { Rc::new(self.clone()) }
            fn clone_dyn(&self) -> Box<dyn Renderable> { Box::new(self.clone()) }
        }
        
    };
}

/**
 * trait to define an object as renderable by the engine
 * requires RenderableBase implementation (use impl_renderable_base(Type, RenderableBase:Type))
 */
pub trait Renderable: RenderableBase {
    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>>;

    fn intersect_get_ray(&self, ray: Ray) -> (Ray, Option<Vec<Intersection>>);

    fn normal_at(&self, pos: Coord) -> Coord;

    fn default() -> Self where Self: Sized;
}

impl Clone for Box<dyn Renderable> {
  fn clone(&self) -> Self {
      self.clone_dyn()
  }
}

pub fn compare_renderables(a: &dyn Renderable, b: &dyn Renderable) {
    assert_eq!(a.get_material(), b.get_material());
    assert_eq!(a.get_pos(), b.get_pos());
    assert_eq!(a.get_transformation(), b.get_transformation());
    assert_eq!(a.get_type(), b.get_type());
}

#[derive(Clone)]
pub struct Intersection {
    t: f32,
    object: Rc<dyn Renderable>,
}

#[allow(dead_code)]
impl Intersection {
    pub fn new(t: f32, object: Rc<dyn Renderable>) -> Self {
        Self { t, object }
    }

    pub fn get_time(&self) -> f32 {
        self.t
    }

    pub fn get_object(&self) -> Rc<dyn Renderable> {
        self.object.clone()
    }

    pub fn get_object_pointer(&self) -> Rc<dyn Renderable> {
        self.object.clone()
    }

    /**
     * returns a sorted list of intersections, the earliest returned first
     */
    pub fn aggregate_intersections(mut data: Vec<Intersection>) -> Vec<Self> {
        data.sort_by(|a, b| a.get_time().total_cmp(&b.get_time()));
        data
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
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
         .field("time", &self.get_time())
         .field("object pos", &self.get_object().get_pos())
         .field("object material", &self.get_object().get_material())
         .field("object transformation", &self.get_object().get_transformation())
         .finish()
    }
}

/**
 * macro to auto-generate tests for classes implementing Renderable
 */
#[macro_export]
macro_rules! impl_renderable_tests {
    ($type:ty, $variant:expr) => {

        #[cfg(test)]
        mod macro_tests {
            use crate::matrix::Matrix;
            use crate::renderable::RenderableBase;
            use crate::material::Material;
            use crate::coord::Coord;
            use crate::ray::Ray;
            use crate::renderable::Renderable;
            use crate::renderable::RenderableType;

            #[test]
            fn test_has_transformation() {
                let o = <$type>::default();
                assert_eq!(o.get_transformation(), Matrix::identity(4));
            }

            #[test]
            fn test_assign_transform() {
                let mut o = <$type>::default();
                o.set_transformation(Matrix::translation(2.0, 3.0, 4.0));
                assert_eq!(o.get_transformation(), Matrix::translation(2.0, 3.0, 4.0))
            }

            #[test]
            fn test_default_shape_material() {
                let o = <$type>::default();
                assert_eq!(o.get_material(), Material::default());
            }

            #[test]
            fn test_set_material() {
                let mut m = Material::default();
                m.set_ambient(1.0);
                let mut o = <$type>::default();
                o.set_material(m.clone());
                assert_eq!(o.get_material(), m);
            }

            #[test]
            fn test_intersect_scaled() {
                let r = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
                let mut s = <$type>::default();
                s.set_transformation(Matrix::scaling(2.0, 2.0, 2.0));
                let (sr, _xs) = s.intersect_get_ray(r);
                assert_eq!(sr.get_origin(), Coord::point(0.0, 0.0, -2.5));
                assert_eq!(sr.get_direction(), Coord::vec(0.0, 0.0, 0.5));
            }

            #[test]
            fn test_intersect_translated() {
                let r = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
                let mut s = <$type>::default();
                s.set_transformation(Matrix::translation(5.0, 0.0, 0.0));
                let (sr, _xs) = s.intersect_get_ray(r);
                assert_eq!(sr.get_origin(), Coord::point(-5.0, 0.0, -5.0));
                assert_eq!(sr.get_direction(), Coord::vec(0.0, 0.0, 1.0));
            }

            #[test]
            fn test_normal_at_translate() {
                let mut s = <$type>::default();
                if s.get_type() == RenderableType::Plane {
                    return      // HACK: fixes immediate issue but should this test just be in Sphere?
                }
                s.set_transformation(Matrix::translation(0.0, 1.0, 0.0));
                let n = s.normal_at(Coord::point(0.0, 1.70711, -0.70711));
                assert_eq!(n, Coord::vec(0.0, 0.7071068, -0.70710677));
            }

            #[test]
            fn test_normal_at_scale() {
                let mut s = <$type>::default();
                if s.get_type() == RenderableType::Plane {
                    return      // HACK
                }
                s.set_transformation(Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotate_z(std::f32::consts::PI/5.0));
                let n = s.normal_at(Coord::point(0.0, 2.0_f32.sqrt()/2.0, -2.0_f32.sqrt()/2.0));
                assert_eq!(n, Coord::vec(0.0, 0.97014254, -0.24253564));
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use crate::{coord::Coord, ray::Ray, renderable::{Renderable, RenderableBase}, sphere::Sphere};
    use super::Intersection;

    fn compare(a: Rc<dyn Renderable>, b: Rc<dyn Renderable>) {
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
        let s = Rc::new(Sphere::default());
        let xs = s.intersect(r);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        compare(xs[0].get_object(), s.clone());
        //assert_eq!(xs[0].get_object(), &s);
        assert_eq!(xs[0].get_time(), 4.0);
        compare(xs[1].get_object(), s);
        //assert_eq!(xs[1].get_object(), &s);
        assert_eq!(xs[1].get_time(), 6.0);
    }

    #[test]
    fn test_intersection_aggregation() {
        let s = Sphere::default();
        let ray = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let mut intersections = Vec::new();
        intersections.append(&mut s.intersect(ray).unwrap());
        intersections.append(&mut s.intersect(ray).unwrap());
        let ray = Ray::new(Coord::point(5.0, 5.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        assert!(s.intersect(ray).is_none());
        let data = Intersection::aggregate_intersections(intersections);
        assert_eq!(data.len(), 4);
        let test = Intersection::new(4.0, Rc::new(s.clone()));
        compare_intersection(&data[0], &test);
        compare_intersection(&data[1], &test);
        //assert_eq!(data[0], test);
        //assert_eq!(data[2], test);
        let test = Intersection::new(6.0, Rc::new(s));
        compare_intersection(&data[2], &test);
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