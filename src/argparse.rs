#![allow(dead_code)]
use std::env::Args;

pub enum Type {
    Audio(String),
    Video(String),
    Image(String)
}

pub enum ActionType {
    Embedd,
    Uproot
}

pub struct Command {
    pub filetype: Type,
    pub action: ActionType,
    pub output: String,
    pub header: String
}

pub fn parse_args(mut args: Args) -> Option<Command> {
    let prog_name = args
        .next()
        .unwrap_or("stego".to_string());

    let mut args = args.peekable();

    if let Some(first) = args.peek() {
        if *first == "--help" || *first == "-h" {
            show_help(&prog_name);
            return None;
        }

        if *first == "--version" || *first == "-v" {
            show_version();
            return None;
        }
    }
    
    let mut file_type: Option<Type> = None;
    let mut action: Option<ActionType> = None;
    let mut output: Option<String> = None;
    let mut header: String = String::from("stego");

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--embedd" => {
                action = Some(ActionType::Embedd);
            },

            "--uproot" => {
                action = Some(ActionType::Uproot);
            },

            "--header" => {
                if let Some(h) = args.next() {
                    header = h;
                } else {
                    println!("[!] Error: expected header after --header\nUsing default header");
                }
            },

            "--image" => {
                if let Some(src) = args.next() {
                    file_type = Some(Type::Image(src));      
                } else {
                    println!("[!] Error: expected filename after --image");
                    return None;
                }
            },
            
            "--video" => {
                if let Some(src) = args.next() {
                    file_type = Some(Type::Video(src));      
                } else {
                    println!("[!] Error: expected filename after --video");
                    return None;
                }
            },

            "--audio" => {
                if let Some(src) = args.next() {
                    file_type = Some(Type::Audio(src));      
                } else {
                    println!("[!] Error: expected filename after --audio");
                    return None;
                }
            },

            "-o" | "--output" => {
                if let Some(f) = args.next() {
                    output = Some(f);
                } else {
                    println!("[!] Error: expected filename after --output");
                    return None;
                }
            },

            unknown => {
                println!("[!] Error: Invalid command '{}'", unknown);
                return None;
            }
        }
    }
    
    if let None = output {
        println!("[!] Please enter a output file.\nTry '{} --help' for more information.", prog_name);
        return None;
    }

    Some(Command {
        filetype: file_type.unwrap(),
        action: action.unwrap(),
        output: output.unwrap(),
        header: header
    })
}

fn show_help(prog_name: &str) {
    println!(
        "Usage: {} [--embedd | --uproot] [--image FILE | --audio FILE | --video FILE] -o OUTPUT [--header STRING]",
        prog_name
    );
    println!();
    println!("Options:");
    println!("  --image <FILE>       Input image file");
    println!("  --audio <FILE>       Input audio file");
    println!("  --video <FILE>       Input video file");
    println!("  --embedd             Embed the data");
    println!("  --uproot             Extract the data");
    println!("  -o, --output <FILE>  Output file path");
    println!("  --header <DATA>      Optional header string to embed");
    println!("  --help               Show this help message");
    println!("  --version            Show version info");
}

fn show_version() {
    println!("{} v{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION"));
}
