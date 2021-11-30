use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
use std::time::Duration;

// TODO: it would be nice to seperate out a bunch of UI type stuff to build
// quick and simple UI components.

struct Button {
    name: String,
    rect: Rect,
    color: Color
}

impl Button {
    fn new(name: String, rect: Rect, color: Color) -> Button {
        Button { name, rect, color }
    }
}

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

    let mut buttons: Vec<Button> = Vec::new();
    buttons.push(Button::new(
        "alpha".to_string(),
        Rect::new(20, 20, 200, 40),
        Color::RGB(255, 255, 255)
    ));
    buttons.push(Button::new(
        "beta".to_string(),
        Rect::new(20, 80, 200, 40),
        Color::RGB(255, 255, 255)
    ));

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
                    mouse_y = y;
                },
                Event::MouseButtonDown {
                    x,
                    y,
                    mouse_btn: MouseButton::Left,
                    ..
                } => {
                    println!("Click at ({}, {}) with the left button", x, y);

                    for button in &buttons {
                        if button.rect.contains_point(Point::new(x, y)) {
                            println!("Button {} was clicked", &button.name);
                        }
                    }
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

        for button in &buttons {
            canvas.set_draw_color(button.color.clone());
            canvas.fill_rect(button.rect.clone());
        }

        canvas.present();

        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }

    println!("Hello, world!");
    Ok(())
}
