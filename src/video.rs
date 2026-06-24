use std::{env::temp_dir, fs::{File, remove_file}, io::Write};

use crate::{args::Args, misc::NORMALIZED_SAMPLE_RATE};

pub fn process(args: &Args, pixel_bytes: Vec<u8>, audio_bytes: Vec<u8>) -> crate::misc::Result<()> {

    let temp_dir = temp_dir();

    let temp_audio_file_path = temp_dir.join("feathered-orbit-file2video.audio");
    let temp_pixel_file_path = temp_dir.join("feathered-orbit-file2video.pixels");

    // Create temporary files and dump the bytes into them.
    let mut temp_audio_file = File::create(temp_audio_file_path.clone()).map_err(|_| "Failed to create temporary audio file.")?;
    temp_audio_file.write_all(&audio_bytes).map_err(|_| "Failed to write to temporary audio file.")?;

    let mut temp_video_file = File::create(temp_pixel_file_path.clone()).map_err(|_| "Failed to create temporary pixel file.")?;
    temp_video_file.write_all(&pixel_bytes).map_err(|_| "Failed to write to temporary pixel file.")?;

    let parent_folder = args.input_file.parent().ok_or("Failed to access parent folder.")?;

    let total_samples = (audio_bytes.len() / size_of::<f64>()) as f64;
    let audio_duration = total_samples / NORMALIZED_SAMPLE_RATE as f64 / args.channels as f64;

    // Calculate the appropriate fps so that video and audio finish together.
    let number_of_frames = pixel_bytes.len() as f64 / (args.resolution.0 * args.resolution.1 * 3) as f64;
    let fps = number_of_frames / audio_duration;

    let mut command = ffmpeg_sidecar::command::FfmpegCommand::new();

    // Video stuff.
    command.format("rawvideo");
    command.pix_fmt("rgb24");
    command.size(args.resolution.0, args.resolution.1);
    command.rate(fps as f32);
    command.input(temp_pixel_file_path.to_string_lossy());

    // Audio stuff.
    command.format("f64le");
    command.args(["-ac", &args.channels.to_string()]);
    command.args(["-ar", &NORMALIZED_SAMPLE_RATE.to_string()]);
    command.input(temp_audio_file_path.to_string_lossy());

    // Idk stuff.
    command.codec_video("libx264");
    command.pix_fmt("yuv420p");
    command.codec_audio("aac");
    command.overwrite();
    command.output(parent_folder.join("output.mp4").to_string_lossy());

    command.spawn().map_err(|_| "An error has occurred while trying to execute FFmpeg, which is what encodes the video. Please make sure that it is installed and on PATH.")?
        .wait().map_err(|_| "FFmpeg, which is what encodes the video, exited abnormally. I have no idea what would cause this to happen so you are on your own honestly.")?;

    // Remove the temporary files and then exit.
    remove_file(temp_audio_file_path).map_err(|_| "Failed to remove temporary audio file. It might have been deleted already, or if not, you are free to delete it yourself. 👍")?;
    remove_file(temp_pixel_file_path).map_err(|_| "Failed to remove temporary pixel file. It might have been deleted already, or if not, you are free to delete it yourself. 👍")?;

    Ok(())
}
