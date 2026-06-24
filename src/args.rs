use std::path::PathBuf;
use std::str::FromStr;
use clap::{Parser, ValueEnum};

#[derive(ValueEnum, Clone, Debug)]
pub enum SampleFormat {
    U8,
    U16,
    U24,
    U32,
    U40,
    U48,
    U56,
    U64,

    I8,
    I16,
    I24,
    I32,
    I40,
    I48,
    I56,
    I64,

    F32,
    F64
}

#[derive(ValueEnum, Clone, Debug)]
pub enum ColorFormat {
    Rgb,
    Hsv,
    Hsl,
    Yuv,
    Ycbcr
}

#[derive(Parser, Debug)]
#[command(
    name = "file2video",
    author = "Feathered Orbit",
    version = "1.1.2",
    about = "A terminal tool capable of turning any given file into a video by using it to build raw audio and video streams."
)]
pub struct Args {

    #[arg(required = true, value_parser = validate_input_path)]
    pub input_file: PathBuf,

    #[arg(long, aliases = ["s:f"], default_value = "u8")]
    pub sample_format: SampleFormat,

    #[arg(long, aliases = ["s:r"], default_value_t = 44100)]
    pub sample_rate: u32,

    #[arg(long, aliases = ["ch"], default_value_t = 2, value_parser = clap::value_parser!(u8).range(1..=8))]
    pub channels: u8,

    #[arg(long, aliases = ["r:a"])]
    pub reverse_audio: bool,

    #[arg(long, aliases = ["rb:a"])]
    pub reverse_bytes_audio: bool,

    #[arg(long, aliases = ["c:f"], default_value="rgb")]
    pub color_format: ColorFormat,

    #[arg(long, aliases = ["r:v"])]
    pub reverse_video: bool,

    #[arg(long, aliases = ["rb:v"])]
    pub reverse_bytes_video: bool,

    #[arg(long, aliases = ["res"], default_value="256x144", value_parser = validate_resolution)]
    pub resolution: (u32, u32)
}

/// Validates the given path by making sure it's valid, exists, and isn't a folder.
/// Also returns the last valid path if the given one is invalid because I'm so based and UX coded.
fn validate_input_path(s: &str) -> Result<PathBuf, String> {

    let path = PathBuf::from_str(s)
        .map_err(|_| { "Not a path." })?;

    let abs_path = std::path::absolute(path)
        .map_err(|_| { "Failed to resolve absolute path." })?;

    if !abs_path.exists() {
        let mut current_path = PathBuf::new();

        for component in abs_path.components() {

            let last_valid_path = current_path.clone();

            current_path.push(component);

            if !current_path.exists() {

                if last_valid_path.as_os_str().is_empty() {
                    return Err(
                        "Path doesn't exist.".to_string()
                    );
                }

                return Err(
                    format!("Path doesn't exist. Last valid path: `{}`", last_valid_path.display())
                );

            }

        }
    }

    if abs_path.is_dir() {
        return Err(format!("The path {s} leads to a directory and not a file."));
    }

    Ok(abs_path)
}

/// Validates the resolution by splitting into width and height and then converting to u32 and blablabla blebleble.
fn validate_resolution(s: &str) -> Result<(u32, u32), String> {
    let resolution = s.split_once(&['x', 'X'][..]).ok_or("Resolution must be in format widthxheight, where width and height are integer values.")?;

    let width = u32::from_str(resolution.0).map_err(|_| "Resolution must be in format widthxheight, where width and height are integer values.")?;
    let height = u32::from_str(resolution.1).map_err(|_| "Resolution must be in format widthxheight, where width and height are integer values.")?;

    if width % 2 != 0 || height % 2 != 0 {
       return Err("Width and height must be even integers".to_string()); 
    }

    if width < 16 || height < 16 {
        return Err("Width and height can't be smaller than 16".to_string());
    }

    if width > 4096 || height > 4096 {
        return Err("Width and height can't be higher than 4096".to_string());
    }

    Ok((width, height))

}