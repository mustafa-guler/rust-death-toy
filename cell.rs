use crate::coord::Coord;
use crate::dists;
use crate::{ CELL_RADIUS };
use rand::Rng;


#[derive(Debug, Clone, Copy)]
pub struct Cell {
    pub vel: f64,   // cell velocity in microns/min
    pub im: i64,    // intermitotic time in min
    im_count: i64,  // time left until cell divides, min
    pub pos: Coord, // position of cell
    move_ang: f64,  // angle for cell movement, radians
    move_time: i64, // time cell moves in that angle, min
}


impl Cell {
    pub fn new(vel: f64, im: i64, pos: Coord) -> Cell {
        let move_ang = dists::sample_angle();
        let move_time = dists::sample_move_time();
        let im_count = im;

        Cell { vel, im, im_count, pos, move_ang, move_time }
    }

    pub fn new_pos(&mut self) {
        let x = self.pos.x + self.vel * self.move_ang.cos();
        let y = self.pos.y + self.vel * self.move_ang.sin();

        self.pos = Coord::new(x, y);
    }

    pub fn divide(&mut self) -> Cell {
        // get new velocities
        let vel1 = dists::sample_vel(self.vel);
        let vel2 = dists::sample_vel(self.vel);

        // get new IM times
        let im1 = dists::sample_im(self.im);
        let im2 = dists::sample_im(self.im);

        // update current cell as one of the new daughter cells
        self.vel = vel1;
        self.im = im1;
        self.im_count = im1;
        self.move_ang = dists::sample_angle();
        self.move_time = dists::sample_move_time();

        // find new position for other daughter cell
        let new_ang = dists::sample_angle();
        let new_x = self.pos.x + 2.0 * CELL_RADIUS * new_ang.cos();
        let new_y = self.pos.y + 2.0 * CELL_RADIUS * new_ang.sin();

        Cell::new(vel2, im2, Coord::new(new_x, new_y))
    }

    pub fn step(&mut self) -> Option<Cell> {
        self.move_time -= 1;
        self.im_count -= 1;
        self.new_pos();

        if self.im_count == 0 {
            Some(self.divide())
        } else if self.move_time == 0 {
            self.move_time = dists::sample_move_time();
            self.move_ang = dists::sample_angle();
            None
        } else {
            None
        }
    }
}
