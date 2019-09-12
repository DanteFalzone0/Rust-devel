
extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

pub mod cube;
pub use cube::Cube;
pub use crate::cube::space::three_d_types::CoordTriple;
pub use crate::cube::space::Space;
pub use crate::cube::*;

fn main() -> Result<(), String> {
    println!("Hello, world!");

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window(
        "Threedox Engine Demo (written in Rust by Dante Falzone)",
        600,
        600
    ).position_centered().opengl().build()
        .map_err(|e| e.to_string())?;

    let mut renderer = window.into_canvas().build()
        .map_err(|e| e.to_string())?;

    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear();
    renderer.set_draw_color(Color::RGB(0, 255, 80));

    let space = Space {
        depth:    200.0,
        screen_x: 300.0,
        screen_y: 300.0
    };

    let mut cube = construct_cube(
        space.point_at(300.0, 300.0, 0.0),
        16.0,
        space
    );

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        cube.x_rot_self(0.5);
        
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

        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();
        renderer.set_draw_color(Color::RGB(0, 255, 80));

        cube.draw_self(&mut renderer);

        renderer.present();
    }

    println!("End of program");

    Ok(())
}
