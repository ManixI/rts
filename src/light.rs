use crate::{canvas::color::Color, coord::Coord, material::Material};



#[derive(Debug, Clone, Copy)]
pub struct Light {
    color: Color,
    pos: Coord,
    intensity: f32
}

#[allow(dead_code)]
impl Light {
    pub fn new(color: Color, pos: Coord, intensity: f32) -> Self {
        Self { color, pos, intensity }
    }
    
    pub fn default() -> Self {
        Self { color: Color::white(), pos: Coord::point(0.0, 0.0, 0.0), intensity: 1.0 }
    }
}

// TODO: attach this to something, camera maybe?
fn lighting(material: Material, light: Light, pos: Coord, camv: Coord, normal: Coord) -> Color {
    todo!();
}
#[cfg(test)]
mod tests {
    use crate::material::Material;

    use super::*;

    #[test]
    fn test_new() {
        let l = Light::default();
        assert_eq!(l.color, Color::white());
        assert_eq!(l.pos, Coord::point(0.0, 0.0, 0.0));
        assert_eq!(l.intensity, 1.0);

        let l = Light::new(Color::red(), Coord::point(1.0, 2.0, 3.0), 5.7);
        assert_eq!(l.color, Color::red());
        assert_eq!(l.pos, Coord::point(1.0, 2.0, 3.0));
        assert_eq!(l.intensity, 5.7);
    }

    #[test]
    fn test_lighting() {
        let material = Material::default();
        let pos = Coord::point(0.0, 0.0, 0.0);

        let camv = Coord::point(0.0, 0.0, -1.0);
        let normal = Coord::vec(0.0, 0.0, -1.0);
        let light = Light::new(Color::white(), Coord::point(0.0, 0.0, -10.0), 1.0);
        let r = lighting(material, light, pos, camv, normal);
        assert_eq!(r, Color::new(1.9, 1.9, 1.9, 0.0));

        let camv = Coord::point(0.0, 2.0_f32.sqrt()/2.0, -(2.0_f32.sqrt()/2.0));
        let r = lighting(material, light, pos, camv, normal);
        assert_eq!(r, Color::new(1.0, 1.0, 1.0, 0.0));

        let light = Light::new(Color::white(), Coord::point(0.0, 10.0, -10.0), 1.0);
        let r = lighting(material, light, pos, camv, normal);
        assert_eq!(r, Color::new(1.6364, 1.6364, 1.6364, 0.0));

        let camv = Coord::point(0.0, 0.0, -1.0);
        let r = lighting(material, light, pos, camv, normal);
        assert_eq!(r, Color::new(0.7364, 0.7364, 0.7364, 0.0));

        let light = Light::new(Color::white(), Coord::point(0.0, 0.0, 10.0), 1.0);
        let r = lighting(material, light, pos, camv, normal);
        assert_eq!(r, Color::new(0.1, 0.1, 0.1, 0.0));
    }
}