use super::Coord;

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct Ray {
    origin: Coord,
    direction: Coord
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: Coord, direction: Coord) -> Self {
        Ray { origin, direction }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let o = Coord::point(1.0, 2.0, 3.0);
        let d = Coord::vec(4.0, 5.0, 6.0);
        let r = Ray::new(o, d);
        assert_eq!(r.direction, d);
        assert_eq!(r.origin, o);
    }
}