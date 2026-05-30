use rtc::impl_getters_setters;
use crate::canvas::color::Color;


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
}

mod test {
    #[test]
    fn test() {
        
    }
}