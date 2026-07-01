use crate::{coord::Coord, impl_renderable_base, impl_renderable_tests, material::Material, matrix::Matrix, ray::Ray, renderable::{Intersection, Renderable, RenderableBase, RenderableType}, tex::color::Color};

use std::sync::Arc;

#[derive(PartialEq, Clone)]
pub struct Cone {
    transformation: Matrix,
    material: Material  // TODO: refactor this to a pointer
}

impl Cone {
    pub fn new(transformation: Matrix, material: Material) -> Self {
        Self { transformation, material }
    }

    fn normal_at_local_space(&self, pos: Coord) -> Coord {
        todo!()
    }
}

impl_renderable_base!(Cone, RenderableType::Cylinder);

impl_renderable_tests!(crate::primitives::cone::Cone, RenderableType::Cone);

impl Renderable for Cone {

    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        todo!()
    }

    fn intersect_get_ray(&self, ray: Ray) -> (Ray, Option<Vec<Intersection>>) {
        todo!()
    }

    fn normal_at(&self, pos: Coord) -> Coord {
        let pos = self.get_transformation().inverse().unwrap() * pos;
        self.normal_at_local_space(pos)
    }

    fn default() -> Self where Self: Sized {
        todo!()
    }
}

#[cfg(test)]
mod tests {

}