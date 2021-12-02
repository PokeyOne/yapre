use super::{Vector, Point};

pub struct Transform {
    pub sequence: Vec<TransformStep>
}

pub enum TransformStep {
    Translate(Vector),
    Rotate(Vector, Vector), // origin and amount
    Scale(Vector, Vector) // origin and amount
}

impl TransformStep {
    pub fn apply(&self, point: Point) -> Point {
        match self {
            &TransformStep::Translate(ref v) => point + v, // TODO: this
            &TransformStep::Rotate(ref u, ref v) => point.rotated(u.as_arr(), v),
            &TransformStep::Scale(ref u, ref v) => point.point_origin_scale(u, v)
        }
    }
}
