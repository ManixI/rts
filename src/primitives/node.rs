use std::sync::Arc;

use crate::{coord::Coord, impl_getters_setters, material::Material, matrix::Matrix, ray::Ray, renderable::{Intersection, Renderable, RenderableBase, RenderableType}, tex::color::Color};



#[derive(Debug, Clone)]
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

    // TODO: this should just be a Renderable method
    fn get_normal_at_local_space(&self, pos: Coord) -> Coord {
        todo!()
    }
}

impl PartialEq for Node {
    fn eq(&self, _other: &Self) -> bool {
        todo!("need to make this a pointer comparison")
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
    fn get_parent(&self) -> Option<Arc<Node>> { self.parent.clone() }
    fn set_parent(&mut self, parent: Option<Arc<Node>>) { self.parent = parent; }
 

    fn get_color_at(&self, pos: Coord) -> Color {
        let local_pos = self.get_transformation().inverse().unwrap() * pos;
        self.get_material().get_color_at(local_pos)
    }
    
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    
    fn compare(&self, _other: &Arc<dyn Renderable>) -> bool {
        false
    }
}

impl Renderable for Node {
    fn intersect(&self, ray: Ray) -> Option<Vec<Intersection>> {
        let (_, out) = self.intersect_get_ray(ray);
        out
    }

    fn intersect_get_ray(&self, ray: Ray) -> (Ray, Option<Vec<Intersection>>) {
        let ray = ray.transform(self.get_transformation().inverse().unwrap());
        let mut xs = Vec::<Intersection>::new();
        for child in self.get_children() {
            let intrs = child.intersect(ray);
            if intrs.is_some() {
                let mut inters = intrs.unwrap();
                xs.append(&mut inters);
            }
        }
        if xs.len() == 0 {
            return (ray, None)
        }
        let xs = Intersection::aggregate_intersections(xs);
        (ray, Some(xs))
    }

    fn normal_at(&self, pos: Coord) -> Coord {
        let pos = self.get_transformation().inverse().unwrap() * pos;
        // TODO: can I do better then a double normalization without duplicating code?
        (self.get_normal_at_local_space(pos) * self.get_transformation()).normalized()
    }

    fn default() -> Self where Self: Sized {
        Self { transformation: Matrix::identity(4), children: Vec::<Arc<dyn Renderable>>::new(), parent: None }
    }
}

mod tests {
    use std::sync::Arc;

use crate::{coord::Coord, matrix::Matrix, primitives::{node::Node, sphere::Sphere}, ray::Ray, renderable::{compare_renderables, Renderable, RenderableBase}};

    #[test]
    fn test_empty_intersect() {
        let g = Node::default();
        let r = Ray::new(Coord::point(0.0, 0.0, 0.0), Coord::vec(0.0, 0.0, 1.0));
        let xs = g.intersect(r);
        assert!(xs.is_none())
    }

    #[test]
    fn test_intersection() {
        let mut g = Node::default();
        let s1 = Sphere::default();
        let mut s2 = Sphere::default();
        s2.set_transformation(Matrix::translation(0.0, 0.0, -3.0));
        let mut s3 = Sphere::default();
        s3.set_transformation(Matrix::translation(5.0, 0.0, 0.0));

        let s1: Arc<dyn Renderable> = Arc::new(s1);
        let s2: Arc<dyn Renderable> = Arc::new(s2);
        let s3: Arc<dyn Renderable> = Arc::new(s3);

        g.add_child(s1.clone());
        g.add_child(s2.clone());
        g.add_child(s3.clone());

        let ray = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let xs = g.intersect(ray).unwrap();
        assert_eq!(xs.len(), 4);
        compare_renderables(&*xs[0].get_object(), &*s2);
        compare_renderables(&*xs[1].get_object(), &*s2);
        compare_renderables(&*xs[2].get_object(), &*s1);
        compare_renderables(&*xs[3].get_object(), &*s1);
    }

    #[test]
    fn test_group_transform() {
        let mut g = Node::default();
        g.apply_transformation(Matrix::scaling(2.0, 2.0, 2.0));
        let mut s = Sphere::default();
        s.apply_transformation(Matrix::translation(5.0, 0.0, 0.0));
        g.add_child(Arc::new(s));

        let r = Ray::new(Coord::point(10.0, 0.0, -10.0), Coord::vec(0.0, 0.0, 1.0));
        let xs = g.intersect(r).unwrap();
        assert_eq!(xs.len(), 2);
    }
}