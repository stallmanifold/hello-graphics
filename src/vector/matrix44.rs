use std::ops;
use vector::vec3d::Vec3d;


// We do matrices in row-major order, which is typical. Note
// that OpenGL does column-major order.
#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
pub struct Matrix4x4<T> where T: Copy {
    rows: [[T; 4]; 4]
}

impl<T> Matrix4x4<T> where T: Copy + ops::Add<T, Output=T> 
                                   + ops::Mul<T, Output=T> 
                                   + ops::Div<T, Output=T> 
{
    #[inline(always)]
    fn new(a00: T, a01: T, a02: T, a03: T, 
           a10: T, a11: T, a12: T, a13: T,
           a20: T, a21: T, a22: T, a23: T, 
           a30: T, a31: T, a32: T, a33: T) -> Matrix4x4<T> 
    {
        let mut a = [[a00; 4]; 4];

        a[0][0] = a00; a[0][1] = a01; a[0][2] = a02; a[0][3] = a03;
        a[1][0] = a10; a[1][1] = a11; a[1][2] = a12; a[1][3] = a13;
        a[2][0] = a20; a[2][1] = a21; a[2][2] = a22; a[2][3] = a23;
        a[3][0] = a30; a[3][1] = a31; a[3][2] = a32; a[3][3] = a33;

        Matrix4x4 {
            rows: a
        }
    }

    #[inline(always)]
    pub fn shape(&self) -> (usize, usize) {
        (self.rows(), self.columns())
    }

    #[inline(always)]
    pub fn rows(&self) -> usize {
        self.rows[0].len()
    }

    #[inline(always)]
    pub fn columns(&self) -> usize {
        self.rows.len()
    }

    pub fn multiply(&self, other: &Matrix4x4<T>, scratch: &mut Matrix4x4<T>) {
        // This is just naive matrix multiplication from the definition. Since we are 
        // using onlt 4x4 matrices here, we do not need to do shape checks. Since this code 
        // is purely for studying the graphics pipeline, I do not bother to optimize it.
        // I would unroll these loops to optimize it.
        for i in 0..scratch.rows() {
            for j in 0..scratch.columns() {
                scratch[i][j] = self[i][0] * other[0][j]
                              + self[i][1] * other[1][j]
                              + self[i][2] * other[2][j]
                              + self[i][3] * other[3][j];
            }
        }
    }

    pub fn transposed(&self) -> Matrix4x4<T> {
        Matrix4x4::new(
            self[0][0], self[1][0], self[2][0], self[3][0],
            self[0][1], self[1][1], self[2][1], self[3][1],
            self[0][2], self[1][2], self[2][2], self[3][2],
            self[0][3], self[1][3], self[2][3], self[3][3],
        )
    }

    pub fn transpose(&mut self) {
        let temp = Matrix4x4::new(
            self[0][0], self[1][0], self[2][0], self[3][0],
            self[0][1], self[1][1], self[2][1], self[3][1],
            self[0][2], self[1][2], self[2][2], self[3][2],
            self[0][3], self[1][3], self[2][3], self[3][3],
        );

        *self = temp;
    }

    // This method is used to computer matrix-vector products (i.e. function application).
    // It computes with the 3x3 submatrix inside of a 4x4 matrix since for graphics purposes
    // we are operating in homogeneous coordinates. So we are going to ignore the displacement
    // terms in the fourth column of a 4x4 matrix.
    pub fn vec_multiply(&self, src: &Vec3d<T>, dest: &mut Vec3d<T>) {
        let x = src[0] * self[0][0] + src[1] * self[1][0] + src[2] * self[2][0];
        let y = src[0] * self[0][1] + src[1] * self[1][1] + src[2] * self[2][1];
        let z = src[0] * self[0][2] + src[1] * self[1][2] + src[2] * self[2][2];

        (*dest)[0] = x;
        (*dest)[1] = y;
        (*dest)[2] = z;
    }

    pub fn vec_disp_multiply(&self, src: &Vec3d<T>, dest: &mut Vec3d<T>) {
        let x = src[0] * self[0][0] + src[1] * self[1][0] + src[2] * self[2][0] + self[0][3];
        let y = src[0] * self[0][1] + src[1] * self[1][1] + src[2] * self[2][1] + self[1][3];
        let z = src[0] * self[0][2] + src[1] * self[1][2] + src[2] * self[2][2] + self[2][3];
        let w = src[0] * self[0][3] + src[1] * self[1][3] + src[2] * self[2][3] + self[3][3];

        (*dest)[0] = x / w;
        (*dest)[1] = y / w;
        (*dest)[2] = z / w;
    }
}

impl<T> ops::Index<usize> for Matrix4x4<T> where T: Copy {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        self.rows[index].as_ref()
    }
}

impl<'a, T> ops::Index<usize> for &'a Matrix4x4<T> where T: Copy {
    type Output = [T];

    fn index(&self, index: usize) -> &Self::Output {
        self.rows[index].as_ref()
    }
}

impl<T> ops::IndexMut<usize> for Matrix4x4<T> where T: Copy {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.rows[index].as_mut()
    }
}

impl<T> ops::Mul<Matrix4x4<T>> for Matrix4x4<T> 
    where T: Copy + ops::Add<T, Output=T> + ops::Mul<T, Output=T> + ops::Div<T, Output=T>
{
    type Output = Matrix4x4<T>;

    fn mul(self, other: Matrix4x4<T>) -> Self::Output {
        let a00 = self[0][0];
        let mut scratch = Matrix4x4::new(a00, a00, a00, a00, 
                                         a00, a00, a00, a00,
                                         a00, a00, a00, a00, 
                                         a00, a00, a00, a00);
        self.multiply(&other, &mut scratch);

        scratch
    }
}

impl<'a, T> ops::Mul<Matrix4x4<T>> for &'a Matrix4x4<T> 
    where T: Copy + ops::Add<T, Output=T> + ops::Mul<T, Output=T> + ops::Div<T, Output=T>
{
    type Output = Matrix4x4<T>;

    fn mul(self, other: Matrix4x4<T>) -> Self::Output {
        let a00 = self[0][0];
        let mut scratch = Matrix4x4::new(a00, a00, a00, a00, 
                                         a00, a00, a00, a00,
                                         a00, a00, a00, a00, 
                                         a00, a00, a00, a00);
        self.multiply(&other, &mut scratch);

        scratch
    }
}

impl<'a, 'b, T> ops::Mul<&'a Matrix4x4<T>> for &'b Matrix4x4<T> 
    where T: Copy + ops::Add<T, Output=T> + ops::Mul<T, Output=T> + ops::Div<T, Output=T>
{
    type Output = Matrix4x4<T>;

    fn mul(self, other: &Matrix4x4<T>) -> Self::Output {
        let a00 = self[0][0];
        let mut scratch = Matrix4x4::new(a00, a00, a00, a00, 
                                         a00, a00, a00, a00,
                                         a00, a00, a00, a00, 
                                         a00, a00, a00, a00);
        self.multiply(&other, &mut scratch);

        scratch
    }
}

impl<T> ops::MulAssign for Matrix4x4<T> 
    where T: Copy + ops::Add<T, Output=T> + ops::Mul<T, Output=T> + ops::Div<T, Output=T>
{   
    fn mul_assign(&mut self, other: Matrix4x4<T>) {
        let a00 = self[0][0];
        let mut scratch = Matrix4x4::new(a00, a00, a00, a00, 
                                         a00, a00, a00, a00,
                                         a00, a00, a00, a00, 
                                         a00, a00, a00, a00);
        self.multiply(&other, &mut scratch);

        *self = scratch;
    }
}

impl Matrix4x4<f32> {
    pub fn identity() -> Matrix4x4<f32> {
        Matrix4x4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn zero() -> Matrix4x4<f32> {
        Matrix4x4::new(
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0
        )
    }
}

impl Matrix4x4<f64> {
    pub fn identity() -> Matrix4x4<f64> {
        Matrix4x4::new(
            1.0, 0.0, 0.0, 0.0,
            0.0, 1.0, 0.0, 0.0,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0
        )
    }

    pub fn zero() -> Matrix4x4<f64> {
        Matrix4x4::new(
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0,
            0.0, 0.0, 0.0, 0.0
        )
    }

    // In a practical implementation it may be necessary to heap allocate 
    // the matrices and put them behind a Box.
    pub fn inverse(&self) -> Option<Matrix4x4<T>> {
        //Matrix44 s;
        let mut scratch = self.clone(); Matrix44 t (*this);
        
        // Forward elimination
        for i in 0..self.len() {
        //for (i = 0; i < 3 ; i++) {
            //int pivot = i;
            let mut pivot = i;

            T pivot_size = scratch[i][i];
            
            if (pivotsize < 0)
                pivot_size = -pivot_size;
                
                /*
                for (j = i + 1; j < 4; j++) {
                    T tmp = t[j][i];
                    
                    if (tmp < 0)
                        tmp = -tmp;
                        
                        if (tmp > pivotsize) {
                            pivot = j;
                            pivotsize = tmp;
                        }
                }
                */
                for j in (i + 1)..4 {
                    let temp = scratch[j][i];

                    if 
                } 
            if pivot_size == 0 {
                // Cannot invert singular matrix
                return None;
            }
            
            if (pivot != i) {
                for (j = 0; j < 4; j++) {
                    T tmp;
                    
                    tmp = t[i][j];
                    t[i][j] = t[pivot][j];
                    t[pivot][j] = tmp;
                    
                    tmp = s[i][j];
                    s[i][j] = s[pivot][j];
                    s[pivot][j] = tmp;
                }
            }
            
            for (j = i + 1; j < 4; j++) {
                T f = t[j][i] / t[i][i];
                
                for (k = 0; k < 4; k++) {
                    t[j][k] -= f * t[i][k];
                    s[j][k] -= f * s[i][k];
                }
            }
        }
        
        // Backward substitution
        for (i = 3; i >= 0; --i) {
            T f;
            
            if ((f = t[i][i]) == 0) {
                // Cannot invert singular matrix
                return Matrix44();
            }
            
            for (j = 0; j < 4; j++) {
                t[i][j] /= f;
                s[i][j] /= f;
            }
            
            for (j = 0; j < i; j++) {
                f = t[j][i];
                
                for (k = 0; k < 4; k++) {
                    t[j][k] -= f * t[i][k];
                    s[j][k] -= f * s[i][k];
                }
            }
        }
        
        return s;
    }

    pub fn invert(&mut self) {
        let inv = self.inverse();

        *self = inv;
    }
}

pub type Matrix4x4f = Matrix4x4<f32>;
pub type Matrix4x4d = Matrix4x4<f64>;

