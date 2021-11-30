// Yet Another Pokey Render Engine (in Rust This Time)
// Copyright (C) 2021 Mateo Carreras
//
// file created: 2021-11-30

#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
    z: f64
}

#[derive(Debug, Clone)]
struct Triangle {
    points: [Point; 3]
}

#[derive(Debug, Clone)]
struct RefTriangle<'a> {
    points: [&'a Point; 3]
}

#[derive(Debug, Clone)]
enum SomeTriangle<'a> {
    Triangle(Triangle),
    RefTriangle(RefTriangle<'a>)
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }
}

impl Triangle {
    pub fn new(points: [Point; 3]) -> Self {
        Triangle { points }
    }
}

impl<'a> RefTriangle<'a> {
    pub fn new(points: [&'a Point; 3]) -> Self {
        RefTriangle { points }
    }

    pub fn into_triangle(self) -> Triangle {
        Triangle::new([
            self.points[0].clone(),
            self.points[1].clone(),
            self.points[2].clone()
        ])
    }
}
