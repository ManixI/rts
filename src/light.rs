use crate::{canvas::color::Color, coord::Coord};



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

#[cfg(test)]
mod tests {
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
}