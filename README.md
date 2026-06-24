
# file2video

A terminal tool written in [Rust](https://github.com/rust-lang/rust.git) capable of turning any given file into a video by using it to build raw audio and video streams.

Inspired by [Binary Waterfall](https://github.com/nimaid/binary-waterfall.git).

## Installation

The easiest way to install is via Cargo:

1. Rust and Cargo need to be installed. If they aren't already, it can be done at https://rustup.rs/.
2. Execute the following command, which will automatically download and compile the tool.

```bash
  cargo install file2video
```

You can also install from source by cloning the repository (this required Git to be installed):

```bash
  git clone https://github.com/FeatheredOrbit/file2video.git
```

Moving into the created directory:

```bash
  cd file2video
```

And building the executable (this also required Rust and Cargo to be installed):

```bash
  cargo build --release
```

## Usage

A basic use of the tool would look like this:

```bash
file2video path/to/file
```

Where `path/to/file` can be either a relative or absolute path. 
The resulting `output.mp4` can be found in the same folder as the input file.

But of course it doesn't stop there! A use such as the example above will take use of many default flags, displayed below:

- 44.1kHz for sample rate.
- 8bit unsigned integer for bit depth.
- 2 audio channels.
- RGB for color format.
- 256 by 144 pixels for frame resolution.

## Flags

Naturally you can override the defaults using different flags!

### `--sample-rate` / `--s:r`
Allows you to choose the sample rate of the audio stream. The stream is normalized to 44.1kHz, meaning higher sample rates will result in faster audio, and smaller sample rates will result in slower audio. The paired value must be an integer and can't be negative.

```bash
file2video path/to/file --sample-rate 96000
```

### `--sample-format` / `--s:f`
Allows you to choose what format will be chosen to interpret the stream as. 

| Argument | Format |
| :--- | :--- |
| **u8** | 8 bits unsigned integer |
| **u16** | 16 bits unsigned integer |
| **u24** | 24 bits unsigned integer |
| **u32** | 32 bits unsigned integer |
| **u40** | 40 bits unsigned integer |
| **u48** | 48 bits unsigned integer |
| **u56** | 56 bits unsigned integer |
| **u64** | 64 bits unsigned integer |
| **i8** | 8 bits signed integer |
| **i16** | 16 bits signed integer |
| **i24** | 24 bits signed integer |
| **i32** | 32 bits signed integer |
| **i40** | 40 bits signed integer |
| **i48** | 48 bits signed integer |
| **i56** | 56 bits signed integer |
| **i64** | 64 bits signed integer |
| **f32** | 32 bits floating point |
| **f64** | 64 bits floating point |

```bash
file2video path/to/file --sample-format u32
```

### `--channels` / `--ch`
Allows you to set the channels used for the audio stream. The paired value must be an integer between 1 and 8.

```bash
file2video /path/to/file --channels 4
```

### `--color-format` / `--c:f`
Allows you to set the color format to use for the video stream.

| Argument | Format |
| :--- | :--- |
| **rgb** | Standard RGB |
| **hsv** | HSV |
| **hsl** | HSL |
| **yuv** | YUV |
| **ycbcr** | YCbCr |

```bash
file2video path/to/file --color-format yuv
```

### `--resolution` / `--res`
Allows you to set the resolution for the frames of the video stream. The value must be in format `widthxheight`, where width and height are even integers between 16 and 4096.

```bash
file2video path/to/file --resolution 150x150
```

### Additional Modifiers

* **`--reverse-video` / `--r:v`**  
  Swaps the order of the pixels in the video stream, making the video play backwards and upside down.
* **`--reverse-audio` / `--r:a`**  
  Swaps the order of the samples in the audio stream, making the audio play backwards.
* **`--reverse-bytes-video` / `--rb:v`**  
  Swaps the color channels of each pixel around, changing how the color is presented.
* **`--reverse-bytes-audio` / `--rb:a`**  
  Swaps the order of how bytes are used to create higher bit depth formats. For example, a `u32` would normally be built in this order `[byte1, byte2, byte3, byte4]`, but by adding this flag it will instead be built like this `[byte4, byte3, byte2, byte1]`.

## Help

All of the above shown flags, including what arguments can be passed to them, can also be seen by using the following flag:

```bash
file2video --help
```

Like this you won't need to come back here every time! 👍👍👍
