use nalgebra::{Vector2, Vector3, Point3};
use alga::general::Real;
use num_traits::Float;
use color::Rgb;


/// 
/// Compute the gouraud shading of a triangle primitive.
///
pub fn gouraud<N>(color0: Vector3<N>, 
                  color1: Vector3<N>, 
                  color2: Vector3<N>, 
                  bary:   Point3<N>) -> Vector3<N>
    where N: Float + Real
{
    color0 * bary[0] + color1 * bary[1] + color2 * bary[2]
}

///
/// Peform a checkerboard shading at a point on a triangle primitive.
/// This computation performs perspective correction.
///
pub fn checkerboard(st0: Vector2<f32>,
                    st1: Vector2<f32>,
                    st2: Vector2<f32>,
                    v0:  Point3<f32>,
                    v1:  Point3<f32>,
                    v2:  Point3<f32>,
                    w:   Point3<f32>) -> Vector3<f32>
{
    let mut s = w[0]*st0[0] + w[1]*st1[0] + w[2]*st2[0];
    let mut t = w[0]*st0[1] + w[1]*st1[1] + w[2]*st2[1];
    let z = 1.0 / (w[0]*v0[2] + w[1]*v1[2] + w[2]*v2[2]);
    let m = 10.0;

    s *= z;
    t *= z;

    let p = ((((s*m % 1.0 > 0.5) as usize) ^ (t*m % 1.0 < 0.5) as usize)) as f32;

    Vector3::new(p,p,p)
}

///
/// Calculate the RGB color of a color vector.
///
#[inline(always)]
pub fn color_rgb(color: Vector3<f32>) -> Rgb {
    let r = (255.0 * color.x).trunc() as u8;
    let g = (255.0 * color.y).trunc() as u8;
    let b = (255.0 * color.z).trunc() as u8;

    Rgb::from_channels(r, g, b)
}

///
/// Perspective correct vertex attributes.
///
#[inline(always)]
pub fn perspective_correct<N>(position: Point3<N>, 
                              vertex: Vector3<N>) -> Vector3<N> 
    where N: Float + Real
{
    Vector3::new(vertex.x / position.z, vertex.y / position.z, vertex.z)
}

///
/// Perspective correct vertex attributes in place.
///
#[inline(always)]
pub fn perspective_correct_inplace<N>(position: Point3<N>, 
                                      vertex: &mut Vector3<N>) 
    where N: Float + Real
{
    vertex.x /= position.z;
    vertex.y /= position.z;
}


#[cfg(test)]
mod tests {
    
}
