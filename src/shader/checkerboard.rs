use shader::texture::TextureMap;
use nalgebra::{Vector2, Vector3, Point3};
use num_traits::Float;
use alga::general::Real;
use std::marker::PhantomData;


///
/// Factory function for creating a new `CheckerboardShader`.
///
pub fn shader<N: Float + Real>(n_squares: usize) -> CheckerboardShader<N> {
    CheckerboardShader::new(n_squares)
}

pub struct CheckerboardShader<N> {
    n_squares: usize,
    _phantom: PhantomData<N>,
}

impl<N> CheckerboardShader<N> where N: Float + Real {
    fn new(n_squares: usize) -> CheckerboardShader<N> {
        CheckerboardShader {
            n_squares: n_squares,
            _phantom: PhantomData,
        }
    }
}

type Args<N> = (Vector2<N>, 
                Vector2<N>, 
                Vector2<N>,
                Point3<N>,
                Point3<N>,
                Point3<N>,
                Point3<N>);


macro_rules! checkerboard_impl {
    ($type_name : ty) => {
        impl TextureMap<$type_name, Args<$type_name>> for CheckerboardShader<$type_name> {
            /// 
            /// Compute the checkerboard shading of a triangle primitive.
            ///
            fn apply(&self, args: Args<$type_name>) -> Vector3<$type_name> {
                let z  = 1.0 / ((args.6)[0]*(args.3)[2] + (args.6)[1]*(args.4)[2] + (args.6)[2]*(args.5)[2]);

                let s = z * ((args.6)[0]*(args.0)[0] + (args.6)[1]*(args.1)[0] + (args.6)[2]*(args.2)[0]);
                let t = z * ((args.6)[0]*(args.0)[1] + (args.6)[1]*(args.1)[1] + (args.6)[2]*(args.2)[1]);

                let m = self.n_squares as $type_name;

                let p = ((((s*m % 1.0 > 0.5) as usize) ^ (t*m % 1.0 < 0.5) as usize)) as $type_name;

                Vector3::new(p,p,p)
            }
        }
    }
}

checkerboard_impl!(f32);
checkerboard_impl!(f64);


specific_fn_impl!(CheckerboardShader<f32>, f32, Args<f32>);
specific_fn_impl!(CheckerboardShader<f64>, f64, Args<f64>);


mod tests {
    use nalgebra::{Vector2, Vector3, Point3};
    use num_traits::Float;
    use alga::general::Real;

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

    #[test]
    fn test_checkerboard_shader() {
        let st0: Vector2<f32> = Vector2::new(0.0, 0.0);
        let st1: Vector2<f32> = Vector2::new(0.0, 1.0);
        let st2: Vector2<f32> = Vector2::new(1.0, 0.0);
        
        let v0: Point3<f32> = Point3::new(-30.0, -30.0, 0.0);
        let v1: Point3<f32> = Point3::new(30.0, 30.0, 0.0);
        let v2: Point3<f32> = Point3::new(30.0, -30.0, 0.0);

        let w: Point3<f32> = Point3::new(0.2, 0.4, 0.4);

        let shader = super::shader::<f32>(10);

        let given = shader(st0, st1, st2, v0, v1, v2, w);
        let expected = checkerboard(st0, st1, st2, v0, v1, v2, w);

        assert_relative_eq!(given, expected);
    }
}
