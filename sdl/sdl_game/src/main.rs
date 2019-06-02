/* To help myself remember how these things (probably) map onto
their C equivalents, I've written comments next to some lines to
indicate what they would be in C */
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

mod point;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    // SDL_CreateWindow()
    let window = video_subsystem.window(
        "Dante's Fun SDL2 Game (but wait it's written in Rust)",
        600,
        600
    ).position_centered().opengl().build()
    .map_err(|e| e.to_string())?;

    // SDL_CreateRenderer
    let mut canvas = window.into_canvas().build()
    .map_err(|e| e.to_string())?;

    canvas.set_draw_color(Color::RGB(0, 0, 0));
    canvas.clear(); // SDL_RenderClear();
    canvas.present(); // SDL_RenderPResent();

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..} | Event::KeyDown {
                    keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                _ => {}
            }
        }
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        // The rest of the game loop goes here...
    }

    Ok(())
}
