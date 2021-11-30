// Yet Another Pokey Render Engine (in Rust This Time)
// Copyright (C) 2021 Mateo Carreras
//
// file created: 2021-11-30

mod tests;

use std::ops::{Add, Sub, Mul};

const ORIGIN: Point = Point { x: 0.0, y: 0.0, z: 0.0 };

#[derive(Debug, Copy, Clone, PartialEq)]
struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Debug, Clone)]
struct Triangle {
    pub points: [Point; 3]
}

#[derive(Debug, Clone)]
struct RefTriangle<'a> {
    pub points: [&'a Point; 3]
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

    pub fn scale(&self, scale: f64) -> Self {
        Point {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale
        }
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point::new(
            self.x + other.x,
            self.y + other.y,
            self.z + other.z
        )
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point::new(
            self.x - other.x,
            self.y - other.y,
            self.z - other.z
        )
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        self.scale(other)
    }
}

impl Triangle {
    pub fn new(points: [Point; 3]) -> Self {
        Triangle { points }
    }
}

impl PartialEq for Triangle {
    fn eq(&self, other: &Triangle) -> bool {
        // These are the indexes of each point of this triangle in the other
        // triangle
        let mut indexes_in_other: [i8; 3] = [-1, -1, -1];

        for i in 0..3 {
            'j_loop: for j in 0..3 {
                // If we have already matched this index with another ith triangle
                // on self, then we can't match it again.
                for iio in indexes_in_other {
                    if iio == j {
                        continue 'j_loop;
                    }
                }

                if self.points[i as usize] == other.points[j as usize] {
                    indexes_in_other[i] = j;
                    break 'j_loop;
                }
            }
        }

        // Now if all the numbers in "indexes_in_other" are non-negative, then
        // we have equality.
        for iio in indexes_in_other {
            if iio < 0 {
                return false;
            }
        }

        return true;
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
