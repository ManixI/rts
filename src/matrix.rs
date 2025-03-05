use std::ops;

// TODO: optimize this to use arrays, generic traits, etc.
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    data: Vec<Vec<f32>>,
}

#[allow(dead_code)]
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

    pub fn identity(size: usize) -> Self {
        let mut data = vec![vec![0.0; size]; size];
        for i in 0..size {
            data[i][i] = 1.0;
        }
        Matrix { data }
    }
}

// TODO: optimize this
impl ops::Mul<Matrix> for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let mut out = Vec::with_capacity(self.data.len());
        for row in 0..self.data.len() {
            let mut new = Vec::with_capacity(self.data.len());
            for col in 0..self.data.len() {
                let mut acc: f32 = 0.0;
                for i in 0..self.data.len() {
                    acc += self.data[row][i] * rhs.data[i][col];
                }
                new.push(acc);
            }
            out.push(new);
        }
        Self::new(out).unwrap()
    }
}

impl ops::Mul<Vec<f32>> for Matrix {
    type Output = Vec<f32>;

    fn mul(self, rhs: Vec<f32>) -> Self::Output {
        assert_eq!(rhs.len(), self.data.len());
        for row in self.data.iter() {
            assert_eq!(rhs.len(), row.len());
        }
        let mut out = Vec::with_capacity(rhs.len());

        for row in self.data {
            let mut acc = 0.0;
            for i in 0..rhs.len() {
                acc += row[i] * rhs[i];
            }
            out.push(acc)
        }

        out
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

    #[test]
    fn test_mat_equality() {
        let data1 = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ];
        let mat1 = Matrix::new(data1.clone()).unwrap();
        let mat2 = Matrix::new(data1.clone()).unwrap();
        assert!(mat1 == mat2);
        
        let data2 = vec![
            vec![2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0],
            vec![8.0, 7.0, 8.0, 5.0],
            vec![4.0, 3.0, 2.0, 1.0],
        ];
        let mat3 = Matrix::new(data2.clone()).unwrap();
        assert!(mat1 != mat3);
        assert!(mat2 != mat3);
    }

    #[test]
    fn test_mat_mul() {
        let data1 = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.0, 6.0, 7.0, 8.0],
            vec![9.0, 8.0, 7.0, 6.0],
            vec![5.0, 4.0, 3.0, 2.0],
        ];
        let data2 = vec![
            vec![-2.0, 1.0, 2.0, 3.0],
            vec![3.0, 2.0, 1.0, -1.0],
            vec![4.0, 3.0, 6.0, 5.0],
            vec![1.0, 2.0, 7.0, 8.0],
        ];
        let mat1 = Matrix::new(data1).unwrap();
        let mat2 = Matrix::new(data2).unwrap();
        let mat3 = mat1 * mat2;

        let data3 = vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ];

        assert_eq!(mat3, Matrix::new(data3).unwrap())
    }

    #[test]
    fn test_mat_tuple_mul() {
        let data = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];
        let mat = Matrix::new(data).unwrap();
        let tuple = vec![1.0, 2.0, 3.0, 1.0];
        assert_eq!(mat * tuple, vec![18.0, 24.0, 33.0, 1.0])
    }

    #[test]
    fn test_identity() {
        let data1 = vec![
            vec![1.0, 0.0, 0.0, 0.0],
            vec![0.0, 1.0, 0.0, 0.0],
            vec![0.0, 0.0, 1.0, 0.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];
        assert_eq!(Matrix::identity(4), Matrix::new(data1).unwrap());

        let data2 = vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ];
        let mat = Matrix::new(data2).unwrap();
        assert_eq!(mat.clone() * Matrix::identity(4), mat);
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