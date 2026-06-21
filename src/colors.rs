use crate::args::{Args, ColorFormat};

pub fn process(args: &Args, bytes: &Vec<u8>) -> Vec<u8> {

    // Interpret bytes as different color formats, then normalize to RGB. WHy are there so many color formats.
    let mut colors: Vec<u8> = match args.color_format {
        ColorFormat::Rgb => {

            bytes.chunks_exact(3).flatten().copied().collect()

        }

        ColorFormat::Hsl => {

            // All the formula was gotten from Google and I just converted it who even knows what is HSL.
            bytes.chunks_exact(3).flat_map(|chunk| {

                let hue = chunk[0] as f32 * (360.0 / 255.0);
                let saturation = chunk[1] as f32 / u8::MAX as f32;
                let lightness = chunk[2] as f32 / u8::MAX as f32;

                let chroma = saturation * (1.0 - (2.0 * lightness - 1.0).abs());
                let segment = hue / 60.0;
                let secondary_component = chroma * (1.0 - ((segment % 2.0) - 1.0).abs());
                let value_matching = lightness - chroma / 2.0;

                // Whatever this is.
                let temporary_rgb = match (segment.floor() % 6.0) as u32 {

                    0 => (chroma, secondary_component, 0.0 as f32),
                    1 => (secondary_component, chroma, 0.0 as f32),
                    2 => (0 as f32, chroma, secondary_component),
                    3 => (0 as f32, secondary_component, chroma),
                    4 => (secondary_component, 0 as f32, chroma),
                    5 => (chroma, 0 as f32, secondary_component),
                    
                    _ => { unreachable!() }

                };

                [
                    ((temporary_rgb.0 + value_matching) * 255.0) as u8,
                    ((temporary_rgb.1 + value_matching) * 255.0) as u8,
                    ((temporary_rgb.2 + value_matching) * 255.0) as u8
                ]

            }).collect()

        }

        ColorFormat::Hsv => {

            // This was also gotten from Google and I converted to code.
            bytes.chunks_exact(3).flat_map(|chunk| {

                let hue = chunk[0] as f32 * (360.0 / 255.0);
                let saturation = chunk[1] as f32 / u8::MAX as f32;
                let value = chunk[2] as f32 / u8::MAX as f32;

                let chroma = value * saturation;
                let segment = hue / 60.0;
                let secondary_component = chroma * (1.0 - ((segment % 2.0) - 1.0).abs());
                let value_matching = value - chroma;

                // Whatever this is.
                let temporary_rgb = match (segment.floor() % 6.0) as u32 {

                    0 => (chroma, secondary_component, 0.0 as f32),
                    1 => (secondary_component, chroma, 0.0 as f32),
                    2 => (0 as f32, chroma, secondary_component),
                    3 => (0 as f32, secondary_component, chroma),
                    4 => (secondary_component, 0 as f32, chroma),
                    5 => (chroma, 0 as f32, secondary_component),
                    
                    _ => { unreachable!() }

                };

                [
                    ((temporary_rgb.0 + value_matching) * 255.0) as u8,
                    ((temporary_rgb.1 + value_matching) * 255.0) as u8,
                    ((temporary_rgb.2 + value_matching) * 255.0) as u8
                ]

            }).collect()
        }

        ColorFormat::Yuv => {
            bytes.chunks_exact(3).flat_map(|chunk| {

                let luminance = chunk[0] as f32;
                let blue_factor = chunk[1] as f32;
                let red_factor = chunk[2] as f32;

                let blue_projection = blue_factor - 128.0;
                let red_projection = red_factor - 128.0;

                [
                    (luminance + red_projection * 1.402).clamp(0.0, 255.0) as u8,
                    (luminance - blue_projection * 0.344136 - red_projection * 0.714136).clamp(0.0, 255.0) as u8,
                    (luminance + blue_projection * 1.772).clamp(0.0, 255.0) as u8
                ]

            }).collect()
        }

        ColorFormat::Ycbcr => {
            bytes.chunks_exact(3).flat_map(|chunk| {

                let luminance = chunk[0] as f32;
                let blue_difference = chunk[1] as f32;
                let red_difference = chunk[2] as f32;

                let luminance_shift = luminance - 16.0;
                let blue_shift = blue_difference - 128.0;
                let red_shift = red_difference - 128.0;

                [
                    (luminance_shift * 1.164 + red_shift * 1.596).clamp(0.0, 255.0) as u8,
                    (luminance_shift * 1.164 - blue_shift * 0.392 - red_shift * 0.813).clamp(0.0, 255.0) as u8,
                    (luminance_shift * 1.164 + blue_shift * 2.017).clamp(0.0, 255.0) as u8
                ]

            }).collect()
        }
    };

    // Reverse the video.
    if args.reverse_video { colors.reverse(); }

    let resolution = args.resolution.0 as usize * args.resolution.1 as usize;
    let channels_per_frame = resolution * 3;

    let remainder = colors.len() % channels_per_frame;
    let padding = channels_per_frame - remainder;

    colors.resize(colors.len() + padding, 0);

    colors
}