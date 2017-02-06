use std::ops;


pub fn vec4d<T: Copy>(x: T, y: T, z: T, w: T) -> Vec4d<T> {
    Vec4d::new(x, y, z, w)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vec4d<T> where T: Copy {
    inner: [T; 4]
}

impl<T> Vec4d<T> where T: Copy {
    #[inline(always)]
    pub fn new(x: T, y: T, z: T, w: T) -> Vec4d<T> {
        Vec4d {
            inner: [x, y, z, w]
        }
    }

    #[inline]
    pub fn dim(&self) -> usize {
        4
    }
}

impl<T> Vec4d<T> where T: Copy + ops::Add<T, Output=T> + ops::Mul<T, Output=T> {
    pub fn norm(&self) -> T {
        (self.inner[0] * self.inner[0]) +
        (self.inner[1] * self.inner[1]) +
        (self.inner[2] * self.inner[2]) +
        (self.inner[3] * self.inner[3])
    }

    pub fn dot_product(&self, other: Vec4d<T>) -> T {
        (self.inner[0] * other.inner[0]) + 
        (self.inner[1] * other.inner[1]) + 
        (self.inner[2] * other.inner[2]) + 
        (self.inner[3] * other.inner[3])
    }
}

impl<T> Vec4d<T> where T: Copy + ops::Add<T, Output=T> 
                               + ops::Mul<T, Output=T> 
                               + ops::Div<T, Output=T> 
{
    fn normalize(&self) -> Vec4d<T> {
        self / self.norm()
    }
}

impl Vec4d<f32> {
    pub fn length(&self) -> f32 {
        f32::sqrt(self.norm())
    }
}

impl Vec4d<f64> {
    pub fn length(&self) -> f64 {
        f64::sqrt(self.norm())
    }
}

impl<T> ops::Add<Vec4d<T>> for Vec4d<T> where T: Copy + ops::Add<T, Output=T> {
    type Output = Vec4d<T>;

    fn add(self, other: Vec4d<T>) -> Vec4d<T> {
        let x = self.inner[0] + other.inner[0];
        let y = self.inner[1] + other.inner[1];
        let z = self.inner[2] + other.inner[2];
        let w = self.inner[3] + other.inner[3];

        vec4d(x, y, z, w)
    }
}

impl<'a, T> ops::Add<Vec4d<T>> for &'a Vec4d<T> where T: Copy + ops::Add<T, Output=T> {
    type Output = Vec4d<T>;

    fn add(self, other: Vec4d<T>) -> Vec4d<T> {
        let x = self.inner[0] + other.inner[0];
        let y = self.inner[1] + other.inner[1];
        let z = self.inner[2] + other.inner[2];
        let w = self.inner[3] + other.inner[3];

        vec4d(x, y, z, w)
    }
}

impl<'a, T> ops::Add<&'a Vec4d<T>> for Vec4d<T> where T: Copy + ops::Add<T, Output=T> {
    type Output = Vec4d<T>;

    fn add(self, other: &Vec4d<T>) -> Vec4d<T> {
        let x = self.inner[0] + other.inner[0];
        let y = self.inner[1] + other.inner[1];
        let z = self.inner[2] + other.inner[2];
        let w = self.inner[3] + other.inner[3];

        vec4d(x, y, z, w)
    }
}

impl<'a, 'b, T> ops::Add<&'a Vec4d<T>> for &'b Vec4d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    type Output = Vec4d<T>;

    fn add(self, other: &'a Vec4d<T>) -> Vec4d<T> {
        let x = self.inner[0] + other.inner[0];
        let y = self.inner[1] + other.inner[1];
        let z = self.inner[2] + other.inner[2];
        let w = self.inner[3] + other.inner[3];

        vec4d(x, y, z, w)
    }
}

impl<T> ops::Sub for Vec4d<T> where T: Copy + ops::Sub<T, Output=T> {
    type Output = Vec4d<T>;

    fn sub(self: Vec4d<T>, other: Vec4d<T>) -> Vec4d<T> {
        let x = self.inner[0] - other.inner[0];
        let y = self.inner[1] - other.inner[1];
        let z = self.inner[2] - other.inner[2];
        let w = self.inner[3] - other.inner[3];

        vec4d(x, y, z, w)
    }
}

impl<'a, T> ops::Sub<Vec4d<T>> for &'a Vec4d<T> where T: Copy + ops::Sub<T, Output=T> {
    type Output = Vec4d<T>;

    fn sub(self, other: Vec4d<T>) -> Vec4d<T> {
        let x = self.inner[0] - other.inner[0];
        let y = self.inner[1] - other.inner[1];
        let z = self.inner[2] - other.inner[2];
        let w = self.inner[3] - other.inner[3];

        vec4d(x, y, z, w)
    }
}

impl<'a, 'b, T> ops::Sub<&'a Vec4d<T>> for &'b Vec4d<T> where T: Copy + ops::Sub<T, Output=T> {
    type Output = Vec4d<T>;

    fn sub(self, other: &'a Vec4d<T>) -> Vec4d<T> {
        let x = self.inner[0] - other.inner[0];
        let y = self.inner[1] - other.inner[1];
        let z = self.inner[2] - other.inner[2];
        let w = self.inner[3] - other.inner[3];

        vec4d(x, y, z, w)
    }
}

impl<T> ops::Mul<T> for Vec4d<T> where T: Copy + ops::Mul<T, Output=T> {
    type Output = Vec4d<T>;

    fn mul(self, scalar: T) -> Vec4d<T> {
        let x = self.inner[0] * scalar;
        let y = self.inner[1] * scalar;
        let z = self.inner[2] * scalar;
        let w = self.inner[3] * scalar;

        vec4d(x, y, z, w)
    }
}

impl<'a, T> ops::Mul<T> for &'a Vec4d<T> where T: Copy + ops::Mul<T, Output=T> {
    type Output = Vec4d<T>;

    fn mul(self, scalar: T) -> Vec4d<T> {
        let x = self.inner[0] * scalar;
        let y = self.inner[1] * scalar;
        let z = self.inner[2] * scalar;
        let w = self.inner[3] * scalar;

        vec4d(x, y, z, w)
    }
}

impl<T> ops::Div<T> for Vec4d<T> where T: Copy + ops::Div<T, Output=T> {
    type Output = Vec4d<T>;

    fn div(self, scalar: T) -> Vec4d<T> {
        let x = self.inner[0] / scalar;
        let y = self.inner[1] / scalar;
        let z = self.inner[2] / scalar;
        let w = self.inner[3] / scalar;

        vec4d(x, y, z, w)
    }
}

impl<'a, T> ops::Div<T> for &'a Vec4d<T> where T: Copy + ops::Div<T, Output=T> {
    type Output = Vec4d<T>;

    fn div(self, scalar: T) -> Vec4d<T> {
        let x = self.inner[0] / scalar;
        let y = self.inner[1] / scalar;
        let z = self.inner[2] / scalar;
        let w = self.inner[3] / scalar;

        vec4d(x, y, z, w)
    }
}

impl<T> ops::AddAssign<Vec4d<T>> for Vec4d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    fn add_assign(&mut self, other: Vec4d<T>) {
        self.inner[0] = self.inner[0] + other.inner[0];
        self.inner[1] = self.inner[1] + other.inner[1];
        self.inner[2] = self.inner[2] + other.inner[2];
        self.inner[3] = self.inner[3] + other.inner[3];
    }
}

impl<'a, T> ops::AddAssign<Vec4d<T>> for &'a mut Vec4d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    fn add_assign(&mut self, other: Vec4d<T>) {
        self.inner[0] = self.inner[0] + other.inner[0];
        self.inner[1] = self.inner[1] + other.inner[1];
        self.inner[2] = self.inner[2] + other.inner[2];
        self.inner[3] = self.inner[3] + other.inner[3];
    }
}

impl<'a, 'b, T> ops::AddAssign<&'a Vec4d<T>> for &'b mut Vec4d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    fn add_assign(&mut self, other: &'a Vec4d<T>) {
        self.inner[0] = self.inner[0] + other.inner[0];
        self.inner[1] = self.inner[1] + other.inner[1];
        self.inner[2] = self.inner[2] + other.inner[2];
        self.inner[3] = self.inner[3] + other.inner[3];
    }
}

impl<T> ops::SubAssign<Vec4d<T>> for Vec4d<T> where T: Copy + ops::Sub<T, Output=T> {
    fn sub_assign(&mut self, other: Vec4d<T>) {
        self.inner[0] = self.inner[0] - other.inner[0];
        self.inner[1] = self.inner[1] - other.inner[1];
        self.inner[2] = self.inner[2] - other.inner[2];
        self.inner[3] = self.inner[3] - other.inner[3];
    }
}

impl<'a, T> ops::SubAssign<Vec4d<T>> for &'a mut Vec4d<T> where T: Copy + ops::Sub<T, Output=T> {
    fn sub_assign(&mut self, other: Vec4d<T>) {
        self.inner[0] = self.inner[0] - other.inner[0];
        self.inner[1] = self.inner[1] - other.inner[1];
        self.inner[2] = self.inner[2] - other.inner[2];
        self.inner[3] = self.inner[3] - other.inner[3];
    }
}

impl<'a, 'b, T> ops::SubAssign<&'a Vec4d<T>> for &'b mut Vec4d<T> where T: Copy + ops::Sub<T, Output=T> {
    fn sub_assign(&mut self, other: &'a Vec4d<T>) {
        self.inner[0] = self.inner[0] - other.inner[0];
        self.inner[1] = self.inner[1] - other.inner[1];
        self.inner[2] = self.inner[2] - other.inner[2];
        self.inner[3] = self.inner[3] - other.inner[3];
    }
}

impl<T> ops::MulAssign<T> for Vec4d<T> where T: Copy + ops::Mul<T, Output=T> {
    fn mul_assign(&mut self, scalar: T) {
        self.inner[0] = self.inner[0] * scalar;
        self.inner[1] = self.inner[1] * scalar;
        self.inner[2] = self.inner[2] * scalar;
        self.inner[3] = self.inner[3] * scalar;
    }
}

impl<'a, T> ops::MulAssign<T> for &'a mut Vec4d<T> where T: Copy + ops::Mul<T, Output=T> {
    fn mul_assign(&mut self, scalar: T) {
        self.inner[0] = self.inner[0] * scalar;
        self.inner[1] = self.inner[1] * scalar;
        self.inner[2] = self.inner[2] * scalar;
        self.inner[3] = self.inner[3] * scalar;
    }
}

impl<T> ops::DivAssign<T> for Vec4d<T> where T: Copy + ops::Div<T, Output=T> {
    fn div_assign(&mut self, scalar: T) {
        self.inner[0] = self.inner[0] / scalar;
        self.inner[1] = self.inner[1] / scalar;
        self.inner[2] = self.inner[2] / scalar;
        self.inner[3] = self.inner[3] / scalar;
    }
}

impl<'a, T> ops::DivAssign<T> for &'a mut Vec4d<T> where T: Copy + ops::Div<T, Output=T> {
    fn div_assign(&mut self, scalar: T) {
        self.inner[0] = self.inner[0] / scalar;
        self.inner[1] = self.inner[1] / scalar;
        self.inner[2] = self.inner[2] / scalar;
        self.inner[3] = self.inner[3] / scalar;
    }
}

impl<T> ops::Neg for Vec4d<T> where T: Copy + ops::Neg<Output=T> {
    type Output = Vec4d<T>;

    fn neg(self) -> Vec4d<T> {
        let x = -self.inner[0];
        let y = -self.inner[1];
        let z = -self.inner[2];
        let w = -self.inner[3];

        vec4d(x, y, z, w)
    }
}

impl<'a, T> ops::Neg for &'a Vec4d<T> where T: Copy + ops::Neg<Output=T> {
    type Output = Vec4d<T>;

    fn neg(self) -> Vec4d<T> {
        let x = -self.inner[0];
        let y = -self.inner[1];
        let z = -self.inner[2];
        let w = -self.inner[3];

        vec4d(x, y, z, w)
    }
}

impl<T> ops::Index<usize> for Vec4d<T> where T: Copy {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.inner[index]
    }
}

impl<'a, T> ops::Index<usize> for &'a Vec4d<T> where T: Copy {
    type Output = T;

    fn index(&self, index: usize) -> &T {
        &self.inner[index]
    }
}


#[cfg(test)]
mod tests {
    struct TestCase<T> where T: Copy {
        vec1: super::Vec4d<T>,
        vec2: super::Vec4d<T>,
        vec3: super::Vec4d<T>,
    }

    struct Test<T> where T: Copy {
        inner: Vec<TestCase<T>>
    }

    fn three_dim_integer_add_test_cases() -> Test<isize> {
        Test {
            inner: vec![
                TestCase {
                    vec1: super::vec4d(-56, -74, -83,  91),
                    vec2: super::vec4d(-11, -93,  96,  45),
                    vec3: super::vec4d(-67, -167, 13, 136),
                },
                TestCase {
                    vec1: super::vec4d(34,  70,  17,  28),
                    vec2: super::vec4d(49, -49, -33, -25),
                    vec3: super::vec4d(83,  21, -16,   3),
                },
                TestCase {
                    vec1: super::vec4d(62, -53,   6,  46),
                    vec2: super::vec4d(8,  -19, -29, -85),
                    vec3: super::vec4d(70, -72, -23, -39),
                },
                TestCase {
                    vec1: super::vec4d(-34, -17, -63, -84),
                    vec2: super::vec4d(-32,  41,  -6,  34),
                    vec3: super::vec4d(-66,  24, -69, -50),
                },
            ]
        }
    }

    fn three_dim_integer_sub_test_cases() -> Test<isize> {
        Test {
            inner: vec![
                TestCase {
                    vec1: super::vec4d(-56, -74,  -83, 100),
                    vec2: super::vec4d(-11, -93,   96,  83),
                    vec3: super::vec4d(-45,  19, -179,  17),
                },
                TestCase {
                    vec1: super::vec4d(34,   70,  17,  -33),
                    vec2: super::vec4d(49,  -49, -33,   90),
                    vec3: super::vec4d(-15, 119,  50, -123),
                },
                TestCase {
                    vec1: super::vec4d(62, -53,   6, -32),
                    vec2: super::vec4d(8,  -19, -29, -68),
                    vec3: super::vec4d(54, -34,  35,  36),
                },
                TestCase {
                    vec1: super::vec4d(-34, -17, -63,  46),
                    vec2: super::vec4d(-32,  41,  -6,  86),
                    vec3: super::vec4d(-2,  -58, -57, -40),
                },
            ]
        }
    }

    #[test]
    fn test_addition() {
        let tests = three_dim_integer_add_test_cases();
        for test in tests.inner {
            let res = test.vec1 + test.vec2;

            assert_eq!(res, test.vec3);
        }
    }

    #[test]
    fn test_subtraction() {
        let tests = three_dim_integer_sub_test_cases();
        for test in tests.inner {
            let res = test.vec1 - test.vec2;
            
            assert_eq!(res, test.vec3);
        }
    }
}

pub type Vec4df = Vec4d<f32>;
pub type Vec4dd = Vec4d<f64>;
