pub struct CoordPair {
    pub x: f64,
    pub y: f64
}

// Is this a pure function, since it doesn't use "mut"?
pub fn rotate(point: CoordPair, about: CoordPair, degrees: f64) -> CoordPair {
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
    let ret = CoordPair {
        x: x_new + about.x,
        y: y_new + about.y
    };

    ret
}
