use std::path::PathBuf;
use clap::{arg, Parser};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
    /// Need blur image or not
    #[arg(short, long)]
    pub blur: Option<bool>,

    /// intense of blur
    #[arg(long)]
    pub blur_intense: Option<f32>,

    /// input file path
    #[arg(short, long, value_name = "FILE")]
    pub infile: Option<PathBuf>,

    /// output file path
    #[arg(short, long, value_name = "FILE")]
    pub outfile: Option<PathBuf>,

    /// Need brighten image or not
    #[arg(long)]
    pub brighten: Option<bool>,

    /// brighten intense
    #[arg(long)]
    pub brighten_intense: Option<i32>,

    /// need crop image or not
    #[arg(long)]
    pub crop: Option<bool>,

    // cropping start x point
    #[arg(long)]
    pub crop_x: Option<u32>,

    // cropping start u point
    #[arg(long)]
    pub crop_y: Option<u32>,

    // cropping width
    #[arg(long)]
    pub crop_width: Option<u32>,

    // cropping height
    #[arg(long)]
    pub crop_height: Option<u32>,
}

impl Args {
    pub fn new() -> Self {
        Args::parse()
    }
}
