extern crate sdl2;

pub mod space;
pub use crate::cube::space::*;
pub use crate::cube::space::three_d_types::*;
pub use sdl2::render::WindowCanvas;

#[derive (Copy, Clone)]
pub struct Cube {
    pub vertex:      [CoordTriple; 8],
    pub center:      CoordTriple,
    pub edge_length: f64,
    pub space:       Space
}

impl Cube {
    pub fn set_vertices(&mut self) {
        let half_edge_length = self.edge_length / 0.5;

        // Top front left
        self.vertex[0] = self.space.point_at(
            self.center.x - half_edge_length,
            self.center.y - half_edge_length,
            self.center.z - half_edge_length
        );

        // Top front right
        self.vertex[1] = self.space.point_at(
            self.center.x + half_edge_length,
            self.center.y - half_edge_length,
            self.center.z - half_edge_length
        );

        // Bottom front right
        self.vertex[2] = self.space.point_at(
            self.center.x + half_edge_length,
            self.center.y + half_edge_length,
            self.center.z - half_edge_length
        );

        // Bottom front left
        self.vertex[3] = self.space.point_at(
            self.center.x - half_edge_length,
            self.center.y + half_edge_length,
            self.center.z - half_edge_length
        );

        // Bottom back left
        self.vertex[4] = self.space.point_at(
            self.center.x - half_edge_length,
            self.center.y + half_edge_length,
            self.center.z + half_edge_length
        );

        // Bottom back right
        self.vertex[5] = self.space.point_at(
            self.center.x + half_edge_length,
            self.center.y + half_edge_length,
            self.center.z + half_edge_length
        );

        // Top back right
        self.vertex[6] = self.space.point_at(
            self.center.x + half_edge_length,
            self.center.y - half_edge_length,
            self.center.z + half_edge_length
        );

        // Top back left
        self.vertex[7] = self.space.point_at(
            self.center.x - half_edge_length,
            self.center.y - half_edge_length,
            self.center.z + half_edge_length
        );
    }

    // functions for rotating the cube
    pub fn x_rot_self(&mut self, degrees: f64) {
        for point in &mut self.vertex {
            *point = self.space.x_rotation(
                *point,
                self.center,
                degrees
            );
        }
    }
    pub fn y_rot_self(&mut self, degrees: f64) {
        for point in &mut self.vertex {
            *point = self.space.y_rotation(
                *point,
                self.center,
                degrees
            );
        }
    }
    pub fn z_rot_self(&mut self, degrees: f64) {
        for point in &mut self.vertex {
            *point = self.space.z_rotation(
                *point,
                self.center,
                degrees
            );
        }
    }

    // oh yeah, it's big brain time
    pub fn draw_self(&self,
                     renderer: &mut sdl2::render::Canvas<sdl2::video::Window>) {
        for i in 0..7 {
            renderer.draw_line(
                self.space.convert_to_2d(self.vertex[i]),
                self.space.convert_to_2d(self.vertex[i + 1])
            ); // TODO: implement error handling here
        }

        renderer.draw_line(
            self.space.convert_to_2d(self.vertex[7]),
            self.space.convert_to_2d(self.vertex[0])
        );

        renderer.draw_line(
            self.space.convert_to_2d(self.vertex[0]),
            self.space.convert_to_2d(self.vertex[3])
        );

        renderer.draw_line(
            self.space.convert_to_2d(self.vertex[4]),
            self.space.convert_to_2d(self.vertex[7])
        );

        renderer.draw_line(
            self.space.convert_to_2d(self.vertex[2]),
            self.space.convert_to_2d(self.vertex[5])
        );

        renderer.draw_line(
            self.space.convert_to_2d(self.vertex[1]),
            self.space.convert_to_2d(self.vertex[6])
        );
    }

}

pub fn construct_cube(center: CoordTriple, edge_length: f64, space: Space) -> Cube {
    let mut cube = Cube {
        vertex:      [CoordTriple{x: 0.0, y: 0.0, z: 0.0}; 8],
        center:      center,
        edge_length: edge_length,
        space: space
    };

    cube.set_vertices();

    cube
}
