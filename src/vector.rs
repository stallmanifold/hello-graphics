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

    fn add(self, rhs: Vec2d<T>) -> Vec2d<T> {
        let x: T = self.x + rhs.x;
        let y: T = self.y + rhs.y;

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

impl<T> ops::Mul<T> for Vec2d<T> where T: Copy + ops::Mul<T, Output=T> {
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

impl<T> ops::AddAssign<Vec2d<T>> for Vec2d<T> where T: Copy + ops::Add<T, Output=T> {
    fn add_assign(&mut self, rhs: Vec2d<T>) {
        self.x = self.x + rhs.x;
        self.y = self.y + rhs.y;
    }
}

impl<T> ops::SubAssign<Vec2d<T>> for Vec2d<T> where T: Copy + ops::Sub<T, Output=T> {
    fn sub_assign(&mut self, rhs: Vec2d<T>) {
        self.x = self.x - rhs.x;
        self.y = self.y - rhs.y;
    }
}

impl<T> ops::MulAssign<T> for Vec2d<T> where T: Copy + ops::Mul<T, Output=T> {
    fn mul_assign(&mut self, rhs: T) {
        self.x = self.x * rhs;
        self.y = self.y * rhs;
    }
}

impl<T> ops::DivAssign<T> for Vec2d<T> where T: Copy + ops::Div<T, Output=T> {
    fn div_assign(&mut self, rhs: T) {
        self.x = self.x / rhs;
        self.y = self.y / rhs;
    }
}
