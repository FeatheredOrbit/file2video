use std::{env::temp_dir, error::Error, fs::{File, remove_file}, io::Write};

use crate::{args::Args, constants::NORMALIZED_SAMPLE_RATE};

pub fn process(args: &Args, pixel_bytes: Vec<u8>, audio_bytes: Vec<u8>) -> Result<(), Box<dyn Error>> {

    let temp_dir = temp_dir();

    let temp_audio_file_path = temp_dir.join("feathered-orbit-file2video.audio");
    let temp_pixel_file_path = temp_dir.join("feathered-orbit-file2video.pixels");

    // Create temporary files and dump the bytes into them.
    let mut temp_audio_file = File::create(temp_audio_file_path.clone())?;
    temp_audio_file.write_all(&audio_bytes)?;

    let mut temp_video_file = File::create(temp_pixel_file_path.clone())?;
    temp_video_file.write_all(&pixel_bytes)?;

    let parent_folder = args.input_file.parent().ok_or("Failed to access parent folder")?;

    // Unwrapping file_name() should be fine as files can't not have names (at least Google says so), 
    // and it's already validated at the start if the path leads to a file. 
    let output_file_name = format!("video_version_of_{}.mp4", args.input_file.file_name().unwrap().to_string_lossy());

    let total_samples = (audio_bytes.len() / size_of::<f64>()) as f64;
    let audio_duration = total_samples / args.sample_rate as f64 / args.channels as f64;

    let number_of_frames = pixel_bytes.len() as f64 / (args.resolution.0 * args.resolution.1 * 3) as f64;
    let fps = number_of_frames / audio_duration;

    let mut command = ffmpeg_sidecar::command::FfmpegCommand::new();

    // Video stuff.
    command.format("rawvideo");
    command.pix_fmt("rgb24");
    command.size(args.resolution.0, args.resolution.1);
    command.rate(fps as f32);
    command.input(temp_pixel_file_path.as_os_str().to_string_lossy());

    // Audio stuff.
    command.format("f64le");
    command.args(["-ac", &args.channels.to_string()]);
    command.args(["-ar", &NORMALIZED_SAMPLE_RATE.to_string()]);
    command.input(temp_audio_file_path.as_os_str().to_string_lossy());

    // Idk stuff.
    command.codec_video("libx264");
    command.pix_fmt("yuv420p");
    command.codec_audio("aac");
    command.overwrite();
    command.output(parent_folder.join(output_file_name).as_os_str().to_string_lossy());

    
    for event in command.spawn()?.iter()?.filter_errors() {
        println!("{:?}", event);
    }

    // Remove the temporary files and then exit.
    remove_file(temp_audio_file_path)?;
    remove_file(temp_pixel_file_path)?;

    Ok(())
}