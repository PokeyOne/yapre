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
