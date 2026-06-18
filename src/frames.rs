use crate::args::{Args, ColorFormat};

fn to_color(args: &Args, bytes: &Vec<u8>) -> Vec<u8> {

    match args.color_format {
        ColorFormat::Rgb => {

            bytes.chunks_exact(3).flatten().copied().collect()

        }
    }
}

pub fn to_frames(args: &Args, bytes: &Vec<u8>) -> Vec<u8> {

    let color = to_color(args, bytes);

    return color;

}