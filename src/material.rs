use rtc::{impl_getters, impl_setters};
use crate::canvas::color::Color;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
    color: Color,
}

impl_getters!(Material,
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
    color: Color
);

impl_setters!(Material, color: Color);

#[allow(dead_code)]
impl Material {
    /// ambient, diffuse, specular values should be 0 <= x <= 1
    /// shininess should be 10 <= x <= 200
    pub fn new(ambient: f32, diffuse: f32, specular: f32, shininess: f32, color: Color) -> Self {
        assert!(ambient >= 0.0);
        assert!(ambient >= 0.0);
        assert!(specular >= 0.0);
        assert!( shininess >= 0.0);
        Self {ambient, diffuse, specular, shininess, color}
    }

    pub fn default() -> Self {
        Self { ambient: 0.1, diffuse: 0.9, specular: 0.9, shininess: 200.0, color: Color::white() }
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m = Material::new(1.0, 0.0, 1.0, 27.0, Color::white());
        assert_eq!(m.ambient, 1.0);
        assert_eq!(m.diffuse, 0.0);
        assert_eq!(m.specular, 1.0);
        assert_eq!(m.shininess, 27.0);
        assert_eq!(m.color, Color::white());
    }

    #[test]
    fn test_getters() {
        let m = Material::default();
        assert_eq!(m.get_ambient(), 0.1);
        assert_eq!(m.get_diffuse(), 0.9);
        assert_eq!(m.get_specular(), 0.9);
        assert_eq!(m.get_shininess(), 200.0);
        assert_eq!(m.get_color(), Color::white());
    }
}