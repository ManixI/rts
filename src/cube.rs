use std::sync::Arc;

use crate::{coord::Coord, impl_renderable_base, impl_renderable_tests, material::{self, Material}, matrix::Matrix, ray::Ray, renderable::{Intersection, Renderable, RenderableBase, RenderableType}, sphere::Sphere};

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

mod tests {
    use super::*;
    
}