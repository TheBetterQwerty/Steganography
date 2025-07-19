use std::io::{self, Write};
use image::RgbImage;
use image::ImageReader;
use image::ImageBuffer;
use std::result::Result;
use crate::{argparse, encrypt};

pub fn image_embedd(command: argparse::Command) -> Result<(), String> {
    let path = match command.filetype {
        argparse::Type::Image(path) => path,
        _ => return Err("Path doesn't exist".to_string())
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
    
    let key = encrypt::hash(input("[+] Enter key: ").as_bytes());
    let nonce = encrypt::rand_bytes();

    let data = input("[+] Enter data: ").into_bytes();
    let header = command.header.as_bytes();
    let end_header = header
        .iter()
        .rev()
        .cloned()
        .collect::<Vec<u8>>();
    
    let mut final_data: Vec<u8> = Vec::new();
    {
        final_data.extend_from_slice(header);
        final_data.extend_from_slice(&data);
        final_data.extend_from_slice(&end_header);
    }

    let final_data = match encrypt::data_enc(&key, &final_data, &nonce) {
        Ok(data) => data,
        Err(data) => return Err(data)
    };

    for i in 0..final_data.len() {
        if i >= pixels.len() {
            return Err("Image too small to embedd message".to_string());
        }
        pixels[i][0] = final_data[i];
    }
    
    let flat_pixels: Vec<u8> = pixels
        .iter()
        .flat_map(|rgb| rgb)
        .copied()
        .collect();

    let output_img: RgbImage = ImageBuffer::from_raw(width, height, flat_pixels)
        .expect("Invalid image buffer size");

    match output_img.save(&command.output) {
        Ok(_) => println!("[+] Image was created and saved at {}", command.output),
        Err(err) => return Err(err.to_string())
    }

    Ok(())
}

fn input(buffer: &str) -> String {
    let mut input = String::new();

    print!("{buffer}");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("[!] Error: Reading from stdin");
    
    input.trim().to_string()
}
