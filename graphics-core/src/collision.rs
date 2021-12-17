// Yet Another Pokey Render Engine (in Rust This Time)
// Copyright (C) 2021 Mateo Carreras
//
// file created: 2021-11-30

#[cfg(test)]
mod tests;

use crate::space::{Line, Point, Triangle};

pub type Ray = Line;

pub struct Collision {
    pub point: Point,
    pub distance: f64
}

impl Collision {
    pub fn new(point: Point, distance: f64) -> Self {
        Self { point, distance }
    }
}

pub trait Collidable {
    fn intersection_point(&self, ray: &Ray) -> Option<Collision>;
}

impl Collidable for Triangle {
    // TODO: This code was written by an AI, so literally no one knows
    //       fully what it does. Should probably go through and rename variables
    //       and comment what things do.
    /// Returns the intersection point of a ray with a triangle.
    fn intersection_point(&self, ray: &Ray) -> Option<Collision> {
        let e1 = self.points[1] - self.points[0];
        let e2 = self.points[2] - self.points[0];
        let p = ray.direction().cross(&e2);
        let det = e1.dot(&p);
        if det.abs() < 1e-6 {
            return None;
        }
        let inv_det = 1.0 / det;
        let t = ray.location() - self.points[0];
        let u = t.dot(&p) * inv_det;
        #[allow(clippy::manual_range_contains)]
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = t.cross(&e1);
        let v = ray.direction().dot(&q) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = e2.dot(&q) * inv_det;
        let location_of_collision = ray.location() + ray.direction() * t;

        // Because the ray direction is normalized, the distance between the
        // ray's location and the collision point is the same as t.
        Some(Collision::new(location_of_collision, t))
    }
}
