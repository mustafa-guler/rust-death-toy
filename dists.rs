use crate::{ VEL_MAX, VEL_MIN, IM_MAX, IM_MIN };
use rand_distr::{ Uniform, Normal, Distribution };
use std::f64::consts::{ PI };

pub fn sample_angle() -> f64 {
    Uniform::new_inclusive(0.0, 2.0 * PI).sample(&mut rand::thread_rng())
}

pub fn sample_vel(old_vel: f64) -> f64 {
    let new_val = old_vel + Uniform::new_inclusive(-3.0, 3.0).sample(&mut rand::thread_rng()) / 60.0;
    match new_val {
        x if x > VEL_MAX => VEL_MAX,
        x if x < VEL_MIN => VEL_MIN,
        x => x
    }
}

pub fn sample_im(old_im: i64) -> i64 {
    let new_im = old_im + Uniform::new_inclusive(-270, 270).sample(&mut rand::thread_rng());

    match new_im {
        x if x > IM_MAX => IM_MAX,
        x if x < IM_MIN => IM_MIN,
        x => x
    }
}

pub fn sample_move_time() -> i64 {
    let move_time = Normal::new(80.0, 40.0).unwrap();
    let mut curr_sample = move_time.sample(&mut rand::thread_rng());
    while curr_sample < 1.0 || curr_sample > 220.0 {
        curr_sample = move_time.sample(&mut rand::thread_rng());
    }

    curr_sample as i64
}

// #[derive(Debug)]
// pub struct Dists {
//     pub angle: Uniform<f64>,    // radians
//     pub vel: Uniform<f64>,      // micron/hour
//     pub im: Uniform<i64>,       // min
//     pub move_time: Normal<f64>  // min
// }

// impl Dists {
//     pub fn new() -> Dists {
//         let angle = Uniform::new_inclusive(0.0, 2.0 * PI);
//         let vel = Uniform::new_inclusive(-3.0, 3.0);
//         let im = Uniform::new_inclusive(-270, 270);
//         let move_time = Normal::new(80.0, 40.0).unwrap();

//         Dists { angle, vel, im, move_time }
//     }

//     pub fn sample_angle(&self) -> f64 {
//         self.angle.sample(&mut rand::thread_rng())
//     }

//     pub fn sample_vel(&self, old_vel: f64) -> f64 {
//         // convert to micron/min from micron/h
//         let new_val = old_vel + self.vel.sample(&mut rand::thread_rng()) / 60.0;

//         match new_val {
//             x if x > VEL_MAX => VEL_MAX,
//             x if x < VEL_MIN => VEL_MIN,
//             x => x
//         }
//     }

//     pub fn sample_im(&self, old_im: i64) -> i64 {
//         let new_im = old_im + self.im.sample(&mut rand::thread_rng());

//         match new_im {
//             x if x > IM_MAX => IM_MAX,
//             x if x < IM_MIN => IM_MIN,
//             x => x
//         }
//     }

//     pub fn sample_move_time(&self) -> i64 {
//         let mut curr_sample = self.move_time.sample(&mut rand::thread_rng());
//         while curr_sample < 1.0 || curr_sample > 220.0 {
//             curr_sample = self.move_time.sample(&mut rand::thread_rng());
//         }

//         curr_sample as i64
//     }
// }
