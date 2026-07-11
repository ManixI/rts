use crate::{coord::Coord, impl_getters_setters, impl_renderable_base, impl_renderable_tests, material::Material, matrix::Matrix, ray::Ray, renderable::{Intersection, Renderable, RenderableBase, RenderableType}, tex::color::Color};

use std::sync::Arc;

#[derive(PartialEq, Clone)]
pub struct Cone {
    transformation: Matrix,
    material: Material,  // TODO: refactor this to a pointer
    min: f32, 
    max: f32,
    closed: bool
}

impl_getters_setters!(Cone, transformation: Matrix, material: Material, min: f32, max: f32, closed: bool);

impl Cone {
    pub fn new(transformation: Matrix, material: Material, min: f32, max: f32, closed: bool) -> Self {
        Self { transformation, material, min, max, closed }
    }

    fn normal_at_local_space(&self, pos: Coord) -> Coord {
        todo!()
    }
}

impl_renderable_base!(Cone, RenderableType::Cone);

impl_renderable_tests!(crate::primitives::cone::Cone, RenderableType::Cone);

impl Renderable for Cone {

    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        let (_, out) = self.intersect_get_ray(ray);
        out
    }

    fn intersect_get_ray(&self, ray: Ray) -> (Ray, Option<Vec<Intersection>>) {
        todo!()
    }

    fn normal_at(&self, pos: Coord) -> Coord {
        let pos = self.get_transformation().inverse().unwrap() * pos;
        self.normal_at_local_space(pos)
    }

    fn default() -> Self where Self: Sized {
        Self { 
            transformation: Matrix::identity(4), 
            material: Material::default(),
            min: -f32::INFINITY,
            max: f32::INFINITY,
            closed: false
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::{coord::Coord, material::Material, matrix::Matrix, primitives::{cone::Cone, cylinder::Cylinder}, ray::Ray, renderable::Renderable};

    #[test_case(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0), 5.0, 5.0 ; "case 1")]
    #[test_case(Coord::point(0.0, 0.0, -5.0), Coord::vec(1.0, 1.0, 1.0), 8.66025, 8.66025 ; "case 2")]
    #[test_case(Coord::point(1.0, 1.0, -5.0), Coord::vec(-0.5, -1.0, 1.0), 4.5506, 49.44994 ; "case 3")]
    fn test_intersect(origin: Coord, direction: Coord, t0: f32, t1: f32) {
        let s = Cone::default();
        let direction = direction.normalized();
        let r = Ray::new(origin, direction);
        let xs = s.intersect(r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_time(), t0);
        assert_eq!(xs[1].get_time(), t1);
    }

    #[test]
    fn test_single_intersect() {
        let s = Cone::default();
        let direction = Coord::vec(0.0, 1.0, 1.0).normalized();
        let r = Ray::new(Coord::point(0.0, 0.0, -1.0), direction);
        let xs = s.intersect(r).unwrap();
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].get_time(), 0.35355);
    }

    #[test_case(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 1.0, 0.0), 0 ; "case 1")]
    #[test_case(Coord::point(0.0, 0.0, -0.25), Coord::vec(0.0, 1.0, 1.0), 2 ; "case 2")]
    #[test_case(Coord::point(0.0, 0.0, -0.25), Coord::vec(0.0, 1.0, 0.0), 4 ; "case 3")]
    fn test_intersect_end_cap(origin: Coord, direction: Coord, count: usize) {
        let s = Cone::new(Matrix::identity(4), Material::default(), -0.5, 0.5, true);
        let direction = direction.normalized();
        let ray = Ray::new(origin, direction);
        let xs = s.intersect(ray);
        if count == 0 {
            assert!(xs.is_none());
        } else {
            let xs = xs.unwrap();
            assert_eq!(xs.len(), count);
        }
    }

    #[test_case(Coord::point(0.0, 0.0, 0.0), Coord::vec(0.0, 0.0, 0.0) ; "case 1")]
    #[test_case(Coord::point(1.0, 1.0, 1.0), Coord::vec(1.0, -2_f32.sqrt(), 1.0) ; "case 2")]
    #[test_case(Coord::point(-1.0, -1.0, 0.0), Coord::vec(-1.0, 1.0, 0.0) ; "case 3")]
    fn test_normal_vector(point: Coord, normal: Coord) {
        let s = Cone::default();
        assert_eq!(s.normal_at(point), normal);
    }
}