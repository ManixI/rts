use crate::{canvas::color::Color, coord::Coord, light::Light, matrix::Matrix, ray::{intersection::Intersection, Ray}, renderable::Renderable, sphere::Sphere};

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
        let s1 = Box::new(s1);
        let objs = vec![s1 as Box<dyn Renderable>];
        Self { light: Some(l), objects: objs }
    }

    fn get_light(&self) -> Option<Light> {
        self.light
    }

    fn get_object(&self) -> Vec<Box<dyn Renderable>> {
        //self.objects.clone()
        todo!()
    }

    fn get_intersections(&self, ray: Ray) -> Vec<Intersection> {
        todo!()
    }    
}



#[cfg(test)]
mod tests {

}