use image::ImageReader;
use std::io::Result;
use crate::argparse;

fn parse_img(command: argparse::Command) -> Result<(), String> {
    let mut path: String;
    if let argparse::Type::Image(x) = command.filetype {
        path = x;
    }

    let img = match ImageReader::open(path)?.decode() {
        Ok(x) => x,
        Err(x) => return Err(format!("Error: {}", x))
    };

    Ok(())
}
