use shader::texture::TextureMap;
use nalgebra::{Vector3, Point3, BaseFloat};


pub fn shader() -> Gouraud {
    Gouraud::Proc
}

pub enum Gouraud { 
    Proc,
}

impl<N> TextureMap<N, (Vector3<N>, Vector3<N>, Vector3<N>, Point3<N>)> for Gouraud
    where N: BaseFloat 
{
    fn apply(&self, 
              args: (Vector3<N>, Vector3<N>, Vector3<N>, Point3<N>)) -> Vector3<N> 
    {
        args.0 * (args.3)[0] + args.1 * (args.3)[1] + args.2 * (args.3)[2]
    }
}

fn_impl!(Gouraud, N, (Vector3<N>, Vector3<N>, Vector3<N>, Point3<N>));


#[cfg(test)]
mod tests {
    use nalgebra::{BaseFloat, ApproxEq, Vector3, Point3};
     

    /// 
    /// Compute the gouraud shading of a triangle primitive.
    ///
    pub fn gouraud<N>(color0: Vector3<N>, 
                      color1: Vector3<N>, 
                      color2: Vector3<N>, 
                      bary:   Point3<N>) -> Vector3<N>
        where N: BaseFloat
    {
        color0 * bary[0] + color1 * bary[1] + color2 * bary[2]
    }

    #[test]
    fn test_gouraud_map_should_yield_same_results_as_function() {
        let shader = super::shader();

        let color0 = Vector3::new(1.0, 0.0, 0.0);
        let color1 = Vector3::new(0.0, 1.0, 0.0);
        let color2 = Vector3::new(0.0, 0.0, 1.0);
        let bary   = Point3::new(0.2, 0.5, 0.3);

        let res = shader(color0, color1, color2, bary);
        let ans = gouraud(color0, color1, color2, bary);

        assert!(res.approx_eq(&ans));
    }
}
