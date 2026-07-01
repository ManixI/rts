use rayon::vec;

use crate::{coord::Coord, impl_renderable_base, impl_renderable_tests, material::Material, matrix::Matrix, ray::Ray, renderable::{Intersection, Renderable, RenderableBase, RenderableType}, tex::color::Color};

use std::sync::Arc;

static EPSILON: f32 = 0.005;

#[derive(PartialEq, Clone)]
pub struct Cylinder {
    transformation: Matrix,
    material: Material  // TODO: refactor this to a pointer
}

impl Cylinder {
    pub fn new(transformation: Matrix, material: Material) -> Self {
        Self { transformation, material }
    }

    fn normal_at_local_space(&self, pos: Coord) -> Coord {
        todo!()
    }
}

impl_renderable_base!(Cylinder, RenderableType::Cylinder);

impl_renderable_tests!(crate::primitives::cylinder::Cylinder, RenderableType::Cylinder);

impl Renderable for Cylinder {

    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        let (_, out) = self.intersect_get_ray(ray);
        out
    }

    fn intersect_get_ray(&self, ray: Ray) -> (Ray, Option<Vec<Intersection>>) {
        let ray = ray.transform(self.get_transformation().inverse().unwrap());

        let a = ray.get_direction().get_x().powi(2) + ray.get_direction().get_z().powi(2);

        // ray is parallel to y axis
        if a.abs() < EPSILON {
            return (ray, None);
        }

        let b = 2.0 * ray.get_origin().get_x() * ray.get_direction().get_x()
            + 2.0 * ray.get_origin().get_z() * ray.get_direction().get_z();
        let c = ray.get_origin().get_x().powi(2) + ray.get_origin().get_z().powi(2) - 1.0;
        let disc = b.powi(2) - 4.0 * a * c;

        // no intersection
        if disc < 0.0 {
            return (ray, None)
        }

        let t0 = (-b - disc.sqrt()) / (2.0 * a);
        let t1 = (-b + disc.sqrt()) / (2.0 * a);

        let obj = Arc::new(self.clone());
        (
            ray,
            Some(vec![
                Intersection::new(t0, obj.clone(), Coord::vec(0.0, 0.0, 0.0)),
                Intersection::new(t1, obj.clone(), Coord::vec(0.0, 0.0, 0.0))
            ])
        )
    }

    fn normal_at(&self, pos: Coord) -> Coord {
        let pos = self.get_transformation().inverse().unwrap() * pos;
        self.normal_at_local_space(pos)
    }

    fn default() -> Self where Self: Sized {
        Self { transformation: Matrix::identity(4), material: Material::default() }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::{coord::Coord, primitives::cylinder::Cylinder, ray::Ray, renderable::Renderable};


    #[test_case(Coord::point(1.0, 0.0, 0.0), Coord::vec(0.0, 1.0, 0.0) ; "case 1")]
    #[test_case(Coord::point(0.0, 0.0, 0.0), Coord::vec(0.0, 1.0, 0.0) ; "case 2")]
    #[test_case(Coord::point(0.0, 0.0, -5.0), Coord::vec(1.0, 1.0, 1.0) ; "case 3")]
    fn test_ray_miss(origin: Coord, direction: Coord) {
        let c = Cylinder::default();
        let d = direction.normalized();
        let r = Ray::new(origin, d);
        let xs = c.intersect(r);
        assert!(xs.is_none());
    }

    #[test_case(Coord::point(1.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0), 5.0, 5.0 ; "case 1")]
    #[test_case(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0), 4.0, 6.0 ; "case 2")]
    #[test_case(Coord::point(0.5, 0.0, -5.0), Coord::vec(0.1, 1.0, 1.0), 6.808006, 7.0886984 ; "case 3")]
    fn test_ray_hit(origin: Coord, direction: Coord, t0: f32, t1: f32) {
        let c = Cylinder::default();
        let direction = direction.normalized();
        let r = Ray::new(origin, direction);
        let xs = c.intersect(r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_time(), t0);
        assert_eq!(xs[1].get_time(), t1);
    }
}