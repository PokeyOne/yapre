// Yet Another Pokey Render Engine (in Rust This Time)
// Copyright (C) 2021 Mateo Carreras
//
// file created: 2021-11-30

#[cfg(test)]
mod tests;

use crate::space::{Line, Mesh, Point, Triangle};

type Ray = Line;

pub trait Collidable {
    fn intersects(&self, ray: &Ray) -> Option<Point>;
}

impl Collidable for Triangle {
    fn intersects(&self, ray: &Ray) -> Option<Point> {
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
        if u < 0.0 || u > 1.0 {
            return None;
        }
        let q = t.cross(&e1);
        let v = ray.direction().dot(&q) * inv_det;
        if v < 0.0 || u + v > 1.0 {
            return None;
        }
        let t = e2.dot(&q) * inv_det;
        if t > 1e-6 {
            Some(ray.location() + ray.direction() * t)
        } else {
            None
        }
    }
}
