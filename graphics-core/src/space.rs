// Yet Another Pokey Render Engine (in Rust This Time)
// Copyright (C) 2021 Mateo Carreras
//
// file created: 2021-11-30

#[cfg(test)]
mod tests;

use std::ops::{Add, Sub, Mul};

const ORIGIN: Point = Point { x: 0.0, y: 0.0, z: 0.0 };

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub points: [Point; 3]
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

    pub fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn length(&self) -> f64 {
        self.dot(&self).sqrt()
    }

    pub fn normalized(&self) -> Self {
        let l = self.length();
        Point::new(self.x / l, self.y / l, self.z / l)
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
        assert_ne!(points[0], points[1]);
        assert_ne!(points[0], points[2]);
        assert_ne!(points[1], points[2]);

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
                // note we don't have to test if we have already matched this
                // jth point before because a Triangle with duplicate points
                // is undefined behaviour and panics in the "new" function.

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

pub struct Line {
    location: Point,
    direction: Point
}

impl Line {
    pub fn new(location: Point, direction: Point) -> Self {
        Line { location, direction: direction.normalized() }
    }

    pub fn from_points(a: Point, b: Point) -> Self {
        Line {
            location: a,
            direction: (b - a).normalized()
        }
    }
}

pub struct Mesh {
    tris: Vec<Triangle>
}
