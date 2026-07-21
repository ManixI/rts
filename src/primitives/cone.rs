use crate::{coord::Coord, impl_getters_setters, impl_renderable_base, impl_renderable_tests, material::Material, matrix::Matrix, primitives::node::Node, ray::Ray, renderable::{Intersection, Renderable, RenderableType}, tex::color::Color};

use std::sync::Arc;

static EPSILON: f32 = 0.005;

#[derive(Debug, PartialEq, Clone)]
pub struct Cone {
    transformation: Matrix,
    material: Material,  // TODO: refactor this to a pointer
    min: f32,
    max: f32,
    closed: bool,
    parent: Option<Arc<Node>>,
}

impl_getters_setters!(Cone, transformation: Matrix, material: Material, min: f32, max: f32, closed: bool, parent: Option<Arc<Node>>);

impl Cone {
    pub fn new(transformation: Matrix, material: Material, min: f32, max: f32, closed: bool) -> Self {
        Self { transformation, material, min, max, closed, parent: None }
    }

    fn normal_at_local_space(&self, pos: Coord) -> Coord {
        let dist = pos.get_x().powi(2) + pos.get_z().powi(2);

        if dist <= pos.get_y().abs() {
            if pos.get_y() >= self.get_max() - EPSILON {
                return Coord::vec(0.0, 1.0, 0.0);
            }
            if pos.get_y() <= self.get_min() + EPSILON {
                return Coord::vec(0.0, -1.0, 0.0)
            }
        }

        let y = if pos.get_y() > 0.0 { -dist.sqrt() } else { dist.sqrt() };
        Coord::vec(pos.get_x(), y, pos.get_z())
    }

    fn intersect_caps(&self, ray: Ray, data: &mut Vec<Intersection>) {
        if !self.get_closed() || ray.get_direction().get_y().abs() < EPSILON {
            return;
        }

        let object = Arc::new(self.clone());
        let t = (self.get_min() - ray.get_origin().get_y()) / ray.get_direction().get_y();
        if Cone::check_cap(ray, t) {
            data.push(Intersection::new(t, object.clone(), self.normal_at_local_space(ray.position(t))));
        }


        let t = (self.get_max() - ray.get_origin().get_y()) / ray.get_direction().get_y();
        if Cone::check_cap(ray, t) {
            data.push(Intersection::new(t, object.clone(), self.normal_at_local_space(ray.position(t))));
        }
    }

    /// checks if intersection lies within radius of cap
    fn check_cap(ray: Ray, t: f32) -> bool {
        let x = ray.get_origin().get_x() + t * ray.get_direction().get_x();
        let y = ray.get_origin().get_y() + t * ray.get_direction().get_y();
        let z = ray.get_origin().get_z() + t * ray.get_direction().get_z();
        x.powi(2) + z.powi(2) <= y.powi(2)
    }
}

impl_renderable_base!(Cone, RenderableType::Cone);

impl_renderable_tests!(crate::primitives::cone::Cone, RenderableType::Cone);

impl Renderable for Cone {

    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        let (_, out) = self.intersect_get_ray(ray);
        out
    }

    fn intersect_get_ray(&self, ray: Ray) -> (Ray, Option<Vec<Intersection>>) {
        let ray = ray.transform(self.get_transformation().inverse().unwrap());

        let dir = ray.get_direction();
        let or = ray.get_origin();

        let a = dir.get_x().powi(2) - dir.get_y().powi(2) + dir.get_z().powi(2);
        let b = 2.0 * or.get_x() * dir.get_x()
            - 2.0 * or.get_y() * dir.get_y()
            + 2.0 * or.get_z() * dir.get_z();

        if a == 0.0 && b == 0.0 {
            return (ray, None);
        }

        let c = or.get_x().powi(2) - or.get_y().powi(2) + or.get_z().powi(2);
        let obj = Arc::new(self.clone());
        if a == 0.0 {
            let t = -c / (2.0 * b);
            let mut data = Vec::<Intersection>::new();
            let y = ray.position(t).get_y();
            if self.get_min() < y && self.get_max() > y {
                data.push(Intersection::new(t, obj.clone(), self.normal_at_local_space(ray.position(t))));
            }
            self.intersect_caps(ray, &mut data);
            if data.is_empty() {
                return (ray, None);
            }
            return (ray, Some(data));
        }

        let disc = b.powi(2) - 4.0 * a * c;

        if disc < -EPSILON {
            return (ray, None);
        }

        let disc = disc.max(0.0);

        let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
        let mut t1 = (-b + disc.sqrt()) / (2.0 * a);

        if t0 > t1 {
            (t0, t1) = (t1, t0);
        }

        let mut data = Vec::<Intersection>::new();
        let y0 = ray.position(t0).get_y();
        if self.get_min() < y0 && self.get_max() > y0 {
            data.push(Intersection::new(t0, obj.clone(), self.normal_at_local_space(ray.position(t0))));
        }

        let y1 = ray.position(t1).get_y();
        if self.get_min() < y1 && self.get_max() > y1 { // BUG: corner cases fail here bc of floating point imprecision
            data.push(Intersection::new(t1, obj.clone(), self.normal_at_local_space(ray.position(t1))));
        }

        self.intersect_caps(ray, &mut data);

        if data.len() == 0 {
            return (ray, None);
        }
        
        (ray, Some(data))
    }

    fn normal_at(&self, pos: Coord) -> Coord {
        let pos = self.get_transformation().inverse().unwrap() * pos;
        (self.normal_at_local_space(pos) * self.get_transformation()).normalized()
    }

    fn default() -> Self where Self: Sized {
        Self {
            transformation: Matrix::identity(4),
            material: Material::default(),
            min: -f32::INFINITY,
            max: f32::INFINITY,
            closed: false,
            parent: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::{coord::Coord, material::Material, matrix::Matrix, primitives::cone::Cone, ray::Ray, renderable::Renderable};

    #[test_case(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0), 5.0, 5.0 ; "case 1")]
    #[test_case(Coord::point(0.0, 0.0, -5.0), Coord::vec(1.0, 1.0, 1.0), 8.6602545, 8.6602545 ; "case 2")]
    #[test_case(Coord::point(1.0, 1.0, -5.0), Coord::vec(-0.5, -1.0, 1.0), 4.5500546, 49.449955 ; "case 3")]
    fn test_intersect(origin: Coord, direction: Coord, t0: f32, t1: f32) {
        let s = Cone::default();
        let direction = direction.normalized();
        let r = Ray::new(origin, direction);
        let xs = s.intersect(r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_time(), t0);
        assert_eq!(xs[1].get_time(), t1);
    }

    #[test]
    fn test_single_intersect() {
        let s = Cone::default();
        let direction = Coord::vec(0.0, 1.0, 1.0).normalized();
        let r = Ray::new(Coord::point(0.0, 0.0, -1.0), direction);
        let xs = s.intersect(r).unwrap();
        assert_eq!(xs.len(), 1);
        assert_eq!(xs[0].get_time(), 0.35355338);
    }

    #[test_case(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 1.0, 0.0), 0 ; "case 1")]
    #[test_case(Coord::point(0.0, 0.0, -0.25), Coord::vec(0.0, 1.0, 1.0), 2 ; "case 2")]
    #[test_case(Coord::point(0.0, 0.0, -0.25), Coord::vec(0.0, 1.0, 0.0), 4 ; "case 3")]
    fn test_intersect_end_cap(origin: Coord, direction: Coord, count: usize) {
        let s = Cone::new(Matrix::identity(4), Material::default(), -0.5, 0.5, true);
        let direction = direction.normalized();
        let ray = Ray::new(origin, direction);
        let xs = s.intersect(ray);
        if count == 0 {
            assert!(xs.is_none());
        } else {
            let xs = xs.unwrap();
            assert_eq!(xs.len(), count);
        }
    }

    #[test_case(Coord::point(0.0, 0.0, 0.0), Coord::vec(0.0, 0.0, 0.0) ; "case 1")]
    #[test_case(Coord::point(1.0, 1.0, 1.0), Coord::vec(1.0, -2_f32.sqrt(), 1.0) ; "case 2")]
    #[test_case(Coord::point(-1.0, -1.0, 0.0), Coord::vec(-1.0, 1.0, 0.0) ; "case 3")]
    fn test_normal_vector(point: Coord, normal: Coord) {
        let s = Cone::default();
        assert_eq!(s.normal_at(point), normal.normalized());
    }
}