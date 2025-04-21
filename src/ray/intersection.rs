use std::rc::Rc;

use super::Intersect;

pub struct Intersection<T> {
    t: f32,
    object: Rc<T>
}

#[allow(dead_code)]
impl<T: Intersect<T>> Intersection <T> {
    pub fn new(t: f32, object: Rc<T>) -> Self {
        Self { t, object }
    }

    pub fn get_time(&self) -> f32 {
        self.t
    }

    pub fn get_object(&self) -> &T {
        self.object.as_ref()
    }

    pub fn get_object_pointer(&self) -> Rc<T> {
        self.object.clone()
    }
}

#[cfg(test)]
mod tests {
    
}