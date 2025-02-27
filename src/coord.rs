use std:: ops;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Coord{
    x: f32,
    y: f32,
    z: f32,
    w: f32
}

#[allow(dead_code)]
impl Coord {
    /// Create a new coordinate that can be either a vec, point or have a separate w value
    /// 
    /// * `x: f32` - x val
    /// * `y: f32` - y val
    /// * `z: f32` - z val
    /// * `w: f32` - w val
    /// 
    /// returns `Self`
    fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Coord { x, y, z, w }
    }

    //fn new(x: usize, y: usize, z: usize, w: usize) -> Self {
    //    Coord { x: x as f32, y: y as f32, z: z as f32, w: w as f32 }
    //}

    fn point(x: f32, y: f32, z: f32) -> Self {
        Coord {x, y, z, w: 1.0}
    }

    fn vec(x: f32, y: f32, z: f32) -> Self {
        Coord {x, y, z, w: 0.0}
    }

    fn is_vec(&self) -> bool {
        self.w == 0.0
    }

    fn is_point(&self) -> bool {
        self.w == 1.0
    }

    fn add(self, other: Coord) -> Coord {
        Coord {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
            w: self.w + other.w
        }
    }

    fn sub(self, other: Coord) -> Coord {
        Coord { 
             x: self.x - other.x,
             y: self.y - other.y, 
             z: self.z - other.z, 
             w: self.w - other.w 
        }
    }

    /// negates the tuple
    fn neg(self) -> Coord {
        Coord { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }.sub(self)
    }

    fn magnitude(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w).sqrt()
    }

    fn normalized(&self) -> Coord {
        let mag = self.magnitude();
        Coord {
            x: self.x / mag,
            y: self.y / mag,
            z: self.z / mag,
            w: self.w / mag
        }
    }

    /// dot product of two vectors
    /// it makes no sense to call this on points, as it calculates angle
    /// will throw error if called on points
    fn dot(self, other: Coord) -> f32 {
        // TODO: if should only be compiled if in compiling for debug
        if self.is_point() || other.is_point() {
            panic!("do not call dot product on points")
        }

        self.x * other.x +
        self.y * other.y +
        self.z * other.z //+
        //self.w * other.w
    }

    fn cross(self, other: Self) -> Self {
        if self.is_point() || other.is_point() {
            panic!("do not call cross product on points")
        }
        Coord::vec(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }
}

// overloaded ops just use func definitions from above
impl ops::Add for Coord {
    type Output = Self;

    fn add(self, other: Coord) -> Coord {
        self.add(other)
    }
}

impl ops::Sub for Coord {
    type Output = Self;
    fn sub(self, other: Coord) -> Coord {
        self.sub(other)
    }
}

impl ops::Neg for Coord {
    type Output = Self;
    fn neg(self) -> Self::Output {
        self.neg()
    }
}

impl ops::Mul<f32> for Coord {
    type Output = Coord;
    fn mul(self, rhs: f32) -> Self::Output {
        Coord {
            x: self.x * rhs,
            y: self.y * rhs, 
            z: self.z * rhs,
            w: self.w * rhs
        }
    }
}

impl ops::Div<f32> for Coord {
    type Output = Coord;
    fn div(self, rhs: f32) -> Self::Output {
        Coord {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
            w: self.w / rhs
        }
    }
}

impl Eq for Coord{}

#[cfg(test)]
mod tests {
    use super::*;

    // margin of error for floating point math
    const EPSILON: f32 = 0.000005; 

    #[test]
    fn create_raw() {
        let p = Coord::new(1.0, -2.0, 3.5, 1.0);
        let v = Coord::new(1.0, -2.0, 3.5, 0.0);
        
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, -2.0);
        assert_eq!(p.z, 3.5);
        assert_eq!(p.w, 1.0);
        assert!(p.is_point());
        assert!(!p.is_vec());
        
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, -2.0);
        assert_eq!(v.z, 3.5);
        assert_eq!(v.w, 0.0);
        assert!(!v.is_point());
        assert!(v.is_vec());
    }

    #[test]
    fn create_point() {
        let p: Coord = Coord::point(1.0, -2.0, 3.5);
        assert_eq!(p.x, 1.0);
        assert_eq!(p.y, -2.0);
        assert_eq!(p.z, 3.5);
        assert_eq!(p.w, 1.0);
        assert!(p.is_point());
        assert!(!p.is_vec());
    }

    #[test]
    fn create_vec() {
        let v: Coord = Coord::vec(1.0, -2.0, 3.5);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, -2.0);
        assert_eq!(v.z, 3.5);
        assert_eq!(v.w, 0.0);
        assert!(!v.is_point());
        assert!(v.is_vec());
    }

    #[test]
    fn test_add() {
        let a = Coord::new(3.0, -2.0, 5.0, 1.0);
        let b = Coord::new(-2.0, 3.0, 1.0, 0.0);
        let c = a.add(b);
        assert_eq!(c.x, 1.0);
        assert_eq!(c.y, 1.0);
        assert_eq!(c.z, 6.0);
        assert_eq!(c.w, 1.0);

        let a = Coord::new(3.0, -2.0, 5.0, 1.0);
        let b = Coord::new(-2.0, 3.0, 1.0, 0.0);
        let c = a + b;
        assert_eq!(c.x, 1.0);
        assert_eq!(c.y, 1.0);
        assert_eq!(c.z, 6.0);
        assert_eq!(c.w, 1.0);
    }

    #[test]
    fn test_sub() {
        // points
        // subtracting 2 points should result in a vec from p2 to p1
        let a = Coord::point(3.0, 2.0, 1.0);
        let b = Coord::point(5.0, 6.0, 7.0);
        let c = a.sub(b);
        assert_eq!(c.x, -2.0);
        assert_eq!(c.y, -4.0);
        assert_eq!(c.z, -6.0);
        assert!(c.is_vec());

        // point and vec
        // subtracting a vec from a point should result in a new point translated along the vec
        let p = Coord::point(3.0, 2.0, 1.0);
        let v = Coord::vec(5.0, 6.0, 7.0);
        let n = p.sub(v);
        assert_eq!(n.x, -2.0);
        assert_eq!(n.y, -4.0);
        assert_eq!(n.z, -6.0);
        assert!(n.is_point());

        // vecs
        let v1 = Coord::vec(3.0, 2.0, 1.0);
        let v2 = Coord::vec(5.0, 6.0, 7.0);
        let v3 = v1.sub(v2);
        assert_eq!(v3.x, -2.0);
        assert_eq!(v3.y, -4.0);
        assert_eq!(v3.z, -6.0);
        assert!(v3.is_vec());

        // overload
        let a = Coord::point(3.0, 2.0, 1.0);
        let b = Coord::point(5.0, 6.0, 7.0);
        let c = a.sub(b);
        assert_eq!(c.x, -2.0);
        assert_eq!(c.y, -4.0);
        assert_eq!(c.z, -6.0);
        assert!(c.is_vec());
    }

    #[test]
    fn test_neg_vec() {
        let v = Coord::vec(1.0, -2.0, 3.0);
        let n = v.neg();
        assert_eq!(n.x, -1.0);
        assert_eq!(n.y, 2.0);
        assert_eq!(n.z, -3.0);
        assert!(n.is_vec());

        let v = Coord::vec(1.0, -2.0, 3.0);
        let n = -v;
        assert_eq!(n.x, -1.0);
        assert_eq!(n.y, 2.0);
        assert_eq!(n.z, -3.0);
        assert!(n.is_vec());
    }

    #[test]
    fn test_neg_tuple() {
        let t = Coord::new(1.0, -2.0, 3.0, -4.0);
        let n = -t;
        assert_eq!(n, Coord::new(-1.0, 2.0, -3.0, 4.0));
    }

    #[test]
    fn test_mul_scalar() {
        let t = Coord::new(1.0, -2.0, 3.0, -4.0);
        let n = t * 3.5;
        assert_eq!(n, Coord::new(3.5, -7.0, 10.5, -14.0));
    }

    #[test]
    fn test_mul_frac() {
        let t = Coord::new(1.0, -2.0, 3.0, -4.0);
        let n = t * 0.5;
        assert_eq!(n, Coord::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_div() {
        let t = Coord::new(1.0, -2.0, 3.0, -4.0);
        let n = t / 2.0;
        assert_eq!(n, Coord::new(0.5, -1.0, 1.5, -2.0));
    }

    #[test]
    fn test_magnitude() {
        assert_eq!(Coord::vec(1.0, 0.0, 0.0).magnitude(), 1.0);
        assert_eq!(Coord::vec(0.0, 1.0, 0.0).magnitude(), 1.0);
        assert_eq!(Coord::vec(0.0, 0.0, 1.0).magnitude(), 1.0);
        assert_eq!(Coord::vec(1.0, 2.0, 3.0).magnitude(), 14.0_f32.sqrt());
        assert_eq!(Coord::vec(-1.0, -2.0, -3.0).magnitude(), 14.0_f32.sqrt());
    }

    #[test]
    fn test_normalize() {
        assert_eq!(Coord::vec(4.0, 0.0, 0.0).normalized(), Coord::vec(1.0, 0.0, 0.0));
        assert_eq!(Coord::vec(0.0, 4.0, 0.0).normalized(), Coord::vec(0.0, 1.0, 0.0));
        assert_eq!(Coord::vec(0.0, 0.0, 4.0).normalized(), Coord::vec(0.0, 0.0, 1.0));
        assert_eq!(Coord::vec(1.0, 2.0, 3.0).normalized(), Coord::vec(1.0/14_f32.sqrt(), 2.0/14_f32.sqrt(), 3.0/14_f32.sqrt()));
        assert!((Coord::vec(1.0, 2.0, 3.0).normalized().magnitude() - 1.0).abs() < EPSILON);
    }

    #[test]
    fn test_dot() {
        let a = Coord::vec(1.0, 2.0, 3.0);
        let b = Coord::vec(2.0, 3.0, 4.0);
        assert_eq!(a.dot(b), 20.0);
    }

    #[test]
    fn test_cross() {
        let a = Coord::vec(1.0, 2.0, 3.0);
        let b = Coord::vec(2.0, 3.0, 4.0);
        assert_eq!(a.cross(b), Coord::vec(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(a), Coord::vec(1.0, -2.0, 1.0));
    }
}