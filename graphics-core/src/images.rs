#[cfg(test)]
mod tests;

use png::Encoder;
use rand;
use std::fmt::{Debug, Error as FormatterError, Formatter};
use std::fs::File;
use std::io::BufWriter;
use std::path::Path;
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
pub const CLEAR: Color = Color {
    r: 255,
    g: 255,
    b: 255,
    a: 0
};

#[derive(Clone, PartialEq)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
        Color { r, g, b, a }
    }

    pub fn rgba(&self) -> u32 {
        (self.r as u32) << 24 | (self.g as u32) << 16 | (self.b as u32) << 8 | (self.a as u32)
    }

    pub fn from_rgba(rgba: u32) -> Color {
        Color {
            r: ((rgba >> 24) & 0xFF) as u8,
            g: ((rgba >> 16) & 0xFF) as u8,
            b: ((rgba >> 8) & 0xFF) as u8,
            a: (rgba & 0xFF) as u8
        }
    }

    pub fn random() -> Color {
        Color::new(
            rand::random::<u8>(),
            rand::random::<u8>(),
            rand::random::<u8>(),
            255
        )
    }
}

impl Debug for Color {
    fn fmt(&self, f: &mut Formatter) -> Result<(), FormatterError> {
        write!(f, "Color {{ {:#X} }}", self.rgba())
    }
}

#[derive(Clone, Debug)]
pub struct Pixel {
    pub color: Color
}

impl Pixel {
    pub fn new(color: Color) -> Pixel {
        Pixel { color }
    }
}

pub struct RawImage {
    pixels: Vec<Vec<Pixel>>
}

impl RawImage {
    pub fn new(width: usize, height: usize) -> Self {
        let mut pixels: Vec<Vec<Pixel>> = Vec::new();
        pixels.reserve(width);

        for _ in 0..width {
            let column: Vec<Pixel> = vec![Pixel::new(CLEAR); height];
            pixels.push(column);
        }

        RawImage { pixels }
    }

    pub fn get_pixel(&self, row: usize, col: usize) -> &Pixel {
        &self.pixels[row][col]
    }

    pub fn set_pixel(&mut self, c: Color, row: usize, col: usize) {
        self.pixels[row][col].color = c;
    }

    pub fn get_width(&self) -> usize {
        self.pixels[0].len()
    }

    pub fn get_height(&self) -> usize {
        self.pixels.len()
    }

    /// Output the image to a file in a temporary directory then return the
    /// path. This method returns a Result and if the file it save it will
    /// return the path to the file, and if it fails then it will be a string
    /// with an error method.
    pub fn save_to_temp_path(&self) -> Result<String, String> {
        let path_string = generate_random_temp_path();

        self.save_image_to_path(&path_string)?;

        Ok(path_string)
    }

    pub fn get_image_data(&self) -> Vec<u8> {
        let mut data: Vec<u8> = Vec::new();
        data.reserve(self.get_width() * self.get_height() * 4);

        for row in 0..self.get_height() {
            for col in 0..self.get_width() {
                let pixel = self.get_pixel(row, col);
                data.push(pixel.color.r);
                data.push(pixel.color.g);
                data.push(pixel.color.b);
                data.push(pixel.color.a);
            }
        }

        data
    }

    pub fn save_image_to_path(&self, path: &str) -> Result<(), String> {
        let path = Path::new(path);
        let file = File::create(path).map_err(|e| format!("{}", e))?;
        let w = &mut BufWriter::new(file);

        let mut encoder = Encoder::new(w, self.get_width() as u32, self.get_height() as u32);
        encoder.set_color(png::ColorType::Rgba);
        encoder.set_depth(png::BitDepth::Eight);
        let mut writer = encoder.write_header().map_err(|e| format!("{}", e))?;

        let data = self.get_image_data();
        writer
            .write_image_data(&data)
            .map_err(|e| format!("{}", e))?;

        Ok(())
    }
}

fn generate_random_temp_path() -> String {
    let uuid = Uuid::new_v4();
    format!("./tmp-yapre/temp_image_{}.png", uuid)
}
