use std::rc::Rc;

use rtc::impl_getters_setters;
use crate::{coord::Coord, tex::{Tex, color::Color}};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternType {
    Stripe, 
    Gradient,
    Checker,
    Bullseye,
    Solid
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pattern {
    pattern_type: PatternType,
    color_a: Color,
    color_b: Color
}

impl_getters_setters!(Pattern, pattern_type: PatternType, color_a: Color, color_b: Color);

#[allow(dead_code)]
impl Pattern {
    pub fn new(pattern_type: PatternType, color_a: Color, color_b: Color) -> Self {
        Self { pattern_type, color_a, color_b }
    }

    pub fn new_stripe(color_a: Color, color_b: Color) -> Self {
        Self { pattern_type: PatternType::Stripe, color_a, color_b }
    }

    pub fn new_gradient(color_a: Color, color_b: Color) -> Self {
        Self { pattern_type: PatternType::Gradient, color_a, color_b }
    }

    pub fn new_checker(color_a: Color, color_b: Color) -> Self {
        Self { pattern_type: PatternType::Checker, color_a, color_b }
    }

    pub fn new_bullseye(color_a: Color, color_b: Color) -> Self {
        Self { pattern_type: PatternType::Bullseye, color_a, color_b }
    }

    pub fn new_solid(color_a: Color) -> Self {
        Self { pattern_type: PatternType::Solid, color_a, color_b: Color::white() }
    }



    pub fn stripe_at(&self, pos: Coord) -> Color {
        let pos = pos.get_x().floor() as i32;
        match pos % 2 {
            0 => self.get_color_a(),
            _ => self.get_color_b(), // modulo can be both positive or negative 1
        }
    }
}

impl Tex for Pattern {
    fn get_color_at(&self, pos: Coord) -> Color {
        match self.get_pattern_type() {
            PatternType::Solid => self.get_color_a(),
            PatternType::Stripe => self.stripe_at(pos),
            PatternType::Gradient => panic!(),
            PatternType::Checker => panic!(),
            PatternType::Bullseye => panic!(),
        }
    }

    fn mul_helper_color(&self, rhs: Color) -> std::rc::Rc<dyn Tex> {
        Rc::new(Self {
            color_a: self.get_color_a() * rhs,
            color_b: self.get_color_b() * rhs,
            pattern_type: self.get_pattern_type()
        })
    }

    fn mul_f32(&self, rhs: f32) -> Rc<dyn Tex> {
        Rc::new(Self {
            color_a: self.get_color_a() * rhs,
            color_b: self.get_color_b() * rhs,
            pattern_type: self.get_pattern_type()
        })
    }

    fn add_helper(&self, rhs: Color) -> Rc<dyn Tex> {
        Rc::new(Self {
            color_a: self.get_color_a() + rhs,
            color_b: self.get_color_b() + rhs,
            pattern_type: self.get_pattern_type()
        })
    }

    fn get_texture_type(&self) -> super::TextureType {
        super::TextureType::Pattern
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }

    fn compare(&self, other: Rc<dyn Tex>) -> bool {
        match other.as_any().downcast_ref::<Pattern>() {
            Some(p) => self == p,
            None => false
        }
    }
}

#[cfg(test)]
mod test {
    use crate::{tex::{color::Color, pattern::Pattern}, coord::Coord};

    #[test]
    fn test_new() {
        let p = Pattern::new_stripe(Color::white(), Color::black());
        assert_eq!(p.get_color_a(), Color::white());
        assert_eq!(p.get_color_b(), Color::black());
    }

    #[test]
    fn test_stripe_y() {
        let p = Pattern::new_stripe(Color::white(), Color::black());
        assert_eq!(p.stripe_at(Coord::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn test_stripe_z() {
        let p = Pattern::new_stripe(Color::white(), Color::black());
        assert_eq!(p.stripe_at(Coord::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn test_stripe_x() {
        let p = Pattern::new_stripe(Color::white(), Color::black());
        assert_eq!(p.stripe_at(Coord::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(p.stripe_at(Coord::point(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(p.stripe_at(Coord::point(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(p.stripe_at(Coord::point(-1.1, 0.0, 0.0)), Color::white());
    }
}