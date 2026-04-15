use crate::{matrix::Matrix, ray::Ray, coord::Coord};
use std::ops;



#[derive(Debug, PartialEq, Clone)]
pub struct Camera {
    hsize: f32,  // should only ever accept integers, but stored internally as floats to reduce conversions
    vsize: f32,
    field_of_view: f32,
    transformation: Matrix,
    pixel_size: f32,
    half_width: f32,
    half_height: f32
}

#[allow(dead_code)]
impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Self {
        let hsize: f32 = hsize as f32;
        let vsize: f32 = vsize as f32;
        let half_view: f32 = (field_of_view/2.0).tan();
        let aspect: f32 = hsize / vsize;
        let mut half_width: f32 = half_view;
        let mut half_height: f32 = half_view / aspect; 
        if aspect < 1.0 {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        // TODO: experiment with different vsize and hsize
        let pixel_size: f32 = (half_width * 2.0) / hsize; // assumes pixels are square, so no need to account for vsize
        Self { hsize, vsize, field_of_view, transformation: Matrix::identity(4), pixel_size, half_height, half_width }
    }

    fn new_transformed(hsize: usize, vsize: usize, field_of_view: f32, transformation: Matrix) -> Self {
        let mut out = Self::new(hsize, vsize, field_of_view);
        out.set_transformation(transformation);
        out
    }

    pub fn set_transformation(&mut self, transformation: Matrix) {
        assert_eq!(transformation.get_size(), 4);
        self.transformation = transformation;
    }

    fn get_transformation(&self) -> Matrix {
        self.transformation.clone()
    }

    pub fn transform(&mut self, transformation: Matrix) {
        self.set_transformation(self.get_transformation() * transformation);
    }

    pub fn set_field_of_view(&mut self, field_of_view: f32) {
        self.field_of_view = field_of_view;
    }

    fn get_field_of_view(&self) -> f32 {
        self.field_of_view
    }

    pub fn set_hsize(&mut self, hsize: usize) {
        self.hsize = hsize as f32;
    }

    fn get_hsize(&self) -> usize {
        self.hsize as usize
    }

    pub fn set_vsize(&mut self, vsize: usize) {
        self.vsize = vsize as f32;
    }

    fn get_vsize(&self) -> usize {
        self.vsize as usize
    }

    fn get_pixel_size(&self) -> f32 {
        self.pixel_size
    }

    fn get_half_width(&self) -> f32 {
        self.half_width
    }

    fn get_half_height(&self) -> f32 {
        self.half_height
    }

    fn ray_for_pixel(&self, x: usize, y: usize) -> Ray {
        // get center pf px
        let x_offset = (x as f32 + 0.5) * self.pixel_size;
        let y_offset = (y as f32 + 0.5) * self.pixel_size;

        // calc world space loc of px
        let world_x = self.get_half_width() - x_offset;
        let world_y = self.get_half_height() - y_offset;

        // transform canvas point and origin to compute ray's dir
        // canvas always at z = -1
        let transform = self.get_transformation().inverse().unwrap();
        let pixel = transform.clone() * Coord::point(world_x, world_y, -1.0);
        let origin = transform * Coord::point(0.0, 0.0, 0.0);
        let dir = (pixel - origin).normalized();

        Ray::new(origin, dir)
    }
}

impl ops::Mul<Matrix> for Camera {
    type Output = Camera;
    
    fn mul(self, rhs: Matrix) -> Self::Output {
        assert_eq!(rhs.get_size(), 4);
        let hsize = self.get_hsize();
        let vsize = self.get_vsize();
        let fov = self.get_field_of_view();
        let mat = self.transformation.mul(rhs);
        Self::new_transformed(
            hsize,
            vsize, 
            fov,
            mat
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use core::f32::consts::PI;
    use crate::{coord::Coord, ray::Ray};

    const EPSILON: f32 = 0.000001;

    #[test]
    fn test_new() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = PI / 2.0;
        let cam = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(cam.field_of_view, field_of_view);
        assert_eq!(cam.get_hsize(), hsize);
        assert_eq!(cam.get_vsize(), vsize);
        assert_eq!(cam.transformation, Matrix::identity(4));
    }

    #[test]
    fn test_pix_size_h_canvas() {
        let cam = Camera::new(200, 125, PI/2.0 );
        assert_eq!(cam.get_pixel_size(), 0.01);
    }

    #[test]
    fn test_pix_size_v_canvas() {
        let cam = Camera::new(125, 200, PI/2.0);
        assert_eq!(cam.get_pixel_size(), 0.01);
    }

    #[test]
    fn test_ray_though_center() {
        let cam = Camera::new(201, 101, PI/2.0);
        let ray: Ray = cam.ray_for_pixel(100, 50);
        Coord::assert_roughly_eq(&ray.get_origin(), &Coord::point(0.0, 0.0, 0.0), EPSILON);
        Coord::assert_roughly_eq(&ray.get_direction(), &Coord::vec(0.0, 0.0, -1.0), EPSILON);
    }

    #[test]
    fn test_ray_though_corner() {
        let cam = Camera::new(201, 101, PI/2.0);
        let ray: Ray = cam.ray_for_pixel(0, 0);
        Coord::assert_roughly_eq(&ray.get_origin(), &Coord::point(0.0, 0.0, 0.0), EPSILON);
        Coord::assert_roughly_eq(&ray.get_direction(), &Coord::vec(0.6651864, 0.33259322, -0.66851234), EPSILON);
    }

    #[test]
    fn test_ray_after_cam_transformation() {
        let mut cam = Camera::new(201, 101, PI/2.0);
        cam.set_transformation(Matrix::rotate_y(PI/4.0) * Matrix::translation(0.0, -2.0, 5.0));
        let ray: Ray = cam.ray_for_pixel(100, 50);
        Coord::assert_roughly_eq(&ray.get_origin(), &Coord::point(0.0, 2.0, -5.0), EPSILON);
        Coord::assert_roughly_eq(&ray.get_direction(), &Coord::vec(2.0_f32.sqrt()/2.0, 0.0, -2.0_f32.sqrt()/2.0), EPSILON);
    }
}