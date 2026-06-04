use std::rc::Rc;

use rtc::impl_getters;
use crate::{coord::Coord, tex::{Tex, TextureType, color::Color}};

#[derive(Clone, Debug)]
pub struct Material {
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
    texture: Rc<dyn Tex>, 
}

impl_getters!(Material,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32
);


#[allow(dead_code)]
impl Material {
    /// ambient, diffuse, specular values should be 0 <= x <= 1
    /// shininess should be 10 <= x <= 200
    pub fn new(ambient: f32, diffuse: f32, specular: f32, shininess: f32, texture: Rc<dyn Tex>) -> Self {
        assert!(ambient >= 0.0);
        assert!(ambient >= 0.0);
        assert!(specular >= 0.0);
        assert!(shininess >= 0.0);
        Self {ambient, diffuse, specular, shininess, texture}
    }

    pub fn default() -> Self {
        Self { ambient: 0.1, diffuse: 0.9, specular: 0.9, shininess: 200.0, texture: Rc::new(Color::white()) }
    }

    pub fn set_ambient(&mut self, ambient: f32) {
        assert!(ambient >= 0.0);
        self.ambient = ambient;
    }

    pub fn set_diffuse(&mut self, diffuse: f32) {
        assert!(diffuse >= 0.0);
        self.diffuse = diffuse;
    }

    pub fn set_specular(&mut self, specular: f32) {
        assert!(specular >= 0.0);
        self.specular = specular;
    }

    pub fn set_shininess(&mut self, shininess: f32) {
        assert!( shininess >= 0.0);
        self.shininess = shininess;
    }

    pub fn set_texture(&mut self, tex: Rc<dyn Tex>) {
        self.texture = tex;
    }

    pub fn get_texture(&self) -> Rc<dyn Tex> {
        self.texture.clone()
    }

    pub fn set_color(&mut self, color: Color) {
        self.set_texture(Rc::new(color));
    }

    pub fn get_color_at(&self, pos: Coord) -> Color {
        self.get_texture().get_color_at(pos)
    }

    /// to make existing tests work, only use for Colors not other textures
    pub fn get_color(&self) -> Color {
        assert_eq!(self.get_texture().get_texture_type(), TextureType::Color);
        self.get_color_at(Coord::point(0.0, 0.0, 0.0))
    }
}

impl PartialEq for Material {
    fn eq(&self, other: &Self) -> bool {
        self.get_ambient() == other.get_ambient() &&
        self.get_diffuse() == other.get_diffuse() &&
        self.get_shininess() == other.get_shininess() &&
        self.get_specular() == other.get_specular() &&
        self.get_texture().compare(other.get_texture())
    }
}

#[cfg(test)]
mod tests {
    use std::path::MAIN_SEPARATOR;

use crate::{light::{Light, lighting}, matrix::Matrix, renderable::{Renderable, RenderableBase}, sphere::Sphere, tex::pattern::Pattern};

use super::*;

    #[test]
    fn test_new() {
        let m = Material::new(1.0, 0.0, 1.0, 27.0, Rc::new(Color::white()));
        assert_eq!(m.ambient, 1.0);
        assert_eq!(m.diffuse, 0.0);
        assert_eq!(m.specular, 1.0);
        assert_eq!(m.shininess, 27.0);
    }

    #[test]
    fn test_getters() {
        let m = Material::default();
        assert_eq!(m.get_ambient(), 0.1);
        assert_eq!(m.get_diffuse(), 0.9);
        assert_eq!(m.get_specular(), 0.9);
        assert_eq!(m.get_shininess(), 200.0);
    }

    #[test]
    fn test_pattern() {
        let m = Material::new(
            1.0,
            0.0, 
            0.0,
            10.0, 
            Rc::new(Pattern::new_stripe(Rc::new(Color::black()), Rc::new(Color::white()), Matrix::identity(4))));
        let mut o = Sphere::default();
        o.set_material(m);
        let o = Rc::new(o);
        let eyev = Coord::vec(0.0, 0.0, -1.0);
        let normalv = Coord::vec(0.0, 0.0, -1.0);
        let light = Light::new(Coord::point(0.0, 0.0, -10.0), Color::white());
        let c1 = lighting(o.clone(), light, Coord::point(0.9, 0.0, 0.0), eyev, normalv, false);
        let c2 = lighting(o, light, Coord::point(1.1, 0.0, 0.0), eyev, normalv, false);
        assert_eq!(c1, Color::black());
        assert_eq!(c2, Color::white());
    }
}