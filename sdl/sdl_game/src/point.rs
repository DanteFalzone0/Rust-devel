use sdl2::rect::Point;

// I'm creating a struct called AccPoint which stands for 'accurate
// point'. I'm doing this because sdl2::rect::Point uses i32 fields,
// which aren't accurate enough to do point rotations.
#[derive(Copy, Clone)] // this allows us to copy AccPoints
pub struct AccPoint {
    pub x: f64,
    pub y: f64
}

// create an sdl2::rect::Point from an AccPoint
pub fn acc_to_p(a: AccPoint) -> Point {
    let ret = Point::new(
        a.x as i32,
        a.y as i32
    );
    ret
}

// create an AccPoint from an sdl2::rect::Point
pub fn p_to_acc(p: Point) -> AccPoint {
    let ret = AccPoint {
        x: p.x as f64,
        y: p.y as f64
    };
    ret
}

// This function creates a rotation of an AccPoint
pub fn rotate(point: AccPoint, about: AccPoint, degrees: f64) -> AccPoint {
    let radians: f64 = degrees * 0.01745329; // ratio of degrees to radians
    let s = radians.sin();
    let c = radians.cos();

    // determine where the point would be if translated back to origin
    let trans_x: f64 = point.x - about.x;
    let trans_y: f64 = point.y - about.y;

    // perform the rotation
    let x_new: f64 = (trans_x * c) - (trans_y * s);
    let y_new: f64 = (trans_x * s) + (trans_y * c);

    // return CoordPair from rotated point
    let ret = AccPoint {
        x: x_new + about.x,
        y: y_new + about.y
    };

    ret
}
