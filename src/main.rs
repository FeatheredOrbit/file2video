use clap::Parser;

mod args;
mod process;

use args::Args;

fn main() {
    let args = Args::parse();

    process::process(args);

}
