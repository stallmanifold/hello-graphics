use nalgebra::{Vector3, Point3};
use alga::general::Real;
use num_traits::Float;


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
