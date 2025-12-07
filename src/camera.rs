use crate::matrix::Matrix;



#[derive(Debug, PartialEq, Clone)]
pub struct Camera {
    hsize: f32,  // should only ever accept integers, but stored internally as floats to reduce conversions
    vsize: f32,
    field_of_view: f32,
    transformation: Matrix,
    pixel_size: f32,
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
        Self { hsize, vsize, field_of_view, transformation: Matrix::identity(4), pixel_size }
    }

    pub fn set_transformation(&mut self, transformation: Matrix) {
        self.transformation = transformation;
    }

    pub fn set_field_of_view(&mut self, field_of_view: f32) {
        self.field_of_view = field_of_view;
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
}

#[cfg(test)]
mod tests {
    use core::f32::consts::PI;

    use super::*;

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
}