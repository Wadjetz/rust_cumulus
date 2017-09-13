extern crate image;

use std::path::Path;
use std::fs::File;

use image::imageops::Nearest;
use image::imageops::Gaussian;
use image::GenericImage;

fn main() {
    println!("Hello from images");
    process_image();
}

fn process_image() {
    let blur_level = 30;
    let img = image::open(Path::new("static/trump.jpg")).unwrap();
    let mut output = File::create(Path::new("static/trump_out.jpg")).unwrap();
    let (w, h) = img.dimensions();
    img
        .resize(w / blur_level, h / blur_level, Nearest)
        .resize(w, h, Nearest)
        .save(&mut output, image::JPEG).unwrap();
}
