use clap::Parser;

mod args;
mod process;
mod audio;
mod colors;
mod video;
mod constants;

use args::Args;

fn main() {
    if let Err(e) = ffmpeg_sidecar::download::auto_download() {

    }

    let args = Args::parse();

    // Eventually I'll handle errors.
    let _ = process::process(args);

}
