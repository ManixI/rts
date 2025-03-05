use std::ops;

// TODO: optimize this to use arrays, generic traits, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    data: Vec<Vec<f32>>,
}

impl Matrix {
    pub fn new(data: Vec<Vec<f32>>) -> Option<Self> {
        for row in data.iter() {
            if row.len() != data.len() {
                return None;
            }
        }
        if data.len() != 2 && data.len() != 3 && data.len() != 4 {
            return None;
        }
        Some(Self{data})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_mat_4x4() {
        let data = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5]
        ];
        let mat = Matrix::new(data);
        assert!(mat.is_some());
        let mat = mat.unwrap();
        assert!(mat.data[0][0] == 1.0);
        assert!(mat.data[0][3] == 4.0);
        assert!(mat.data[1][0] == 5.5);
        assert!(mat.data[1][2] == 7.5);
        assert!(mat.data[2][2] == 11.0);
        assert!(mat.data[3][0] == 13.5);
        assert!(mat.data[3][2] == 15.5);
    }

    #[test]
    fn test_new_mat_3x3() {
        let data = vec![
            vec![-3.0, 5.0, 0.0],
            vec![1.0, -2.0, -7.0],
            vec![0.0, 1.0, 1.0],
        ];
        let mat = Matrix::new(data);
        assert!(mat.is_some());
        let mat = mat.unwrap();
        assert!(mat.data[0][0] == -3.0);
        assert!(mat.data[1][1] == -2.0);
        assert!(mat.data[2][2] == 1.0)
    }

    #[test]
    fn test_new_mat_2x2() {
        let data = vec![
            vec![-3.0, 5.0],
            vec![1.0, -2.0],
        ];
        let mat = Matrix::new(data);
        assert!(mat.is_some());
        let mat = mat.unwrap();
        assert_eq!(mat.data[0][0], -3.0);
        assert_eq!(mat.data[0][1], 5.0);
        assert_eq!(mat.data[1][0], 1.0);
        assert_eq!(mat.data[1][1], -2.0);
    }
}

/*struct Mat2x2 {
    data: [[f32; 2]; 2],
}

struct Mat3x3 {
    data: [[f32; 3]; 3],
}

struct Mat4x4 {
    data: [[f32; 4]; 4],
}


impl<T> Matrix<T> {
    pub fn new(d: Vec<Vec<f32>>) -> Option<Self> {
        match (d.len(), d[0].len()) {
            (2,2) => Mat2x2 { data: [
                [d[0][0], d[0][1]],
                [d[1][0], d[1][1]],
                ]},
            (3, 3) => Mat3x3 { data: [
                [d[0][0], d[0][1], d[0][2]],
                [d[1][0], d[1][1], d[1][2]],
                [d[2][0], d[2][1], d[2][2]],
                ]},
            (4, 4) => Mat4x4 { data: [
                [d[0][0], d[0][1], d[0][2], d[0][3]],
                [d[1][0], d[1][1], d[1][2], d[1][3]],
                [d[2][0], d[2][1], d[2][2], d[2][3]],
                [d[3][0], d[3][1], d[3][2], d[3][3]],
                ]},
            _ => None
        }
    }
}*/