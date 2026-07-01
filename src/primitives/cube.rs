use std::sync::Arc;

use crate::{coord::Coord, impl_getters_setters, impl_renderable_base, impl_renderable_tests, material::Material, matrix::Matrix, ray::Ray, renderable::{Intersection, Renderable, RenderableBase, RenderableType}, tex::color::Color};

static EPSILON: f32 = 0.005; // TODO: unify this with other epsilon values

#[derive(PartialEq, Clone)]
pub struct Cube {
    transformation: Matrix,
    material: Material
}

impl_getters_setters!(Cube, transformation: Matrix, material: Material);

impl Cube {
    pub fn new(transformation: Matrix, material: Material) -> Self {
        Self { transformation, material }
    }

    /// given a 1d coord and dir, returns the two times of intersection with a cube on that plane
    fn check_axis(pos: f32, dir: f32) -> (f32, f32) {
        let max_numerator = 1.0 - pos;
        let min_numerator = -1.0 - pos;

        let (tmin, tmax) = if dir.abs() >= EPSILON {
            (
                min_numerator / dir,
                max_numerator / dir
            )
        } else {
            (
                f32::INFINITY * min_numerator.signum(),
                f32::INFINITY * max_numerator.signum()
            )
        };

        (
            f32::min(tmin, tmax),
            f32::max(tmin, tmax)
        )
        //(tmin, tmax)
    }

    fn normal_at_local_space(&self, pos: Coord) -> Coord {
        let (x, y, z) = (pos.get_x(), pos.get_y(), pos.get_z());
        let max_axis = x.abs().max(y.abs()).max(z.abs());

        let local_normal = if max_axis == x.abs() {
            Coord::vec(x, 0.0, 0.0)
        } else if max_axis == y.abs() {
            Coord::vec(0.0, y, 0.0)
        } else {
            Coord::vec(0.0, 0.0, z)
        };

        let mut out = self
            .get_transformation()
            .inverse()
            .unwrap()
            .transpose()
            * local_normal;
        out.set_w(0.0);
        out.normalized()
    }
}

impl_renderable_base!(Cube, RenderableType::Cube);

impl_renderable_tests!(crate::primitives::cube::Cube, RenderableType::Cube);

impl Renderable for Cube {
    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        let (_, out) = self.intersect_get_ray(ray);
        out
    }

    fn intersect_get_ray(&self, ray: Ray) -> (Ray, Option<Vec<Intersection>>) {
        let ray = ray.transform(self.get_transformation().inverse().unwrap());

        // TODO: can optimize by skipping rest after it's clear the ray is a miss
        let (xtmin, xtmax) = Cube::check_axis(ray.get_origin().get_x(), ray.get_direction().get_x());
        let (ytmin, ytmax) = Cube::check_axis(ray.get_origin().get_y(), ray.get_direction().get_y());
        let (ztmin, ztmax) = Cube::check_axis(ray.get_origin().get_z(), ray.get_direction().get_z());
    
        let tmin = vec![xtmin, ytmin, ztmin].into_iter().reduce(f32::max).unwrap();
        let tmax = vec![xtmax, ytmax, ztmax].into_iter().reduce(f32::min).unwrap();

        let obj = Arc::new(self.clone());   // TODO: remove this clone
        let data = if tmax >= tmin {
            Some(
                vec![
                    Intersection::new(tmin, obj.clone(), self.normal_at_local_space(ray.position(tmin))),
                    Intersection::new(tmax, obj        , self.normal_at_local_space(ray.position(tmax)))        
            ])
        } else {
            None
        };

        (ray, data)
    }

    // note, striking a corner or edge right on the seem is undefined, it could be a normal in any of the 3(2) valid directions
    fn normal_at(&self, pos: Coord) -> Coord {
        let pos = self.get_transformation().inverse().unwrap() * pos; // TODO: isn't this already done in intersect method
        self.normal_at_local_space(pos)
    }

    fn default() -> Self where Self: Sized {
        Self { transformation: Matrix::identity(4), material: Material::default() }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use super::*;

    #[test_case(Coord::point(5.0, 0.5, 0.0),  Coord::vec(-1.0, 0.0, 0.0), 4.0,  6.0 ; "pos x face")]
    #[test_case(Coord::point(-5.0, 0.5, 0.0), Coord::vec(1.0, 0.0, 0.0),  4.0,  6.0 ; "neg x face")]
    #[test_case(Coord::point(0.5, 5.0, 0.0),  Coord::vec(0.0, -1.0, 0.0), 4.0,  6.0 ; "pos y face")]
    #[test_case(Coord::point(0.5, -5.0, 0.0), Coord::vec(0.0, 1.0, 0.0),  4.0,  6.0 ; "neg y face")]
    #[test_case(Coord::point(0.5, 0.5, 5.0),  Coord::vec(0.0, 0.0, -1.0), 4.0,  6.0 ; "pos z face")]
    #[test_case(Coord::point(0.5, 0.5, -5.0), Coord::vec(0.0, 0.0, 1.0),  4.0,  6.0 ; "neg z face")]
    #[test_case(Coord::point(0.0, 0.5, 0.0),  Coord::vec(0.0, 0.0, 1.0), -1.0,  1.0 ; "inside")]
    fn test_intersection_faces(origin: Coord, direction: Coord, t1: f32, t2: f32) {
        let c = Cube::default();
        let r = Ray::new(origin, direction);
        let xs = c.intersect(r).unwrap();
        assert_eq!(xs.len(), 2);
        assert_eq!(xs[0].get_time(), t1);
        assert_eq!(xs[1].get_time(), t2);
    }

    #[test_case(Coord::point(-2.0, 0.0, 0.0), Coord::vec(0.2673, 0.5345, 0.8018) ; "diag 1")]
    #[test_case(Coord::point(0.0, -2.0, 0.0), Coord::vec(0.8018, 0.2673, 0.5345) ; "diag 2")]
    #[test_case(Coord::point(0.0, 0.0, -2.0), Coord::vec(0.5345, 0.8018, 0.2673) ; "diag 3")]
    #[test_case(Coord::point(2.0, 0.0, 2.0), Coord::vec(0.0, 0.0, -1.0) ; "parallel z")]
    #[test_case(Coord::point(0.0, 2.0, 2.0), Coord::vec(0.0, -1.0, 0.0) ; "parallel y")]
    #[test_case(Coord::point(2.0, 2.0, 0.0), Coord::vec(-1.0, 0.0, 0.0) ; "parallel x")]
    fn test_intersect_miss(origin: Coord, direction: Coord) {
        let c = Cube::default();
        let r = Ray::new(origin, direction);
        let xs = c.intersect(r);
        assert!(xs.is_none());
    }

    #[test_case(Coord::point(1.0, 0.5, -0.8), Coord::vec(1.0, 0.0, 0.0) ; "pos x")]
    #[test_case(Coord::point(-1.0, -0.2, 0.9), Coord::vec(-1.0, 0.0, 0.0) ; "neg x")]
    #[test_case(Coord::point(-0.4, 1.0, -0.1), Coord::vec(0.0, 1.0, 0.0) ; "pos y")]
    #[test_case(Coord::point(0.3, -1.0, -0.7), Coord::vec(0.0, -1.0, 0.0) ; "neg y")]
    #[test_case(Coord::point(-0.6, 0.3, 1.0), Coord::vec(0.0, 0.0, 1.0) ; "pos z")]
    #[test_case(Coord::point(0.4, 0.4, -1.0), Coord::vec(0.0, 0.0, -1.0) ; "neg z")]
    #[test_case(Coord::point(1.0, 1.0, 1.0), Coord::vec(1.0, 0.0, 0.0) ; "corner 1")]
    #[test_case(Coord::point(-1.0, -1.0, -1.0), Coord::vec(-1.0, 0.0, 0.0) ; "corner 2")]
    fn test_normal_at(pos: Coord, normal: Coord) {
        let c = Cube::default();
        let n = c.normal_at(pos);
        assert_eq!(n, normal);
    }

    #[test]
    fn test_normal_at_world_to_local() {
        let mut c = Cube::default();
        c.set_transformation(Matrix::rotate_z(std::f32::consts::PI / 2.0));
        let world_point = c.get_transformation() * Coord::point(1.0, 0.3, 0.0);
        let n = c.normal_at(world_point);
        assert_eq!(n, Coord::vec(0.0, 1.0, 0.0));
    }
}