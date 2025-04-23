use std::ops;

use crate::matrix::Matrix;

const EPSILON: f32 = 0.000001;

#[derive(Debug, Clone, Copy)]
pub struct Coord{
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
    pub fn new(x: f32, y: f32, z: f32, w: f32) -> Self {
        Coord { x, y, z, w }
    }

    pub fn get_x(&self) -> f32 {
        self.x
    }

    pub fn get_y(&self) -> f32 {
        self.y
    }

    pub fn get_z(&self) -> f32 {
        self.z
    }

    pub fn get_w(&self) -> f32 {
        self.w
    }
    //fn new(x: usize, y: usize, z: usize, w: usize) -> Self {
    //    Coord { x: x as f32, y: y as f32, z: z as f32, w: w as f32 }
    //}

    pub fn set_x(&mut self, x: f32) {
        self.x = x;
    }

    pub fn set_y(&mut self, y: f32) {
        self.y = y;
    }
    
    pub fn set_z(&mut self, z: f32) {
        self.z = z;
    }
    
    pub fn set_w(&mut self, w: f32) {
        self.w = w;
    }
    
    pub fn get_as_list(&self) -> [f32; 4] {
        let mut out = [0.0; 4];
        out[0] = self.get_x();
        out[1] = self.get_y();
        out[2] = self.get_z();
        out[3] = self.get_w();
        out
    }

    pub fn from_list(vec: &[f32; 4]) -> Self {
        Self {
            x: vec[0],
            y: vec[1],
            z: vec[2],
            w: vec[3]
        }
    }

    pub fn point(x: f32, y: f32, z: f32) -> Self {
        Coord {x, y, z, w: 1.0}
    }

    pub fn vec(x: f32, y: f32, z: f32) -> Self {
        Coord {x, y, z, w: 0.0}
    }

    pub fn is_vec(&self) -> bool {
        self.w == 0.0
    }

    pub fn is_point(&self) -> bool {
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

    pub fn magnitude(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w).sqrt()
    }

    pub fn normalized(&self) -> Coord {
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
    pub fn dot(self, other: Coord) -> f32 {
        // TODO: if should only be compiled if in compiling for debug
        if self.is_point() || other.is_point() {
            panic!("do not call dot product on points")
        }

        self.x * other.x +
        self.y * other.y +
        self.z * other.z //+
        //self.w * other.w
    }

    pub fn cross(self, other: &Self) -> Self {
        if self.is_point() || other.is_point() {
            panic!("do not call cross product on points")
        }
        Coord::vec(
            self.y * other.z - self.z * other.y,
            self.z * other.x - self.x * other.z,
            self.x * other.y - self.y * other.x
        )
    }

    pub fn len(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }


    //TODO: optimize this
    pub fn reflect(&self, norm: &Self) -> Self {
        self.clone() - norm.clone() * 2.0 * self.dot(norm.clone())
    }
    /// given two vectors, returns the scalar s required to convert this vector to the other vector
    /// or None if vectors aren't in the same direction or if one vec is a point
    // TODO: Move EPSILON to single const file rather then adding it wherever it is used
    const EPSILON: f32 = 0.000001;
    pub fn scalar_multiple(&self, other: &Self) -> Option<f32> {
        //println!("{:?} {:?}", self, other);
        if self.is_point() || other.is_point() {
            return None;
        }
        if self.cross(other).len() > EPSILON {
            //println!("{:?}", self.cross(other));
            //println!("{}", self.cross(other).len());
            return None;
        }
        

        if self.get_x() != 0.0 && other.get_x() != 0.0 {
            return Some(other.get_x() / self.get_x());
        }
        else if self.get_y() != 0.0 && other.get_y() != 0.0 {
            return Some(other.get_y() / self.get_y());
        }
        else if self.get_z() != 0.0 && other.get_z() != 0.0 {
            return Some(other.get_z() / self.get_z());
        }
        return Some(0.0)
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

impl PartialEq for Coord {
    fn eq(&self, other: &Self) -> bool {
        if (self.x - other.x).abs() > EPSILON {
            return false;
        }
        if (self.y - other.y).abs() > EPSILON {
            return false;
        }
        if (self.z - other.z).abs() > EPSILON {
            return false;
        }
        if (self.w - other.w).abs() > EPSILON {
            return false;
        }
        true
    }
}

impl ops::Mul<Matrix> for Coord{
    type Output = Self;

    fn mul(self, rhs: Matrix) -> Self::Output {
        rhs * self
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

impl ops::AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
        self.z = self.z + rhs.z;
        self.w = self.w + rhs.w;
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
        assert_eq!(a.cross(&b), Coord::vec(-1.0, 2.0, -1.0));
        assert_eq!(b.cross(&a), Coord::vec(1.0, -2.0, 1.0));
    }

    #[test]
    fn test_eq() {
        // test PartialEq works even with floating point nonsense
        let a = Coord::point(-0.1, 0.2, 0.3);
        let c = a * 0.8;
        let c = c * 0.8;
        let c = c * 0.8;
        let c = c * 0.8;
        println!("{:?}", c);
        assert_eq!(c, Coord::new(-0.04096, 0.08192, 0.12288, 0.4096));
    }

    #[test]
    fn test_scalar() {
        let a = Coord::point(0.0, 0.0, 0.0);
        assert!(a.scalar_multiple(&Coord::vec(0.0, 0.0, 0.0)).is_none());

        let a = Coord::vec(0.0, 0.0, 0.0);
        let b = Coord::vec(1.0, 0.0, 0.0);
        assert!(a.scalar_multiple(&a).is_some());
        assert_eq!(a.scalar_multiple(&a).unwrap(), 0.0);
        assert!(a.scalar_multiple(&b).is_some());
        assert_eq!(a.scalar_multiple(&b).unwrap(), 0.0);

        let a = Coord::vec(1.0, 0.0, 0.0);
        let b = Coord::vec(2.0, 0.0, 0.0);
        assert!(a.scalar_multiple(&b).is_some());
        assert_eq!(a.scalar_multiple(&b).unwrap(), 2.0);

        let b = Coord::vec(0.0, 2.0, 0.0);
        assert!(a.scalar_multiple(&b).is_none());

        let a = Coord::vec(0.0, 0.0, 2.0);
        let b = Coord::vec(0.0, 0.0, 6.0);
        assert!(a.scalar_multiple(&b).is_some());
        assert_eq!(a.scalar_multiple(&b).unwrap(), 3.0);
    }

    #[test]
    fn test_get_as_list() {
        let p = Coord::point(2.0, 3.0, 4.0);
        let l = p.get_as_list();
        assert_eq!(l.len(), 4);
        assert_eq!(l, [2.0, 3.0, 4.0, 1.0]);
    }

    #[test]
    fn test_from_list() {
        let l: [f32; 4] = [1.0, 2.0, 3.0, 0.0];
        let v = Coord::from_list(&l);
        assert_eq!(v, Coord::vec(1.0, 2.0, 3.0))
    }

    #[test]
    fn test_reflection() {
        let vec = Coord::vec(1.0, -1.0, 0.0);
        let norm = Coord::vec(0.0, 1.0, 0.0);
        assert_eq!(vec.reflect(&norm), Coord::vec(1.0, 1.0, 0.0));

        let vec = Coord::vec(0.0, -1.0, 0.0);
        let norm = Coord::vec(2.0_f32.sqrt()/2.0, 2.0_f32.sqrt()/2.0, 0.0);
        assert_eq!(vec.reflect(&norm), Coord::vec(1.0, 0.0, 0.0));
    }
}