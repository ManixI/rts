use crate::{canvas::color::Color, coord::Coord, light::Light, material::Material, matrix::Matrix, ray::{intersection::Intersection, Ray}, renderable::Renderable, sphere::Sphere};

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
        let mat = Material::new(0.2, 0.7, 0.2, 200.0, Color::new(0.8, 1.0, 0.6, 0.0));
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
        //todo!()
    }

    fn get_intersections(&self, ray: Ray) -> Vec<Intersection> {
        todo!()
    }    
}



#[cfg(test)]
mod tests {
    use crate::{canvas::color::Color, coord::Coord, light::Light, material::Material, matrix::Matrix, renderable::{compare_renderables, Renderable}, sphere::Sphere};

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
        let mat = Material::new(0.2, 0.7, 0.2, 200.0, Color::new(0.8, 1.0, 0.6, 0.0));
        s2.set_material(mat);
        s2.set_transformation(Matrix::scaling(0.5, 0.5, 0.5));

        assert!(compare_renderables(objs[1].as_ref(), &s2));
    }

}