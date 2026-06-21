use std::error::Error;
use clap::CommandFactory;
use crate::{args::Args, audio, colors, video};

pub fn process(args: Args) -> Result<(), Box<dyn Error>> {

    let bytes = to_bytes(&args);

    let audio_bytes = audio::process(&args, &bytes)?;
    let color_bytes = colors::process(&args, &bytes);

    video::process(&args, color_bytes, audio_bytes)

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

