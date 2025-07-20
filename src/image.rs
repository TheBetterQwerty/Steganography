use std::io::{self, Write};
use image::RgbImage;
use image::ImageReader;
use image::ImageBuffer;
use std::result::Result;
use crate::{argparse, encrypt};

const NONCE_SIZE: usize = 12;
const SHA_LEN: usize = 32;

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
    
    println!("[#] Image Loaded:");
    println!("   • Dimensions : {} x {}\n", width, height);

    let key = encrypt::hash(input("[+] Enter key for encryption\n> ").as_bytes());
    let nonce = encrypt::rand_bytes();

    let header = encrypt::hash(command.header.as_bytes());
    let data = match encrypt::data_enc(&key, &input("[+] Enter data to embed\n> ").into_bytes(), &nonce) {
        Ok(data) => data,
        Err(data) => return Err(data)
    };
    let end_header: Vec<u8> = header.iter().rev().cloned().collect();
    
    let mut final_data: Vec<u8> = Vec::new();
    final_data.extend_from_slice(&header);
    final_data.extend_from_slice(&nonce);
    final_data.extend_from_slice(&data);
    final_data.extend_from_slice(&end_header);

    if final_data.len() > pixels.len() {
        return Err("Picture bytes to small".to_string());
    }

    for i in 0..final_data.len() {
        pixels[i][0] = final_data[i];
    }
    
    let flat_pixels: Vec<u8> = pixels
        .iter()
        .flat_map(|rgb| rgb)
        .copied()
        .collect();
    
    let output_img: RgbImage = match ImageBuffer::from_raw(width, height, flat_pixels) {
        Some(x) => x,
        None => return Err("Invalid image buffer size".to_string())
    };
    
    let output_path = match command.output {
        Some(x) => x,
        None => return Err(String::from(""))
    };

    match output_img.save(&output_path) {
        Ok(_) => println!("[+] Image was created and saved at {}", output_path),
        Err(err) => return Err(err.to_string())
    }

    Ok(())
}

pub fn image_uproot(command: argparse::Command) -> Result<String, String> {
    let path = match command.filetype {
        argparse::Type::Image(path) => path,
        _ => return Err("Path doesn't exist".to_string())
    };
    
    let img = ImageReader::open(&path)
        .map_err(|e| e.to_string())?
        .decode()
        .map_err(|e| e.to_string())?
        .to_rgb8();
    
    let (width, height) = img.dimensions();
    println!("[#] Image Loaded");
    println!("   • Path       : {}", path);
    println!("   • Dimensions : {} x {}\n", width, height);
    
    let pixels: Vec<u8> = img
        .pixels()
        .map(|p| {
            let a = p.0;
            a[0]
        }).collect();
    
    let key = encrypt::hash(input("[+] Enter the key for decryption\n> ").as_bytes());
    let header = encrypt::hash(command.header.as_bytes());
    let end_header: Vec<u8> = header.iter().rev().copied().collect();
    
    let header_idx = match find_subarray(&pixels, &header) {
        Some(idx) => idx,
        None => return Err("Header doesnt exists in the image".to_string())
    };

    let end_header_idx = match find_subarray(&pixels, &end_header) {
        Some(idx) => idx,
        None => return Err("End Header doesnt exists in the image".to_string())
    };
    
    if end_header_idx <= header_idx + SHA_LEN + NONCE_SIZE {
        return Err("Invalid data range in image".to_string());
    }

    let nonce = &pixels[(header_idx + SHA_LEN) .. (header_idx + SHA_LEN + NONCE_SIZE)];
    let data = &pixels[(header_idx + SHA_LEN + NONCE_SIZE) .. (end_header_idx)];

    encrypt::data_dec(&key, data, nonce)
}

fn find_subarray(array: &[u8], subarray: &[u8]) -> Option<usize> {
    if array.len() < subarray.len() {
        return None;
    }

    if array.len() == 0 || subarray.len() == 0 {
        return None;
    }

    for i in 0..array.len() {
        let mut j = 0usize;
        while j < subarray.len() {
            if array[i + j] != subarray[j] {
                break;
            }
            j += 1;
        }

        if j == subarray.len() {
            return Some(i);
        }
    }
    
    None
}

fn input(buffer: &str) -> String {
    let mut input = String::new();

    print!("{buffer}");
    let _ = io::stdout().flush();
    io::stdin().read_line(&mut input).expect("[!] Error: Reading from stdin");
    
    input.trim().to_string()
}
