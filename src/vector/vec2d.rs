use std::ops;
use std::f32;
use std::f64;


pub fn vec2d<T: Copy>(x: T, y: T) -> Vec2d<T> {
    Vec2d::new(x, y)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vec2d<T> where T: Copy {
    inner: [T; 2]
}


impl<T> Vec2d<T> where T: Copy {
    #[inline]
    pub fn new(x: T, y: T) -> Vec2d<T> {
        Vec2d {
            inner: [x,y]
        }
    }
}

impl<T> Vec2d<T> where T: Copy + ops::Add<T, Output=T> + ops::Mul<T, Output=T> {
    pub fn norm(&self) -> T {
        (self.inner[0] * self.inner[0]) + (self.inner[1] * self.inner[1])
    }

    pub fn dot_product(&self, other: Vec2d<T>) -> T {
        (self.inner[0] * other.inner[0]) + (self.inner[1] * other.inner[1])
    }
}

impl Vec2d<f32> {
    pub fn length(&self) -> f32 {
        f32::sqrt(self.norm())
    }
}

impl Vec2d<f64> {
    pub fn length(&self) -> f64 {
        f64::sqrt(self.norm())
    }
}

impl<T> ops::Add<Vec2d<T>> for Vec2d<T> where T: Copy + ops::Add<T, Output=T> {
    type Output = Vec2d<T>;

    fn add(self, other: Vec2d<T>) -> Vec2d<T> {
        let x: T = self.inner[0] + other.inner[0];
        let y: T = self.inner[1] + other.inner[1];

        vec2d(x, y)
    }
}

impl<'a, T> ops::Add<Vec2d<T>> for &'a Vec2d<T> where T: Copy + ops::Add<T, Output=T> {
    type Output = Vec2d<T>;

    fn add(self, other: Vec2d<T>) -> Vec2d<T> {
        let x: T = self.inner[0] + other.inner[0];
        let y: T = self.inner[1] + other.inner[1];

        vec2d(x, y)
    }
}

impl<'a, 'b, T> ops::Add<&'a Vec2d<T>> for &'b Vec2d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    type Output = Vec2d<T>;

    fn add(self, other: &'a Vec2d<T>) -> Vec2d<T> {
        let x = self.inner[0] + other.inner[0];
        let y = self.inner[1] + other.inner[1];

        vec2d(x, y)
    }
}

impl<T> ops::Sub for Vec2d<T> where T: Copy + ops::Sub<T, Output=T> {
    type Output = Vec2d<T>;

    fn sub(self: Vec2d<T>, other: Vec2d<T>) -> Vec2d<T> {
        let x = self.inner[0] - other.inner[0];
        let y = self.inner[1] - other.inner[1];

        vec2d(x, y)
    }
}

impl<'a, T> ops::Sub<Vec2d<T>> for &'a Vec2d<T> where T: Copy + ops::Sub<T, Output=T> {
    type Output = Vec2d<T>;

    fn sub(self, other: Vec2d<T>) -> Vec2d<T> {
        let x = self.inner[0] - other.inner[0];
        let y = self.inner[1] - other.inner[1];

        vec2d(x, y)
    }
}

impl<'a, 'b, T> ops::Sub<&'a Vec2d<T>> for &'b Vec2d<T> where T: Copy + ops::Sub<T, Output=T> {
    type Output = Vec2d<T>;

    fn sub(self, other: &'a Vec2d<T>) -> Vec2d<T> {
        let x = self.inner[0] - other.inner[0];
        let y = self.inner[1] - other.inner[1];

        vec2d(x, y)
    }
}

impl<T> ops::Mul<T> for Vec2d<T> where T: Copy + ops::Mul<T, Output=T> {
    type Output = Vec2d<T>;

    fn mul(self, scalar: T) -> Vec2d<T> {
        let x = self.inner[0] * scalar;
        let y = self.inner[1] * scalar;

        vec2d(x, y)
    }
}

impl<'a, T> ops::Mul<T> for &'a Vec2d<T> where T: Copy + ops::Mul<T, Output=T> {
    type Output = Vec2d<T>;

    fn mul(self, scalar: T) -> Vec2d<T> {
        let x = self.inner[0] * scalar;
        let y = self.inner[1] * scalar;

        vec2d(x, y)
    }
}

impl<T> ops::Div<T> for Vec2d<T> where T: Copy + ops::Div<T, Output=T> {
    type Output = Vec2d<T>;

    fn div(self, scalar: T) -> Vec2d<T> {
        let x = self.inner[0] / scalar;
        let y = self.inner[1] / scalar;

        vec2d(x, y)
    }
}

impl<'a, T> ops::Div<T> for &'a Vec2d<T> where T: Copy + ops::Div<T, Output=T> {
    type Output = Vec2d<T>;

    fn div(self, other: T) -> Vec2d<T> {
        let x = self.inner[0] / other;
        let y = self.inner[1] / other;

        vec2d(x, y)
    }
}

impl<T> ops::AddAssign<Vec2d<T>> for Vec2d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    fn add_assign(&mut self, other: Vec2d<T>) {
        self.inner[0] = self.inner[0] + other.inner[0];
        self.inner[1] = self.inner[1] + other.inner[1];
    }
}

impl<'a, T> ops::AddAssign<Vec2d<T>> for &'a mut Vec2d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    fn add_assign(&mut self, other: Vec2d<T>) {
        self.inner[0] = self.inner[0] + other.inner[0];
        self.inner[1] = self.inner[1] + other.inner[1];
    }
}

impl<'a, 'b, T> ops::AddAssign<&'a Vec2d<T>> for &'b mut Vec2d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    fn add_assign(&mut self, other: &'a Vec2d<T>) {
        self.inner[0] = self.inner[0] + other.inner[0];
        self.inner[1] = self.inner[1] + other.inner[1];
    }
}

impl<T> ops::SubAssign<Vec2d<T>> for Vec2d<T> where T: Copy + ops::Sub<T, Output=T> {
    fn sub_assign(&mut self, other: Vec2d<T>) {
        self.inner[0] = self.inner[0] - other.inner[0];
        self.inner[1] = self.inner[1] - other.inner[1];
    }
}

impl<'a, T> ops::SubAssign<Vec2d<T>> for &'a mut Vec2d<T> where T: Copy + ops::Sub<T, Output=T> {
    fn sub_assign(&mut self, other: Vec2d<T>) {
        self.inner[0] = self.inner[0] - other.inner[0];
        self.inner[1] = self.inner[1] - other.inner[1];
    }
}

impl<'a, 'b, T> ops::SubAssign<&'a Vec2d<T>> for &'b mut Vec2d<T> where T: Copy + ops::Sub<T, Output=T> {
    fn sub_assign(&mut self, other: &'a Vec2d<T>) {
        self.inner[0] = self.inner[0] - other.inner[0];
        self.inner[1] = self.inner[1] - other.inner[1];
    }
}

impl<T> ops::MulAssign<T> for Vec2d<T> where T: Copy + ops::Mul<T, Output=T> {
    fn mul_assign(&mut self, other: T) {
        self.inner[0] = self.inner[0] * other;
        self.inner[1] = self.inner[1] * other;
    }
}

impl<'a, T> ops::MulAssign<T> for &'a mut Vec2d<T> where T: Copy + ops::Mul<T, Output=T> {
    fn mul_assign(&mut self, other: T) {
        self.inner[0] = self.inner[0] * other;
        self.inner[1] = self.inner[1] * other;
    }
}

impl<T> ops::DivAssign<T> for Vec2d<T> where T: Copy + ops::Div<T, Output=T> {
    fn div_assign(&mut self, other: T) {
        self.inner[0] = self.inner[0] / other;
        self.inner[1] = self.inner[1] / other;
    }
}

impl<'a, T> ops::DivAssign<T> for &'a mut Vec2d<T> where T: Copy + ops::Div<T, Output=T> {
    fn div_assign(&mut self, other: T) {
        self.inner[0] = self.inner[0] / other;
        self.inner[1] = self.inner[1] / other;
    }
}


#[cfg(test)]
mod tests {
    struct TestCase<T> where T: Copy {
        vec1: super::Vec2d<T>,
        vec2: super::Vec2d<T>,
        vec3: super::Vec2d<T>,
    }

    struct Test<T> where T: Copy {
        inner: Vec<TestCase<T>>
    }

    fn two_dim_integer_add_test_cases() -> Test<isize> {
        Test {
            inner: vec![
                TestCase {
                    vec1: super::vec2d(93, 41),
                    vec2: super::vec2d(22, 2),
                    vec3: super::vec2d(115, 43),
                },
                TestCase {
                    vec1: super::vec2d(14, 16),
                    vec2: super::vec2d(11, 40),
                    vec3: super::vec2d(25, 56),
                },
                TestCase {
                    vec1: super::vec2d(100, 76),
                    vec2: super::vec2d(-63, 49),
                    vec3: super::vec2d(37, 125),
                },
                TestCase {
                    vec1: super::vec2d(63, 49),
                    vec2: super::vec2d(-74, -62),
                    vec3: super::vec2d(-11, -13),
                },
            ]
        }
    }

    fn two_dim_integer_sub_test_cases() -> Test<isize> {
        Test {
            inner: vec![
                TestCase {
                    vec1: super::vec2d(93, 41),
                    vec2: super::vec2d(22, 2),
                    vec3: super::vec2d(71, 39),
                },
                TestCase {
                    vec1: super::vec2d(14, 16),
                    vec2: super::vec2d(11, 40),
                    vec3: super::vec2d(3, -24),
                },
                TestCase {
                    vec1: super::vec2d(100, 76),
                    vec2: super::vec2d(-63, 49),
                    vec3: super::vec2d(163, 27),
                },
                TestCase {
                    vec1: super::vec2d(63, 49),
                    vec2: super::vec2d(-74, -62),
                    vec3: super::vec2d(137, 111),
                },
            ]
        }
    }


    #[test]
    fn test_addition() {
        let tests = two_dim_integer_add_test_cases();
        for test in tests.inner {
            let res = test.vec1 + test.vec2;
            assert_eq!(res, test.vec3);
        }
    }

    #[test]
    fn test_subtraction() {
        let tests = two_dim_integer_sub_test_cases();
        for test in tests.inner {
            let res = test.vec1 - test.vec2;
            assert_eq!(res, test.vec3);
        }
    }
}