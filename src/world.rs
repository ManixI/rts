use std::sync::Arc;
use crate::impl_getters_setters;
use crate::{camera::Camera, canvas::Canvas, tex::color::Color, coord::Coord, light::{Light, lighting}, material::Material, matrix::Matrix, ray::Ray, renderable::{Intersection, Renderable, RenderableBase}, primitives::sphere::Sphere};
use rayon::prelude::*;

// I'm going to need to re-work this to add all objects, not just renderable ones aren't I
// probably just make a node type or something

static EPSILON: f32 = 0.005; // this needs to be surprisingly big

#[derive(Clone)]
pub(crate) struct Comps {
    object: Arc<dyn Renderable>,
    point: Coord,
    eyev: Coord,
    normalv: Coord,
    time: f32,
    inside: bool,
    reflectv: Coord,
    n1: f32,
    n2: f32
}

impl_getters_setters!(Comps,
    //object: Arc<dyn Renderable>,
    point: Coord,
    eyev: Coord,
    normalv: Coord,
    time: f32,
    inside: bool,
    reflectv: Coord,
    n1: f32,
    n2: f32
);

// precomputed data about an intersection of ray and renderable
#[allow(dead_code)]
impl Comps {
    fn new(object: Arc<dyn Renderable>, point: Coord, eyev: Coord, normalv: Coord, time: f32, inside: bool, reflectv: Coord, n1: f32, n2: f32) -> Self {
        Self { object, point, eyev, normalv, time, inside, reflectv, n1, n2 }
    }

    fn get_object(&self) -> Arc<dyn Renderable> {
        self.object.clone()
    }

    fn get_over_point(&self) -> Coord {
        self.get_point() + self.get_normalv() * EPSILON
    }

    fn get_under_point(&self) -> Coord {
        self.get_point() - self.get_normalv() * EPSILON
    }

    pub(crate) fn prepare_computations(intersection: Intersection, ray: Ray, inter_list: Vec<Intersection>) -> Self {
        let mut inside = false;
        let mut normalv = intersection.get_object().normal_at(ray.position(intersection.get_time()));
        if normalv.dot(-ray.get_direction()) < 0.0 {
            inside = true
        }
        if inside {
            normalv = -normalv;
        }

        let mut containers: Vec<Intersection> = Vec::new();
        let mut n1 = 1.0;
        let mut n2 = 1.0;
        // TODO: there's got to be a more optimal way to do this
        for obj in inter_list {
            if obj == intersection {
                if containers.len() != 0 {
                    n1 = containers[containers.len()-1].get_object().get_material().get_refractive_index();
                } 
            }
            let mut skip_push = true;
            for i in 0..containers.len() {
                if containers[i].get_object().compare(obj.get_object()) {
                    containers.remove(i);
                    skip_push = false;
                    break;
                }
            }
            if  skip_push {
                containers.push(obj.clone());
            }
            if obj == intersection {
                if containers.len() != 0 {
                    n2 = containers[containers.len()-1].get_object().get_material().get_refractive_index();
                }
                break
            }
        }

        Self::new(
            intersection.get_object(), 
            ray.position(intersection.get_time()), 
            -ray.get_direction(), 
            normalv, 
            intersection.get_time(),
            inside,
            intersection.get_reflectv(),
            n1, 
            n2
        )
    }

    /// Fresnel effect factor
    fn schlick(&self) -> f32 {
        let mut cos = self.get_eyev().dot(self.get_normalv());

        if self.get_n1() > self.get_n2() {
            let n = self.get_n1() / self.get_n2();
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));
            if sin2_t > 1.0 {
                return 1.0;
            }

            let cos_t = (1.0 - sin2_t).sqrt();
            
            cos = cos_t;
        }

        let r0 = ((self.get_n1() - self.get_n2()) / (self.get_n1() + self.get_n2())).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

pub struct World {
    light: Vec<Light>,
    objects: Vec<Arc<dyn Renderable>>,
    max_depth: usize
}

// TODO: need to implement way to remove objects

impl_getters_setters!(World, max_depth: usize);

#[allow(dead_code)]
impl World {
    pub fn new() -> Self {
        Self { light: Vec::new(), objects: Vec::<Arc<dyn Renderable>>::new(), max_depth: 10 }
    }

    pub fn default() -> Self {
        let l = Light::new(Coord::point(-10.0, 10.0, -10.0), Color::white());
        let mut s1 = Sphere::new(Coord::point(0.0, 0.0, 0.0));
        let mut s2 = Sphere::new(Coord::point(0.0, 0.0, 0.0));
        s2.set_transformation(Matrix::scaling(0.5, 0.5, 0.5));
        let mat = Material::new(0.1, 0.7, 0.2, 200.0, 0.0, 1.0, 0.0, Arc::new(Color::new(0.8, 1.0, 0.6, 0.0)));
        s1.set_material(mat);        


        let s1 = Arc::new(s1) as Arc<dyn Renderable>;
        let s2 = Arc::new(s2) as Arc<dyn Renderable>;
        let objs = vec![s1, s2];
        Self { light: vec![l], objects: objs, max_depth: 10 }
    }

    pub fn get_light(&self) -> &Vec<Light> {
        &self.light
    }

    pub fn set_light(&mut self, light: Light) {
        self.light = vec![light]
    }

    pub fn add_light(&mut self, light: Light) {
        self.light.push(light);
    }

    fn get_object(&self) -> Vec<Arc<dyn Renderable>> {
        self.objects.clone()
    }

    pub fn add_obj(&mut self, obj: Arc<dyn Renderable>) {
        self.objects.push(obj);
    }

    fn get_intersections(&self, ray: Ray) -> Vec<Intersection> {
        let mut data = Vec::<Intersection>::new();
        for obj in self.get_object() {
            match obj.intersect(ray) {
                None => continue,
                Some(mut val) => data.append(&mut val),
            }
        }
        Intersection::aggregate_intersections(data)
    }

    fn shade_hit(&self, comps: Comps, depth: usize) -> Color {
        let mut color = Color::black();
        for light in self.get_light() {
            color = color + lighting(
            comps.get_object(), 
            *light, 
            comps.get_point(), 
            comps.get_eyev(), 
            comps.get_normalv(),
            self.in_shadow(comps.get_over_point())
            );
        }
        let reflected =  self.reflected_color(comps.clone(), depth + 1);
        let refracted = self.refracted_color(comps.clone(), depth + 1);

        let mat = comps.get_object().get_material();
        if mat.get_reflection() > 0.0 && mat.get_transparency() > 0.0 {
            let reflectance = comps.schlick();
            return color + 
                reflected * reflectance + 
                refracted * (1.0 - reflectance);
        }
        color + reflected + refracted
    }

    fn color_at(&self, ray: Ray, depth: usize) -> Color {
        if depth >= self.get_max_depth() {
            return Color::black();
        }
        let intersections = self.get_intersections(ray);
        let hit = Intersection::find_hit(&intersections);
        if hit.is_none() {
            return Color::black();
        }
        let comps = Comps::prepare_computations(hit.unwrap().clone(), ray, intersections);
        self.shade_hit(comps, depth)
    }

    pub fn render_world(&self, cam: &Camera) -> Canvas {
        let mut out = Canvas::new(cam.get_hsize(), cam.get_vsize());
        // TODO: multithread this
        for y in 0..(cam.get_vsize()-1) {
            for x in 0..(cam.get_hsize()-1) {
                let ray = cam.ray_for_pixel(x, y);
                let color = self.color_at(ray, 0);
                out.set_pixel(x, y, color);
            }
        }
        out
    }

    pub fn render_world_multi(&self, cam: &Camera) -> Canvas {
        let pixels: Vec<(usize, usize, Color)> = (0..cam.get_vsize()-1)
            .into_par_iter()
            .flat_map(|y| {
                (0..cam.get_hsize()-1)
                    .into_par_iter()
                    .map(move |x| {
                        let ray = cam.ray_for_pixel(x, y);
                        let color = self.color_at(ray, 0);
                        (x, y, color)
                    })  
            })
            .collect();
        let mut out = Canvas::new(cam.get_hsize(), cam.get_vsize());
        for (x, y, color) in pixels {
            out.set_pixel(x, y, color);
        }
        out
    }

    fn in_shadow(&self, p: Coord) -> bool { // TODO: change this to a float that is the inverse of the intersected object's transparency, and keep going until it is above 1.0 or hit's the object in question (don't count exiting just entering)
        // TODO: current impl only supports 1 light source
        let l = self.get_light()[0];
        let dir = l.get_pos() - p;
        let dist = dir.magnitude();
        let dir = dir.normalized();
        let ray = Ray::new(p, dir);
        let intersections = self.get_intersections(ray);    // T of all of these is < 0
        let intersections = Intersection::find_hit(&intersections);
        match intersections {
            None => false,
            Some(val) => val.get_time() <= dist // TODO: add EPSILON to cover for floating point errors
        }
    }

    fn reflected_color(&self, data: Comps, depth: usize) ->  Color {
        let reflective = data.get_object().get_material().get_reflection();
        if reflective <= 0.0 {
            return Color::black();
        }
        let ray = Ray::new(data.get_over_point(), data.get_reflectv());
        let color = self.color_at(ray, depth);
        color * reflective
    }

    fn refracted_color(&self, data: Comps, depth: usize) -> Color {
        if depth > self.get_max_depth() || data.get_object().get_material().get_transparency() == 0.0 {
            return Color::black();
        }

        // find refraction angle via Snell's law
        let ratio = data.get_n1() / data.get_n2();
        let cosi = data.get_eyev().dot(data.get_normalv());
        let sin2 = ratio.powi(2) * (1.0 - cosi.powi(2));

        if sin2 > 1.0 {
            // full internal refraction
            return Color::black();
        }

        let cost = (1.0 - sin2).sqrt();
        let direction = data.get_normalv() * (ratio * cosi - cost) - data.get_eyev() * ratio;
        let refracted_ray = Ray::new(data.get_under_point(), direction);
        self.color_at(refracted_ray, depth) * data.get_object().get_material().get_transparency()
    }
}


#[cfg(test)]
mod tests {
    use std::sync::Arc;

use crate::{camera::Camera, coord::Coord, light::Light, material::Material, matrix::Matrix, primitives::plane::Plane, ray::Ray, renderable::{Intersection, Renderable, RenderableBase, compare_renderables}, primitives::sphere::Sphere, tex::{color::Color, pattern::Pattern}, world::EPSILON};

    use super::{Comps, World};


    fn test_colors_roughly_equal(a: &Color, b: &Color) {
        const EPSILON: f32 = 0.0000001;
        //assert!((a.get_a() - b.get_a()).abs() > EPSILON);
        assert!((a.get_r() - b.get_r()).abs() > EPSILON);
        assert!((a.get_g() - b.get_g()).abs() > EPSILON);
        assert!((a.get_b() - b.get_b()).abs() > EPSILON);
    }

    #[test]
    fn test_new() {
        let w = World::new();
        assert_eq!(w.light.len(), 0);
        assert_eq!(w.objects.len(), 0);
    }

    #[test]
    fn test_getters() {
        let w = World::new();
        assert_eq!(w.light.len(), 0);
        assert_eq!(w.get_object().len(), 0);
    }

    #[test]
    fn test_default() {
        let w = World::default();
        
        assert_eq!(w.get_light().len(), 1);
        assert_eq!(w.get_light()[0], Light::new(Coord::point(-10.0, 10.0, -10.0), Color::white()));
        
        let mut s1 = Sphere::default();
        let objs = w.get_object();
        let mat = Material::new(0.1, 0.7, 0.2, 200.0, 0.0, 1.0, 0.0, Arc::new(Color::new(0.8, 1.0, 0.6, 0.0)));
        s1.set_material(mat);
        assert_eq!(objs.len(), 2);
        compare_renderables(objs[0].as_ref(), &s1);

        let mut s2 = Sphere::default();
        s2.set_transformation(Matrix::scaling(0.5, 0.5, 0.5));

        compare_renderables(objs[1].as_ref(), &s2);
    }

    #[test]
    fn test_get_intersections() {
        let w = World::default();
        let r = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let xs = w.get_intersections(r);
        //for x in &xs {
        //    println!("{}", x.get_time());
        //    println!("{:?}", x);
        //}
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].get_time(), 4.0);
        assert_eq!(xs[1].get_time(), 4.5);
        assert_eq!(xs[2].get_time(), 5.5);
        assert_eq!(xs[3].get_time(), 6.0);
    }


    #[test]
    fn test_prepare_computations() {
        let ray = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let shape = Arc::new(Sphere::default());
        let i = Intersection::new(4.0, shape.clone(), Coord::vec(0.0, 0.0, 0.0));
        let comp = Comps::prepare_computations(i.clone(), ray, vec![i.clone()]);
        assert_eq!(comp.get_time(), i.get_time());
        assert_eq!(comp.get_object().get_transformation(), shape.get_transformation());
        assert_eq!(comp.get_object().get_type(), shape.get_type());
        assert_eq!(comp.get_object().get_pos(), shape.get_pos());
        //assert_eq!(comp.get_object(), shape);
        assert_eq!(comp.get_point(), Coord::point(0.0, 0.0, -1.0));
        assert_eq!(comp.get_eyev(), Coord::vec(0.0, 0.0, -1.0));
        assert_eq!(comp.get_normalv(), Coord::vec(0.0, 0.0, -1.0));
    }

    #[test]
    fn test_prepare_computations_inside() {
        let ray = Ray::new(Coord::point(0.0, 0.0, 0.0), Coord::vec(0.0, 0.0, 1.0));
        let shape = Arc::new(Sphere::default());
        let i = Intersection::new(1.0, shape.clone(), Coord::vec(0.0, 0.0, 0.0));
        let comp = Comps::prepare_computations(i.clone(), ray, vec![i.clone()]);

        assert_eq!(comp.get_point(), Coord::point(0.0, 0.0, 1.0));
        assert_eq!(comp.get_eyev(), Coord::vec(0.0, 0.0, -1.0));
        assert_eq!(comp.get_inside(), true);
        assert_eq!(comp.get_normalv(), Coord::vec(0.0, 0.0, -1.0))
    }

    #[test]
    fn test_shade_hit() {
        let w = World::default();
        let ray = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let shape = w.get_object()[0].clone();
        let i = Intersection::new(4.0, shape, Coord::vec(0.0, 0.0, 0.0));
        let comps = Comps::prepare_computations(i.clone(), ray, vec![i.clone()]);
        let c = w.shade_hit(comps, 0);
        assert_eq!(c, Color::new(0.38066125, 0.4758265, 0.28549594, 0.0));

        let mut w = World::default();
        w.set_light(Light::new(Coord::point(0.0, 0.25, 0.0), Color::white()));
        let ray = Ray::new(Coord::point(0.0, 0.0, 0.0), Coord::vec(0.0, 0.0, 1.0));
        let shape = w.get_object()[1].clone();
        let i = Intersection::new(0.5, shape, Coord::vec(0.0, 0.0, 0.0));
        let comps = Comps::prepare_computations(i.clone(), ray, vec![i.clone()]);
        let c = w.shade_hit(comps, 0);
        assert_eq!(c, Color::new(0.9049845, 0.9049845, 0.9049845, 0.0));
    }

    #[test]
    fn test_color_at() {
        let w = World::default();
        let ray = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 1.0, 0.0));
        let c = w.color_at(ray, 0);
        assert_eq!(c, Color::black());

        let w = World::default();
        let ray = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let c = w.color_at(ray, 0);
        assert_eq!(c, Color::new(0.38066125, 0.4758265, 0.28549594, 0.0));
    
        // not sure if this actually works as I think it should, get material is suspect
        // I think I need to change getters to return reference, not clone (unless rc)
        // probably need to use RefCells not Rcs https://stackoverflow.com/questions/52994205/what-is-the-standard-way-to-call-a-mutable-method-in-a-rc-wrapped-object
        // alternatively, use Box and world class is the holder of all objects
        w.get_object()[0].get_material().set_ambient(1.0);
        w.get_object()[1].get_material().set_ambient(1.0);
        let ray = Ray::new(Coord::point(0.0, 0.0, 0.75), Coord::vec(0.0, 0.0, -1.0));
        let c = w.color_at(ray, 0);
        //assert_eq!(c, w.get_object()[0].get_material().get_color());
        test_colors_roughly_equal(&c, &w.get_object()[0].get_material().get_color_at(Coord::point(0.0, 0.0, 0.0)));
    }

    #[test]
    fn test_render_world() {
        let w = World::default();
        let mut c = Camera::new(11, 11, core::f32::consts::PI/2.0);
        let from = Coord::point(0.0, 0.0, -5.0);
        let to = Coord::point(0.0, 0.0, 0.0);
        let up = Coord::vec(0.0, 1.0, 0.0);
        c.transform(Matrix::view_transformation(from, to, up));

        let image = w.render_world(&c);
        assert_eq!(image.get_pixel(5, 5), Color::new(0.38066125, 0.4758265, 0.28549594, 0.0));
    }

    #[test]
    fn test_not_in_shadow() {
        let w = World::default();
        let p = Coord::point(0.0, 10.0, 0.0);
        assert!(!w.in_shadow(p));
    }

    #[test]
    fn test_in_shadow_behind() {
        let w = World::default();
        let p = Coord::point(10.0, -10.0, 10.0);
        assert!(w.in_shadow(p));
    }

    #[test]
    fn test_in_shadow_behind_light() {
        let w = World::default();
        let p = Coord::point(-20.0, 20.0, -20.0);
        assert!(!w.in_shadow(p));
    }

    #[test]
    fn test_in_shadow_in_front_object() {
        let w = World::default();
        let p = Coord::point(-2.0,2.0,-2.0);
        assert!(!w.in_shadow(p));
    }

    #[test]
    fn test_shadow() {
        let mut w = World::new();
        let l = Light::new(Coord::point(0.0, 0.0, -10.0), Color::white());
        w.set_light(l);

        let s1 = Sphere::default();
        w.add_obj(Arc::new(s1));

        let s2 = Arc::new(Sphere::new(Coord::point(0.0, 0.0, 10.0)));
        w.add_obj(s2.clone());

        let r = Ray::new(Coord::point(0.0, 0.0, 5.0), Coord::vec(0.0, 0.0, 1.0));
        let i = Intersection::new(4.0, s2, Coord::vec(0.0, 0.0, 0.0));

        let comps = Comps::prepare_computations(i.clone(), r, vec![i.clone()]);
        let c = w.shade_hit(comps, 0);
        assert_eq!(c, Color::new(0.1, 0.1, 0.1, 0.0))
    }

    #[test]
    fn test_shadow_over_point() {
        let r = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Arc::new(Sphere::new(Coord::point(0.0, 0.0, 1.0)));
        let i = Intersection::new(5.0, s, Coord::vec(0.0, 0.0, 0.0));
        let comps = Comps::prepare_computations(i.clone(), r, vec![i.clone()]);
        assert!(comps.get_over_point().get_z() < -EPSILON/2.0);
        assert!(comps.get_point().get_z() > comps.get_over_point().get_z());
    }

    #[test]
    fn test_reflection_of_mat() {
        let l = Light::new(Coord::point(-10.0, 10.0, -10.0), Color::white());
        let mut s2 = Sphere::new(Coord::point(0.0, 0.0, 0.0));
        s2.set_transformation(Matrix::scaling(0.5, 0.5, 0.5));
        let mut m = Material::default();
        m.set_ambient(1.0);

        let mut w = World::new();
        w.add_light(l);
        w.add_obj(Arc::new(s2));

        let ray = Ray::new(Coord::point(0.0, 0.0, 0.0), Coord::vec(0.0, 0.0, 1.0));
        let xs = w.get_intersections(ray);
        let xs2 = Intersection::aggregate_intersections(xs.clone())[0].clone();
        let data = Comps::prepare_computations(xs2, ray, xs);
        assert_eq!(w.reflected_color(data, 0), Color::black());
    }

    #[test]
    fn test_reflection() {
        let mut w = World::default();
        let mut mat = Material::default();
        mat.set_reflection(0.5);
        let p = Plane::new(Matrix::translation(0.0, -1.0, 0.0), mat);
        w.add_obj(Arc::new(p.clone()));

        let ray = Ray::new(Coord::point(0.0, 0.0, -3.0), Coord::vec(0.0, -2_f32.sqrt()/2.0, 2_f32.sqrt()/2.0));
        let i = p.intersect(ray);
        let comps = Comps::prepare_computations(i.clone().unwrap()[0].clone(), ray, i.unwrap().clone());
        assert_eq!(w.reflected_color(comps.clone(), 0), Color::new(0.1911927, 0.23899086, 0.14339453, 0.0));
        assert_eq!(w.reflected_color(comps, 10), Color::black())
    }

    #[test]
    fn test_infinite_reflection() {
        // TODO: need a more ergonomic way to create worlds/objects/etc.
        let mut w = World::new();
        let mut mat = Material::default();
        mat.set_reflection(1.0);
        let upper = Plane::new(Matrix::translation(0.0, 1.0, 0.0), mat.clone());
        let lower = Plane::new(Matrix::translation(1.0, -1.0, 1.0), mat);
        w.add_obj(Arc::new(upper));
        w.add_obj(Arc::new(lower));
        w.add_light(Light::new(Coord::point(0.0, 0.0, 0.0), Color::white()));
        let ray = Ray::new(Coord::point(0.0, 0.0, 0.0), Coord::vec(0.0, 1.0, 0.0));
        assert_ne!(w.color_at(ray, 0), Color::black())
    }

    #[test]
    fn test_refraction() {
        // TODO: this test is a good example of what is bad about the current impl of getters/setters/etc.
        let mut mat = Material::default();
        mat.set_transparency(1.0);
        mat.set_refractive_index(1.5);
        let mut s1 = Sphere::default();
        s1.set_material(mat.clone());
        s1.set_transformation(Matrix::scaling(2.0, 2.0, 2.0));

        let mut s2 = Sphere::default();
        s2.set_transformation(Matrix::translation(0.0, 0.0, -0.25));
        mat.set_refractive_index(2.0);
        s2.set_material(mat.clone());

        let mut s3 = Sphere::default();
        s3.set_transformation(Matrix::translation(0.0, 0.0, 0.25));
        mat.set_refractive_index(2.5);
        s3.set_material(mat);

        let s1 = Arc::new(s1);
        let s2 = Arc::new(s2);
        let s3 = Arc::new(s3);

        let ray = Ray::new(Coord::point(0.0, 0.0, -4.0), Coord::vec(0.0, 0.0, 1.0));
        let reflectv = Coord::vec(0.0, 0.0, 0.0);
        let xs = vec![
            (Intersection::new(2.0, s1.clone(), reflectv), 1.0, 1.5),
            (Intersection::new(2.75, s2.clone(), reflectv), 1.5, 2.0),
            (Intersection::new(3.25, s3.clone(), reflectv), 2.0, 2.5),
            (Intersection::new(4.75, s2, reflectv), 2.5, 2.5),
            (Intersection::new(5.25, s3, reflectv), 2.5, 1.5),
            (Intersection::new(6.0, s1, reflectv), 1.5, 1.0)
        ];

        for (intersection, n1, n2) in xs.clone() {
            let comps = Comps::prepare_computations(intersection, ray, xs.iter().map(|x| x.0.clone()).collect());
            assert_eq!(comps.get_n1(), n1);
            assert_eq!(comps.get_n2(), n2);
        }
    }

    #[test]
    fn test_under_point() {
        let r = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let mut s = Sphere::glass_sphere();
        s.apply_transformation(Matrix::translation(0.0, 0.0, 1.0));
        let i = Intersection::new(5.0, Arc::new(s), Coord::vec(0.0, 0.0, 0.0));
        let xs = vec![i.clone()];
        let comps = Comps::prepare_computations(i, r, xs);
        assert!(comps.get_under_point().get_z() > EPSILON / 2.0);
        assert!(comps.get_under_point().get_z() > comps.get_point().get_z());
    }

    #[test]
    fn test_refracted_opaque() {
        let w = World::default();
        let s = w.get_object()[0].clone();      // TODO: should be renamed to get_objects
        let r = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let xs = vec![
            Intersection::new(4.0, s.clone(), Coord::vec(0.0, 0.0, 0.0)),
            Intersection::new(6.0, s.clone(), Coord::vec(0.0, 0.0, 0.0))
        ];
        let comps = Comps::prepare_computations(xs[0].clone(), r, xs);
        assert_eq!(w.refracted_color(comps, 5), Color::black())
    }

    #[test]
    fn test_refracted_depth() {
        let w = World::default();
        let l = w.get_light()[0].clone();
        let s = Arc::new(Sphere::glass_sphere());
        let mut w = World::new();
        w.add_obj(s.clone());
        w.add_light(l);

        let r = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let xs = vec![
            Intersection::new(4.0, s.clone(), Coord::vec(0.0, 0.0, 0.0)),
            Intersection::new(6.0, s.clone(), Coord::vec(0.0, 0.0, 0.0))
        ];
        let comps = Comps::prepare_computations(xs[0].clone(), r, xs);

        w.set_max_depth(5);
        assert_eq!(w.refracted_color(comps, 6), Color::black());
    }

    #[test]
    fn test_refracted_total_internal() {
        let w = World::default();
        let l = w.get_light()[0].clone();
        let s = Arc::new(Sphere::glass_sphere());
        let mut w = World::new();
        w.add_obj(s.clone());
        w.add_light(l);

        let r = Ray::new(
            Coord::point(0.0, 0.0, 2_f32.sqrt()/2.0),
            Coord::vec(0.0, 1.0, 0.0)
        );
        let xs = vec![
            Intersection::new(-2_f32.sqrt()/2.0, s.clone(), Coord::vec(0.0, 0.0, 0.0)),
            Intersection::new(2_f32.sqrt()/2.0, s.clone(), Coord::vec(0.0, 0.0, 0.0))
        ];

        let comps = Comps::prepare_computations(xs[1].clone(), r, xs);
        assert_eq!(w.refracted_color(comps, 0), Color::black());
    }

    #[test]
    fn test_refracted_color() {
        let mut a = Sphere::default();
        let mut mat = Material::default();
        mat.set_ambient(1.0);
        mat.set_texture(Arc::new(Pattern::test_pattern(Matrix::identity(4))));
        a.set_material(mat);
        let a = Arc::new(a);

        let mut b = Sphere::default();
        let mut mat = Material::default();
        mat.set_transparency(1.0);
        mat.set_refractive_index(1.5);
        b.set_material(mat);
        let b = Arc::new(b);

        let mut w = World::new();
        w.add_obj(a.clone());
        w.add_obj(b.clone());
        let l= Light::new(Coord::point(-10.0, 10.0, -10.0), Color::white());
        w.add_light(l);

        let r = Ray::new(Coord::point(0.0, 0.0, 0.1), Coord::vec(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(-0.9899, a.clone(), Coord::vec(0.0, 0.0, 0.0)),
            Intersection::new(-0.4899, b.clone(), Coord::vec(0.0, 0.0, 0.0)),
            Intersection::new(0.4899, b.clone(), Coord::vec(0.0, 0.0, 0.0)),
            Intersection::new(0.9899, a.clone(), Coord::vec(0.0, 0.0, 0.0))
        ];
        let comps = Comps::prepare_computations(xs[2].clone(), r, xs);
        assert_eq!(w.refracted_color(comps, 0), Color::new(0.0, 0.9988119, 0.048732005, 0.0))
    }


    #[test]
    fn test_shade_hit_refracted() {
        let mut w = World::default();
        let mut mat = Plane::default().get_material();
        mat.set_transparency(0.5);
        mat.set_refractive_index(1.5);
        let mut p = Plane::default();
        p.set_material(mat);
        p.apply_transformation(Matrix::translation(0.0, -1.0, 0.0));
        let p = Arc::new(p);

        let mut mat = Sphere::default().get_material();
        mat.set_color(Color::new(1.0, 0.0, 0.0, 0.0));
        mat.set_ambient(0.5);
        let mut s = Sphere::default();
        s.set_material(mat);
        s.apply_transformation(Matrix::translation(0.0, -3.5, -0.5));
        let s = Arc::new(s);

        w.add_obj(p.clone());
        w.add_obj(s.clone());

        let r = Ray::new(Coord::point(0.0, 0.0, -3.0), Coord::vec(0.0, -2_f32.sqrt()/2.0, 2_f32.sqrt()/2.0));
        let xs = vec![Intersection::new(2_f32.sqrt(), p.clone(), Coord::vec(0.0, 0.0, 0.0))];
        let comps = Comps::prepare_computations(xs[0].clone(), r, xs);
        assert_eq!(w.shade_hit(comps, 0), Color::new(0.93642543, 0.68642545, 0.68642545, 0.0));
    }

    #[test]
    fn test_schlick_total_internal_refraction() {
        let s = Arc::new(Sphere::glass_sphere());
        let r = Ray::new(Coord::point(0.0, 0.0, 2_f32.sqrt()), Coord::vec(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(-2_f32.sqrt()/2.0, s.clone(), Coord::vec(0.0, 0.0, 0.0)),
            Intersection::new(2_f32.sqrt()/2.0, s.clone(), Coord::vec(0.0, 0.0, 0.0))
        ];
        let comps = Comps::prepare_computations(xs[1].clone(), r, xs);
        assert_eq!(comps.schlick(), 1.0)
    }

    #[test]
    fn test_schlick_perpendicular_ray() {
        let s = Arc::new(Sphere::glass_sphere());
        let r = Ray::new(Coord::point(0.0, 0.0, 0.0), Coord::vec(0.0, 1.0, 0.0));
        let xs = vec![
            Intersection::new(-1.0, s.clone(), Coord::vec(0.0, 0.0, 0.0)),
            Intersection::new(1.0, s.clone(), Coord::vec(0.0, 0.0, 0.0))
        ];
        let comps = Comps::prepare_computations(xs[1].clone(), r, xs);
        assert_eq!(comps.schlick(), 0.040000003) // dam floating point errors
    }

    #[test]
    fn test_schlick_slight_angle() {
        let s = Arc::new(Sphere::glass_sphere());
        let r = Ray::new(Coord::point(0.0, 0.99, -2.0), Coord::vec(0.0, 0.0, 1.0));
        let xs = vec![
            Intersection::new(1.8589, s.clone(), Coord::vec(0.0, 0.0, 0.0))
        ];
        let comps = Comps::prepare_computations(xs[0].clone(), r, xs);
        assert_eq!(comps.schlick(), 0.48873067)
    }

    #[test]
    fn test_shade_hit_refracted_and_reflected() {
        let mut w = World::default();
        let mut mat = Plane::default().get_material();
        mat.set_transparency(0.5);
        mat.set_refractive_index(1.5);
        mat.set_reflection(0.5);
        let mut p = Plane::default();
        p.set_material(mat);
        p.apply_transformation(Matrix::translation(0.0, -1.0, 0.0));
        let p = Arc::new(p);

        let mut mat = Sphere::default().get_material();
        mat.set_color(Color::new(1.0, 0.0, 0.0, 0.0));
        mat.set_ambient(0.5);
        let mut s = Sphere::default();
        s.set_material(mat);
        s.apply_transformation(Matrix::translation(0.0, -3.5, -0.5));
        let s = Arc::new(s);

        w.add_obj(p.clone());
        w.add_obj(s.clone());

        let r = Ray::new(Coord::point(0.0, 0.0, -3.0), Coord::vec(0.0, -2_f32.sqrt()/2.0, 2_f32.sqrt()/2.0));
        let xs = vec![Intersection::new(2_f32.sqrt(), p.clone(), Coord::vec(0.0, -2_f32.sqrt()/2.0, 2_f32.sqrt()/2.0))];
        let comps = Comps::prepare_computations(xs[0].clone(), r, xs);
        assert_eq!(w.shade_hit(comps, 0), Color::new(0.945551, 0.70107293, 0.70098877, 0.0)); // TODO: some significant floating point error propigation here compared to correct value on p164
    }
}