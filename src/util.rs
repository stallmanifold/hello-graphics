use std::cmp;


pub fn min3<T: Ord>(x: T, y: T, z: T) -> T {
    cmp::min(cmp::min(x, y), z)
}

pub fn max3<T: Ord>(x: T, y: T, z: T) -> T {
    cmp::max(cmp::max(x, y), z)
}
