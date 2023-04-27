#[cfg(test)]
pub mod tests;

use crate::animation::{AnimatedFloat, AnimatedValue};
use crate::space::Point;

/// An alias for `AnimatedPoint` because it can be more semantically correct
/// to refer to something as a vector rather than a point.
pub type AnimatedVector = AnimatedPoint;

/// A basic point in 3D space that has animated values.
#[derive(Debug, Clone)]
pub struct AnimatedPoint {
    values: [AnimatedFloat; 3]
}

#[derive(Debug, Clone)]
/// A set of three animated points in 3D space.
pub struct AnimatedTriangle {
    points: [AnimatedPoint; 3]
}

impl AnimatedPoint {
    /// Create a new point from 3 already constructed animated values.
    pub fn new(values: [AnimatedFloat; 3]) -> Self {
        Self { values }
    }

    /// Creates a constant-values animated point, from a static, constant point.
    /// The resulting AnimatedPoint structure will always have a value equal
    /// to the supplied point for any input time.
    pub fn from_point(p: Point) -> Self {
        Self {
            values: [
                AnimatedValue::constant(p.x),
                AnimatedValue::constant(p.y),
                AnimatedValue::constant(p.z)
            ]
        }
    }

    /// Calculates the value of the animated point at the time requested. The
    /// time is essentially the frame number, but can also be a float to specify
    /// in-between frames.
    pub fn get_value<T: Into<f64>>(&self, time: T) -> Point {
        let t = time.into();
        Point::new(
            self.values[0].get_value(t),
            self.values[1].get_value(t),
            self.values[2].get_value(t)
        )
    }
}
