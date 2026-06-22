use crate::{args::Args, audio, colors, video};

pub fn process(args: Args) -> crate::misc::Result<()> {

    let bytes = to_bytes(&args)?;

    let audio_bytes = audio::process(&args, &bytes)?;
    let color_bytes = colors::process(&args, &bytes);

    video::process(&args, color_bytes, audio_bytes)

}

fn to_bytes(args: &Args) -> crate::misc::Result<Vec<u8>> {

    std::fs::read(args.input_file.clone()).map_err(|_| format!("Failed to read file at path: `{}`", args.input_file.display()).into())

}

