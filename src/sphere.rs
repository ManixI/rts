use core::f32;
use std::rc::Rc;
use crate::material::Material;
use crate::matrix::Matrix;
use crate::ray::Ray;
use crate::renderable::{Intersection, Renderable, RenderableType};
use super::Coord;


#[derive(Debug, PartialEq, Clone)]
pub struct Sphere {
    //origin: Coord,
    //radius: f32,
    transformation: Matrix,
    material: Material,
}

#[allow(dead_code)]
impl Sphere {
    /// a sphere at position (0, 0, 0) with a radius of 1
    pub fn default() -> Self {
        Self { 
            transformation: Matrix::identity(4), 
            material: Material::default()
        }
    }

    /**
     * TODO: spheres should use constructor with default characteristics rather then
     * current form.
     * ex creating a sphere should look like this:
     * let s = Sphere::new()
     *      .set_transformation(matrix)
     *      .set_material(material)ee;
     */
    pub fn new(origin: Coord) -> Self {
        assert!(origin.is_point());
        Self { 
            transformation: Matrix::from_point(&origin),
            material: Material::default()
        }
    }

    pub fn set_material(&mut self, mat: Material) {
        self.material = mat;
    }

    pub fn set_transformation(&mut self, mat: Matrix) {
        self.transformation = mat;
    }

    // TODO: remove clone here
    pub fn apply_transformation(&mut self, mat: Matrix) {
        self.transformation = self.transformation.clone() * mat;
    }

    pub fn get_origin(&self) -> Coord {
        self.transformation.to_point()
    }

    pub fn get_transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    // breaks when you transform the sphere
    fn geometric_intersect(&self, ray: &Ray) -> Option<[f32; 2]> {
        // ref: https://discussions.unity.com/t/how-do-i-find-the-closest-point-on-a-line/588895/3
        /*let dir = ray.get_direction();//.normalized();
        let v = self.get_origin() - ray.get_origin();
        let d = v.dot(dir);
        
        let nearest = ray.get_origin() + dir * d;
        let dist = v.dot(v) - d * d; 

        // better to square radius for comparison then sqrt the dist as dist isn't actually needed
        //let dist = nearest.get_x().powi(2) + nearest.get_y().powi(2) + nearest.get_z().powi(2);
        //if dist > 1.0 { // radius is always 1
        if dist > 1.0 {
            return None;
        }*/
        let l = self.get_origin() - ray.get_origin();
        let tca = l.dot(ray.get_direction());
        let d2 = l.dot(l) - tca.powi(2);
        if d2 > 1.0 {
            return None;
        }
        let thc = (1.0-d2).sqrt();
        let out = [
            tca - thc,
            tca + thc
        ];
        println!("{:?}", out);


        /*
        // assume nearest point is exactly radius far away
        let mut c = 0.0;
        // if not, calculate actual distance
        if dist != 1.0 {
            //let a = ray.get_direction().dot(ray.get_direction());
            let b = dist;
            c = (1.0 + b.powi(2)).sqrt();
        }

        let mut out: [f32; 2] = [0.0; 2];

        let vec = nearest - (dir*c) - ray.get_origin();
        out[0] = dir.scalar_multiple(&vec).unwrap();

        let vec = nearest + (dir*c)- ray.get_origin();
        out[1] = dir.scalar_multiple(&vec).unwrap();

        if out[0] > out[1] {
            let tmp = out[0];
            out[0] = out[1];
            out[1] = tmp;
        }*/
        Some(out)
    }

    fn analytical_intersect(&self, ray: &Ray) -> Option<[f32; 2]> {
        let l = ray.get_origin().to_vec();// - self.get_origin(); origin should always be 0
        let a = ray.get_direction().dot(ray.get_direction());
        let b = 2.0 * ray.get_direction().dot(l);
        let c = l.dot(l) - 1.0;
        quadratic_formula_helper(a, b, c) 
    }

    
}

/// assumes `a` value is 1 (ie ray direction is normalized)
/// TODO: look into optimization: https://lomont.org/posts/2022/a-better-quadratic-formula-algorithm/
fn quadratic_formula_helper(a: f32,b: f32, c: f32) -> Option<[f32; 2]> {
    //println!("{}", a);
    let disc = b.powi(2) - 4.0 * c * a;
    if disc < 0.0 {
        return None;
    } else if disc == 0.0 {
        let out = -0.5 * b;
        return Some([out, out]);
    }
    let val = (-b + disc.sqrt()) / (2.0 * a);
    let mut out = [
        val,
        -val - (b/a)
    ];
    //println!("{:?}", out);
    if out[0] > out[1] {
        let tmp = out[0];
        out[0] = out[1];
        out[1] = tmp;
    }
    Some(out)
}

//const EPSILON: f32 = 0.02;

impl Renderable for Sphere {
    fn get_material(&self) -> Material {
        self.material
    }

    fn get_pos(&self) -> Coord {
        self.get_origin()
    }

    fn get_transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    fn get_type(&self) -> RenderableType {
        RenderableType::Sphere
    }

    fn clone_rc(&self) -> Rc<dyn Renderable> {
        Rc::new(self.clone())
    }

    fn clone_dyn(&self) -> Box<dyn Renderable> {
        Box::new(self.clone())
    }
    
    fn intersect(&self, ray: &Ray) -> Option<[Intersection; 2]> {
        let ray = ray.transform(self.get_transformation().inverse().unwrap());
        let data = self.analytical_intersect(&ray);
        //let data = self.geometric_intersect(&ray);
        if data.is_none() {
            return None;
        }
        let data = data.unwrap();
        let t = Rc::new(self.clone());
        Some([Intersection::new(data[0], t.clone()), Intersection::new(data[1], t)])
    }

    /// func assumes pos is on the sphere, if it is not results are undefined
    fn normal_at(&self, pos: Coord) -> Coord {
        let object_pos = self.transformation.inverse().unwrap() * pos;
        let obj_normal = object_pos - Coord::point(0.0, 0.0, 0.0);
        let mut world_norm = self.transformation.inverse()
            .unwrap()
            .transpose()
            * obj_normal;
        world_norm.set_w(0.0);
        world_norm.normalized()
    }
}

#[allow(unused_imports, dead_code)]
mod tests {
    use core::f32;
    use std::thread::spawn;

    use crate::sphere;

    use super::*;
    //use crate::{coord::Coord, matrix::Matrix, ray::{Intersect, Ray}, sphere::Sphere};

    const EPSILON: f32 = 0.0000001;

    fn test_near_0(vec: &Coord) -> Coord {
        let mut vec = vec.get_as_list();
        for (i, val) in vec.clone().iter().enumerate() {
            if val.abs() < EPSILON {
                vec[i] = 0.0;
            }
        }
        Coord::from_list(&vec)
    }

    #[test]
    fn test_sphere_creation() {
        let s = Sphere::default();
        //assert_eq!(s.radius, 1.0);
        assert_eq!(s.transformation, Matrix::identity(4));
        assert_eq!(s.material, Material::default());

        let s = Sphere::new(Coord::point(0.0, 0.0, 0.0));
        //assert_eq!(s.radius, 2.0);
        assert_eq!(s.transformation, Matrix::identity(4));
        assert_eq!(s.material, Material::default());
    }

    #[test]
    fn test_get_origin() {
        let mut s = Sphere::default();
        assert_eq!(s.get_origin(), Coord::point(0.0, 0.0, 0.0));
        s.apply_transformation(Matrix::scaling(0.5, 0.5, 0.5));
        assert_eq!(s.get_origin(), Coord::point(0.0, 0.0, 0.0));
    }

    #[test]
    fn test_set_transformation() {
        let mut s = Sphere::default();
        let mat = Matrix::translation(2.0, 3.0, 4.0);
        s.set_transformation(mat.clone());
        assert_eq!(s.get_transformation(), mat);
        assert_eq!(s.get_origin(), mat.to_point());
    }

    #[test]
    fn test_transformation() {
        let ray = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let mut s = Sphere::default();
        s.set_transformation(Matrix::translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&ray);
        assert!(xs.is_none());

        s.set_transformation(Matrix::identity(4));
        s.apply_transformation(Matrix::translation(5.0, 0.0, 0.0));
        let xs = s.intersect(&ray);
        assert!(xs.is_none());
    }

    #[test]
    fn test_sphere_intersection_no_position() {
        let r = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());
    
        let r = Ray::new(Coord::point(0.0, 1.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());

        let r = Ray::new(Coord::point(0.0, 2.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_none());

        let r = Ray::new(Coord::point(0.0, 0.0, 0.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());

        let r = Ray::new(Coord::point(0.0, 0.0, 5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());

        let r = Ray::new(Coord::point(0.0, 1.0, -5.0), Coord::vec(0.0, -0.1, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());
    }

    #[test]
    fn test_sphere_intersection() {
        let r = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        let xs = [xs[0].get_time(), xs[1].get_time()];
        assert_eq!(xs[0], 4.0);
        assert_eq!(xs[1], 6.0);
    
        let r = Ray::new(Coord::point(0.0, 1.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        let xs = [xs[0].get_time(), xs[1].get_time()];
        assert_eq!(xs[0], 5.0);
        assert_eq!(xs[1], 5.0);

        let r = Ray::new(Coord::point(0.0, 2.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_none());

        let r = Ray::new(Coord::point(0.0, 0.0, 0.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        let xs = [xs[0].get_time(), xs[1].get_time()];
        assert_eq!(xs[0], -1.0);
        assert_eq!(xs[1], 1.0);

        let r = Ray::new(Coord::point(0.0, 0.0, 5.0), Coord::vec(0.0, 0.0, 1.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        let xs = [xs[0].get_time(), xs[1].get_time()];
        assert_eq!(xs[0], -6.0);
        assert_eq!(xs[1], -4.0);

        let r = Ray::new(Coord::point(0.0, 0.0, -5.0), Coord::vec(0.0, 0.0, 1.0));
        let mut s = Sphere::default();
        s.apply_transformation(Matrix::scaling(0.5, 0.5, 0.5));
        let xs = r.intersect(&s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        let xs = [xs[0].get_time(), xs[1].get_time()];
        assert_eq!(xs[0], 4.5);
        assert_eq!(xs[1], 5.5);

        let r = Ray::new(Coord::point(0.0, 0.0, -10.0), Coord::vec(0.0, 0.0, 2.0));
        let s = Sphere::default();
        let xs = r.intersect(&s);
        assert!(xs.is_some());
        let xs = xs.unwrap();
        let xs = [xs[0].get_time(), xs[1].get_time()];
        assert_eq!(xs[0], 4.5);
        assert_eq!(xs[1], 5.5);
    }

    #[test]
    fn test_normal_at() {
        let s = Sphere::default();
        let n = s.normal_at(Coord::point(1.0, 0.0, 0.0));
        assert_eq!(n, Coord::vec(1.0, 0.0, 0.0));
        
        let n = s.normal_at(Coord::point(0.0, 1.0, 0.0));
        assert_eq!(n, Coord::vec(0.0, 1.0, 0.0));
        
        let n = s.normal_at(Coord::point(0.0, 0.0, 1.0));
        assert_eq!(n, Coord::vec(0.0, 0.0, 1.0));
        
        let n = s.normal_at(Coord::point(3.0_f32.sqrt()/3.0, 3.0_f32.sqrt()/3.0, 3.0_f32.sqrt()/3.0));
        assert_eq!(n, Coord::vec(3.0_f32.sqrt()/3.0, 3.0_f32.sqrt()/3.0, 3.0_f32.sqrt()/3.0));
        assert_eq!(n, n.normalized());

        let mut s = Sphere::default();
        s.set_transformation(Matrix::translation(0.0, 1.0, 0.0));
        let n = s.normal_at(Coord::point(0.0, 1.70711, -0.70711));
        assert_eq!(n, Coord::vec(0.0, 0.7071068, -0.70710677));

        let mut s = Sphere::default();
        s.set_transformation(Matrix::scaling(1.0, 0.5, 1.0) * Matrix::rotate_z(f32::consts::PI/5.0));
        let n = s.normal_at(Coord::point(0.0, (2.0_f32.sqrt())/2.0, -(2.0_f32.sqrt())/2.0));
        let n = test_near_0(&n);
        assert_eq!(n, Coord::vec(0.0, 0.97014254, -0.24253564));
    }
}