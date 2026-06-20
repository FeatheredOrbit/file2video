use clap::Parser;

mod args;
mod process;
mod audio;
mod colors;
mod video;

use args::Args;

fn main() {
    ffmpeg_sidecar::download::auto_download().unwrap();

    let args = Args::parse();

    let _ = process::process(args);

}
