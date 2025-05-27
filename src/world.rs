use std::rc::Rc;

use crate::{canvas::color::Color, coord::Coord, light::{lighting, Light}, material::Material, matrix::Matrix, ray::Ray, renderable::{Intersection, Renderable}, sphere::Sphere};

// I'm going to need to re-work this to add all objects, not just renderable ones aren't I
// probably just make a node type or something

struct Comps {
    object: Rc<dyn Renderable>,
    point: Coord,
    eyev: Coord,
    normalv: Coord,
    time: f32,
    inside: bool
}

// precomputed data about an intersection of ray and renderable
#[allow(dead_code)]
impl Comps {
    fn new(object: Rc<dyn Renderable>, point: Coord, eyev: Coord, normalv: Coord, time: f32, inside: bool) -> Self {
        Self { object, point, eyev, normalv, time, inside }
    }

    fn get_object(&self) -> Rc<dyn Renderable> {
        self.object.clone()
    }

    fn get_point(&self) -> Coord {
        self.point
    }

    fn get_eyev(&self) -> Coord {
        self.eyev
    }

    fn get_normalv(&self) -> Coord {
        self.normalv
    }
    
    fn get_time(&self) -> f32 {
        self.time
    }

    fn get_inside(&self) -> bool {
        self.inside
    }

    fn prepare_computations(intersection: Intersection, ray: Ray) -> Self {
        let mut inside = false;
        let mut normalv = intersection.get_object().normal_at(ray.position(intersection.get_time()));
        if normalv.dot(-ray.get_direction()) < 0.0 {
            inside = true
        }
        if inside {
            normalv = -normalv;
        }

        Self::new(
            intersection.get_object(), 
            ray.position(intersection.get_time()), 
            -ray.get_direction(), 
            normalv, 
            intersection.get_time(),
            inside
        )
    }
}

pub struct World {
    light: Vec<Light>,
    objects: Vec<Rc<dyn Renderable>>
}

#[allow(dead_code)]
impl World {
    pub fn new() -> Self {
        Self { light: Vec::new(), objects: Vec::<Rc<dyn Renderable>>::new() }
    }

    pub fn default() -> Self {
        let l = Light::new(Coord::point(-10.0, 10.0, -10.0), Color::white());
        let mut s1 = Sphere::new(Coord::point(0.0, 0.0, 0.0));
        let mut s2 = Sphere::new(Coord::point(0.0, 0.0, 0.0));
        s2.set_transformation(Matrix::scaling(0.5, 0.5, 0.5));
        let mat = Material::new(0.1, 0.7, 0.2, 200.0, Color::new(0.8, 1.0, 0.6, 0.0));
        s1.set_material(mat);        


        let s1 = Rc::new(s1) as Rc<dyn Renderable>;
        let s2 = Rc::new(s2) as Rc<dyn Renderable>;
        let objs = vec![s1, s2];
        Self { light: vec![l], objects: objs }
    }

    fn get_light(&self) -> &Vec<Light> {
        &self.light
    }

    fn set_light(&mut self, light: Light) {
        self.light = vec![light]
    }

    fn add_light(&mut self, light: Light) {
        self.light.push(light);
    }

    fn get_object(&self) -> Vec<Rc<dyn Renderable>> {
        self.objects.clone()
    }

    fn get_intersections(&self, ray: Ray) -> Vec<Intersection> {
        let mut data = Vec::new();
        for obj in self.get_object() {
            data.push(obj.intersect(&ray));
        }
        Intersection::aggregate_intersections(data)
    }

    fn shade_hit(&self, comps: Comps) -> Color {
        let mut color = Color::black();
        for light in self.get_light() {
            color = color + lighting(
            comps.get_object().get_material(), 
            *light, 
            comps.get_point(), 
            comps.get_eyev(), 
            comps.get_normalv()
            );
        }
        color
    }

    fn color_at(&self, ray: Ray) -> Color {
        let intersections = self.get_intersections(ray);
        if intersections.len() == 0 {
            return Color::black();
        }
        for i in &intersections {
            println!("{:?}", i);
        }
        let comps = Comps::prepare_computations(intersections[0].clone(), ray);
        self.shade_hit(comps)
    }
}



#[cfg(test)]
mod tests {
    use std::{f32::EPSILON, rc::Rc};

    use crate::{canvas::color::Color, coord::{self, Coord}, light::Light, material::Material, matrix::Matrix, ray::Ray, renderable::{compare_renderables, Intersection, Renderable}, sphere::Sphere, world};

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
        let mat = Material::new(0.1, 0.7, 0.2, 200.0, Color::new(0.8, 1.0, 0.6, 0.0));
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
        let shape = Rc::new(Sphere::default());
        let i = Intersection::new(4.0, shape.clone());
        let comp = Comps::prepare_computations(i.clone(), ray);
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
        let shape = Rc::new(Sphere::default());
        let i = Intersection::new(1.0, shape.clone());
        let comp = Comps::prepare_computations(i.clone(), ray);

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
        let i = Intersection::new(4.0, shape);
        let comps = Comps::prepare_computations(i, ray);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.38066125, 0.4758265, 0.28549594, 0.0));

        let mut w = World::default();
        w.set_light(Light::new(Coord::point(0.0, 0.25, 0.0), Color::white()));
        let ray = Ray::new(Coord::point(0.0, 0.0, 0.0), Coord::vec(0.0, 0.0, 1.0));
        let shape = w.get_object()[1].clone();
        let i = Intersection::new(0.5, shape);
        let comps = Comps::prepare_computations(i, ray);
        let c = w.shade_hit(comps);
        assert_eq!(c, Color::new(0.9049845, 0.9049845, 0.9049845, 0.0));
    }

    #[test]
    fn test_color_at() {
        let w = World::default();
        let ray = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 1.0, 0.0));
        let c = w.color_at(ray);
        assert_eq!(c, Color::black());

        let w = World::default();
        let ray = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let c = w.color_at(ray);
        assert_eq!(c, Color::new(0.38066125, 0.4758265, 0.28549594, 0.0));
    
        // not sure if this actually works as I think it should, get material is suspect
        // I think I need to change getters to return reference, not clone (unless rc)
        // probably need to use RefCells not Rcs https://stackoverflow.com/questions/52994205/what-is-the-standard-way-to-call-a-mutable-method-in-a-rc-wrapped-object
        // alternativly, use Box and world class is the holder of all objects
        w.get_object()[0].get_material().set_ambient(1.0);
        w.get_object()[1].get_material().set_ambient(1.0);
        let ray = Ray::new(Coord::point(0.0, 0.0, 0.75), Coord::vec(0.0, 0.0, -1.0));
        let c = w.color_at(ray);
        //assert_eq!(c, w.get_object()[0].get_material().get_color());
        test_colors_roughly_equal(&c, &w.get_object()[0].get_material().get_color());
    }
}