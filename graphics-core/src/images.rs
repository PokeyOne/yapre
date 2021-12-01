use sdl2::pixels::{Color, PixelFormatEnum};
use sdl2::render::Canvas;
use sdl2::surface::Surface;
use sdl2::rect::Point as SdlPoint;

pub struct RawImage<'a> {
    canvas: Canvas<Surface<'a>>
}

impl RawImage<'_> {
    pub fn new(width: u32, height: u32, pixel_format: PixelFormatEnum) -> Result<Self, String> {
        let surface = Surface::new(width, height, pixel_format)?;
        let canvas = surface.into_canvas()?;

        Ok(RawImage { canvas })
    }

    pub fn set_pixel(&mut self, c: Color, x: i32, y: i32) -> Result<(), String> {
        self.canvas.set_draw_color(c);
        self.canvas.draw_point(SdlPoint::new(x, y))?;

        Ok(())
    }
}
