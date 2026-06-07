use clap::Parser;

mod args;
use args::Args;

fn main() {
    let args = Args::parse();

    println!("{}", args.input_file.exists());

    println!("{:?}", args.input_file);
    println!("{:?}", args.sample_type);
    println!("{:?}", args.sample_rate);
    println!("{:?}", args.channels);
}
