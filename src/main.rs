use clap::Parser;

mod args;
mod process;
mod audio;
mod colors;
mod video;
mod constants;

use args::Args;

fn main() {
    ffmpeg_sidecar::download::auto_download().unwrap();

    let args = Args::parse();

    // Eventually I'll handle errors.
    let _ = process::process(args);

}
