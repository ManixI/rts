use crate::{coord::Coord, material::Material, matrix::Matrix};

#[derive(PartialEq, Debug)]
pub enum RenderableType {
    Sphere,
}

pub trait Renderable {
    fn get_material(&self) -> Material;

    fn get_pos(&self) -> Coord;

    fn get_transformation(&self) -> Matrix;

    fn get_type(&self) -> RenderableType;

    fn clone_dyn(&self) -> Box<dyn Renderable>;
}

impl Clone for Box<dyn Renderable> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

pub fn compare_renderables(a: &dyn Renderable, b: &dyn Renderable) -> bool {
    a.get_material() == b.get_material()
    && a.get_pos() == b.get_pos()
    && a.get_transformation() == b.get_transformation()
    && a.get_type() == b.get_type()
}