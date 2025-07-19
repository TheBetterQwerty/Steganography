use std::env::args;

mod argparse;
mod image;
mod encrypt;

fn main() {
    let arguments = match argparse::parse_args(args()) {
        Some(arguments) => {
            arguments
        },
        None => {
            return;
        }
    };
    
    match arguments.filetype {
        argparse::Type::Image(_) => {
            match arguments.action {
                argparse::ActionType::Embedd => {
                    match image::image_embedd(arguments) {
                        Ok(()) => {},
                        Err(x) => {
                            println!("[!] Error: {x}");
                            return;
                        }
                    }
                },

                argparse::ActionType::Uproot => {
                    match image::image_uproot(arguments) {
                        Ok(x) => println!("[MESSAGE] {x}"),
                        Err(x) => {
                            println!("[!] Error: {x}");
                            return;
                        }
                    }
                },
            }
        },

        _ => {} // make later
    }
}
