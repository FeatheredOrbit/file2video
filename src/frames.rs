use crate::args::{Args, ColorFormat};

pub fn process(args: &Args, bytes: &Vec<u8>) -> Vec<u8> {

    let mut colors: Vec<u8> = match args.color_format {
        ColorFormat::Rgb => {

            bytes.chunks_exact(3).flatten().copied().collect()

        }
    };

    let resolution = args.resolution.0 * args.resolution.1;
    let channels_per_frame = resolution as f32 * 3.0;

    let frames = (colors.len() as f32 / channels_per_frame).ceil();

    colors.resize((frames * channels_per_frame) as usize, 0);

    colors
}