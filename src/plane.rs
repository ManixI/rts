use crate::{coord::Coord, impl_renderable_base, impl_renderable_tests, material::Material, matrix::Matrix, ray::Ray, renderable::{Intersection, Renderable, RenderableBase, RenderableType}};
use std::rc::Rc;


#[derive(Debug, PartialEq, Clone)]
pub struct Plane {
    transformation: Matrix,
    material: Material,
}

#[allow(dead_code)]
impl Plane {

    pub fn new(origin: Coord) -> Self {
        todo!()
    }

}

impl_renderable_base!(Plane, RenderableType::Plane);

impl Renderable for Plane {
    
    fn intersect(&self, ray: Ray) -> Option<[Intersection; 2]> {
        let (_, out) = self.intersect_get_ray(ray);
        out
    }

    fn intersect_get_ray(&self, ray: Ray) -> (Ray, Option<[Intersection; 2]>) {
        if ray.get_direction().get_y().abs() - 0.0001 > 0.0 {
            return (ray, None);
        }
        todo!()
    }

    fn normal_at(&self, pos: Coord) -> Coord {
        todo!()
    }

    fn default() -> Self {
        todo!();
    }
}

impl_renderable_tests!(crate::plane::Plane, RenderableType::Plane);

#[cfg(test)]
mod tests {
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
        let xs = p.intersect(r);
        let xs = Intersection::aggregate_intersections(vec![xs]);
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].get_time(), 1.0);
        assert_eq!(xs[0].get_object(), p);
    }
}