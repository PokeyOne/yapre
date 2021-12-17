#[cfg(test)]
mod tests;

use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;
use yapre_graphics_core::space::object::{Mesh, Object};
use yapre_graphics_core::space::{Point, Triangle};

pub fn load_object_from_file(path: &str) -> Result<Object, String> {
    let path = Path::new(path);
    println!("Working directory: {:?}", Path::new(".").canonicalize());
    println!(
        "Loading object from file: {}",
        path.canonicalize().unwrap().display()
    );
    let mut file = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(format!("opening file: {}", e))
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(contents) => contents,
        Err(e) => return Err(format!("reading file: {}", e))
    };

    load_object_from_contents(contents)
}

macro_rules! unwrap_parse_type_or_return {
    ($expr:expr, $ty:ty) => {
        match $expr {
            Some(val) => match val.parse::<$ty>() {
                Ok(val) => val,
                Err(_) => {
                    return Err(format!(
                        "could not parse {} from {}",
                        std::any::type_name::<$ty>(),
                        val
                    ))
                }
            },
            None => {
                return Err(format!(
                    "could not get next token to parse {}",
                    std::any::type_name::<$ty>()
                ))
            }
        }
    };
}

pub fn load_object_from_contents(contents: String) -> Result<Object, String> {
    let mut point_registry = PointRegistry::new();
    let mut triangle_data: Vec<[usize; 3]> = Vec::new();

    // Process each line of the file
    for line in contents.lines() {
        let mut line_iter = line.split_whitespace();
        let line_type = match line_iter.next() {
            Some(line_type) => line_type,
            None => continue // blank line
        };

        match line_type {
            "v" => {
                let x = unwrap_parse_type_or_return!(line_iter.next(), f64);
                let y = unwrap_parse_type_or_return!(line_iter.next(), f64);
                let z = unwrap_parse_type_or_return!(line_iter.next(), f64);
                point_registry.add_point(Point::new(x, y, z));
            }
            "f" => {
                let a = unwrap_parse_type_or_return!(line_iter.next(), usize);
                let b = unwrap_parse_type_or_return!(line_iter.next(), usize);
                let c = unwrap_parse_type_or_return!(line_iter.next(), usize);
                triangle_data.push([a - 1, b - 1, c - 1]);
            }
            _ => {} // comments and other stuff
        }
    }

    // Now un-anonymize the points
    let mut triangles = Vec::new();
    for td in triangle_data {
        triangles.push(Triangle::new([
            // Unwrap is assumed safe because we just added the points to the
            // registry
            point_registry.get_point(td[0]).unwrap(),
            point_registry.get_point(td[1]).unwrap(),
            point_registry.get_point(td[2]).unwrap()
        ]));
    }

    // Construct the mesh and the object
    let mesh = Mesh::new(triangles);
    Ok(Object::new(mesh))
}

struct PointRegistry {
    points: Vec<Point>
}

impl PointRegistry {
    fn new() -> PointRegistry {
        PointRegistry { points: Vec::new() }
    }

    /// Returns the index of the point in the registry, or adds it if it doesn't
    /// exist yet.
    fn add_point(&mut self, point: Point) -> usize {
        for i in 0..self.points.len() {
            if self.points[i] == point {
                return i;
            }
        }

        let index = self.points.len();
        self.points.push(point);
        index
    }

    fn get_point(&self, index: usize) -> Option<Point> {
        if index >= self.points.len() {
            return None;
        }

        Some(self.points[index])
    }
}

pub fn generate_obj_file(object: &Object) -> String {
    let mut registry = PointRegistry::new();
    let mut triangle_data: Vec<[usize; 3]> = Vec::new();
    for tri in object.triangles() {
        let mut i = 0;
        let mut registry_triangle = [0, 0, 0];
        for point in tri.points {
            registry_triangle[i] = registry.add_point(point);
            i += 1;
        }
        triangle_data.push(registry_triangle);
    }

    let mut output_file_contents = String::new();
    output_file_contents.push_str("# OBJ file generated by yapre\n");
    for point in &registry.points {
        output_file_contents.push_str(&format!("v {:.6} {:.6} {:.6}\n", point.x, point.y, point.z));
    }
    for triangle in &triangle_data {
        output_file_contents.push_str(&format!(
            "f {:.6} {:.6} {:.6}\n",
            triangle[0] + 1,
            triangle[1] + 1,
            triangle[2] + 1
        ));
    }

    output_file_contents
}

pub fn save_obj_file(path: &str, object: &Object) -> Result<(), String> {
    let mut output_file = std::fs::File::create(path).map_err(|e| e.to_string())?;
    output_file
        .write_all(generate_obj_file(object).as_bytes())
        .map_err(|e| e.to_string())?;
    Ok(())
}
