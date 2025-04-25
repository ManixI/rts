use crate::canvas::color::Color;



#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
    color: Color,
}

#[allow(dead_code)]
impl Material {
    /// ambient, diffuse, specular values should be 0 <= x <= 1
    /// shininess should be 10 <= x <= 200
    pub fn new(ambient: f32, diffuse: f32, specular: f32, shininess: f32, color: Color) -> Self {
        Self {ambient, diffuse, specular, shininess, color}
    }

    pub fn default() -> Self {
        Self { ambient: 0.1, diffuse: 0.9, specular: 0.9, shininess: 200.0, color: Color::white() }
    }

    pub fn get_color(&self) -> Color {
        self.color
    }

    pub fn get_ambient(&self) -> f32 {
        self.ambient
    }

    pub fn get_diffuse(&self) -> f32 {
        self.diffuse
    }

    pub fn get_specular(&self) -> f32 {
        self.specular
    }

    pub fn get_shininess(&self) -> f32 {
        self.shininess
    }

    pub fn set_color(&mut self, color: Color) {
        self.color = color;
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