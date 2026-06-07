use clap::{CommandFactory, Parser};

mod args;
mod process;

use args::Args;

fn main() {
    if !ffmpeg_is_installed() {

        Args::command()
            .error(clap::error::ErrorKind::Io, "ffmpeg is not installed on the device.")
            .exit()

    }

    let args = Args::parse();

    process::process(args);

}

fn ffmpeg_is_installed() -> bool {

    std::process::Command::new("ffmpeg")
        .arg("-version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)

}
