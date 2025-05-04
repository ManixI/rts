use crate::{coord::Coord, material::Material, matrix::Matrix};

pub trait Renderable {
    fn get_material(&self) -> Material;

    fn get_pos(&self) -> Coord;

    fn get_transformation(&self) -> Matrix;
}