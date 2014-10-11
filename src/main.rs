extern crate libc;
extern crate num;
extern crate image;

use std::rand::{task_rng, Rng};
use std::iter::range_step;
use std::io::File;
use image::GenericImage;
use num::complex::Complex;

mod noise;

fn save_png(world: &mut World) {
    let max_iterations = 256u16;

    let imgx: u32 = world.width as u32;
    let imgy: u32 = world.height as u32;

    let mut imbuf = image::ImageBuf::new(imgx, imgy);

    for y in range(0, imgy) {
        for x in range(0, imgx) {
            let pixel = match world.sample(x as int, y as int) {
                f if f < -0.2 => image::Rgb(20, 80, 163),
                f if f <  0.0 => image::Rgb(156, 196, 251),
                f if f <  0.2 => image::Rgb(235, 224, 168),
                f if f <  0.5 => image::Rgb(158, 191, 105),
                f if f <  1.2 => image::Rgb(79, 120, 14),
                f if f <  1.5 => image::Rgb(94, 102, 112),
                _ => image::Rgb(250, 250, 250),
            };
            imbuf.put_pixel(x, y, pixel);
        }
    }

   // Save the image as "fractal.png"
   let fout = File::create(&Path::new("fractal.png")).unwrap();

   //We must indicate the image's color type and what format to save as.
   let _    = image::ImageRgb8(imbuf).save(fout, image::PNG);
}

fn modulo(a: int, b: int) -> int {
    (((a % b) + b) % b)
}

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
                    rng.gen_range(-1.0, 1.0)
                };

                
                self.map.push(value);
            }
        }
        self.set_sample(0, 0, -2.0);
        self.set_sample(200, 0, -2.0);
        self.set_sample(0, 200, -2.0);
        self.set_sample(200, 200, -2.0);
        self.set_sample(100, 100, 2.0);
    }


    pub fn sample(&mut self, x: int, y: int) -> f32 {
        self.map[(modulo(x ,(self.width - 1)) + modulo(y ,(self.height - 1)) * self.width) as uint]
    }

    pub fn set_sample(&mut self, x: int, y: int, val: f32) {
        *self.map.get_mut((modulo(x ,(self.width)) + modulo(y, (self.height)) * self.width) as uint) = val;
    }

    pub fn sample_square(&mut self, x: int, y: int, step: int, value: f32) {
        let hs: int = step / 2;

        let a = self.sample(x - hs, y - hs);
        let b = self.sample(x + hs, y - hs);
        let c = self.sample(x - hs, y + hs);
        let d = self.sample(x + hs, y + hs);

        self.set_sample(x, y, ((a + b + c + d) / 4.0) + value);
    }

    pub fn sample_diamond(&mut self, x: int, y: int, step: int, value: f32) {
        let hs: int = step / 2;

        let a = self.sample(x - hs, y);
        let b = self.sample(x + hs, y);
        let c = self.sample(x,  y - hs);
        let d = self.sample(x,  y + hs);

        self.set_sample(x, y, ((a + b + c + d) / 4.0) + value);
    }

    pub fn diamond_square(&mut self, step: int, scale: f32) {
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

    save_png(&mut world);

    noise::tester();
}








