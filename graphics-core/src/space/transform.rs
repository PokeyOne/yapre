use super::{Point, Vector};

pub struct Transform {
    pub sequence: Vec<TransformStep>
}

pub enum TransformStep {
    Translate(Vector),
    Rotate(Vector, Vector), // origin and amount
    Scale(Vector, Vector)   // origin and amount
}

impl Transform {
    pub fn new() -> Transform {
        Transform {
            sequence: Vec::new()
        }
    }

    pub fn add_step(&mut self, step: TransformStep) {
        self.sequence.push(step);
    }

    pub fn apply(&self, point: &Point) -> Point {
        let mut result: Point = *point;

        for step in &self.sequence {
            result = step.apply(&result);
        }

        result
    }
}

impl TransformStep {
    pub fn apply(&self, point: &Point) -> Point {
        match self {
            &TransformStep::Translate(ref v) => point + v, // TODO: this
            &TransformStep::Rotate(ref u, ref v) => point.rotated(u.as_arr(), v),
            &TransformStep::Scale(ref u, ref v) => point.point_origin_scaled(u, v)
        }
    }
}
