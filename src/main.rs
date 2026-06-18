use clap::Parser;

mod args;
mod process;
mod audio;
mod frames;

use args::Args;

fn main() {
    ffmpeg_sidecar::download::auto_download().unwrap();

    let args = Args::parse();

    process::process(args);

}
