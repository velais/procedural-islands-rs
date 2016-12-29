extern crate num;
extern crate rand;
#[macro_use] extern crate cfor;


use rand::{ Rng, thread_rng };

mod png;
mod util;

pub struct World {
    map: Vec<f32>,
    width: i32,
    height: i32,
}

impl World {
    pub fn new(w: i32, h: i32) -> World {
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

    pub fn generate(&mut self, size: i32) {
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
        for y in 0 .. self.height {
            for x in 0 .. self.width {
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

    pub fn get_neighbourhood(&mut self, x: i32, y: i32) -> Vec<util::Point> {
        let mut result: Vec<util::Point>  = Vec::new();

        for a in -1 .. 2 {
            for b in -1 .. 2 {
                if a != 0 || b != 0 {
                    if x + a >= 0 && x + a < self.width && y + b >= 0 && y + b < self.height {
                        result.push(util::Point{ x: x + a, y: y + b });
                    }
                }
            }
        }

        let mut shuffle = result.as_mut_slice();
        thread_rng().shuffle(shuffle);
        shuffle.to_vec()
    }

    pub fn rolling_particles(&mut self) {
        let mut rng = thread_rng();

        let center_bias = false;
        let edge_bias = 25f32;
        let particle_length = 100;

        for _ in 0 .. 3000 {
            let (mut source_x, mut source_y) =  match center_bias {
                true =>
                    ((rng.gen_range(0.0, 1.0) * (self.width as f32 - (edge_bias * 2f32)) + edge_bias) as i32,
                     (rng.gen_range(0.0, 1.0) * (self.height as f32 - (edge_bias * 2f32)) + edge_bias) as i32),
                false =>
                    ((rng.gen_range(0.0, 1.0) * self.width as f32 - 1f32) as i32,
                     (rng.gen_range(0.0, 1.0) * self.height as f32 - 1f32) as i32)
            };
                    
            for _ in 0 .. particle_length {
                source_x += ((rng.gen_range(0.0, 1.0) * 2f32 - 1f32).round()) as i32;
                source_y += ((rng.gen_range(0.0, 1.0) * 2f32 - 1f32).round()) as i32;

                if source_x < 1 || source_x > self.width as i32 - 2 || source_y < 1 || source_y > self.height as i32 - 2 {
                    break;
                }

                let hood = self.get_neighbourhood(source_x as i32, source_y as i32);
                for i in 0 .. hood.len() {
                    if self.sample(hood[i].x, hood[i].y) < self.sample(source_x as i32, source_y as i32) {
                        source_x = hood[i].x as i32;
                        source_y = hood[i].y as i32;
                        break;
                    }
                }
                let current = self.sample(source_x as i32, source_y as i32);
                self.set_sample(source_x as i32, source_y as i32, current + 0.1f32);
            }
        }
    }

    fn sample(&mut self, x: i32, y: i32) -> f32 {
        self.map[(util::modulo(x ,(self.width - 1)) + util::modulo(y ,(self.height - 1)) * self.width) as usize]
    }

    fn set_sample(&mut self, x: i32, y: i32, val: f32) {
        let index = (util::modulo(x ,(self.width)) + util::modulo(y, (self.height)) * self.width) as usize;
        if let Some(sample) = self.map.get_mut(index) {
            *sample = val;
        };
    }

    fn sample_square(&mut self, x: i32, y: i32, step: i32, value: f32) {
        let hs: i32 = step / 2;

        let a = self.sample(x - hs, y - hs);
        let b = self.sample(x + hs, y - hs);
        let c = self.sample(x - hs, y + hs);
        let d = self.sample(x + hs, y + hs);

        self.set_sample(x, y, ((a + b + c + d) / 4.0) + value);
    }

    fn sample_diamond(&mut self, x: i32, y: i32, step: i32, value: f32) {
        let hs: i32 = step / 2;

        let a = self.sample(x - hs, y);
        let b = self.sample(x + hs, y);
        let c = self.sample(x,  y - hs);
        let d = self.sample(x,  y + hs);

        self.set_sample(x, y, ((a + b + c + d) / 4.0) + value);
    }

    fn diamond_square(&mut self, step: i32, scale: f32) {
        let hs: i32 = step / 2;
        let mut rng = thread_rng();

        cfor!{let mut y = hs; y < self.height + hs; y += step; {
            cfor!{let mut x = hs; x < self.width + hs; x += step; {
                self.sample_square(x, y, step, rng.gen_range(-1.0, 1.0) * scale);
            }}
        }}

        cfor!{let mut y = 0; y < self.height; y += step; {
            cfor!{let mut x = 0; x < self.width; x += step; {
                self.sample_diamond(x + hs, y, step, rng.gen_range(-1.0, 1.0) * scale);
                self.sample_diamond(x, y + hs, step, rng.gen_range(-1.0, 1.0) * scale);
            }}
        }}
    }
}


fn main() {
    let mut world = World::new(200, 200);
    world.generate(16);
    world.rolling_particles();

    png::save(&mut world);
}








