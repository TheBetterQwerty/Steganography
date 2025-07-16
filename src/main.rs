use std::env::args;

mod argparse;
mod image;

fn main() {
    let arguments = match argparse::parse_args(args()) {
        Some(arguments) => arguments,
        None => return
    };
    
    match arguments.filetype {
        argparse::Type::Image(_) => {
            image::image_parse(arguments);
        },

        _ => {} // make later
    }
}
