use std::ops;
use std::f32;
use std::f64;


pub fn vec2d<T>(x: T, y: T) -> Vec2d<T> {
    Vec2d::new(x, y)
}

pub fn vec3d<T>(x: T, y: T, z: T) -> Vec3d<T> {
    Vec3d::new(x, y, z)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
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

impl<T> Vec2d<T> where T: Copy + ops::Add<T, Output=T> + ops::Mul<T, Output=T> {
    pub fn norm(&self) -> T {
        (self.x * self.x) + (self.y * self.y)
    }

    pub fn dot_product(&self, other: Vec2d<T>) -> T {
        (self.x * other.x) + (self.y * other.y)
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


#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vec3d<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> Vec3d<T> {
    #[inline]
    pub fn new(x: T, y: T, z: T) -> Vec3d<T> {
        Vec3d {
            x: x,
            y: y,
            z: z
        }
    }
}

impl<T> Vec3d<T> where T: Copy + ops::Add<T, Output=T> + ops::Mul<T, Output=T> {
    pub fn norm(&self) -> T {
        (self.x * self.x) + (self.y * self.y) + (self.z * self.z)
    }

    pub fn dot_product(&self, other: Vec3d<T>) -> T {
        (self.x * other.x) + (self.y * other.y) + (self.z * other.z)
    }
} 

impl Vec3d<f32> {
    pub fn length(&self) -> f32 {
        f32::sqrt(self.norm())
    }
}

impl Vec3d<f64> {
    pub fn length(&self) -> f64 {
        f64::sqrt(self.norm())
    }
}

impl<T> ops::Add<Vec3d<T>> for Vec3d<T> where T: ops::Add<T, Output=T> {
    type Output = Vec3d<T>;

    fn add(self, other: Vec3d<T>) -> Vec3d<T> {
        let x: T = self.x + other.x;
        let y: T = self.y + other.y;
        let z: T = self.z + other.z;

        vec3d(x, y, z)
    }
}

impl<'a, T> ops::Add<Vec3d<T>> for &'a Vec3d<T> where T: Copy + ops::Add<T, Output=T> {
    type Output = Vec3d<T>;

    fn add(self, other: Vec3d<T>) -> Vec3d<T> {
        let x: T = self.x + other.x;
        let y: T = self.y + other.y;
        let z: T = self.z + other.z;

        vec3d(x, y, z)
    }
}

impl<'a, T> ops::Add<&'a Vec3d<T>> for Vec3d<T> where T: Copy + ops::Add<T, Output=T> {
    type Output = Vec3d<T>;

    fn add(self, other: &Vec3d<T>) -> Vec3d<T> {
        let x: T = self.x + other.x;
        let y: T = self.y + other.y;
        let z: T = self.z + other.z;

        vec3d(x, y, z)
    }
}

impl<'a, 'b, T> ops::Add<&'a Vec3d<T>> for &'b Vec3d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    type Output = Vec3d<T>;

    fn add(self, other: &'a Vec3d<T>) -> Vec3d<T> {
        let x = self.x + other.x;
        let y = self.y + other.y;
        let z = self.z + other.z;

        vec3d(x, y, z)
    }
}

impl<T> ops::Sub for Vec3d<T> where T: ops::Sub<T, Output=T> {
    type Output = Vec3d<T>;

    fn sub(self: Vec3d<T>, other: Vec3d<T>) -> Vec3d<T> {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;

        vec3d(x, y, z)
    }
}

impl<'a, T> ops::Sub<Vec3d<T>> for &'a Vec3d<T> where T: Copy + ops::Sub<T, Output=T> {
    type Output = Vec3d<T>;

    fn sub(self, other: Vec3d<T>) -> Vec3d<T> {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;

        vec3d(x, y, z)
    }
}

impl<'a, 'b, T> ops::Sub<&'a Vec3d<T>> for &'b Vec3d<T> where T: Copy + ops::Sub<T, Output=T> {
    type Output = Vec3d<T>;

    fn sub(self, other: &'a Vec3d<T>) -> Vec3d<T> {
        let x = self.x - other.x;
        let y = self.y - other.y;
        let z = self.z - other.z;

        vec3d(x, y, z)
    }
}

impl<T> ops::Mul<T> for Vec3d<T> where T: Copy + ops::Mul<T, Output=T> {
    type Output = Vec3d<T>;

    fn mul(self, other: T) -> Vec3d<T> {
        let x = self.x * other;
        let y = self.y * other;
        let z = self.z * other;

        vec3d(x, y, z)
    }
}

impl<'a, T> ops::Mul<T> for &'a Vec3d<T> where T: Copy + ops::Mul<T, Output=T> {
    type Output = Vec3d<T>;

    fn mul(self, other: T) -> Vec3d<T> {
        let x = self.x * other;
        let y = self.y * other;
        let z = self.z * other;

        vec3d(x, y, z)
    }
}

impl<T> ops::Div<T> for Vec3d<T> where T: Copy + ops::Div<T, Output=T> {
    type Output = Vec3d<T>;

    fn div(self, other: T) -> Vec3d<T> {
        let x = self.x / other;
        let y = self.y / other;
        let z = self.z / other;

        vec3d(x, y, z)
    }
}

impl<'a, T> ops::Div<T> for &'a Vec3d<T> where T: Copy + ops::Div<T, Output=T> {
    type Output = Vec3d<T>;

    fn div(self, other: T) -> Vec3d<T> {
        let x = self.x / other;
        let y = self.y / other;
        let z = self.z / other;

        vec3d(x, y, z)
    }
}

impl<T> ops::AddAssign<Vec3d<T>> for Vec3d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    fn add_assign(&mut self, other: Vec3d<T>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl<'a, T> ops::AddAssign<Vec3d<T>> for &'a mut Vec3d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    fn add_assign(&mut self, other: Vec3d<T>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl<'a, 'b, T> ops::AddAssign<&'a Vec3d<T>> for &'b mut Vec3d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    fn add_assign(&mut self, other: &'a Vec3d<T>) {
        self.x = self.x + other.x;
        self.y = self.y + other.y;
        self.z = self.z + other.z;
    }
}

impl<T> ops::SubAssign<Vec3d<T>> for Vec3d<T> where T: Copy + ops::Sub<T, Output=T> {
    fn sub_assign(&mut self, other: Vec3d<T>) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
    }
}

impl<'a, T> ops::SubAssign<Vec3d<T>> for &'a mut Vec3d<T> where T: Copy + ops::Sub<T, Output=T> {
    fn sub_assign(&mut self, other: Vec3d<T>) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
    }
}

impl<'a, 'b, T> ops::SubAssign<&'a Vec3d<T>> for &'b mut Vec3d<T> where T: Copy + ops::Sub<T, Output=T> {
    fn sub_assign(&mut self, other: &'a Vec3d<T>) {
        self.x = self.x - other.x;
        self.y = self.y - other.y;
        self.z = self.z - other.z;
    }
}

impl<T> ops::MulAssign<T> for Vec3d<T> where T: Copy + ops::Mul<T, Output=T> {
    fn mul_assign(&mut self, other: T) {
        self.x = self.x * other;
        self.y = self.y * other;
        self.z = self.z * other;
    }
}

impl<'a, T> ops::MulAssign<T> for &'a mut Vec3d<T> where T: Copy + ops::Mul<T, Output=T> {
    fn mul_assign(&mut self, other: T) {
        self.x = self.x * other;
        self.y = self.y * other;
        self.z = self.z * other;
    }
}

impl<T> ops::DivAssign<T> for Vec3d<T> where T: Copy + ops::Div<T, Output=T> {
    fn div_assign(&mut self, other: T) {
        self.x = self.x / other;
        self.y = self.y / other;
        self.z = self.z / other;
    }
}

impl<'a, T> ops::DivAssign<T> for &'a mut Vec3d<T> where T: Copy + ops::Div<T, Output=T> {
    fn div_assign(&mut self, other: T) {
        self.x = self.x / other;
        self.y = self.y / other;
        self.z = self.z / other;
    }
}
