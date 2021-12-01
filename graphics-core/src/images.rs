#[cfg(test)]
mod tests;

use std::path::Path;
use std::fmt::{Formatter, Error as FormatterError};
use uuid::Uuid;

pub const WHITE: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 255
};
pub const BLACK: Color = Color {
    r: 0,
    g: 0,
    b: 0,
    a: 255
};
pub const RED: Color = Color {
    r: 255,
    g: 0,
    b: 0,
    a: 255
};
pub const GREEN: Color = Color {
    r: 0,
    g: 255,
    b: 0,
    a: 255
};
pub const BLUE: Color = Color {
    r: 0,
    g: 0,
    b: 255,
    a: 255
};

pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color {
            r: r,
            g: g,
            b: b,
            a: a
        }
    }

    pub fn rgba(&self) -> u32 {
        (self.r as u32) << 24 | (self.g as u32) << 16 | (self.b as u32) << 8 | (self.a as u32)
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatterError> {
        write!(f, "Color {{ {:#X} }}", self.rgba());
    }
}

pub struct Pixel {
    color: Color
}

impl Pixel {
    pub fn new(color: Color) -> Pixel {
        Pixel {
            color: color
        }
    }
}

pub struct RawImage {
    pixels: Vec<Vec<Pixel>>
}

impl RawImage<'_> {
    pub fn new(width: u32, height: u32) -> Self {

    }

    pub fn set_pixel(&mut self, c: Color, x: i32, y: i32) -> Result<(), String> {
        self.canvas.set_draw_color(c);
        self.canvas.draw_point(SdlPoint::new(x, y))?;

        Ok(())
    }

    /// Output the image to a file in a temporary directory then return the
    /// path. This method returns a Result and if the file it save it will
    /// return the path to the file, and if it fails then it will be a string
    /// with an error method.
    pub fn save_to_temp_path(&self) -> Result<String, String> {
        let path_string = generate_random_temp_path();
        let path = Path::new(&path_string);
        println!("Path would be {:?}", path_string);

        Err(String::from("Not implemented"))
    }
}

fn generate_random_temp_path() -> String {
    let uuid = Uuid::new_v4();
    format!("./tmp-yapre/temp_image_{}.bmp", uuid)
}
