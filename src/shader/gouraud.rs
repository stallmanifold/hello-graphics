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

}