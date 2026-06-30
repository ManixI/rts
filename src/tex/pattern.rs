use std::sync::Arc;

use crate::impl_getters_setters;
use crate::{coord::Coord, matrix::Matrix, tex::{Tex, color::Color}};


#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PatternType {
    Stripe, 
    Gradient,
    Checker,
    Bullseye,
    Solid,
    Blended,
    Perturbed,
    Test
}

#[derive(Debug, Clone)]
pub struct Pattern {
    pattern_type: PatternType,
    color_a: Arc<dyn Tex>,
    color_b: Arc<dyn Tex>,
    transformation: Matrix
}

impl_getters_setters!(
    Pattern, 
    pattern_type: PatternType, 
    color_a: Arc<dyn Tex>, 
    color_b: Arc<dyn Tex>
);

#[allow(dead_code)]
impl Pattern {
    pub fn new(pattern_type: PatternType, color_a: Arc<dyn Tex>, color_b: Arc<dyn Tex>, transformation: Matrix) -> Self {
        Self { pattern_type, color_a, color_b, transformation }
    }

    pub fn debug_pattern() -> Self {
        Self::new_checker(Arc::new(Color::purple()), Arc::new(Color::black()), Matrix::identity(4))
    }

    pub fn new_stripe(color_a: Arc<dyn Tex>, color_b: Arc<dyn Tex>, transformation: Matrix) -> Self {
        Self { pattern_type: PatternType::Stripe, color_a, color_b, transformation }
    }

    pub fn new_gradient(color_a: Arc<dyn Tex>, color_b: Arc<dyn Tex>, transformation: Matrix) -> Self {
        Self { pattern_type: PatternType::Gradient, color_a, color_b, transformation }
    }

    pub fn new_checker(color_a: Arc<dyn Tex>, color_b: Arc<dyn Tex>, transformation: Matrix) -> Self {
        Self { pattern_type: PatternType::Checker, color_a, color_b, transformation }
    }

    pub fn new_bullseye(color_a: Arc<dyn Tex>, color_b: Arc<dyn Tex>, transformation: Matrix) -> Self {
        Self { pattern_type: PatternType::Bullseye, color_a, color_b, transformation }
    }

    pub fn test_pattern(transformation: Matrix) -> Self {
        Self { pattern_type: PatternType::Test, color_a: Arc::new(Color::purple()), color_b: Arc::new(Color::black()), transformation }
    }
    
    pub fn new_solid(color_a: Arc<dyn Tex>, transformation: Matrix) -> Self {
        Self { pattern_type: PatternType::Solid, color_a, color_b: Arc::new(Color::white()), transformation }
    }  

    /// adds organic jitter to a subpattern
    /// color b is used to store the rng val, so do not use
    pub fn new_perturbed(color_a: Arc<dyn Tex>) -> Self {
        Self { pattern_type: PatternType::Perturbed, 
            color_a, 
            color_b: Arc::new(Color::new(rand::random_range(0.0..=1.0), 0.0, 0.0, 0.0)), 
            transformation: Matrix::identity(4) }
    }

    /// blends 2 sub patterns for every pixel by summing them
    pub fn new_blended(color_a: Arc<dyn Tex>, color_b: Arc<dyn Tex>, transformation: Matrix) -> Self {
        Self { pattern_type: PatternType::Blended, color_a, color_b, transformation }
    }

    fn stripe_at(&self, pos: Coord) -> Color {
        let x = pos.get_x().floor() as i32;
        match x % 2 {
            0 => self.get_color_a().get_color_at(pos),
            _ => self.get_color_b().get_color_at(pos), // modulo can be both positive or negative 1
        }
    }

    /// this currently only generates a gradient from 0 - 1
    /// this needs to bd scaled properly to actually work
    fn gradient_at(&self, pos: Coord) -> Color {
        // TODO: use Sin wave for smooth up and down gradient
        // could also try mod + interpolation to get sharp sin wave
        let cola = self.get_color_a().get_color_at(pos);
        let colb = self.get_color_b().get_color_at(pos);
        let x = pos.get_x();
        cola + (colb - cola) * (x - x.floor())
    }

    fn checker_at(&self, pos: Coord) -> Color {
        let x = pos.get_x();
        let y = pos.get_y();
        let z = pos.get_z();
        if (x.floor() + y.floor() + z.floor()).abs() as usize % 2 == 0 {
            return self.get_color_a().get_color_at(pos);
        }
        self.get_color_b().get_color_at(pos)
    }

    /// bullseye across XZ plane
    fn bullseye_at(&self, pos: Coord) -> Color {
        let x = pos.get_x();
        let z = pos.get_z();
        if (x.powi(2) + z.powi(2)).sqrt().floor() as usize % 2 == 0 {
            return self.get_color_a().get_color_at(pos);
        }
        self.get_color_b().get_color_at(pos)
    }

    fn blended_at(&self, pos: Coord) -> Color {
        self.get_color_a().get_color_at(pos) + self.get_color_b().get_color_at(pos)
    }

    fn perturbed_at(&self, pos: Coord) -> Color {
        let origin = Coord::point(0.0, 0.0, 0.0);
        let perlin_seed = self.get_color_b().get_color_at(origin).get_r();
        // TODO: apply jitter to pos
        let noise = crate::purlin_noise(perlin_seed, pos);
        self.get_color_a().get_color_at(pos + noise)
    }

    fn test_pattern_at(&self, pos: Coord) -> Color {
        Color::new(pos.get_x(), pos.get_y(), pos.get_z(), 0.0)
    }
}

impl Tex for Pattern {

    fn get_color_at(&self, pos: Coord) -> Color {
        let local_pos = self.get_transformation().inverse().unwrap() * pos;
        match self.get_pattern_type() {
            PatternType::Solid => self.get_color_a().get_color_at(local_pos),
            PatternType::Stripe => self.stripe_at(local_pos),
            PatternType::Gradient => self.gradient_at(local_pos),
            PatternType::Checker => self.checker_at(local_pos),
            PatternType::Bullseye => self.bullseye_at(local_pos),
            PatternType::Blended => self.blended_at(local_pos),
            PatternType::Perturbed => self.perturbed_at(local_pos),
            PatternType::Test => self.test_pattern_at(local_pos,)
        }
    }

    fn mul_helper_color(&self, rhs: Color) -> Arc<dyn Tex> {
        Arc::new(Self {
            color_a: self.get_color_a() * rhs,
            color_b: self.get_color_b() * rhs,
            pattern_type: self.get_pattern_type(),
            transformation: self.get_transformation()
        })
    }

    fn add_helper(&self, rhs: Color) -> Arc<dyn Tex> {
        Arc::new(Self {
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

    fn compare(&self, other: Arc<dyn Tex>) -> bool {
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

impl PartialEq for Pattern {
    fn eq(&self, other: &Self) -> bool {
        self.pattern_type == other.pattern_type
            && self.transformation == other.transformation
            && self.get_color_a().compare(other.get_color_a())
            && self.get_color_b().compare(other.get_color_b())
    }
}

#[cfg(test)]
mod test {
    use std::sync::Arc;
    use test_case::test_case;
    use crate::{coord::Coord, material::Material, matrix::Matrix, renderable::{Renderable, RenderableBase}, primitives::sphere::Sphere, tex::{Tex, color::Color, pattern::{Pattern, PatternType::{self, *}}}};

    // TODO: nested pattern tests
    // TODO: blended pattern tests
    // TODO: perturbed tests

    #[test]
    fn test_new() {
        let p = Pattern::new_stripe(Arc::new(Color::white()), Arc::new(Color::black()), Matrix::identity(4));
        assert_eq!(p.get_color_a().get_color_at(Coord::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.get_color_b().get_color_at(Coord::point(0.0, 0.0, 0.0)), Color::black());
    }

    #[test]
    fn test_stripe_y() {
        let p = Pattern::new_stripe(Arc::new(Color::white()), Arc::new(Color::black()), Matrix::identity(4));
        assert_eq!(p.stripe_at(Coord::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(0.0, 1.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(0.0, 2.0, 0.0)), Color::white());
    }

    #[test]
    fn test_stripe_z() {
        let p = Pattern::new_stripe(Arc::new(Color::white()), Arc::new(Color::black()), Matrix::identity(4));
        assert_eq!(p.stripe_at(Coord::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(0.0, 0.0, 1.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(0.0, 0.0, 2.0)), Color::white());
    }

    #[test]
    fn test_stripe_x() {
        let p = Pattern::new_stripe(Arc::new(Color::white()), Arc::new(Color::black()), Matrix::identity(4));
        assert_eq!(p.stripe_at(Coord::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(0.9, 0.0, 0.0)), Color::white());
        assert_eq!(p.stripe_at(Coord::point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(p.stripe_at(Coord::point(-0.1, 0.0, 0.0)), Color::black());
        assert_eq!(p.stripe_at(Coord::point(-1.0, 0.0, 0.0)), Color::black());
        assert_eq!(p.stripe_at(Coord::point(-1.1, 0.0, 0.0)), Color::white());
    }

    #[test]
    fn test_gradient() {
        let p = Pattern::new_gradient(Arc::new(Color::white()), Arc::new(Color::black()), Matrix::identity(4));
        assert_eq!(p.get_color_at(Coord::point(0.0, 0.0, 0.0)), Color::white());
        assert_eq!(p.get_color_at(Coord::point(0.25, 0.0, 0.0)), Color::new(0.75, 0.75, 0.75, 0.0));
        assert_eq!(p.get_color_at(Coord::point(0.5, 0.0, 0.0)), Color::new(0.5, 0.5, 0.5, 0.0));
        assert_eq!(p.get_color_at(Coord::point(0.75, 0.0, 0.0)), Color::new(0.25, 0.25, 0.25, 0.0));
        //assert_eq!(p.get_color_at(Coord::point(1.0, 0.0, 0.0)), Color::black());
    }

    #[test]
    fn test_bullseye() {
        let p = Pattern::new_bullseye(Arc::new(Color::red()), Arc::new(Color::black()), Matrix::identity(4));
        assert_eq!(p.get_color_at(Coord::point(0.0, 0.0, 0.0)), Color::red());
        assert_eq!(p.get_color_at(Coord::point(1.0, 0.0, 0.0)), Color::black());
        assert_eq!(p.get_color_at(Coord::point(0.0, 0.0, 1.0)), Color::black());
        assert_eq!(p.get_color_at(Coord::point(0.708, 0.0, 0.708)), Color::black());
    }

    #[test]
    fn test_checker_x() {
        let p = Pattern::new_checker(Arc::new(Color::red()), Arc::new(Color::black()), Matrix::identity(4));
        assert_eq!(p.get_color_at(Coord::point(0.0, 0.0, 0.0)), Color::red());
        assert_eq!(p.get_color_at(Coord::point(0.99, 0.0, 0.0)), Color::red());
        assert_eq!(p.get_color_at(Coord::point(1.1, 0.0, 0.0)), Color::black());
    }

    #[test]
    fn test_checker_y() {
        let p = Pattern::new_checker(Arc::new(Color::red()), Arc::new(Color::black()), Matrix::identity(4));
        assert_eq!(p.get_color_at(Coord::point(0.0, 0.0, 0.0)), Color::red());
        assert_eq!(p.get_color_at(Coord::point(0.0, 0.99, 0.0)), Color::red());
        assert_eq!(p.get_color_at(Coord::point(0.0, 1.1, 0.0)), Color::black());
    }

    #[test]
    fn test_checker_z() {
        let p = Pattern::new_checker(Arc::new(Color::red()), Arc::new(Color::black()), Matrix::identity(4));
        assert_eq!(p.get_color_at(Coord::point(0.0, 0.0, 0.0)), Color::red());
        assert_eq!(p.get_color_at(Coord::point(0.0, 0.0, 0.99)), Color::red());
        assert_eq!(p.get_color_at(Coord::point(0.0, 0.0, 1.1)), Color::black());
    }

    #[test_case(Stripe,   Color::red()                    ; "stripe")]
    #[test_case(Gradient, Color::new(0.25, 0.0, 0.0, 0.0) ; "gradient")]
    #[test_case(Checker,  Color::black()                  ; "checker")]
    #[test_case(Bullseye, Color::red()                    ; "bullseye")]
    #[test_case(Solid,    Color::red()                    ; "solid")]
    fn test_obj_transformed(pattern: PatternType, expected: Color) {
        let mut o = Sphere::default();
        o.set_transformation(Matrix::scaling(2.0, 2.0, 2.0));
        let p = Pattern::new(pattern, Arc::new(Color::red()), Arc::new(Color::black()), Matrix::identity(4));
        let m = Material::new(1.0, 0.0, 0.0, 10.0, 0.0, 1.0, 0.0, Arc::new(p));
        o.set_material(m);
        
        let c = o.get_color_at(Coord::point(1.5, 2.5, 0.0));
        assert_eq!(c, expected);
    }

    #[test_case(Stripe,   Color::red()                    ; "stripe")]
    #[test_case(Gradient, Color::new(0.25, 0.0, 0.0, 0.0) ; "gradient")]
    #[test_case(Checker,  Color::red()                    ; "checker")]
    #[test_case(Bullseye, Color::red()                    ; "bullseye")]
    #[test_case(Solid,    Color::red()                    ; "solid")]
    fn test_stripe_pattern_transformed(pattern: PatternType, expected: Color) {
        let p = Pattern::new(pattern, Arc::new(Color::red()), Arc::new(Color::black()), Matrix::scaling(2.0, 2.0, 2.0));
        let m = Material::new(1.0, 0.0, 0.0, 10.0, 0.0, 1.0, 0.0, Arc::new(p));
        let mut o = Sphere::default();
        o.set_material(m);
        assert_eq!(o.get_color_at(Coord::point(1.5, 0.0, 0.0)), expected)
    }

    #[test_case(Stripe,   Color::red()                    ; "stripe")]
    #[test_case(Gradient, Color::new(0.25, 0.0, 0.0, 0.0) ; "gradient")]
    #[test_case(Checker,  Color::red()                    ; "checker")]
    #[test_case(Bullseye, Color::red()                    ; "bullseye")]
    #[test_case(Solid,    Color::red()                    ; "solid")]
    fn test_stripe_both_transformed(pattern: PatternType, expected: Color) {
        let p = Pattern::new(pattern, Arc::new(Color::red()), Arc::new(Color::black()), Matrix::translation(0.5, 0.0, 0.0));
        let m = Material::new(1.0, 0.0, 0.0, 10.0, 0.0, 1.0, 0.0, Arc::new(p));
        let mut o = Sphere::default();
        o.set_material(m);
        o.set_transformation(Matrix::scaling(2.0, 2.0, 2.0));
        assert_eq!(o.get_color_at(Coord::point(2.5, 0.0, 0.0)), expected);
    }
}