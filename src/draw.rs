use image;
use super::*;

const IMAGE_DIM : u32 = 800; // 1600 x 1600 dimensions
pub fn draw_history(function : &dyn Function, history : &History) {   
    
}

fn draw_function_space(function : &dyn Function) -> image::ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    let [[x_min, x_max], [y_min, y_max]] = function.domain();
    let [min_val, max_val] = function.range();
    let color_scalar = (255.0) / max_val;
    // Define the step size at which to evaluate the function
    let dx = ((x_max - x_min).abs() as f64) / (IMAGE_DIM as f64); 
    let dy = ((y_max - y_min).abs() as f64) / (IMAGE_DIM as f64);
    let mut imgbuf = image::ImageBuffer::new(IMAGE_DIM, IMAGE_DIM);
    for (x,y,pixel) in imgbuf.enumerate_pixels_mut(){
        let point = (x_min + (x as f64) * dx, y_min + (y as f64) * dy);
        let value = function.eval(point).unwrap() * color_scalar;
        let ratio = (value - min_val) / (value - max_val);

        let r = value as u8;
        *pixel = image::Rgb([r, 112, 112]);
    }   
    return imgbuf
}

const START_COLOR : [u8;3] = [250, 250, 110];
const END_COLOR : [u8;3] = [42, 72, 88];
fn lerp(start : u8, end : u8, ratio : f64) -> image::Rgb<u8> {
    let r = (START_COLOR[0] as f64 * (1.0 - ratio) + END_COLOR[0] as f64 * ratio) as u8;
    let g = (START_COLOR[1] as f64 * (1.0 - ratio) + END_COLOR[1] as f64 * ratio) as u8;
    let b = (START_COLOR[2] as f64 * (1.0 - ratio) + END_COLOR[2] as f64 * ratio) as u8;
    image::Rgb([r,g,b])
}