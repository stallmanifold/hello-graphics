pub use self::rgb::Rgb;
use self::rgb::RgbCast;

mod rgb;

///
/// Calculate the RGB color of a color vector.
///
#[inline(always)]
pub fn rgb<V, R: RgbCast<V, RgbValue=R>>(color: V) -> R {
    R::rgb_cast(color)
}
