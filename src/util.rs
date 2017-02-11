#![allow(dead_code)]
use num_traits::Float;


/// Calculate the minimum of three different items.
pub fn min3<N: Float>(x: N, y: N, z: N) -> N {
    Float::min(Float::min(x, y), z)
}

// Calculate the maximum of three different items.
pub fn max3<N: Float>(x: N, y: N, z: N) -> N {
    Float::max(Float::max(x, y), z)
}
