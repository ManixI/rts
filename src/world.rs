use crate::{canvas::color::Color, coord::Coord, light::Light, matrix::Matrix, ray::intersection::{Intersect, Intersection}, sphere::Sphere};



pub struct world {
    light: Option<Light>,
    objects: Vec<Sphere>
}

#[allow(dead_code)]
impl world {
    pub fn new() -> Self {
        world { light: None, objects: Vec::<Sphere>::new() }
    }

    pub fn default() -> Self {
        let l = Light::new(Coord::point(-10.0, 10.0, -10.0), Color::white());
        let s1 = Sphere::new(Coord::point(0.0, 0.0, 0.0));
        let mut s2 = Sphere::new(Coord::point(0.0, 0.0, 0.0));
        s2.set_transformation(Matrix::scaling(0.5, 0.5, 0.5));
        let objs = vec![s1, s2];
        Self { light: Some(l), objects: objs }
    }    
}

impl Intersect<Self> for world {

    fn intersect(&self, ray: &crate::ray::Ray) -> Option<[Intersection<Self>; 2]> {
        todo!()
    }
}


#[cfg(test)]
mod tests {

}