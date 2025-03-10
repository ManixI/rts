use std::ops;
use super::Coord;

// TODO: optimize this to use arrays, generic traits, etc.
// TODO: replace options with results to better do error handling
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    data: Vec<Vec<f32>>,
}

#[allow(dead_code)]
impl Matrix {
    /// expects a 2d vec that is square and of a size 2x2 - 4x4
    pub fn new(data: Vec<Vec<f32>>) -> Self {
        assert!(data.len() >= 2 && data.len() <= 4);
        for row in data.iter() {
            assert!(row.len() == data.len());
        }
        Self{data}
    }

    pub fn identity(size: usize) -> Self {
        assert!(size >= 2 && size <= 4);
        let mut data = vec![vec![0.0; size]; size];
        for i in 0..size {
            data[i][i] = 1.0;
        }
        Matrix { data }
    }

    pub fn translation_from_coord(vec: Coord) -> Self {
        let mut new = Self::identity(4);
        new.data[0][3] = vec.get_x();
        new.data[1][3] = vec.get_y();
        new.data[2][3] = vec.get_z();
        new
    }

    pub fn translation(x: f32, y: f32, z: f32) -> Self {
        let mut new = Self::identity(4);
        new.data[0][3] = x;
        new.data[1][3] = y;
        new.data[2][3] = z;
        new
    }

    pub fn to_vec(&self) -> Coord {
        Coord::vec(self.data[0][3], self.data[1][3], self.data[2][3])
    }

    /// returns transposed version of matrix
    pub fn transpose(&self) -> Self {
        let mut new = vec![vec![0.0; self.data.len()]; self.data.len()];
        for i in 0..self.data.len() {
            for j in 0..self.data.len() {
                new[j][i] = self.data[i][j];
            }
        }
        Self::new(new)
    }

    fn determinate_2x2(&self) -> f32 {
        self.data[0][0] * self.data[1][1] - self.data[1][0] * self.data[0][1]
    }

    // create a submatrix by removing 1 row and 1 col
    fn sub_matrix(&self, row_num: i32, col_num: i32) -> Self {
        assert!(self.data.len() > 1);
        let mut out = Vec::with_capacity(self.data.len()-1);

        let mut r = -1;
        for row in self.data.as_slice() {
            r += 1;
            if r == row_num {
                continue;
            }
            let mut new = Vec::with_capacity(self.data.len()-1);
            let mut c = -1;
            for val in row {
                c += 1;
                if c == col_num {
                    continue;
                }
                new.push(*val);
            }
            out.push(new);
        } 
        Self::new(out)
    }  

    fn minor(&self, row_num: i32, col_num: i32) -> Option<f32> {
        let sub = self.sub_matrix(row_num, col_num);
        if sub.data.len() >= 2 {
            // recursively call determinate func to get minors util reaches 2x2 size
            return Some(sub.determinate());
        } else if sub.data.len() < 2 {
            return None;
        }
        Some(sub.determinate_2x2())
    }

    fn cofactor(&self, row_num: i32, col_num: i32) -> f32 {
        let out = self.minor(row_num, col_num).expect("\nERROR: bad matrix in cofactor\n");
        if (row_num + col_num) % 2 == 1 {
            return -out;
        }
        out
    }

    pub fn determinate(&self) -> f32 {
        if self.data.len() == 2 {
            return self.determinate_2x2();
        }
        let mut det = 0.0;
        for (idx, val) in self.data[0].iter().enumerate() {
            det += val * self.cofactor(0, idx as i32);
        }
        det
    }

    pub fn inverse(&self) -> Option<Self> {
        if self.determinate() == 0.0 {
            return None;
        }
        let mut new = vec![vec![0.0; self.data.len()]; self.data.len()];
        for i in 0..self.data.len() {
            for j in 0..self.data.len() {
                new[i][j] = self.cofactor(i as i32, j as i32);
            }
        }
        let mut out = Self::new(new).transpose();
        let det = self.determinate();
        for i in 0..self.data.len() {
            for j in 0..self.data.len() {
                out.data[i][j] /= det;
            }
        }
        Some(out)
    }


}

// TODO: optimize this
// TODO: multiplication should not consume variables
// TODO: error handling for mis-matched multiplication
impl ops::Mul<Matrix> for Matrix {
    type Output = Self;

    /// remember, order matters for matrix multiplication
    fn mul(self, rhs: Self) -> Self::Output {
        assert_eq!(self.data.len(), rhs.data.len());
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
        Self::new(out)
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

impl ops::Mul<Coord> for Matrix {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Self::Output {
        assert_eq!(self.data.len(), 4);
        for row in self.data.iter() {
            assert_eq!(row.len(), 4);
        }
        let mut data = Vec::with_capacity(4);
        for row in self.data {
            data.push(row[0]*rhs.get_x() + row[1]*rhs.get_y() + row[2]*rhs.get_z() + row[3]*rhs.get_w())
        }
        Coord::new(data[0], data[1], data[2], data[3])
    }
}

#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;

    /// Tests roughly equal, necessary if testing floating point operations
    const EPSILON: f32 = 0.00001;
    fn test_roughly_equal(a: &Matrix, b: &Matrix) -> bool {
        if a.data.len() != b.data.len() {
            println!("\na and b have different number of columns\n");
            return false;
        }
        for (row_a, row_b) in a.data.iter().zip(b.data.clone()) {
            if row_a.len() != row_b.len() {
                println!("\na and b have different number of rows");
                return false;
            }
        }
        for (row_a, row_b) in a.data.iter().zip(b.data.clone()) {
            for (val_a, val_b) in row_a.iter().zip(row_b) {
                if (val_a-val_b).abs() > EPSILON {
                    println!("\n{} !~= {}\n", val_a, val_b);
                    return false;
                }
            }
        }
        true
    }

    #[test]
    fn test_new_mat_4x4() {
        let data = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![5.5, 6.5, 7.5, 8.5],
            vec![9.0, 10.0, 11.0, 12.0],
            vec![13.5, 14.5, 15.5, 16.5]
        ];
        let mat = Matrix::new(data);
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
        let mat1 = Matrix::new(data1.clone());
        let mat2 = Matrix::new(data1.clone());
        assert!(mat1 == mat2);
        
        let data2 = vec![
            vec![2.0, 3.0, 4.0, 5.0],
            vec![6.0, 7.0, 8.0, 9.0],
            vec![8.0, 7.0, 8.0, 5.0],
            vec![4.0, 3.0, 2.0, 1.0],
        ];
        let mat3 = Matrix::new(data2.clone());
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
        let mat1 = Matrix::new(data1);
        let mat2 = Matrix::new(data2);
        let mat3 = mat1 * mat2;

        let data3 = vec![
            vec![20.0, 22.0, 50.0, 48.0],
            vec![44.0, 54.0, 114.0, 108.0],
            vec![40.0, 58.0, 110.0, 102.0],
            vec![16.0, 26.0, 46.0, 42.0],
        ];

        assert_eq!(mat3, Matrix::new(data3))
    }

    #[test]
    fn test_mat_tuple_mul() {
        let data = vec![
            vec![1.0, 2.0, 3.0, 4.0],
            vec![2.0, 4.0, 4.0, 2.0],
            vec![8.0, 6.0, 4.0, 1.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];
        let mat = Matrix::new(data);
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
        assert_eq!(Matrix::identity(4), Matrix::new(data1));

        let data2 = vec![
            vec![0.0, 1.0, 2.0, 4.0],
            vec![1.0, 2.0, 4.0, 8.0],
            vec![2.0, 4.0, 8.0, 16.0],
            vec![4.0, 8.0, 16.0, 32.0],
        ];
        let mat = Matrix::new(data2);
        assert_eq!(mat.clone() * Matrix::identity(4), mat);
    }

    #[test]
    fn test_transpose() {
        let data1 = vec![
            vec![0.0, 9.0, 3.0, 0.0],
            vec![9.0, 8.0, 0.0, 8.0],
            vec![1.0, 8.0, 5.0, 3.0],
            vec![0.0, 0.0, 5.0, 8.0],
        ];
        let data2 = vec![
            vec![0.0, 9.0, 1.0, 0.0],
            vec![9.0, 8.0, 8.0, 0.0],
            vec![3.0, 0.0, 5.0, 5.0],
            vec![0.0, 8.0, 3.0, 8.0],
        ];
        let mat = Matrix::new(data1);
        assert_eq!(mat.transpose(), Matrix::new(data2));
    }

    #[test]
    fn test_determinate2x2() {
        let data = vec![
            vec![1.0, 5.0],
            vec![-3.0, 2.0],
        ];
        let mat = Matrix::new(data);
        assert_eq!(mat.determinate_2x2(), 17.0)
    }

    #[test]
    fn test_sub_matrix() {
        let data = vec![
            vec![1.0, 5.0, 0.0],
            vec![-3.0, 2.0, 7.0],
            vec![0.0, 6.0, -3.0],
        ];
        let expected = vec![
            vec![-3.0, 2.0],
            vec![0.0, 6.0],
        ];
        let mat = Matrix::new(data).sub_matrix(0, 2);
        assert_eq!(mat, Matrix::new(expected));

        let data = vec![
            vec![-6.0, 1.0, 1.0, 6.0],
            vec![-8.0, 5.0, 8.0, 6.0],
            vec![-1.0, 0.0, 8.0, 2.0],
            vec![-7.0, 1.0, -1.0, 1.0],
        ];
        let expected = vec![
            vec![-6.0, 1.0, 6.0],
            vec![-8.0, 8.0, 6.0],
            vec![-7.0, -1.0, 1.0],
        ];
        let mat = Matrix::new(data).sub_matrix(2, 1);
        assert_eq!(mat, Matrix::new(expected))
    }

    #[test]
    fn test_minor() {
        let data = vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ];
        let mat = Matrix::new(data);
        assert_eq!(mat.minor(1, 0).unwrap(), 25.0);
    }

    #[test]
    fn test_cofactor() {
        let data = vec![
            vec![3.0, 5.0, 0.0],
            vec![2.0, -1.0, -7.0],
            vec![6.0, -1.0, 5.0],
        ];
        let mat = Matrix::new(data);
        assert_eq!(mat.minor(0, 0).unwrap(), -12.0);
        assert_eq!(mat.cofactor(0, 0), -12.0);
        assert_eq!(mat.minor(1, 0).unwrap(), 25.0);
        assert_eq!(mat.cofactor(1, 0), -25.0);
    }

    #[test]
    fn test_determinate() {
        let data = vec![
            vec![1.0, 2.0, 6.0],
            vec![-5.0, 8.0, -4.0],
            vec![2.0, 6.0, 4.0],
        ];
        let mat = Matrix::new(data);
        assert_eq!(mat.cofactor(0, 0), 56.0);
        assert_eq!(mat.cofactor(0, 1), 12.0);
        assert_eq!(mat.cofactor(0, 2), -46.0);
        assert_eq!(mat.determinate(), -196.0);

        let data = vec![
            vec![-2.0, -8.0, 3.0, 5.0],
            vec![-3.0, 1.0, 7.0, 3.0],
            vec![1.0, 2.0, -9.0, 6.0],
            vec![-6.0, 7.0, 7.0, -9.0],
        ];
        let mat = Matrix::new(data);
        assert_eq!(mat.cofactor(0, 0), 690.0);
        assert_eq!(mat.cofactor(0, 1), 447.0);
        assert_eq!(mat.cofactor(0, 2), 210.0);
        assert_eq!(mat.cofactor(0, 3), 51.0);
        assert_eq!(mat.determinate(), -4071.0);
    }

    #[test]
    fn test_is_invertable() {
        let data = vec![
            vec![6.0, 4.0, 4.0, 4.0],
            vec![5.0, 5.0, 7.0, 6.0],
            vec![4.0, -9.0, 3.0, -7.0],
            vec![9.0, 1.0, 7.0, -6.0],
        ];
        let mat = Matrix::new(data);
        assert_eq!(mat.determinate(), -2120.0);
        assert_eq!(mat.inverse().is_some(), true);

        let data = vec![
            vec![-4.0, 2.0, -2.0, -3.0],
            vec![9.0, 6.0, 2.0, 6.0],
            vec![0.0, -5.0, 1.0, -5.0],
            vec![0.0, 0.0, 0.0, 0.0],
        ];
        let mat = Matrix::new(data);
        assert_eq!(mat.determinate(), 0.0);
        assert_eq!(mat.inverse(), None);
    }
        
    #[test]
    fn test_invert() {
        let data = vec![
            vec![-5.0, 2.0, 6.0, -8.0],
            vec![1.0, -5.0, 1.0, 8.0],
            vec![7.0, 7.0, -6.0, -7.0],
            vec![1.0, -3.0, 7.0, 4.0],
        ];
        let mat = Matrix::new(data);
        let inverse = mat.inverse().unwrap();
        assert_eq!(mat.determinate(), 532.0);
        assert_eq!(mat.cofactor(2, 3), -160.0);
        assert_eq!(inverse.data[3][2], -160.0/532.0);
        assert_eq!(mat.cofactor(3, 2), 105.0);
        assert_eq!(inverse.data[2][3], 105.0/532.0);

        let test = vec![
            vec![0.21804512, 0.45112783, 0.24060151, -0.04511278],
            vec![-0.80827070, -1.456767, -0.44360903, 0.5206767],
            vec![-0.078947365, -0.22368420, -0.05263158, 0.19736843],
            vec![-0.52255636, -0.81390977, -0.30075186, 0.30639097],
        ];
        let test_mat = Matrix::new(test);
        assert_eq!(inverse, test_mat);
    }

    #[test]
    fn test_invert_2() {
        let data1 = vec![
            vec![3.0, -9.0, 7.0, 3.0],
            vec![3.0, -8.0, 2.0, -9.0],
            vec![-4.0, 4.0, 4.0, 1.0],
            vec![-6.0, 5.0, -1.0, 1.0],
        ];
        let data2 = vec![
            vec![8.0, 2.0, 2.0, 2.0],
            vec![3.0, -1.0, 7.0, 0.0],
            vec![7.0, -1.0, 5.0, 4.0],
            vec![6.0, -2.0, 0.0, 5.0],
        ];
        let a = Matrix::new(data1);
        let b = Matrix::new(data2);
        let c = a.clone() * b.clone();
        assert!(test_roughly_equal(&(c * b.inverse().unwrap()), &a));
        //assert_eq!(c * b.invert().unwrap(), a);
    }

    #[test]
    fn test_translation() {
        let vec = Coord::point(5.0, -3.0, 2.0);
        let mat = Matrix::translation_from_coord(vec);
        let data = vec![
            vec![1.0, 0.0, 0.0, 5.0],
            vec![0.0, 1.0, 0.0, -3.0],
            vec![0.0, 0.0, 1.0, 2.0],
            vec![0.0, 0.0, 0.0, 1.0],
        ];
        let test = Matrix::new(data);
        assert_eq!(mat, test);

        let mat = Matrix::translation(5.0, -3.0, 2.0);
        assert_eq!(mat, test);
        
        // test translation
        let mat = Matrix::translation_from_coord(Coord::vec(5.0, -3.0, 2.0));
        let p = Coord::point(-3.0, 4.0, 5.0);
        assert_eq!(mat * p, Coord::point(2.0, 1.0, 7.0));

        // test inverse translation
        let mat = Matrix::translation(5.0, -3.0, 2.0);
        let inv = mat.inverse().expect("inverse was none");
        let p = Coord::point(-3.0, 4.0, 5.0);
        assert_eq!(inv * p, Coord::point(-8.0, 7.0, 3.0));
    }

    #[test]
    fn test_scaling() {

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