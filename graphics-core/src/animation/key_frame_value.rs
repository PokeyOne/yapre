#[cfg(test)]
mod tests;

use std::fmt::Debug;

/// A value that can be animated.
///
/// Essentially allows for the definition of a interpolation formula between
/// two values. This is already implemented for
/// - `f32`
/// - `f64`
/// - `i8`
/// - `i16`
/// - `i32`
/// - `i64`
/// - `i128`
/// - `u8`
/// - `u16`
/// - `u32`
/// - `u64`
/// - `u128`
/// - `usize`
///
/// The general formula for numbers would be:
/// ```text
/// a * (1.0 - t) + b * t
/// ```
/// Where `a` and `b` are the start and end values, and `t` is the interpolation
/// factor (i.e. a number between 0 and 1).
///
/// It also defines the other traits that are required for the animation system.
pub trait KeyFrameValue: Debug + Clone {
    /// Get a mixture of this value and the supplied value based on the
    /// interpolation factor.
    ///
    /// The interpolation factor is usually a number between 0 and 1, but this
    /// function should not assume that. The implementations for the primitive
    /// types, for example, just does the math without any clamping.
    ///
    /// # Parameters
    ///
    /// - `other`: The value to mix with.
    /// - `t`: The interpolation factor.
    ///
    /// # Returns
    ///
    /// The mixed value.
    ///
    /// # Examples
    ///
    /// ```
    /// use yapre_graphics_core::animation::KeyFrameValue;
    ///
    /// let a: f64 = 1.0;
    /// let b: f64 = 5.0;
    /// let t: f64 = 0.5;
    /// assert_eq!(a.interpolate(&b, t), 3.0);
    /// ```
    fn interpolate(&self, other: &Self, t: f64) -> Self;
}

macro_rules! impl_key_frame_value {
    ($t:ty) => {
        impl KeyFrameValue for $t {
            fn interpolate(&self, other: &Self, t: f64) -> Self {
                let result = (*self as f64) * (1.0 - t) + (*other as f64) * t;

                result as $t
            }
        }
    };
}

macro_rules! impl_key_frame_value_int {
    ($t:ty) => {
        impl KeyFrameValue for $t {
            fn interpolate(&self, other: &Self, t: f64) -> Self {
                let result = (*self as f64) * (1.0 - t) + (*other as f64) * t;

                result.round() as $t
            }
        }
    };
}

impl_key_frame_value!(f32);
impl_key_frame_value!(f64);
impl_key_frame_value_int!(i8);
impl_key_frame_value_int!(i16);
impl_key_frame_value_int!(i32);
impl_key_frame_value_int!(i64);
impl_key_frame_value_int!(i128);
impl_key_frame_value_int!(u8);
impl_key_frame_value_int!(u16);
impl_key_frame_value_int!(u32);
impl_key_frame_value_int!(u64);
impl_key_frame_value_int!(u128);
impl_key_frame_value_int!(usize);