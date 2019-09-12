/**
 * Written by Dante Falzone
 * @author dfalzone
 */

extern crate sdl2;
pub use sdl2::rect::Point;

/* Here I define some data types. This is translated fairly 
 * literally from `3d_types.hpp`, the equivalent component of
 * djf_3d, my previous engine written in C++. */

#[derive(Copy, Clone)] // so we can copy CoordPairs
pub struct CoordPair {
    pub x: f64,
    pub y: f64
}

#[derive(Copy, Clone)]
pub struct CoordTriple {
    pub x: f64,
    pub y: f64,
    pub z: f64
}
