pub mod coord;
pub mod matrix;
pub mod canvas;
pub mod camera;
pub mod light;
pub mod material;
pub mod plane;
pub mod ray;
pub mod renderable;
pub mod sphere;
pub mod tex;
pub mod world;
use crate::coord::Coord;

#[macro_export]
macro_rules! impl_getters {
    ($struct:ty, $($field:ident: $type:ty),*) => {
        paste::paste! {
            impl $struct {
                $(#[inline] pub fn [<get_ $field>](&self) -> $type { self.$field.clone() })*
            }
        }
    }
}

#[macro_export]
macro_rules! impl_setters {
    ($struct:ty, $($field:ident: $type:ty),*) => {
        paste::paste! {
            impl $struct {
                $(#[inline] pub fn [<set_ $field>](&mut self, new: $type) { self.$field = new })*
            }
        }
    }
}

#[macro_export]
#[allow(dead_code)]
macro_rules! impl_getters_setters {
    ($struct:ty, $($field:ident: $type:ty),*) => {
        paste::paste! {
            impl $struct {
                $(#[inline] pub fn [<get_ $field>](&self) -> $type { self.$field.clone() })*
                $(#[inline] pub fn [<set_ $field>](&mut self, new: $type) { self.$field = new })*
            }
        }
    };
}

pub fn purlin_noise(seed: f32, pos: Coord) -> Coord {
    todo!()
}
