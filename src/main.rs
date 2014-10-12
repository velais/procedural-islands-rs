extern crate libc;
extern crate num;

use std::rand::{task_rng, Rng};
use std::iter::range_step;
use num::complex::Complex;

mod png;
mod mask;
mod util;

struct World {
    map: Vec<f32>,
    width: int,
    height: int,
}

impl World {
    pub fn new(w: int, h: int) -> World {
        World {
            map: Vec::new(),
            width: w,
            height: h,
        }
    }

    pub fn print(&mut self) {
        let mut place = self.width - 1;
        for x in self.map.iter() {
            print!("{}", World::map_float(x));
            if place == 0 {
                print!("\n");
                place = self.width
            }
            place -= 1;
        }
    }

    fn map_float(val: &f32) -> char {
        // . * o O @
        match val {
            f if *f <= -1.0 => ' ',
            f if *f <= -0.5 => '.',
            f if *f <=  0.0 => '*',
            f if *f <=  0.5 => 'o',
            f if *f <=  1.0 => 'O',
            _  => '@',
        }
    }

    pub fn generate(&mut self, size: int) {
        let mut scale: f32 = 1.0;
        let mut sample_size = size;

        self.seed();
        while sample_size > 1 {
            self.diamond_square(sample_size, scale);
            sample_size /= 2;
            scale /= 2.0;
        }
    }

    fn seed(&mut self) {
        let mut rng = task_rng();
        for y in range(0i, self.height) {
            for x in range(0i, self.width) {
                let value = if y < 5 || y > 195 || x < 5 || x > 195 {
                    -1.5f32
                } else {
                    //rng.gen_range(-1.0, 1.0)
                    -0.5
                };
                self.map.push(value);
            }
        }
    }

    pub fn get_neighbourhood(&mut self, x: int, y: int) -> Vec<util::Point> {
        let mut result: Vec<util::Point>  = Vec::new();

        for a in range(-1i, 2) {
            for b in range(-1i, 2) {
                if a != 0 || b != 0 {
                    if x + a >= 0 && x + a < self.width && y + b >= 0 && y + b < self.height {
                        result.push(util::Point{ x: x + a, y: y + b });
                    }
                }
            }
        }

        let mut shuffle = result.as_mut_slice();
        task_rng().shuffle(shuffle);
        shuffle.into_vec()
    }

    pub fn rolling_particles(&mut self) {
        let mut rng = task_rng();

        let center_bias = false;
        let edge_bias = 25f32;
        let particle_length = 100u;

        for iterations in range(0i, 3000) {
            let (mut source_x, mut source_y) =  match center_bias {
                true =>
                    ((rng.gen_range(0.0, 1.0) * (self.width as f32 - (edge_bias * 2f32)) + edge_bias) as uint,
                     (rng.gen_range(0.0, 1.0) * (self.height as f32 - (edge_bias * 2f32)) + edge_bias) as uint),
                false =>
                    ((rng.gen_range(0.0, 1.0) * self.width as f32 - 1f32) as uint,
                     (rng.gen_range(0.0, 1.0) * self.height as f32 - 1f32) as uint)
            };
                    
            for l in range(0, particle_length) {
                source_x += ((rng.gen_range(0.0, 1.0) * 2f32 - 1f32).round()) as uint;
                source_y += ((rng.gen_range(0.0, 1.0) * 2f32 - 1f32).round()) as uint;

                if source_x < 1 || source_x > self.width as uint - 2 || source_y < 1 || source_y > self.height as uint - 2 {
                    break;
                }

                let mut hood = self.get_neighbourhood(source_x as int, source_y as int);
                for i in range(0u, hood.len()) {
                    if self.sample(hood[i].x, hood[i].y) < self.sample(source_x as int, source_y as int) {
                        source_x = hood[i].x as uint;
                        source_y = hood[i].y as uint;
                        break;
                    }
                }
                let current = self.sample(source_x as int, source_y as int);
                self.set_sample(source_x as int, source_y as int, current + 0.1f32);
            }
        }
    }

    fn sample(&mut self, x: int, y: int) -> f32 {
        self.map[(util::modulo(x ,(self.width - 1)) + util::modulo(y ,(self.height - 1)) * self.width) as uint]
    }

    fn set_sample(&mut self, x: int, y: int, val: f32) {
        *self.map.get_mut((util::modulo(x ,(self.width)) + util::modulo(y, (self.height)) * self.width) as uint) = val;
    }

    fn sample_square(&mut self, x: int, y: int, step: int, value: f32) {
        let hs: int = step / 2;

        let a = self.sample(x - hs, y - hs);
        let b = self.sample(x + hs, y - hs);
        let c = self.sample(x - hs, y + hs);
        let d = self.sample(x + hs, y + hs);

        self.set_sample(x, y, ((a + b + c + d) / 4.0) + value);
    }

    fn sample_diamond(&mut self, x: int, y: int, step: int, value: f32) {
        let hs: int = step / 2;

        let a = self.sample(x - hs, y);
        let b = self.sample(x + hs, y);
        let c = self.sample(x,  y - hs);
        let d = self.sample(x,  y + hs);

        self.set_sample(x, y, ((a + b + c + d) / 4.0) + value);
    }

    fn diamond_square(&mut self, step: int, scale: f32) {
        let hs: int = step / 2;
        let mut rng = task_rng();

        for y in range_step(hs, self.height + hs, step) {
            for x in range_step(hs, self.width + hs, step) {
                self.sample_square(x, y, step, rng.gen_range(-1.0, 1.0) * scale);
            }
        }

        for y in range_step(0i, self.height, step) {
            for x in range_step(0i, self.width, step) {
                self.sample_diamond(x + hs, y, step, rng.gen_range(-1.0, 1.0) * scale);
                self.sample_diamond(x, y + hs, step, rng.gen_range(-1.0, 1.0) * scale);
            }
        }
    }
}


fn main() {
    let mut world = World::new(200i, 200i);
    world.generate(16i);
    world.seed();

    let pm = world.rolling_particles();
    png::save(&mut world);
}








