use super::Coord;

#[derive(Debug,PartialEq, Clone, Copy)]
pub struct Ray {
    origin: Coord,
    direction: Coord
}

#[allow(dead_code)]
impl Ray {
    pub fn new(origin: Coord, direction: Coord) -> Self {
        Ray { origin, direction }
    }

    fn position(&self, time: f32) -> Coord {
        self.origin + self.direction * time
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

    #[test]
    fn test_position() {
        let r = Ray::new(Coord::point(2.0, 3.0, 4.0), Coord::vec(1.0, 0.0, 0.0));
        assert_eq!(r.position(0.0), Coord::point(2.0, 3.0, 4.0));
        assert_eq!(r.position(1.0), Coord::point(3.0, 3.0, 4.0));
        assert_eq!(r.position(-1.0), Coord::point(1.0, 3.0, 4.0));
        assert_eq!(r.position(2.5), Coord::point(4.5, 3.0, 4.0));
    }
}
