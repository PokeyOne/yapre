use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Point;
use std::time::Duration;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem
        .window("Yet Another Pokey Render Engine", 800, 600)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(128, 128, 128));
    canvas.clear();
    canvas.present();
    let mut event_pump = sdl_context.event_pump()?;

    // The x and y position in pixels from the top-left corner
    let mut mouse_x = 0;
    let mut mouse_y = 0;

    'main_loop: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'main_loop,
                Event::MouseMotion {
                    x,
                    y,
                    ..
                } => {
                    mouse_x = x;
                    mouse_y = y
                },
                _ => {}
            }
        }

        canvas.set_draw_color(Color::RGB(128, 128, 128));
        canvas.clear();

        canvas.set_draw_color(Color::RGB(0, 0, 0));
        canvas.draw_line(Point::new(0, 0), Point::new(mouse_x, mouse_y));
        canvas.draw_line(Point::new(800, 0), Point::new(mouse_x, mouse_y));
        canvas.draw_line(Point::new(0, 600), Point::new(mouse_x, mouse_y));
        canvas.draw_line(Point::new(800, 600), Point::new(mouse_x, mouse_y));

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    println!("Hello, world!");
    Ok(())
}
