use crate::args::{Args, ColorFormat};

pub fn process(args: &Args, bytes: &Vec<u8>) -> Vec<u8> {

    let mut colors: Vec<u8> = match args.color_format {
        ColorFormat::Rgb => {

            bytes.chunks_exact(3).flatten().copied().collect()

        }
    };

    let resolution = args.resolution.0 as usize * args.resolution.1 as usize;
    let channels_per_frame = resolution * 3;

    let remainder = colors.len() % channels_per_frame;
    let padding = channels_per_frame - remainder;

    colors.resize(colors.len() + padding, 0);

    colors
}