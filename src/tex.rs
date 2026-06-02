pub mod color;
pub mod pattern;

use std::{fmt, ops::{Add, Mul}, rc::Rc, any::Any};

use crate::{coord::Coord, matrix::Matrix};
use color::Color;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum TextureType {
    Color,
    Pattern,
    Texture
}

pub trait Tex: fmt::Debug {
    fn get_color_at(&self, pos: Coord) -> Color;
    fn mul_helper_color(&self, rhs: Color) -> Rc<dyn Tex>;
    fn mul_f32(&self, rhs: f32) -> Rc<dyn Tex>;
    fn add_helper(&self, rhs: Color) -> Rc<dyn Tex>;    // TODO: expand this to be able to sum patterns maybe? Would require re-work of how patterns work though (ie stacking patterns)
    fn get_texture_type(&self) -> TextureType;
    fn compare(&self, other: Rc<dyn Tex>) -> bool;
    fn as_any(&self) -> &dyn Any;
    fn get_transformation(&self) -> Matrix;
    fn set_transformation(&mut self, mat: Matrix);
}

impl Mul<Color> for Rc<dyn Tex> {
    type Output = Self;

    fn mul(self, rhs: Color) -> Self {
        self.mul_helper_color(rhs)
    }
}

impl Add<Color> for Rc<dyn Tex> {
    type Output = Self;

    fn add(self, rhs: Color) -> Self {
        self.add_helper(rhs)
    }
}
