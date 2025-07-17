use std::io::{self, Write};
use image::RgbImage;
use image::ImageReader;
use image::ImageBuffer;
use std::result::Result;
use crate::argparse;

pub fn image_parse(command: argparse::Command) -> Result<(), String> {
    let path = match command.filetype {
        argparse::Type::Image(path) => path,
        _ => return Err("[!] Error: path doesn't exist".to_string())
    };
    
    let img = ImageReader::open(path)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?
        .to_rgb8();
    
    let (width, height) = img.dimensions();

    let mut pixels: Vec<[u8; 3]> = img
        .pixels()
        .map(|p| p.0)
        .collect();

    let data = input("[+] Enter data : ").into_bytes();
    let header = command.header.as_bytes();
    let end_header = header
        .iter()
        .rev()
        .cloned()
        .collect::<Vec<u8>>();
    
    
    /*
    * f(x) -> enc
    * g(x) -> hash
    * g(header) + f(data) + g(header[::-1])
    *
    * uproot:
    *   f(data)
    *   find in data
    * */
    let mut final_data: Vec<u8> = Vec::new();
    final_data.extend_from_slice(&data);
    final_data.extend_from_slice(header);
    final_data.extend_from_slice(&end_header);
    
    for i in 0..final_data.len() {
        if i >= pixels.len() {
            continue;
        }
        pixels[i][0] = final_data[i];
    }
    
    let flat_pixels: Vec<u8> = pixels.iter().flat_map(|rgb| rgb).copied().collect();

    let output_img: RgbImage = ImageBuffer::from_raw(width, height, flat_pixels)
        .expect("[!] Error: Invalid image buffer size");

    output_img.save(command.output).expect("[!] Error: Failed to save image");
    Ok(())
}

fn input(buffer: &'static str) -> String {
    let mut input = String::new();

    print!("{buffer}");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("[!] Error: Reading from stdin");
    
    input.trim().to_string()
}
