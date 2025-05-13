use crate::{canvas::color::Color, coord::Coord, light::Light, material::Material, matrix::Matrix, ray::Ray, renderable::{Intersection, Renderable}, sphere::Sphere};

// I'm going to need to re-work this to add all objects, not just renderable ones aren't I
// probably just make a node type or something

pub struct World {
    light: Option<Light>,
    objects: Vec<Box<dyn Renderable>>
}

#[allow(dead_code)]
impl World {
    pub fn new() -> Self {
        Self { light: None, objects: Vec::<Box<dyn Renderable>>::new() }
    }

    pub fn default() -> Self {
        let l = Light::new(Coord::point(-10.0, 10.0, -10.0), Color::white());
        let s1 = Sphere::new(Coord::point(0.0, 0.0, 0.0));
        let mut s2 = Sphere::new(Coord::point(0.0, 0.0, 0.0));
        s2.set_transformation(Matrix::scaling(0.5, 0.5, 0.5));
        let mat = Material::new(0.1, 0.7, 0.2, 200.0, Color::new(0.8, 1.0, 0.6, 0.0));
        s2.set_material(mat);        


        let s1 = Box::new(s1) as Box<dyn Renderable>;
        let s2 = Box::new(s2) as Box<dyn Renderable>;
        let objs = vec![s1, s2];
        Self { light: Some(l), objects: objs }
    }

    fn get_light(&self) -> Option<Light> {
        self.light
    }

    fn get_object(&self) -> Vec<Box<dyn Renderable>> {
        self.objects.clone()
    }

    fn get_intersections(&self, ray: Ray) -> Vec<Intersection> {
        let mut data = Vec::new();
        for obj in self.get_object() {
            data.push(obj.intersect(&ray));
        }
        Intersection::aggregate_intersections(data)
    }    
}



#[cfg(test)]
mod tests {
    use crate::{canvas::color::Color, coord::Coord, light::Light, material::Material, matrix::Matrix, ray::Ray, renderable::compare_renderables, sphere::Sphere};

    use super::World;


    #[test]
    fn test_new() {
        let w = World::new();
        assert!(w.light.is_none());
        assert_eq!(w.objects.len(), 0);
    }

    #[test]
    fn test_getters() {
        let w = World::new();
        assert!(w.get_light().is_none());
        assert_eq!(w.get_object().len(), 0);
    }

    #[test]
    fn test_default() {
        let w = World::default();
        
        assert!(w.get_light().is_some());
        assert_eq!(w.get_light().unwrap(), Light::new(Coord::point(-10.0, 10.0, -10.0), Color::white()));
        
        let s1 = Sphere::default();
        let objs = w.get_object();
        assert_eq!(objs.len(), 2);
        assert!(compare_renderables(objs[0].as_ref(), &s1));

        let mut s2 = Sphere::default();
        let mat = Material::new(0.1, 0.7, 0.2, 200.0, Color::new(0.8, 1.0, 0.6, 0.0));
        s2.set_material(mat);
        s2.set_transformation(Matrix::scaling(0.5, 0.5, 0.5));

        assert!(compare_renderables(objs[1].as_ref(), &s2));
    }

    #[test]
    fn test_get_intersections() {
        let w = World::default();
        let r = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let xs = w.get_intersections(r);
        for x in &xs {
            println!("{}", x.get_time());
            println!("{:?}", x);
        }
        assert_eq!(xs.len(), 4);
        assert_eq!(xs[0].get_time(), 4.0);
        assert_eq!(xs[1].get_time(), 4.5);
        assert_eq!(xs[2].get_time(), 5.5);
        assert_eq!(xs[3].get_time(), 6.0);
    }

}