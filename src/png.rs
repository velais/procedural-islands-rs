extern crate image;

use std::io::File;
use self::image::GenericImage;

use World;

//Save world as a png
pub fn save(world: &mut World) {
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
   let _  = image::ImageRgb8(imbuf).save(fout, image::PNG);
}
