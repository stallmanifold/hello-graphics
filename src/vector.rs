use std::ops;


pub fn vec2d<T: Sized>(x: T, y: T) -> Vec2d<T> {
    Vec2d::new(x, y)
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Vec2d<T> {
    pub x: T,
    pub y: T
}

impl<T> Vec2d<T> {
    #[inline]
    pub fn new(x: T, y: T) -> Vec2d<T> {
        Vec2d {
            x: x,
            y: y
        }
    }
}

impl<T> ops::Add<Vec2d<T>> for Vec2d<T> where T: ops::Add<T, Output=T> {
    type Output = Vec2d<T>;

    fn add(self, other: Vec2d<T>) -> Vec2d<T> {
        let x: T = self.x + other.x;
        let y: T = self.y + other.y;

        vec2d(x, y)
    }
}

impl<'a, T> ops::Add<Vec2d<T>> for &'a Vec2d<T> where T: Copy + ops::Add<T, Output=T> {
    type Output = Vec2d<T>;

    fn add(self, other: Vec2d<T>) -> Vec2d<T> {
        let x: T = self.x + other.x;
        let y: T = self.y + other.y;

        vec2d(x, y)
    }
}

impl<'a, 'b, T> ops::Add<&'a Vec2d<T>> for &'b Vec2d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    type Output = Vec2d<T>;

    fn add(self, other: &'a Vec2d<T>) -> Vec2d<T> {
        let x = self.x + other.x;
        let y = self.y + other.y;

        vec2d(x, y)
    }
}

impl<T> ops::Sub for Vec2d<T> where T: ops::Sub<T, Output=T> {
    type Output = Vec2d<T>;

    fn sub(self: Vec2d<T>, other: Vec2d<T>) -> Vec2d<T> {
        let x: T = self.x - other.x;
        let y: T = self.y - other.y;

        vec2d(x, y)
    }
}

impl<'a, T> ops::Sub<Vec2d<T>> for &'a Vec2d<T> where T: Copy + ops::Sub<T, Output=T> {
    type Output = Vec2d<T>;

    fn sub(self, other: Vec2d<T>) -> Vec2d<T> {
        let x: T = self.x - other.x;
        let y: T = self.y - other.y;

        vec2d(x, y)
    }
}

impl<'a, 'b, T> ops::Sub<&'a Vec2d<T>> for &'b Vec2d<T> where T: Copy + ops::Sub<T, Output=T> {
    type Output = Vec2d<T>;

    fn sub(self, other: &'a Vec2d<T>) -> Vec2d<T> {
        let x: T = self.x - other.x;
        let y: T = self.y - other.y;

        vec2d(x, y)
    }
}

impl<T> ops::Mul<T> for Vec2d<T> where T: Copy + ops::Mul<T, Output=T> {
    type Output = Vec2d<T>;

    fn mul(self, other: T) -> Vec2d<T> {
        let x = self.x * other;
        let y = self.y * other;

        vec2d(x, y)
    }
}

impl<'a, T> ops::Mul<T> for &'a Vec2d<T> where T: Copy + ops::Mul<T, Output=T> {
    type Output = Vec2d<T>;

    fn mul(self, other: T) -> Vec2d<T> {
        let x = self.x * other;
        let y = self.y * other;

        vec2d(x, y)
    }
}

impl<T> ops::Div<T> for Vec2d<T> where T: Copy + ops::Div<T, Output=T> {
    type Output = Vec2d<T>;

    fn div(self, other: T) -> Vec2d<T> {
        let x = self.x / other;
        let y = self.y / other;

        vec2d(x, y)
    }
}

impl<'a, T> ops::Div<T> for &'a Vec2d<T> where T: Copy + ops::Div<T, Output=T> {
    type Output = Vec2d<T>;

    fn div(self, other: T) -> Vec2d<T> {
        let x = self.x / other;
        let y = self.y / other;

        vec2d(x, y)
    }
}

impl<T> ops::AddAssign<Vec2d<T>> for Vec2d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    fn add_assign(&mut self, other: Vec2d<T>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl<'a, T> ops::AddAssign<Vec2d<T>> for &'a mut Vec2d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    fn add_assign(&mut self, other: Vec2d<T>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl<'a, 'b, T> ops::AddAssign<&'a Vec2d<T>> for &'b mut Vec2d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    fn add_assign(&mut self, other: &'a Vec2d<T>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
    }
}

impl<T> ops::SubAssign<Vec2d<T>> for Vec2d<T> where T: Copy + ops::Sub<T, Output=T> {
    fn sub_assign(&mut self, other: Vec2d<T>) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }
}

impl<'a, T> ops::SubAssign<Vec2d<T>> for &'a mut Vec2d<T> where T: Copy + ops::Sub<T, Output=T> {
    fn sub_assign(&mut self, other: Vec2d<T>) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }
}

impl<'a, 'b, T> ops::SubAssign<&'a Vec2d<T>> for &'b mut Vec2d<T> where T: Copy + ops::Sub<T, Output=T> {
    fn sub_assign(&mut self, other: &'a Vec2d<T>) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
    }
}

impl<T> ops::MulAssign<T> for Vec2d<T> where T: Copy + ops::Mul<T, Output=T> {
    fn mul_assign(&mut self, other: T) {
        self.x = self.x * other;
        self.y = self.y * other;
    }
}

impl<'a, T> ops::MulAssign<T> for &'a mut Vec2d<T> where T: Copy + ops::Mul<T, Output=T> {
    fn mul_assign(&mut self, other: T) {
        self.x = self.x * other;
        self.y = self.y * other;
    }
}

impl<T> ops::DivAssign<T> for Vec2d<T> where T: Copy + ops::Div<T, Output=T> {
    fn div_assign(&mut self, other: T) {
        self.x = self.x / other;
        self.y = self.y / other;
    }
}

impl<'a, T> ops::DivAssign<T> for &'a mut Vec2d<T> where T: Copy + ops::Div<T, Output=T> {
    fn div_assign(&mut self, other: T) {
        self.x = self.x / other;
        self.y = self.y / other;
    }
}