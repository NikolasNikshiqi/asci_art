#![allow(unused_variables)]

use image::{RgbaImage, imageops::FilterType};
use termsize::Size;

struct Args {
    path: String,
}
fn main() {

    let cmd: Vec<String> = std::env::args().collect();

    if cmd.len() < 2 {
        panic!("Usage: program <file path>");
    }
    let args = Args {
        path: cmd[1].to_string()
    };

    let mut output = String::new();
    let asci_values: [char; 9] = ['@','&', '$', 'x', '=', '+', ':', '.', ' '];
    let img = image::open(args.path).unwrap();
    let gray_img = img.grayscale();

    //Get terminal dimensions
    let (term_height, term_width) = match termsize::get() {
        Some(Size { rows, cols }) => {
            println!("{} {}",rows,cols);
            (rows as u32, cols as u32)
        }
        _ => (100, 100),
    };

    let resized_gimage = gray_img.resize_exact(term_width , term_height, FilterType::Nearest);
    let buffer: RgbaImage = resized_gimage.to_rgba8();

    for (count, row_pixels) in buffer.rows().enumerate() {
        for pixel in row_pixels {
            let r = pixel[0] as u32;
            let g = pixel[1] as u32;
            let b = pixel[2] as u32;

            let average_brightness: u32 = (r + g + b) / 3;
            let index: usize = (average_brightness / 31) as usize;
            output.push(asci_values[index]);
        }
        output.push('\n');
    }
    println!("{}", output);
}
