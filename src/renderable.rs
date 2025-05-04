use crate::material::Material;

pub trait Renderable {
    fn get_material(&self) -> Material;
}