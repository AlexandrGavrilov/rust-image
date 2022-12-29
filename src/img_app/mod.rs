mod utils;
mod args;

use utils::Crop;
use args::Args;

pub fn get_matches() {
    let args = Args::new();

    if let (
        Some(infile_path),
        Some(outfile_path),
    ) = (
        args.infile.as_deref(),
        args.outfile.as_deref(),
    ) {
        let infile = &infile_path.display().to_string();
        let outfile = &outfile_path.display().to_string();

        println!("Value for infile_path: {}", infile);
        println!("Value for outfile_path: {}", infile);

        if let Some(blur) = args.blur {
            println!("Value for blur: {}", blur);
            let blur_intense = args.blur_intense;
            utils::blur_image(infile, outfile, blur_intense);
        } else {
            println!("You can pass blur and blur_intense params to blur your image")
        }

        if let Some(brighten) = args.brighten {
            println!("Value for brighten: {}", brighten);
            let brighten_intense = args.brighten_intense;
            utils::brighten_image(infile, outfile, brighten_intense);
        } else {
            println!("You can pass brighten and brighten_intense params to blur your image")
        }

        if let Some(crop) = args.crop {
            let (
                x,
                y,
                width,
                height
            ) = get_crop_args(&args);

            println!("Value for crop {}", crop);
            println!("Crop params is: x - {:?}, y - {:?}, width - {:?}, height - {:?}", x, y, width, height);
            utils::crop(infile, outfile, Crop::new(x, y, width, height));
        } else {
            println!("You can pass crop param to crop your image, crop-x, crop-y, crop-width, crop-height is required")
        }
    } else {
        println!("No files paths found =(. Please pass infile and outfile parameters")
    }
}

fn get_crop_args(args: &Args) -> (u32, u32, u32, u32) {
    (
        if let Some(x) = args.crop_x { x } else { panic!("{}", utils::out_of_param_error("crop-x", "crop")); },
        if let Some(y) = args.crop_y { y } else { panic!("{}", utils::out_of_param_error("crop-y", "crop")); },
        if let Some(width) = args.crop_width { width } else { panic!("{}", utils::out_of_param_error("crop-width", "crop")); },
        if let Some(height) = args.crop_height { height } else { panic!("{}", utils::out_of_param_error("crop-height", "crop")); },
    )
}

