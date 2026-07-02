use crate::{coord::Coord, impl_getters_setters, impl_renderable_base, impl_renderable_tests, material::Material, matrix::Matrix, ray::Ray, renderable::{Intersection, Renderable, RenderableBase, RenderableType}, tex::color::Color};

use std::sync::Arc;

static EPSILON: f32 = 0.005;

#[derive(PartialEq, Clone)]
pub struct Cylinder {
    transformation: Matrix,
    material: Material,  // TODO: refactor this to a pointer
    min: f32,
    max: f32,
    closed: bool
}

impl_getters_setters!(Cylinder, transformation: Matrix, material: Material, min: f32, max: f32, closed: bool);

impl Cylinder {
    pub fn new(transformation: Matrix, material: Material, min: f32, max: f32, closed: bool) -> Self {
        Self { transformation, material, min, max, closed }
    }

    fn normal_at_local_space(&self, pos: Coord) -> Coord {
        Coord::vec(pos.get_x(), 0.0, pos.get_z())
    }

    /// checks if intersection lies within radius of cap
    fn check_cap(ray: Ray, t: f32) -> bool {
        let x = ray.get_origin().get_x() + t * ray.get_direction().get_x();
        let z = ray.get_origin().get_z() + t * ray.get_direction().get_z();
        x.powi(2) + z.powi(2) <= 1.0
    }

    fn intersect_caps(&self, ray: Ray, data: &mut Vec<Intersection>) {
        if !self.get_closed() || ray.get_direction().get_y().abs() < EPSILON {
            return;
        }

        let object = Arc::new(self.clone());
        let t = (self.get_min() - ray.get_origin().get_y()) / ray.get_direction().get_y();
        if Cylinder::check_cap(ray, t) {
            data.push(Intersection::new(t, object.clone(), self.normal_at_local_space(ray.position(t))));
        }


        let t = (self.get_max() - ray.get_origin().get_y()) / ray.get_direction().get_y();
        if Cylinder::check_cap(ray, t) {
            data.push(Intersection::new(t, object.clone(), self.normal_at_local_space(ray.position(t))));
        }
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
            if self.get_closed() {
                let mut data = Vec::<Intersection>::new();
                self.intersect_caps(ray, &mut data);

                if data.len() == 0 {
                    return (ray, None);
                }
                
                return (ray, Some(data));
            } else {
                return (ray, None);
            }
        }

        let b = 2.0 * ray.get_origin().get_x() * ray.get_direction().get_x()
            + 2.0 * ray.get_origin().get_z() * ray.get_direction().get_z();
        let c = ray.get_origin().get_x().powi(2) + ray.get_origin().get_z().powi(2) - 1.0;
        let disc = b.powi(2) - 4.0 * a * c;

        // no intersection
        if disc < 0.0 {
            return (ray, None)
        }

        let mut t0 = (-b - disc.sqrt()) / (2.0 * a);
        let mut t1 = (-b + disc.sqrt()) / (2.0 * a);

        if t0 > t1 {
            (t0, t1) = (t1, t0);
        }

        let obj = Arc::new(self.clone());
        
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
        self.normal_at_local_space(pos)
    }

    fn default() -> Self where Self: Sized {
        Self { 
            transformation: Matrix::identity(4), 
            material: Material::default(),
            min: -f32::INFINITY,
            max: f32::INFINITY,
            closed: false
        }
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use crate::{coord::Coord, material::Material, matrix::Matrix, primitives::cylinder::Cylinder, ray::Ray, renderable::Renderable};


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

    #[test_case(Coord::point(1.0, 0.0, 0.0), Coord::vec(1.0, 0.0, 0.0) ; "case 1")]
    #[test_case(Coord::point(0.0, 5.0, -1.0), Coord::vec(0.0, 0.0, -1.0) ; "case 2")]
    #[test_case(Coord::point(0.0, -2.0, 1.0), Coord::vec(0.0, 0.0, 1.0) ; "case 3")]
    #[test_case(Coord::point(-1.0, 1.0, 0.0), Coord::vec(-1.0, 0.0, 0.0) ; "case 4")]
    fn test_normal_at(pos: Coord, normal: Coord) {
        let c = Cylinder::default();
        let n = c.normal_at_local_space(pos);
        assert_eq!(n, normal)
    }

    #[test_case(Coord::point(0.0, 1.5, 0.0), Coord::vec(0.1, 1.0, 0.0), 0 ; "case 1")]
    #[test_case(Coord::point(0.0, 3.0, -5.0), Coord::vec(0.0, 0.0, 1.0), 0 ; "case 2")]
    #[test_case(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0), 0 ; "case 3")]
    #[test_case(Coord::point(0.0, 2.0, -5.0), Coord::vec(0.0, 0.0, 1.0), 0 ; "case 4")]
    #[test_case(Coord::point(0.0, 1.0, -5.0), Coord::vec(0.0, 0.0, 1.0), 0 ; "case 5")]
    #[test_case(Coord::point(0.0, 1.5, -2.0), Coord::vec(0.0, 0.0, 1.0), 2 ; "case 6")]
    fn test_truncated_intersection(point: Coord, direction: Coord, count: usize) {
        let c = Cylinder::new(Matrix::identity(4), Material::default(), 1.0, 2.0, false);
        let direction = direction.normalized();
        let ray = Ray::new(point, direction);
        let xs =  c.intersect(ray);
        if count == 0 {
            assert!(xs.is_none());
        } else {
            assert_eq!(xs.unwrap().len(), count);
        }
    }

    #[test_case(Coord::point(0.0, 3.0, 0.0), Coord::vec(0.0, -1.0, 0.0) ; "case 1")]
    #[test_case(Coord::point(0.0, 3.0, -2.0), Coord::vec(0.0, -1.0, 2.0) ; "case 2")]
    #[test_case(Coord::point(0.0, 4.0, -2.0), Coord::vec(0.0, -1.0, 1.0) ; "case 3")]
    #[test_case(Coord::point(0.0, 0.0, -2.0), Coord::vec(0.0, 1.0, 2.0) ; "case 4")]
    #[test_case(Coord::point(0.0, -1.0, -2.0), Coord::vec(0.0, 1.0, 1.0) ; "case 5")]
    fn test_caps(point: Coord, direction: Coord) {
        let c = Cylinder::new(Matrix::identity(4), Material::default(), 1.0, 2.0, true);
        let direction = direction.normalized();
        let ray = Ray::new(point, direction);
        let xs = c.intersect(ray).unwrap();
        assert_eq!(xs.len(), 2);
    } 
}