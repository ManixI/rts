use crate::matrix::Matrix;



#[derive(Debug, PartialEq, Clone)]
pub struct Camera {
    hsize: usize,
    vsize: usize,
    field_of_view: f32,
    transformation: Matrix,
}

#[allow(dead_code)]
impl Camera {
    pub fn new(hsize: usize, vsize: usize, field_of_view: f32) -> Self {
        Self { hsize, vsize, field_of_view, transformation: Matrix::identity(4) }
    }

    pub fn set_transformation(&mut self, transformation: Matrix) {
        self.transformation = transformation;
    }

    pub fn set_field_of_view(&mut self, field_of_view: f32) {
        self.field_of_view = field_of_view;
    }

    pub fn set_hsize(&mut self, hsize: usize) {
        self.hsize = hsize;
    }

    pub fn set_vsize(&mut self, vsize: usize) {
        self.vsize = vsize;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new() {
        let hsize = 160;
        let vsize = 120;
        let field_of_view = std::f32::consts::PI / 2.0;
        let cam = Camera::new(hsize, vsize, field_of_view);
        assert_eq!(cam.field_of_view, field_of_view);
        assert_eq!(cam.hsize, hsize);
        assert_eq!(cam.vsize, vsize);
        assert_eq!(cam.transformation, Matrix::identity(4));
    }
}