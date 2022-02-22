#[cfg(test)]
mod tests;

use std::fmt::Debug;

/// A value that can be animated.
pub trait KeyFrameValue: Debug + Clone {
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