use crate::{canvas::color::Color, coord::Coord, material::Material};



#[derive(Debug, Clone, Copy)]
pub struct Light {
    pos: Coord,
    intensity: Color
}

#[allow(dead_code)]
impl Light {
    pub fn new(pos: Coord, intensity: Color) -> Self {
        Self { pos, intensity }
    }
    
    pub fn default() -> Self {
        Self { pos: Coord::point(0.0, 0.0, 0.0), intensity: Color::white() }
    }

    // TODO: Dose negative intensity make sense or should this be bounded to >= 0?
    pub fn get_intensity(&self) -> Color {
        self.intensity
    }

    pub fn get_pos(&self) -> Coord {
        self.pos
    }

    pub fn set_intensity(&mut self, color: Color) {
        self.intensity = color
    }

    pub fn set_pos(&mut self, pos: Coord) {
        assert!(pos.is_point());
        self.pos = pos;
    }
}

// TODO: attach this to something, camera maybe?
pub fn lighting(material: Material, light: Light, pos: Coord, camv: Coord, normal: Coord) -> Color {
    let effective_color = material.get_color() * light.get_intensity();
    let light_v = (light.get_pos() - pos).normalized();
    let ambient = effective_color * material.get_ambient();
    let light_dot_normal = light_v.dot(normal);
    //let mut diffuse = Color::black();
    //let mut specular = Color::black();
    if light_dot_normal < 0.0 {
        return ambient;
    }
    let diffuse = effective_color * material.get_diffuse() * light_dot_normal;
    let reflect_v = (-light_v).reflect(&normal);
    let reflect_dot_cam = reflect_v.dot(camv);
    if reflect_dot_cam < 0.0 {
        return ambient + diffuse;
    }
    let factor = reflect_dot_cam.powf(material.get_shininess());
    let specular = light.get_intensity() * material.get_specular() * factor;
    ambient + diffuse + specular
}
#[cfg(test)]
mod tests {
    use crate::material::Material;

    use super::*;

    #[test]
    fn test_new() {
        let l = Light::default();
        assert_eq!(l.pos, Coord::point(0.0, 0.0, 0.0));
        assert_eq!(l.intensity, Color::white());

        let l = Light::new(Coord::point(1.0, 2.0, 3.0), Color::red());
        assert_eq!(l.pos, Coord::point(1.0, 2.0, 3.0));
        assert_eq!(l.intensity, Color::red());
    }

    #[test]
    fn test_get_pos() {
        let l = Light::default();
        assert_eq!(l.get_pos(), Coord::point(0.0, 0.0, 0.0));

        let l = Light::new(Coord::point(1.0, 2.0, 3.0), Color::white());
        assert_eq!(l.get_pos(), Coord::point(1.0, 2.0, 3.0));
    }

    #[test]
    fn test_get_intensity() {
        let l = Light::default();
        assert_eq!(l.get_intensity(), Color::white());

        let l = Light::new(Coord::point(0.0, 0.0, 0.0), Color::green());
        assert_eq!(l.get_intensity(), Color::green());
    }

    #[test]
    fn test_lighting() {
        let material = Material::default();
        let pos = Coord::point(0.0, 0.0, 0.0);

        // 1
        let camv = Coord::vec(0.0, 0.0, -1.0);
        let normal = Coord::vec(0.0, 0.0, -1.0);
        let light = Light::new(Coord::point(0.0, 0.0, -10.0), Color::white());
        let r = lighting(material, light, pos, camv, normal);
        assert_eq!(r, Color::new(1.9, 1.9, 1.9, 0.0));

        // 2
        let camv = Coord::vec(0.0, 2.0_f32.sqrt()/2.0, -(2.0_f32.sqrt()/2.0));
        let normal = Coord::vec(0.0, 0.0, -1.0);
        let light = Light::new(Coord::point(0.0, 0.0, -10.0), Color::white());
        let r = lighting(material, light, pos, camv, normal);
        assert_eq!(r, Color::new(1.0, 1.0, 1.0, 0.0));

        // either this or the next test is wrong
        // 3
        let camv = Coord::vec(0.0, 0.0, -1.0);
        let normal = Coord::vec(0.0, 0.0, -1.0);
        let light = Light::new(Coord::point(0.0, 10.0, -10.0), Color::white());
        let r = lighting(material, light, pos, camv, normal);
        assert_eq!(r, Color::new(0.7363961, 0.7363961, 0.7363961, 0.0));

        // 4
        let camv = Coord::vec(0.0, -(2.0_f32.sqrt())/2.0, -(2.0_f32.sqrt()/2.0));
        let normal = Coord::vec(0.0, 0.0, -1.0);
        let light = Light::new(Coord::point(0.0, 10.0, -10.0), Color::white());
        let r = lighting(material, light, pos, camv, normal);
        assert_eq!(r, Color::new(1.6363853, 1.6363853, 1.6363853, 0.0));

        let light = Light::new(Coord::point(0.0, 0.0, 10.0), Color::white());
        let r = lighting(material, light, pos, camv, normal);
        assert_eq!(r, Color::new(0.1, 0.1, 0.1, 0.0));
    }
}