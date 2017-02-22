use shader::texture::TextureMap;
use nalgebra::{Vector3, Point3};
use num_traits::Float;
use alga::general::Real;


///
/// Factory function for creating a new `MonochromeShader`.
///
pub fn shader<N: Float + Real>(color: Vector3<N>) -> MonochromeShader<N> {
    MonochromeShader::new(color)
}


pub struct MonochromeShader<N> where N: Float + Real {
    color: Vector3<N>,
}

impl<N> MonochromeShader<N> where N: Float + Real {
    fn new(color: Vector3<N>) -> MonochromeShader<N> {
        MonochromeShader {
            color: color,
        }
    }
}

type Args<N> = (Point3<N>,);

impl<N> TextureMap<N, Args<N>> for MonochromeShader<N> where N: Float + Real {
    /// 
    /// Compute the monochrome shading of a triangle primitive.
    ///
    #[allow(unused_variables)]
    fn apply(&self, args: Args<N>) -> Vector3<N> {
        self.color
    }
}

fn_impl!(MonochromeShader<N>, N, Args<N>);


#[cfg(test)]
mod tests {
    use nalgebra::{Vector3, Point3};
    use num_traits::Float;
    use alga::general::Real;


    #[test]
    fn test_monochrome_shader_should_return_same_color() {
        let color = Vector3::new(0.4, 0.3, 0.3);
        let shader = super::shader(color);
        let bary   = Point3::new(0.2, 0.5, 0.3);

        let given = shader(bary);
        let expected = color;

        assert_relative_eq!(given, expected);
    }
}
