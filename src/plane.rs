use crate::{coord::Coord, impl_renderable_base, impl_renderable_tests, material::Material, matrix::Matrix, ray::Ray, renderable::{Intersection, Renderable, RenderableBase, RenderableType}};
use std::rc::Rc;


#[derive(Debug, PartialEq, Clone)]
pub struct Plane {
    transformation: Matrix,
    material: Material
}

#[allow(dead_code)]
impl Plane {
    pub fn default() -> Self {
        todo!()
    }

    pub fn new(origin: Coord) -> Self {
        todo!()
    }

}

impl_renderable_base!(Plane, RenderableType::Plane);

impl Renderable for Plane {
    
    fn intersect(&self, ray: &Ray) -> Option<[Intersection; 2]> {
        todo!()
    }

    fn normal_at(&self, pos: Coord) -> Coord {
        todo!()
    }
}

impl_renderable_tests!(crate::plane::Plane, RenderableType::Plane);

#[cfg(test)]
mod tests {
    use crate::plane;
    use super::*;

    
}