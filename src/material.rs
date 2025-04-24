

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    ambient: f32,
    diffuse: f32,
    specular: f32,
    shininess: f32,
}

#[allow(dead_code)]
impl Material {
    /// ambient, diffuse, specular values should be 0 <= x <= 1
    /// shininess should be 10 <= x <= 200
    pub fn new(ambient: f32, diffuse: f32, specular: f32, shininess: f32) -> Self {
        Self {ambient, diffuse, specular, shininess}
    }

    pub fn default() -> Self {
        Self { ambient: 1.0, diffuse: 1.0, specular: 1.0, shininess: 200.0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let m = Material::new(1.0, 0.0, 1.0, 27.0);
        assert_eq!(m.ambient, 1.0);
        assert_eq!(m.diffuse, 0.0);
        assert_eq!(m.specular, 1.0);
        assert_eq!(m.shininess, 27.0);
    }
}