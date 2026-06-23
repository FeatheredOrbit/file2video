
# file2video

A terminal tool capable of turning any given file into a video by using it to build raw audio and video streams.

Inspired by [Binary Waterfall](https://github.com/nimaid/binary-waterfall.git).


## Installation

The easiest way to install would be via Cargo:

1. Rust and Cargo need to be installed. If they aren't already, it can be done at https://rustup.rs/.
2. Execute the following command, which will automatically download and compile the tool.

```bash
  cargo install file2video
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

- `--sample-rate` / `--s:r` : Allows you to choose the sample rate of the audio stream. The stream is normalized to 44.1kHz, meaning higher sample rates will result in faster audio, and smaller sample rates will result in slower audio.
The paired value must be an integer and can't be negative.

```bash
file2video path/to/file --sample-rate 96000
```
---
- `--sample-format` / `--s:f` : Allows you to choose what format will be chosen to interpret the stream as. There are many formats to choose from, shown below:

<div style="display: flex; justify-content: center;">
  <table border="1" cellpadding="5" cellspacing="0" style="border-collapse: collapse; text-align: center;">
    <thead>
      <tr>
        <th>Argument</th>
        <th>Format</th>
      </tr>
    </thead>
    <tbody>
      <tr><td>u8</td><td>8bits unsigned integer</td></tr>
      <tr><td>u16</td><td>16bits unsigned integer</td></tr>
      <tr><td>u24</td><td>24bits unsigned integer</td></tr>
      <tr><td>u32</td><td>32bits unsigned integer</td></tr>
      <tr><td>u40</td><td>40bits unsigned integer</td></tr>
      <tr><td>u48</td><td>48bits unsigned integer</td></tr>
      <tr><td>u56</td><td>56bits unsigned integer</td></tr>
      <tr><td>u64</td><td>64bits unsigned integer</td></tr>
      <tr><td>i8</td><td>8bits signed integer</td></tr>
      <tr><td>i16</td><td>16bits signed integer</td></tr>
      <tr><td>i24</td><td>24bits signed integer</td></tr>
      <tr><td>i32</td><td>32bits signed integer</td></tr>
      <tr><td>i40</td><td>40bits signed integer</td></tr>
      <tr><td>i48</td><td>48bits signed integer</td></tr>
      <tr><td>i56</td><td>56bits signed integer</td></tr>
      <tr><td>i64</td><td>64bits signed integer</td></tr>
      <tr><td>f32</td><td>32bits floating point</td></tr>
      <tr><td>f64</td><td>64bits floating point</td></tr>
    </tbody>
  </table>
</div>

```bash
file2video path/to/file --sample-format u32
```
---
- `--channels` / `--ch` : Allows you to set the channels used for the audio stream. The paired value must be an integer between 1 and 8.

```bash
file2video /path/to/file --channels 4
```
---
- `--color-format` / `--c:f` : Allows you to set the color format to use for the video stream.

<div style="display: flex; justify-content: center;">
  <table border="1" cellpadding="5" cellspacing="0" style="border-collapse: collapse; text-align: center;">
    <thead>
      <tr>
        <th>Argument</th>
        <th>Format</th>
      </tr>
    </thead>
    <tbody>
      <tr><td>rgb</td><td>Standard RGB</td></tr>
      <tr><td>hsv</td><td>HSV</td></tr>
      <tr><td>hsl</td><td>HSL</td></tr>
      <tr><td>yuv</td><td>YUV</td></tr>
      <tr><td>ycbcr</td><td>YCbCr</td></tr>
    </tbody>
  </table>
</div>

```bash
file2video path/to/file --color-format yuv
```
---
- `--resolution` / `--res` : Allows you to set the resolution for the frames of the video stream. The value must be in format `widthxheight`, where `width` and `height` are even integers between 16 and 4096.

```bash
file2video path/to/file --resolution 150x150
```

---

There are also additional flags, such as:

- `--reverse-video` / `--r:v` : Adding this flag will swap the order of the pixels in the video stream, making the video play backwards and upside down.

- `--reverse-audio` / `--r:a` : Adding this flag will swap the order of the samples in the audio stream, making the audio play backwards.

- `--swap-bytes-video` / `--sb:v` : Adding this flag will swap the color channels of each pixel around, changing how the color is presented.

- `--swap-bytes-audio` / `--sb:a` : Adding this flag will swap the order of how bytes are used to create higher bitdepth formats. For example, a u32 would normally be built in this order `[byte1, byte2, byte3, byte4]`, but by adding this flag it will instead be built like this `[byte4, byte3, byte2, byte1]`.

## Help

Naturally I don't expect people to just memorize all of these options, this is why it is also possible to use the following flag:
```bash
file2video --help
```
This flag will display every flag used by my tool, including the values paired with the flag.


