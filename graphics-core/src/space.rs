// Yet Another Pokey Render Engine (in Rust This Time)
// Copyright (C) 2021 Mateo Carreras
//
// file created: 2021-11-30

#[cfg(test)]
mod tests;

use std::ops::{Add, Div, Mul, Sub};

pub const ORIGIN: Point = Point {
    x: 0.0,
    y: 0.0,
    z: 0.0
};

/// The most basic unit of free space, respresenting a single location using
/// the x, y, and z axes.
#[derive(Debug, Copy, Clone)]
pub struct Point {
    /// x coordinate
    pub x: f64,
    /// y coordinate
    pub y: f64,
    /// z coordinate
    pub z: f64
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        (self.x - other.x).abs() < 0.00001
        && (self.y - other.y).abs() < 0.00001
        && (self.z - other.z).abs() < 0.00001
    }
}

#[derive(Debug, Clone)]
pub struct Triangle {
    pub points: [Point; 3]
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    /// Computes the result of scalar multiplication of this Point.
    ///
    /// # Example
    /// ```
    /// # use yapre_graphics_core::space::Point;
    /// let scaled = Point::new(5.0, 4.0, 3.0).scale(2.0);
    /// assert_eq!(Point::new(10.0, 8.0, 6.0), scaled);
    /// ```
    pub fn scale(&self, scale: f64) -> Self {
        Point {
            x: self.x * scale,
            y: self.y * scale,
            z: self.z * scale
        }
    }

    /// Essentially the inverse of scale. Instead of scalar multiplication, this
    /// computes scalar division.
    ///
    /// # Example
    /// ```
    /// # use yapre_graphics_core::space::Point;
    /// let shrunk = Point::new(10.0, 8.0, 6.0).shrink(2.0);
    /// assert_eq!(Point::new(5.0, 4.0, 3.0), shrunk);
    /// ```
    pub fn shrink(&self, shrinkage: f64) -> Self {
        Point {
            x: self.x / shrinkage,
            y: self.y / shrinkage,
            z: self.z / shrinkage
        }
    }

    pub fn dot(&self, other: &Point) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Point) -> Point {
        Point {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x
        }
    }

    pub fn length(&self) -> f64 {
        self.dot(&self).sqrt()
    }

    /// Calculates a vector in the same direction as self, but with a magnitude
    /// equal to one. This is sometimes referred to as the direction vector, the
    /// normal vector, or simply the direction of a vector.
    ///
    /// It is also worth noting that when called on a vector with length 0, this
    /// function with return a clone of itself. This deviates from pure
    /// mathematics, but avoids the division by zero and for our applications
    /// this should work just fine.
    ///
    /// # Example
    /// ```
    /// # use yapre_graphics_core::space::Point;
    /// use yapre_graphics_core::space::ORIGIN;
    ///
    /// assert_eq!(ORIGIN, ORIGIN.normalized());
    /// assert_eq!(Point::new(0.0, 0.6, 0.8), Point::new(0.0, 3.0, 4.0).normalized());
    /// ```
    pub fn normalized(&self) -> Self {
        // For the zero vector we return the zero vector. This is not 100%
        // mathematically accuration, but for our applications is appropriate.
        if *self == ORIGIN {
            return self.clone();
        }

        let l = self.length();
        Point::new(self.x / l, self.y / l, self.z / l)
    }

    /// Rotates the point around the given point by the given angle.
    ///
    /// # Example
    /// ```
    /// # use yapre_graphics_core::space::Point;
    /// let point = Point::new(1.0, 0.0, 0.0);
    /// let rotated = point.rotated([0.0, std::f64::consts::PI, 0.0], &Point::new(0.0, 0.0, 0.0));
    /// assert_eq!(Point::new(-1.0, 0.0, 0.0), rotated);
    /// ```
    pub fn rotated(&self, angle: [f64; 3], origin: &Point) -> Self {
        let x = self.x - origin.x;
        let y = self.y - origin.y;
        let z = self.z - origin.z;

        let x_rot = x * angle[0].cos() - y * angle[0].sin();
        let y_rot = x * angle[0].sin() + y * angle[0].cos();
        let z_rot = z;

        let x_rot = x_rot * angle[1].cos() - z_rot * angle[1].sin();
        let z_rot = x_rot * angle[1].sin() + z_rot * angle[1].cos();
        let y_rot = y_rot;

        let x_rot = x_rot * angle[2].cos() - y_rot * angle[2].sin();
        let y_rot = x_rot * angle[2].sin() + y_rot * angle[2].cos();
        let z_rot = z_rot;

        let x_rot = x_rot + origin.x;
        let y_rot = y_rot + origin.y;
        let z_rot = z_rot + origin.z;

        Point::new(x_rot, y_rot, z_rot)
    }
}

impl Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<&Point> for Point {
    type Output = Self;

    fn add(self, other: &Point) -> Self {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<Point> for &Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Add<&Point> for &Point {
    type Output = Point;

    fn add(self, other: &Point) -> Point {
        Point::new(self.x + other.x, self.y + other.y, self.z + other.z)
    }
}

impl Sub for Point {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<&Point> for Point {
    type Output = Self;

    fn sub(self, other: &Point) -> Self {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<Point> for &Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Sub<&Point> for &Point {
    type Output = Point;

    fn sub(self, other: &Point) -> Point {
        Point::new(self.x - other.x, self.y - other.y, self.z - other.z)
    }
}

impl Mul<f64> for Point {
    type Output = Self;

    fn mul(self, other: f64) -> Self {
        self.scale(other)
    }
}

impl Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, other: f64) -> Point {
        self.scale(other)
    }
}

impl Div<f64> for Point {
    type Output = Self;

    fn div(self, other: f64) -> Self {
        self.shrink(other)
    }
}

impl Triangle {
    pub fn new(points: [Point; 3]) -> Self {
        assert_ne!(points[0], points[1]);
        assert_ne!(points[0], points[2]);
        assert_ne!(points[1], points[2]);

        Triangle { points }
    }

    pub fn shift(&mut self, by: Point) {
        for i in 0..3 {
            self.points[i] = self.points[i] + by;
        }
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
        Line {
            location,
            direction: direction.normalized()
        }
    }

    pub fn from_points(a: Point, b: Point) -> Self {
        Line {
            location: a,
            direction: (b - a).normalized()
        }
    }

    pub fn location(&self) -> &Point {
        &self.location
    }

    pub fn direction(&self) -> &Point {
        &self.direction
    }
}

pub struct Mesh {
    tris: Vec<Triangle>
}
