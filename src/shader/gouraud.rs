use shader::texture::TextureMap;
use nalgebra::{Vector3, Point3};
use num_traits::Float;
use alga::general::Real;


pub fn shader() -> GouraudShader {
    GouraudShader::Proc
}

pub enum GouraudShader { 
    Proc,
}

type Args<N> = (Vector3<N>, Vector3<N>, Vector3<N>, Point3<N>);

impl<N> TextureMap<N, Args<N>> for GouraudShader
    where N: Float + Real 
{
    /// 
    /// Compute the Gouraud shading of a triangle primitive.
    ///
    fn apply(&self, args: Args<N>) -> Vector3<N> 
    {
        args.0 * (args.3)[0] + args.1 * (args.3)[1] + args.2 * (args.3)[2]
    }
}

fn_impl!(GouraudShader, N, Args<N>);


#[cfg(test)]
mod tests {
    use nalgebra::{Vector3, Point3};
    use num_traits::Float;
    use alga::general::Real;

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

    #[test]
    fn test_gouraud_shader() {
        let shader = super::shader();

        let color0 = Vector3::new(1.0, 0.0, 0.0);
        let color1 = Vector3::new(0.0, 1.0, 0.0);
        let color2 = Vector3::new(0.0, 0.0, 1.0);
        let bary   = Point3::new(0.2, 0.5, 0.3);

        let given = shader(color0, color1, color2, bary);
        let expected = gouraud(color0, color1, color2, bary);

        assert_relative_eq!(given, expected);
    }
}
