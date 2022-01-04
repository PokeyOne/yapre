#[cfg(test)]
pub mod tests;

use crate::space::{Point, Triangle, Line};
use crate::animation::AnimatedValue;

/// An alias for `AnimatedPoint` because it can be more semantically correct
/// to refer to something as a vector rather than a point.
pub type AnimatedVector = AnimatedPoint;

/// A basic point in 3D space that has animated values.
#[derive(Debug, Clone)]
pub struct AnimatedPoint {
    values: [AnimatedValue; 3]
}

#[derive(Debug, Clone)]
pub struct AnimatedTriangle {
    points: [AnimatedPoint; 3]
}

impl AnimatedPoint {
    pub fn new(values: [AnimatedValue; 3]) -> Self {
        Self { values }
    }

    pub fn from_point(p: Point) -> Self {
        Self {
            values: [
                AnimatedValue::constant(p.x),
                AnimatedValue::constant(p.y),
                AnimatedValue::constant(p.z)
            ]
        }
    }
}
