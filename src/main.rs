extern crate image;
use num::complex;
use image::{ImageBuffer, RgbImage};

fn main() {
    let width  = 5000;
    let height = 5000;
    let iterations = 200;

    let mut img: RgbImage = ImageBuffer::new(width, height);

    let mut z = complex::Complex::new(0.0, 0.0);
    let mut c = complex::Complex::new(0.0, 0.0);

    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let r = (0.3 * x as f32) as u8;
        let b = (0.3 * y as f32) as u8;
        *pixel = image::Rgb([r, 0, b]);
    }
    
    for y in 0..height {
        for x in 0..width {
            c = complex::Complex64::new(convert_range(x as f64, 0.0, width as f64, -2.5, 2.5), convert_range(y as f64, 0.0, width as f64, -2.5, 2.5));
            z = c;
            let mut iters = 0;
            for iter in 0..iterations {
                z = z.powi(2) + c;
                iters = iter;
                if z.im > 4.0 || z.re > 4.0 {
                    break
                }
            }
            let pixel = img.get_pixel_mut(x, y);
            let image::Rgb(data) = *pixel;
//            *pixel = image::Rgb([data[0], iters as u8, data[2]]);
            *pixel = image::Rgb([iters as u8, iters as u8, iters as u8]);
        }
    }
    img.save("fractal.png").unwrap();
}

fn convert_range(value: f64, old_range_min: f64, old_range_max: f64, new_range_min: f64, new_range_max: f64) -> f64 {
    #[warn(unused_parens)]
    (((value - old_range_min) * (new_range_max - new_range_min)) / (old_range_max - old_range_min) + new_range_min)
}
