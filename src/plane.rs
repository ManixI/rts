use crate::{tex::color::Color, coord::Coord, impl_renderable_base, impl_renderable_tests, material::Material, matrix::Matrix, ray::Ray, renderable::{Intersection, Renderable, RenderableBase, RenderableType}};
use std::rc::Rc;


#[derive(Clone, PartialEq)]
pub struct Plane {
    transformation: Matrix,
    material: Material,
}

#[allow(dead_code)]
impl Plane {

    pub fn new(transformation: Matrix, material: Material) -> Self {
        Self { transformation, material }
    }

}

impl_renderable_base!(Plane, RenderableType::Plane);

impl Renderable for Plane {
    
    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        let (_, out) = self.intersect_get_ray(ray);
        out
    }

    fn intersect_get_ray(&self, ray: Ray) -> (Ray, Option<Vec<Intersection>>) {
        // plane only exists on xz plane in local space (before transformation is applied)
        let ray = ray.transform(self.get_transformation().inverse().unwrap()); 
        if ray.get_direction().get_y().abs() < 0.00001 {    // TODO: need a global EPSILON value rather then this magic value
            return (ray, None);
        }
        let t = -ray.get_origin().get_y() / ray.get_direction().get_y();
        // TODO: would this work if I just returned a reference to self instead of a RC box of it?
        // TODO: is there a better way to do the RC then to make a new one here?
        let reflection = ray.get_direction().reflect(self.normal_at(Coord::point(0.0, 0.0, 0.0)));
        (ray, Some(vec![Intersection::new(t, Rc::new(self.clone()), reflection)]))
    }

    /// normal is always strait up translated by local transformation regardless of the pos
    fn normal_at(&self, _pos: Coord) -> Coord {
        let out = self.get_transformation()
            .inverse()
            .unwrap()
            .transpose() 
            * Coord::vec(0.0, 1.0, 0.0);
        out
            .to_vec()
            .normalized()
    }

    fn default() -> Self {
        Self { 
            transformation: Matrix::identity(4), 
            material: Material::default() 
        }
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn compare(&self, other: Rc<dyn Renderable>) -> bool {
        match other.as_any().downcast_ref::<Plane>() {
            Some(p) => self == p,
            None => false
        }
    }
}

impl_renderable_tests!(crate::plane::Plane, RenderableType::Plane);

#[cfg(test)]
mod tests {
    use crate::renderable::RenderableBase;
    use super::*;

    #[test]
    fn test_normal() {
        let p = Plane::default();
        let n1 = p.normal_at(Coord::point(0.0, 0.0, 0.0));
        let n2 = p.normal_at(Coord::point(10.0, 0.0, -10.0));
        let n3 = p.normal_at(Coord::point(-5.0, 0.0, 150.0));
        let tn = Coord::vec(0.0, 1.0, 0.0);
        assert_eq!(n1, tn);
        assert_eq!(n2, tn);
        assert_eq!(n3, tn); 
    }
    
    #[test]
    fn test_ray_intersect_above() {
        let p = Plane::default();
        let r = Ray::new(Coord::point(0.0, 1.0, 0.0), Coord::vec(0.0, -1.0, 0.0));
        let xs = p.intersect(r).unwrap();
        let xs = Intersection::aggregate_intersections(xs);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].get_time(), 1.0);
        let o = xs[0].get_object();
        assert_eq!(o.get_material().get_color_at(Coord::point(0.0, 0.0, 0.0)), p.get_material().get_color_at(Coord::point(0.0, 0.0, 0.0)));
        assert_eq!(o.get_pos(), p.get_pos());
        assert_eq!(o.get_transformation(), p.get_transformation());
        assert_eq!(o.get_type(), p.get_type());
    }

    #[test]
    fn test_reflectv_computation() {
        let p = Plane::default();
        let r = Ray::new(
            Coord::point(0.0, 1.0, -1.0), 
            Coord::vec(0.0, -2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0)
        );
        let xs = p.intersect(r).unwrap();
        let xs = Intersection::aggregate_intersections(xs);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].get_reflectv(), Coord::vec(0.0, 2.0_f32.sqrt() / 2.0, 2.0_f32.sqrt() / 2.0))
    }
}