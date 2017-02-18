#![allow(dead_code)]
use num_traits::Float;
use alga::general::Real;


///
/// Calculate the minimum of three different items.
///
pub fn min3<N: Float + Real>(x: N, y: N, z: N) -> N {
    Float::min(Float::min(x, y), z)
}

///
/// Calculate the maximum of three different items.
///
pub fn max3<N: Float + Real>(x: N, y: N, z: N) -> N {
    Float::max(Float::max(x, y), z)
}


#[cfg(test)]
mod tests {
    #[test]
    fn test_min_with_equal_values() {
        let a: f32 = 15.0;
        let b: f32 = 36.0;
        let c: f32 = 15.0;

        let res = super::min3(a, b, c);

        assert_eq!(res, 15.0);
    }

    #[test]
    fn test_min() {
        let a: f32 = 5665.0;
        let b: f32 = 36.0;
        let c: f32 = 15.0;

        let res = super::min3(a, b, c);

        assert_eq!(res, c);
    }

    #[test]
    fn test_max_with_equal_values() {
        let a: f32 = 5665.014;
        let b: f32 = 5665.014;
        let c: f32 = 15.0;

        let res = super::max3(a, b, c);

        assert_eq!(res, a);
    }

    #[test]
    fn test_max() {
        let a: f32 = 5665.0;
        let b: f32 = 36.0;
        let c: f32 = 15.0;

        let res = super::max3(a, b, c);

        assert_eq!(res, a);
    }
}
