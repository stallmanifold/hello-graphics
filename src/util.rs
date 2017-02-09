use std::cmp;

/// Calculate the minimum of three different items.
pub fn min3<T: Ord>(x: T, y: T, z: T) -> T {
    cmp::min(cmp::min(x, y), z)
}

// Calculate the maximum of three different items.
pub fn max3<T: Ord>(x: T, y: T, z: T) -> T {
    cmp::max(cmp::max(x, y), z)
}
