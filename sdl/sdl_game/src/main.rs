extern crate sdl2;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use std::time::Duration;

// Here we import code I wrote in `point.rs`, which uses sdl2::rect::Point.
// The 'pub' keyword gets rid of the "function is never used: `p_to_acc`" compiler warning.
pub mod point;

fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?; // initialize the SDL stuff
    let video_subsystem = sdl_context.video()?;

    // Create an SDL window
    let window = video_subsystem.window(
        "Dante's Fun SDL2 Game (but wait it's written in Rust)",
        600,
        600
    ).position_centered().opengl().build()
        .map_err(|e| e.to_string())?; // catch any errors

    // create a new SDL rendering context
    let mut renderer = window.into_canvas().build()
        .map_err(|e| e.to_string())?;

    renderer.set_draw_color(Color::RGB(0, 0, 0)); // pitch-black
    renderer.clear(); // paint the whole window
    renderer.set_draw_color(Color::RGB(0, 255, 80)); // hacker green

    // create a mutable AccPoint as defined in `point.rs` at (100, 300).
    // This AccPoint is mutable because it's the one that moves.
    let mut my_point_0 = point::AccPoint{x: 100.0, y: 300.0};

    // create an immutable AccPoint (immutable because it's fixed)
    let my_point_1 = point::AccPoint{x: 300.0, y: 300.0};

    /* draw_line() is written to take instances of sdl2::rect::Point.
    However, each member of sdl2::rect::Point is an i32, which doesn't
    give us the accuracy we need to perform point rotations. Therefore,
    the data for the points is stored as f64 values in point::AccPoints,
    from which sdl2::rect::Points are then created on the fly as needed.
    Hence passing the return values of point::acc_to_p() directly to
    draw_line(). */
    renderer.draw_line(point::acc_to_p(my_point_0), point::acc_to_p(my_point_1))
        .map_err(|e| e.to_string())?;

    renderer.present(); // done rendering for now

    // This event loop kills the program if the Escape key is pressed
    // or if the X button is clicked
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

        // clear the screen
        renderer.set_draw_color(Color::RGB(0, 0, 0));
        renderer.clear();

        // redraw the line
        renderer.set_draw_color(Color::RGB(0, 255, 80));
        renderer.draw_line(point::acc_to_p(my_point_0), point::acc_to_p(my_point_1))
            .map_err(|e| e.to_string())?;

        // rotate the point
        my_point_0 = point::rotate(my_point_0, my_point_1, 1.0);

        // done rendering
        renderer.present();
    }

    Ok(())
}
