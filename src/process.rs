use std::{env::temp_dir, fs::File, io::Write};

use clap::CommandFactory;
use crate::{args::Args, audio, frames};

pub fn process(args: Args) {

    let bytes = to_bytes(&args);

    let audio_bytes = audio::process(&args, &bytes);
    let color_bytes = frames::process(&args, &bytes);

    to_video(&args, color_bytes, audio_bytes);

}

fn to_bytes(args: &Args) -> Vec<u8> {

    let Ok(bytes) = std::fs::read(args.input_file.clone()) else {

        Args::command().error(
            clap::error::ErrorKind::Io,
            format!("Failed to read file at path: `{}`", args.input_file.display())
        ).exit();

    };

    bytes

}

fn to_video(args: &Args, pixel_bytes: Vec<u8>, audio_bytes: Vec<u8>) {

    let temp_dir = temp_dir();
    let temp_audio_file_path = temp_dir.join("audio.raw");
    let temp_pixel_file_path = temp_dir.join("pixels.raw");

    // Unwrap EVERYTHING, what could possibly go wrong.
    let mut temp_audio_file = File::create(temp_audio_file_path.clone()).unwrap();
    temp_audio_file.write_all(&audio_bytes).unwrap();

    let mut temp_video_file = File::create(temp_pixel_file_path.clone()).unwrap();
    temp_video_file.write_all(&pixel_bytes).unwrap();

    let output_file_name = format!("{}_processed", args.input_file.file_name().unwrap().to_str().unwrap());

}