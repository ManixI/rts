use std::sync::Arc;

use crate::{coord::Coord, impl_getters_setters, material::Material, matrix::Matrix, ray::Ray, renderable::{Intersection, Renderable, RenderableBase, RenderableType}, tex::color::Color};



#[derive(Clone)]
pub struct Node {
    transformation: Matrix,
    children: Vec<Arc<dyn Renderable>>,
    parent: Option<Arc<Node>>
}

impl_getters_setters!(Node, transformation: Matrix, children: Vec<Arc<dyn Renderable>>, parent: Option<Arc<Node>>);

impl Node {
    pub fn new(transformation: Matrix, parent: Option<Arc<Node>>) -> Self {
        Self { transformation, children: Vec::<Arc<dyn Renderable>>::new(), parent }
    }

    pub fn add_child(&mut self, child: Arc<dyn Renderable>) {
        self.children.push(child);
    }

}

impl RenderableBase for Node {
    // TODO: there's got to be a better way to handle materials for nodes
    fn get_material(&self) -> Material { panic!("nodes do not have material") }
    fn set_material(&mut self, mat: Material) { panic!("nodes do not have material") }
    fn get_pos(&self) -> Coord { self.transformation.to_point() }
    fn get_transformation(&self) -> Matrix { self.transformation.clone() }
    fn set_transformation(&mut self, transform: Matrix) { self.transformation = transform }
    fn apply_transformation(&mut self, transform: Matrix) { self.transformation = self.get_transformation() * transform }
    fn get_type(&self) -> RenderableType { RenderableType::Node }
    fn clone_rc(&self) -> Arc<dyn Renderable> { Arc::new(self.clone()) }
    fn clone_dyn(&self) -> Box<dyn Renderable> { Box::new(self.clone()) }
    
    fn get_color_at(&self, pos: Coord) -> Color {
        let local_pos = self.get_transformation().inverse().unwrap() * pos;
        self.get_material().get_color_at(local_pos)
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    // TODO: is there a better way to do this?
    fn compare(&self, other: Arc<dyn Renderable>) -> bool {
        match other.as_any().downcast_ref::<Node>() {
            Some(_p) => false,
            None => false
        }   
    } 
}

impl Renderable for Node {
    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        todo!()
    }

    fn intersect_get_ray(&self, ray: Ray) -> (Ray, Option<Vec<Intersection>>) {
        todo!()
    }

    fn normal_at(&self, pos: Coord) -> Coord {
        todo!()
    }

    fn default() -> Self where Self: Sized {
        Self { transformation: Matrix::identity(4), children: Vec::<Arc<dyn Renderable>>::new(), parent: None }
    }
}