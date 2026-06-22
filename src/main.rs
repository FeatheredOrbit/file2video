use clap::{CommandFactory, Parser};

mod args;
mod process;
mod audio;
mod colors;
mod video;
mod constants;

use args::Args;

fn main() {
    if let Err(_) = ffmpeg_sidecar::download::auto_download() {
        Args::command().error(
        clap::error::ErrorKind::Io, 
        "An error has occurred while trying to install ffmpeg (which is required to encode the video blablabla). Please check your internet connection, or maybe just install the library yourself. Have fun!."
        ).exit();
    }

    let args = Args::parse();
    
    if let Err(e) = process::process(args) {

        Args::command().error(
            clap::error::ErrorKind::Io, 
            e
        ).exit();

    }

    println!("File successfully converted!");

}
