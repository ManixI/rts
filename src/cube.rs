use std::sync::Arc;

use crate::{coord::Coord, impl_renderable_base, impl_renderable_tests, material::{self, Material}, matrix::Matrix, ray::Ray, renderable::{Intersection, Renderable, RenderableBase, RenderableType}, sphere::Sphere, tex::color::Color};

// TODO: move all primitives under primitive subdir

#[derive(PartialEq, Clone)]
pub struct Cube {
    transformation: Matrix,
    material: Material
}

impl Cube {
    pub fn new(transformation: Matrix, material: Material) -> Self {
        Self { transformation, material }
    }

    pub fn default() -> Self {
        Self { transformation: Matrix::identity(4), material: Material::default() }
    }
}

impl_renderable_base!(Cube, RenderableType::Cube);

impl_renderable_tests!(crate::cube::Cube, RenderableType::Cube);

impl Renderable for Cube {
    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        todo!()
    }

    fn intersect_get_ray(&self, ray: Ray) -> (Ray, Option<Vec<Intersection>>) {
        todo!()
    }

    fn normal_at(&self, pos: Coord) -> Coord {
        todo!()
    }

    fn default() -> Self where Self: Sized {
        todo!()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        todo!()
    }

    fn compare(&self, other: Arc<dyn Renderable>) -> bool {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test_case(Coord::point(5.0, 0.5, 0.0),  Coord::vec(-1.0, 0.0, 0.0), 4.0,  6.0 ; "pos x face")]
    #[test_case(Coord::point(-5.0, 0.5, 0.0), Coord::vec(1.0, 0.0, 0.0),  4.0,  6.0 ; "neg x face")]
    #[test_case(Coord::point(0.5, 5.0, 0.0),  Coord::vec(0.0, -1.0, 0.0), 4.0,  6.0 ; "pos y face")]
    #[test_case(Coord::point(0.5, -5.0, 0.0), Coord::vec(0.0, 1.0, 0.0),  4.0,  6.0 ; "neg y face")]
    #[test_case(Coord::point(0.5, 0.5, 5.0),  Coord::vec(0.0, 0.0, -1.0), 4.0,  6.0 ; "pos z face")]
    #[test_case(Coord::point(0.5, 0.5, -5.0), Coord::vec(0.0, 0.0, 1.0),  4.0,  6.0 ; "neg z face")]
    #[test_case(Coord::point(0.0, 0.5, 0.0),  Coord::vec(0.0, 0.0, 1.0), -1.0,  1.0 ; "inside")]
    fn test_intersection_faces(origin: Coord, direction: Coord, t1: f32, t2: f32) {
        let c = Cube::default();
        let r = Ray::new(origin, direction);
        let xs = c.intersect(r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_time(), t1);
        assert_eq!(xs[1].get_time(), t2);
    }
}