extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

// It took me a bit of trial-and-error to get this right. Having gotten
// used to header files from C and C++, modules are a bit of an adjustment.
pub mod cube;
pub use cube::Cube;
pub use crate::cube::space::three_d_types::CoordTriple;
pub use crate::cube::space::Space;
pub use crate::cube::*;

// main() returns either an error message or nothing, hence `-> Result<(), String>`.
fn main() -> Result<(), String> {
    println!("Hello, world!");

    // bit of SDL2 boilerplate here
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window(
        "Threedox Engine Demo (written in Rust by Dante Falzone)",
        600,
        600
    ).position_centered().opengl().build()
        .map_err(|e| e.to_string())?;
    
    /* I must confess that I have no idea what `.opengl().build()` or
    `.into_canvas().build()` means. */

    let mut renderer = window.into_canvas().build()
        .map_err(|e| e.to_string())?;

    renderer.set_draw_color(Color::RGB(0, 0, 0));
    renderer.clear(); // fill the whole screen
    renderer.set_draw_color(Color::RGB(0, 255, 80));

    let space = Space {
        depth:    200.0, // TODO: make the 2d-projection a little less hacky
        screen_x: 300.0,
        screen_y: 300.0
    };

    let mut cube = construct_cube(
        space.point_at(300.0, 300.0, 0.0), // centerpoint
        16.0, // length of edges
        space // where the Cube exists
    );

    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        // do some funky rotations to make it look cooler
        cube.x_rot_self(0.5);
        cube.y_rot_self(0.5);
        cube.z_rot_self(0.1);

        for event in event_pump.poll_iter() {
            // TODO: gain a clearer understanding of the syntax here;
            // I have a pretty good idea of what it says but I wouldn't have
            // known to write it this way
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

        cube.draw_self(&mut renderer).map_err(|e| e.to_string())?;

        renderer.present(); // show what the renderer has drawn
    }

    println!("End of program");

    Ok(()) // TODO: learn more about Rust exit statuses
}
