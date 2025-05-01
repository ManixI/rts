use std::ops;

#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: f32, // red
    g: f32, // green
    b: f32, // blue
    a: f32  // alpha
}

#[allow(dead_code)]
impl Color {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Color {r, g, b, a}
    }

    pub fn values_as_str(&self, max: usize) -> String {
        let mut data: Vec<f32> = Vec::with_capacity(3);
        data.push(self.r);
        data.push(self.g);
        data.push(self.b);

        data.iter().map(|val| {
            match val {
                x if x > &1.0 => max,
                x if x < &0.0 => 0,
                _ => (max as f32 * val) as usize
            }
        })
        .fold(String::new(), |s1, s2| format!("{}{} ", s1, s2))
    }

    #[inline]
    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0, 0.0)
    }

    #[inline]
    pub fn red() -> Self {
        Self::new(1.0, 0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn green() -> Self {
        Self::new(0.0, 1.0, 0.0, 0.0)
    }

    #[inline]
    pub fn blue() -> Self {
        Self::new(0.0, 0.0, 1.0, 0.0)
    }

    #[inline]
    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0, 0.0)
    }

    #[inline]
    pub fn yellow() -> Self {
        Self::red() + Self::green()
    }

    #[inline]
    pub fn purple() -> Self {
        Self::red() + Self::blue()
    }

    #[inline]
    pub fn turquoise() -> Self {
        Self::blue() + Self::green()
    }

    #[inline]
    pub fn gray() -> Self {
        Self::new(0.5, 0.5, 0.5, 0.0)
    }

    pub fn clamped(&self) -> Self {
        let mut r = self.r;
        if r > 1.0 {
            r = 1.0;
        } else if r < 0.0 {
            r = 0.0;
        }
        
        let mut g = self.g;
        if g > 1.0 {
            g = 1.0;
        } else if g < 0.0 {
            g = 0.0;
        }
        
        let mut b = self.b;
        if b > 1.0 {
            b = 1.0;
        } else if b < 0.0 {
            b = 0.0;
        }

        Self { r, g, b, a: self.a }
    }

    pub fn inverse(&self) -> Self {
        let mut out = self.clone();
        
        //out.b %= 1.0;
        out.b = (out.b - 1.0).abs();

        //out.g %= 1.0;
        out.g = (out.g - 1.0).abs();

        //out.r %= 1.0;
        out.r = (out.r - 1.0).abs();

        out
    }
}

impl ops::Add for Color {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        let out = Color {
            r: self.r + rhs.r,
            g: self.g + rhs.g,
            b: self.b + rhs.b,
            a: self.a + rhs.a
        };
        out
        //out.clamped()
    }
}

impl ops::Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
            a: self.a - rhs.a,
        }
    }
}

impl ops::Mul<Color> for Color {
    type Output = Self;
    // technically the Hadamard product
    fn mul(self, rhs: Color) -> Self::Output {
        Color {
            r: self.r * rhs.r,
            g: self.g * rhs.g,
            b: self.b * rhs.b,
            a: self.a * rhs.a,
        }
    }
}

impl ops::Mul<f32> for Color {
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
            a: self.a * rhs,
        }
    }
}

impl PartialEq for Color {
    // TODO: should probably implement epsilon for this
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r &&
        self.g == other.g &&
        self.b == other.b &&
        self.a == other.a
    }
}

impl Eq for Color {}

#[cfg(test)]
mod tests {
    use super::*;

    const EPSILON: f32 = 0.000005;

    fn test_eq_floats(a: f32, b: f32) -> bool {
        (a - b).abs() < EPSILON
    }

    fn test_eq_colors(a: Color, b: Color) -> bool {
        test_eq_floats(a.r, b.r) &&
        test_eq_floats(a.g, b.g) &&
        test_eq_floats(a.b, b.b) &&
        test_eq_floats(a.a, b.a)
    }

    #[test]
    fn test_create() {
        let c = Color::new(-0.5, 0.4, 1.7, 0.0);
        assert!(test_eq_floats(c.r, -0.5));
        assert!(test_eq_floats(c.g, 0.4));
        assert!(test_eq_floats(c.b, 1.7));
        assert!(test_eq_floats(c.a, 0.0));
    }

    #[test]
    fn test_add() {
        let c1 = Color::new(0.9, 0.6, 0.75, 0.5);
        let c2 = Color::new(0.7, 0.1, 0.25, 1.5);
        let n = c1 + c2;
        assert!(test_eq_colors(n, Color::new(1.6, 0.7, 1.0, 2.0)));
    }

    #[test]
    fn test_sub() {
        let c1 = Color::new(0.9, 0.6, 0.75, 0.5);
        let c2 = Color::new(0.7, 0.1, 0.25, 1.5);
        let n = c1 - c2;
        assert!(test_eq_colors(n, Color::new(0.2, 0.5, 0.5, -1.0)));
    }

    #[test]
    fn test_mul_scalar() {
        let c = Color::new(0.2, 0.3, 0.4, 0.5);
        assert_eq!(c * 2.0, Color::new(0.4, 0.6, 0.8, 1.0));
    }

    #[test]
    fn test_mul_colors() {
        let c1 = Color::new(1.0, 0.2, 0.4, 0.0);
        let c2 = Color::new(0.9, 1.0, 0.1, 0.0);
        let n = c1 * c2;
        assert!(test_eq_colors(n, Color::new(0.9, 0.2, 0.04, 0.0)))
    }

    #[test]
    fn test_inverse() {
        assert_eq!(Color::red().inverse(), Color::turquoise());
        assert_eq!(Color::white().inverse(), Color::black());
        assert_eq!(Color::gray().inverse(), Color::gray());
    }
}