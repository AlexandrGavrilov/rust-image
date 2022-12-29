mod structs;

use std::string::String;
use image::DynamicImage;

pub use structs::Crop;

fn get_image(infile: &String) -> DynamicImage {
    image::open(infile).expect("Failed to open INFILE")
}

fn save_image(img: DynamicImage, outfile: &String) {
    img.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn blur_image(infile: &String, outfile: &String, blur: Option<f32>) {
    let img = get_image(infile);

    let blur_intense = if let Some(blur) = blur {
        blur
    } else {
        2.0
    };

    save_image(img.blur(blur_intense), outfile);
}

pub fn brighten_image(infile: &String, outfile: &String, brighten: Option<i32>) {
    let img = get_image(infile);

    let brighten_intense = if let Some(brighten) = brighten {
        brighten
    } else {
        1
    };

    save_image(img.brighten(brighten_intense), outfile);
}

pub fn crop(infile: &String, outfile: &String, crop: Crop) {
    let mut img = get_image(infile);

    save_image(img.crop(crop.x, crop.y, crop.width, crop.height), outfile);
}

pub fn out_of_param_error(param: &str, command: &str) -> String {
    let mut str = String::from(param);

    str.push_str(" parameter not found, but it's required for ");
    str.push_str(command);
    str.push_str(" command");

    str
}