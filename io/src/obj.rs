#[cfg(test)]
mod tests;

use std::path::Path;
use obj;
use obj::{Obj, ObjData, ObjError, Group, SimplePolygon};
use yapre_graphics_core::space::object::Object as ObjectCore;
use yapre_graphics_core::space::Point;

pub fn load_obj(path: &str) -> Result<(), ObjError> {
    let raw_obj: Obj = Obj::load(Path::new(path))?;

    for object in raw_obj.data.objects {
        println!("analyzing object {:?}", object.name);

        for group in object.groups {
            println!("  analyzing group {:?} ({})", group.name, group.index);

            for poly in group.polys {
                println!("    polygon {:?}", poly);
            }
        }
    }

    panic!("not implemented yet")
}

struct PointRegistry {
    points: Vec<Point>
}

impl PointRegistry {
    fn new() -> PointRegistry {
        PointRegistry {
            points: Vec::new()
        }
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
        if index < 0 || index >= self.points.len() {
            return None;
        }

        Some(self.points[index])
    }
}

pub fn save_obj_file(path: &str, object: &ObjectCore) -> Result<(), String> {
    Err("not implemented yet".to_string())
}