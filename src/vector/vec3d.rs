use std::ops;


pub fn vec3d<T: Copy>(x: T, y: T, z: T) -> Vec3d<T> {
    Vec3d::new(x, y, z)
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Vec3d<T> where T: Copy {
    inner: [T; 3]
}

impl<T> Vec3d<T> where T: Copy {
    #[inline(always)]
    pub fn new(x: T, y: T, z: T) -> Vec3d<T> {
        Vec3d {
            inner: [x,y,z]
        }
    }
}

impl<T> Vec3d<T> where T: Copy + ops::Add<T, Output=T> + ops::Mul<T, Output=T> {
    pub fn norm(&self) -> T {
        (self.inner[0] * self.inner[0]) +
        (self.inner[1] * self.inner[1]) +
        (self.inner[2] * self.inner[2])
    }

    pub fn dot_product(&self, other: Vec3d<T>) -> T {
        (self.inner[0] * other.inner[0]) + 
        (self.inner[1] * other.inner[1]) + 
        (self.inner[2] * other.inner[2])
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

impl<T> ops::Add<Vec3d<T>> for Vec3d<T> where T: Copy + ops::Add<T, Output=T> {
    type Output = Vec3d<T>;

    fn add(self, other: Vec3d<T>) -> Vec3d<T> {
        let x = self.inner[0] + other.inner[0];
        let y = self.inner[1] + other.inner[1];
        let z = self.inner[2] + other.inner[2];

        vec3d(x, y, z)
    }
}

impl<'a, T> ops::Add<Vec3d<T>> for &'a Vec3d<T> where T: Copy + ops::Add<T, Output=T> {
    type Output = Vec3d<T>;

    fn add(self, other: Vec3d<T>) -> Vec3d<T> {
        let x = self.inner[0] + other.inner[0];
        let y = self.inner[1] + other.inner[1];
        let z = self.inner[2] + other.inner[2];

        vec3d(x, y, z)
    }
}

impl<'a, T> ops::Add<&'a Vec3d<T>> for Vec3d<T> where T: Copy + ops::Add<T, Output=T> {
    type Output = Vec3d<T>;

    fn add(self, other: &Vec3d<T>) -> Vec3d<T> {
        let x = self.inner[0] + other.inner[0];
        let y = self.inner[1] + other.inner[1];
        let z = self.inner[2] + other.inner[2];

        vec3d(x, y, z)
    }
}

impl<'a, 'b, T> ops::Add<&'a Vec3d<T>> for &'b Vec3d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    type Output = Vec3d<T>;

    fn add(self, other: &'a Vec3d<T>) -> Vec3d<T> {
        let x = self.inner[0] + other.inner[0];
        let y = self.inner[1] + other.inner[1];
        let z = self.inner[2] + other.inner[2];

        vec3d(x, y, z)
    }
}

impl<T> ops::Sub for Vec3d<T> where T: Copy + ops::Sub<T, Output=T> {
    type Output = Vec3d<T>;

    fn sub(self: Vec3d<T>, other: Vec3d<T>) -> Vec3d<T> {
        let x = self.inner[0] - other.inner[0];
        let y = self.inner[1] - other.inner[1];
        let z = self.inner[2] - other.inner[2];

        vec3d(x, y, z)
    }
}

impl<'a, T> ops::Sub<Vec3d<T>> for &'a Vec3d<T> where T: Copy + ops::Sub<T, Output=T> {
    type Output = Vec3d<T>;

    fn sub(self, other: Vec3d<T>) -> Vec3d<T> {
        let x = self.inner[0] - other.inner[0];
        let y = self.inner[1] - other.inner[1];
        let z = self.inner[2] - other.inner[2];

        vec3d(x, y, z)
    }
}

impl<'a, 'b, T> ops::Sub<&'a Vec3d<T>> for &'b Vec3d<T> where T: Copy + ops::Sub<T, Output=T> {
    type Output = Vec3d<T>;

    fn sub(self, other: &'a Vec3d<T>) -> Vec3d<T> {
        let x = self.inner[0] - other.inner[0];
        let y = self.inner[1] - other.inner[1];
        let z = self.inner[2] - other.inner[2];

        vec3d(x, y, z)
    }
}

impl<T> ops::Mul<T> for Vec3d<T> where T: Copy + ops::Mul<T, Output=T> {
    type Output = Vec3d<T>;

    fn mul(self, scalar: T) -> Vec3d<T> {
        let x = self.inner[0] * scalar;
        let y = self.inner[1] * scalar;
        let z = self.inner[2] * scalar;

        vec3d(x, y, z)
    }
}

impl<'a, T> ops::Mul<T> for &'a Vec3d<T> where T: Copy + ops::Mul<T, Output=T> {
    type Output = Vec3d<T>;

    fn mul(self, scalar: T) -> Vec3d<T> {
        let x = self.inner[0] * scalar;
        let y = self.inner[1] * scalar;
        let z = self.inner[2] * scalar;

        vec3d(x, y, z)
    }
}

impl<T> ops::Div<T> for Vec3d<T> where T: Copy + ops::Div<T, Output=T> {
    type Output = Vec3d<T>;

    fn div(self, scalar: T) -> Vec3d<T> {
        let x = self.inner[0] / scalar;
        let y = self.inner[1] / scalar;
        let z = self.inner[2] / scalar;

        vec3d(x, y, z)
    }
}

impl<'a, T> ops::Div<T> for &'a Vec3d<T> where T: Copy + ops::Div<T, Output=T> {
    type Output = Vec3d<T>;

    fn div(self, scalar: T) -> Vec3d<T> {
        let x = self.inner[0] / scalar;
        let y = self.inner[1] / scalar;
        let z = self.inner[2] / scalar;

        vec3d(x, y, z)
    }
}

impl<T> ops::AddAssign<Vec3d<T>> for Vec3d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    fn add_assign(&mut self, other: Vec3d<T>) {
        self.inner[0] = self.inner[0] + other.inner[0];
        self.inner[1] = self.inner[1] + other.inner[1];
        self.inner[2] = self.inner[2] + other.inner[2];
    }
}

impl<'a, T> ops::AddAssign<Vec3d<T>> for &'a mut Vec3d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    fn add_assign(&mut self, other: Vec3d<T>) {
        self.inner[0] = self.inner[0] + other.inner[0];
        self.inner[1] = self.inner[1] + other.inner[1];
        self.inner[2] = self.inner[2] + other.inner[2];
    }
}

impl<'a, 'b, T> ops::AddAssign<&'a Vec3d<T>> for &'b mut Vec3d<T> 
    where T: Copy + ops::Add<T, Output=T> 
{
    fn add_assign(&mut self, other: &'a Vec3d<T>) {
        self.inner[0] = self.inner[0] + other.inner[0];
        self.inner[1] = self.inner[1] + other.inner[1];
        self.inner[2] = self.inner[2] + other.inner[2];
    }
}

impl<T> ops::SubAssign<Vec3d<T>> for Vec3d<T> where T: Copy + ops::Sub<T, Output=T> {
    fn sub_assign(&mut self, other: Vec3d<T>) {
        self.inner[0] = self.inner[0] - other.inner[0];
        self.inner[1] = self.inner[1] - other.inner[1];
        self.inner[2] = self.inner[2] - other.inner[2];
    }
}

impl<'a, T> ops::SubAssign<Vec3d<T>> for &'a mut Vec3d<T> where T: Copy + ops::Sub<T, Output=T> {
    fn sub_assign(&mut self, other: Vec3d<T>) {
        self.inner[0] = self.inner[0] - other.inner[0];
        self.inner[1] = self.inner[1] - other.inner[1];
        self.inner[2] = self.inner[2] - other.inner[2];
    }
}

impl<'a, 'b, T> ops::SubAssign<&'a Vec3d<T>> for &'b mut Vec3d<T> where T: Copy + ops::Sub<T, Output=T> {
    fn sub_assign(&mut self, other: &'a Vec3d<T>) {
        self.inner[0] = self.inner[0] - other.inner[0];
        self.inner[1] = self.inner[1] - other.inner[1];
        self.inner[2] = self.inner[2] - other.inner[2];
    }
}

impl<T> ops::MulAssign<T> for Vec3d<T> where T: Copy + ops::Mul<T, Output=T> {
    fn mul_assign(&mut self, scalar: T) {
        self.inner[0] = self.inner[0] * scalar;
        self.inner[1] = self.inner[1] * scalar;
        self.inner[2] = self.inner[2] * scalar;
    }
}

impl<'a, T> ops::MulAssign<T> for &'a mut Vec3d<T> where T: Copy + ops::Mul<T, Output=T> {
    fn mul_assign(&mut self, scalar: T) {
        self.inner[0] = self.inner[0] * scalar;
        self.inner[1] = self.inner[1] * scalar;
        self.inner[2] = self.inner[2] * scalar;
    }
}

impl<T> ops::DivAssign<T> for Vec3d<T> where T: Copy + ops::Div<T, Output=T> {
    fn div_assign(&mut self, scalar: T) {
        self.inner[0] = self.inner[0] / scalar;
        self.inner[1] = self.inner[1] / scalar;
        self.inner[2] = self.inner[2] / scalar;
    }
}

impl<'a, T> ops::DivAssign<T> for &'a mut Vec3d<T> where T: Copy + ops::Div<T, Output=T> {
    fn div_assign(&mut self, scalar: T) {
        self.inner[0] = self.inner[0] / scalar;
        self.inner[1] = self.inner[1] / scalar;
        self.inner[2] = self.inner[2] / scalar;
    }
}


#[cfg(test)]
mod tests {
    
}