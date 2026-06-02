use std::rc::Rc;

use rtc::impl_getters_setters;
use crate::{coord::Coord, matrix::Matrix, tex::{Tex, color::Color}};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternType {
    Stripe, 
    Gradient,
    Checker,
    Bullseye,
    Solid
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    pattern_type: PatternType,
    color_a: Color,
    color_b: Color,
    transformation: Matrix
}

impl_getters_setters!(
    Pattern, 
    pattern_type: PatternType, 
    color_a: Color, 
    color_b: Color
);

#[allow(dead_code)]
impl Pattern {
    pub fn new(pattern_type: PatternType, color_a: Color, color_b: Color, transformation: Matrix) -> Self {
        Self { pattern_type, color_a, color_b, transformation }
    }

    pub fn new_stripe(color_a: Color, color_b: Color, transformation: Matrix) -> Self {
        Self { pattern_type: PatternType::Stripe, color_a, color_b, transformation }
    }

    pub fn new_gradient(color_a: Color, color_b: Color, transformation: Matrix) -> Self {
        Self { pattern_type: PatternType::Gradient, color_a, color_b, transformation }
    }

    pub fn new_checker(color_a: Color, color_b: Color, transformation: Matrix) -> Self {
        Self { pattern_type: PatternType::Checker, color_a, color_b, transformation }
    }

    pub fn new_bullseye(color_a: Color, color_b: Color, transformation: Matrix) -> Self {
        Self { pattern_type: PatternType::Bullseye, color_a, color_b, transformation }
    }

    pub fn new_solid(color_a: Color, transformation: Matrix) -> Self {
        Self { pattern_type: PatternType::Solid, color_a, color_b: Color::white(), transformation }
    }  

    fn stripe_at(&self, pos: Coord) -> Color {
        let pos = pos.get_x().floor() as i32;
        match pos % 2 {
            0 => self.get_color_a(),
            _ => self.get_color_b(), // modulo can be both positive or negative 1
        }
    }

    fn gradiant_at(&self, pos: Coord) -> Color {
        todo!()
    }

    fn checker_at(&self, pos: Coord) -> Color {
        todo!();
    }

    fn bullseye_at(&self, pos: Coord) -> Color {
        todo!()
    }
}

impl Tex for Pattern {

    fn get_color_at(&self, pos: Coord) -> Color {
        let local_pos = self.get_transformation().inverse().unwrap() * pos;
        match self.get_pattern_type() {
            PatternType::Solid => self.get_color_a(),
            PatternType::Stripe => self.stripe_at(local_pos),
            PatternType::Gradient => self.gradiant_at(local_pos),
            PatternType::Checker => self.checker_at(local_pos),
            PatternType::Bullseye => self.bullseye_at(local_pos),
        }
    }

    fn mul_helper_color(&self, rhs: Color) -> std::rc::Rc<dyn Tex> {
        Rc::new(Self {
            color_a: self.get_color_a() * rhs,
            color_b: self.get_color_b() * rhs,
            pattern_type: self.get_pattern_type(),
            transformation: self.get_transformation()
        })
    }

    fn mul_f32(&self, rhs: f32) -> Rc<dyn Tex> {
        Rc::new(Self {
            color_a: self.get_color_a() * rhs,
            color_b: self.get_color_b() * rhs,
            pattern_type: self.get_pattern_type(),
            transformation: self.get_transformation()
        })
    }

    fn add_helper(&self, rhs: Color) -> Rc<dyn Tex> {
        Rc::new(Self {
            color_a: self.get_color_a() + rhs,
            color_b: self.get_color_b() + rhs,
            pattern_type: self.get_pattern_type(),
            transformation: self.get_transformation()
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

    fn get_transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    fn set_transformation(&mut self, mat: Matrix) {
        self.transformation = mat;
    }
}

#[cfg(test)]
mod test {
    use std::rc::Rc;
    use parameterized_macro::parameterized;
    use crate::{coord::Coord, material::Material, matrix::Matrix, renderable::{Renderable, RenderableBase}, sphere::Sphere, tex::{color::Color, pattern::Pattern, pattern::PatternType, pattern::PatternType::*}};

    #[test]
    fn test_new() {
        let p = Pattern::new_stripe(Color::white(), Color::black(), Matrix::identity(4));
        assert_eq!(p.get_color_a(), Color::white());
        assert_eq!(p.get_color_b(), Color::black());
    }

    #[test]
    fn test_stripe_y() {
        let p = Pattern::new_stripe(Color::white(), Color::black(), Matrix::identity(4));
        assert_eq!(p.stripe_at(Coord::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn test_stripe_z() {
        let p = Pattern::new_stripe(Color::white(), Color::black(), Matrix::identity(4));
        assert_eq!(p.stripe_at(Coord::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn test_stripe_x() {
        let p = Pattern::new_stripe(Color::white(), Color::black(), Matrix::identity(4));
        assert_eq!(p.stripe_at(Coord::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(p.stripe_at(Coord::point(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(p.stripe_at(Coord::point(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(p.stripe_at(Coord::point(-1.1, 0.0, 0.0)), Color::white());
    }

    #[parameterized(pattern = {
            Stripe, 
            Gradient,
            Checker,
            Bullseye,
            Solid
    }, expected = {
        Color::red(),
        Color::white(),
        Color::black(),
        Color::white(),
        Color::red()
    })]
    fn test_obj_transformed(pattern: PatternType, expected: Color) {
        let mut o = Sphere::default();
        o.set_transformation(Matrix::scaling(2.0, 2.0, 2.0));
        let p = Pattern::new(pattern, Color::red(), Color::black(), Matrix::identity(4));
        let m = Material::new(1.0, 0.0, 0.0, 10.0, Rc::new(p));
        o.set_material(m);
        
        let c = o.get_color_at(Coord::point(1.5, 2.5, 0.0));
        assert_eq!(c, expected);
    }

    #[parameterized(pattern = {
        Stripe, 
        Gradient,
        Checker,
        Bullseye,
        Solid
    }, expected = {
        Color::red(),
        Color::white(),
        Color::black(),
        Color::white(),
        Color::red()
    })]
    fn test_stripe_pattern_transformed(pattern: PatternType, expected: Color) {
        let p = Pattern::new(pattern, Color::red(), Color::black(), Matrix::scaling(2.0, 2.0, 2.0));
        let m = Material::new(1.0, 0.0, 0.0, 10.0, Rc::new(p));
        let mut o = Sphere::default();
        o.set_material(m);
        assert_eq!(o.get_color_at(Coord::point(1.5, 0.0, 0.0)), expected)
    }

    #[parameterized(pattern = {
        Stripe, 
        Gradient,
        Checker,
        Bullseye,
        Solid
    }, expected = {
        Color::red(),
        Color::white(),
        Color::black(),
        Color::white(),
        Color::red()
    })]
    fn test_stripe_both_transformed(pattern: PatternType, expected: Color) {
        let p = Pattern::new(pattern, Color::red(), Color::black(), Matrix::translation(0.5, 0.0, 0.0));
        let m = Material::new(1.0, 0.0, 0.0, 10.0, Rc::new(p));
        let mut o = Sphere::default();
        o.set_material(m);
        o.set_transformation(Matrix::scaling(2.0, 2.0, 2.0));
        assert_eq!(o.get_color_at(Coord::point(2.5, 0.0, 0.0)), expected);
    }
}