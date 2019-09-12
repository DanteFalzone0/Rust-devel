pub mod three_d_types;
use crate::cube::space::three_d_types::*;
use sdl2::rect::Point;

const DEGREE_RADIAN_RATIO: f64 = 0.0174533;

/* Definition of axes:
Z-axis: lower values mean closer to the viewer;
        higher values mean further away.
        (close/far)
X-axis: lower values mean further to the left;
        higher values mean further to the right.
        (left/right)
Y-axis: lower values mean higher on the screen;
        higher values mean closer to the bottom.
        (high/low)
*/

#[derive(Copy, Clone)]
pub struct Space {
    pub depth:    f64,
    pub screen_x: f64,
    pub screen_y: f64
}

impl Space {
    pub fn point_at(&self, x: f64, y: f64, z: f64) -> CoordTriple {
        let ret = CoordTriple {
            x: x,
            y: y,
            z: z
        };
        ret
    }

    // Take a 3d coordinate and project it onto a 2d screen
    pub fn convert_to_2d(&self, q: CoordTriple) -> Point {
        let ret = Point::new(
            ((q.x + (self.screen_x * (q.z / self.depth))) /
            ((q.z / self.depth) + 1.0)) as i32,
            ((q.y + (self.screen_y * (q.z / self.depth))) /
            ((q.z / self.depth) + 1.0)) as i32
        );

        if q.z > (-1.0 * self.depth) {
            ret
        } else {
            // point that won't appear on the screen
            Point::new(-1, -1)
        }
    }

    pub fn z_rotation(
        &self,
        point: CoordTriple,
        about: CoordTriple,
        degrees: f64
    ) -> CoordTriple {
        let theta: f64 = degrees * DEGREE_RADIAN_RATIO;
        let s: f64 = theta.sin();
        let c: f64 = theta.cos();

        // translate back to origin
        let trans_x: f64 = point.x - about.x;
        let trans_y: f64 = point.y - about.y;

        // perform the rotation
        let x_new: f64 = (trans_x * c) - (trans_y * s);
        let y_new: f64 = (trans_x * s) + (trans_y * c);

        // translate back and create the CoordTriple
        let ret = CoordTriple {
            x: x_new + about.x,
            y: y_new + about.y,
            z: point.z
        };

        ret
    }

    // next two functions do rotations similarly to above, but along different axes
    pub fn x_rotation(
        &self,
        point: CoordTriple,
        about: CoordTriple,
        degrees: f64
    ) -> CoordTriple {
        let theta: f64 = degrees * DEGREE_RADIAN_RATIO;
        let s: f64 = theta.sin();
        let c: f64 = theta.cos();
        let trans_y: f64 = point.y - about.y;
        let trans_z: f64 = point.z - about.z;
        let y_new: f64 = (trans_y * c) - (trans_z * s);
        let z_new: f64 = (trans_y * s) + (trans_z * c);
        let ret = CoordTriple {
            x: point.x,
            y: y_new + about.y,
            z: z_new + about.z
        };
        ret
    }
    pub fn y_rotation(
        &self,
        point: CoordTriple,
        about: CoordTriple,
        degrees: f64
    ) -> CoordTriple {
        let theta: f64 = degrees * DEGREE_RADIAN_RATIO;
        let s: f64 = theta.sin();
        let c: f64 = theta.cos();
        let trans_x: f64 = point.x - about.x;
        let trans_z: f64 = point.z - about.z;
        let x_new: f64 = (trans_x * c) - (trans_z * s);
        let z_new: f64 = (trans_x * s) + (trans_z * c);
        let ret = CoordTriple {
            x: x_new + about.x,
            y: point.y,
            z: z_new + about.z
        };
        ret
    }

}
