use clap::CommandFactory;
use dasp_sample::Sample;
use dasp_sample::conv::u64;
use crate::args::{Args, Endianness, SampleFormat};

pub fn process(args: Args) {

    let bytes = to_bytes(&args);

    to_audio(&args, &bytes);

    // IG it reduces memory a bit since this isn't needed anymore.
    drop(bytes);

}

fn to_bytes(args: &Args) -> Vec<u8> {

    let Ok(bytes) = std::fs::read(args.input_file.clone()) else {

        Args::command().error(
            clap::error::ErrorKind::Io,
            format!("Failed to read file at path: `{}`", args.input_file.display())
        ).exit();

    };

    bytes

}

fn to_audio(args: &Args, bytes: &Vec<u8>) -> Vec<f32> {

    let mut resampled_bytes: Vec<f32> = Vec::new();

    match args.sample_format {

        SampleFormat::U8 => {
            return bytes.iter().map(|&byte| {f32::from_sample(byte)}).collect();
        }

        SampleFormat::U16 => {

            for chunk in bytes.chunks(2) {

                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => {
                        u16::from_be_bytes([byte_1, byte_2])
                    },
                    Endianness::Little => {
                        u16::from_le_bytes([byte_1, byte_2])
                    }
                };

                resampled_bytes.push(f32::from_sample(result));

            }

        }

        SampleFormat::U24 => {
            for chunk in bytes.chunks(3) {

                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();

                // Makeshift u24.
                let result = match args.endianness {
                    Endianness::Big => {
                        u32::from_be_bytes([byte_1, byte_2, byte_3, 0])
                    },
                    Endianness::Little => {
                        u32::from_le_bytes([byte_1, byte_2, byte_3, 0])
                    }
                };

                resampled_bytes.push(f32::from_sample(result));

            }
        }

        SampleFormat::U32 => {
            for chunk in bytes.chunks(4) {

                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => {
                        u32::from_be_bytes([byte_1, byte_2, byte_3, byte_4])
                    },
                    Endianness::Little => {
                        u32::from_le_bytes([byte_1, byte_2, byte_3, byte_4])
                    }
                };

                resampled_bytes.push(f32::from_sample(result));

            }
        }

        SampleFormat::U40 => {
            for chunk in bytes.chunks(5) {

                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();
                let byte_5 = chunk.get(4).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => {
                        u64::from_be_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            0,
                            0,
                            0
                        ])
                    },
                    Endianness::Little => {
                        u64::from_le_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            0,
                            0,
                            0
                        ])
                    }
                };

                resampled_bytes.push(f32::from_sample(result));

            }
        }

        SampleFormat::U48 => {
            for chunk in bytes.chunks(6) {

                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();
                let byte_5 = chunk.get(4).unwrap_or(&0).clone();
                let byte_6 = chunk.get(5).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => {
                        u64::from_be_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            byte_6,
                            0,
                            0
                        ])
                    },
                    Endianness::Little => {
                        u64::from_le_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            byte_6,
                            0,
                            0
                        ])
                    }
                };

                resampled_bytes.push(f32::from_sample(result));

            }
        }

        SampleFormat::U56 => {
            for chunk in bytes.chunks(7) {

                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();
                let byte_5 = chunk.get(4).unwrap_or(&0).clone();
                let byte_6 = chunk.get(5).unwrap_or(&0).clone();
                let byte_7 = chunk.get(6).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => {
                        u64::from_be_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            byte_6,
                            byte_7,
                            0
                        ])
                    },
                    Endianness::Little => {
                        u64::from_le_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            byte_6,
                            byte_7,
                            0
                        ])
                    }
                };

                resampled_bytes.push(f32::from_sample(result));

            }
        }

        SampleFormat::U64 => {
            for chunk in bytes.chunks(8) {

                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();
                let byte_5 = chunk.get(4).unwrap_or(&0).clone();
                let byte_6 = chunk.get(5).unwrap_or(&0).clone();
                let byte_7 = chunk.get(6).unwrap_or(&0).clone();
                let byte_8 = chunk.get(7).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => {
                        u64::from_be_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            byte_6,
                            byte_7,
                            byte_8
                        ])
                    },
                    Endianness::Little => {
                        u64::from_le_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            byte_6,
                            byte_7,
                            byte_8
                        ])
                    }
                };

                resampled_bytes.push(f32::from_sample(result));

            }
        }

        SampleFormat::I8 => {
            return bytes.iter().map(|&byte| {
                f32::from_sample(i8::from_be_bytes([byte]))
            }).collect();
        }

        SampleFormat::I16 => {

            for chunk in bytes.chunks(2) {

                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => {
                        i16::from_be_bytes([byte_1, byte_2])
                    },
                    Endianness::Little => {
                        i16::from_le_bytes([byte_1, byte_2])
                    }
                };

                resampled_bytes.push(f32::from_sample(result));

            }

        }

        SampleFormat::I24 => {
            for chunk in bytes.chunks(3) {

                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();

                // Makeshift u24.
                let result = match args.endianness {
                    Endianness::Big => {
                        i32::from_be_bytes([byte_1, byte_2, byte_3, 0])
                    },
                    Endianness::Little => {
                        i32::from_le_bytes([byte_1, byte_2, byte_3, 0])
                    }
                };

                resampled_bytes.push(f32::from_sample(result));

            }
        }

        SampleFormat::I32 => {
            for chunk in bytes.chunks(4) {

                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => {
                        i32::from_be_bytes([byte_1, byte_2, byte_3, byte_4])
                    },
                    Endianness::Little => {
                        i32::from_le_bytes([byte_1, byte_2, byte_3, byte_4])
                    }
                };

                resampled_bytes.push(f32::from_sample(result));

            }
        }

        SampleFormat::I40 => {
            for chunk in bytes.chunks(5) {

                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();
                let byte_5 = chunk.get(4).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => {
                        i64::from_be_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            0,
                            0,
                            0
                        ])
                    },
                    Endianness::Little => {
                        i64::from_le_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            0,
                            0,
                            0
                        ])
                    }
                };

                resampled_bytes.push(f32::from_sample(result));

            }
        }

        SampleFormat::I48 => {
            for chunk in bytes.chunks(6) {

                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();
                let byte_5 = chunk.get(4).unwrap_or(&0).clone();
                let byte_6 = chunk.get(5).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => {
                        i64::from_be_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            byte_6,
                            0,
                            0
                        ])
                    },
                    Endianness::Little => {
                        i64::from_le_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            byte_6,
                            0,
                            0
                        ])
                    }
                };

                resampled_bytes.push(f32::from_sample(result));

            }
        }

        SampleFormat::I56 => {
            for chunk in bytes.chunks(7) {

                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();
                let byte_5 = chunk.get(4).unwrap_or(&0).clone();
                let byte_6 = chunk.get(5).unwrap_or(&0).clone();
                let byte_7 = chunk.get(6).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => {
                        i64::from_be_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            byte_6,
                            byte_7,
                            0
                        ])
                    },
                    Endianness::Little => {
                        i64::from_le_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            byte_6,
                            byte_7,
                            0
                        ])
                    }
                };

                resampled_bytes.push(f32::from_sample(result));

            }
        }

        SampleFormat::I64 => {
            for chunk in bytes.chunks(8) {

                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();
                let byte_5 = chunk.get(4).unwrap_or(&0).clone();
                let byte_6 = chunk.get(5).unwrap_or(&0).clone();
                let byte_7 = chunk.get(6).unwrap_or(&0).clone();
                let byte_8 = chunk.get(7).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => {
                        i64::from_be_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            byte_6,
                            byte_7,
                            byte_8
                        ])
                    },
                    Endianness::Little => {
                        i64::from_le_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            byte_6,
                            byte_7,
                            byte_8
                        ])
                    }
                };

                resampled_bytes.push(f32::from_sample(result));

            }
        }

        SampleFormat::F32 => {
            for chunk in bytes.chunks(4) {

                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();

                let mut as_u32 = match args.endianness {
                    Endianness::Big => {
                        u32::from_be_bytes([byte_1, byte_2, byte_3, byte_4])
                    },
                    Endianness::Little => {
                        u32::from_le_bytes([byte_1, byte_2, byte_3, byte_4])
                    }
                };

                // Apparently NaN happens when all the bits of the exponent are 1, so technically
                // just zeroing the least significant bit of the exponent will remove the NaN issue
                // with the least loss.
                as_u32 &= 0b1_11111110_11111111111111111111111;

                let result = f32::from_bits(as_u32);

                resampled_bytes.push(result);

            }
        }

        SampleFormat::F64 => {
            for chunk in bytes.chunks(8) {

                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();
                let byte_5 = chunk.get(4).unwrap_or(&0).clone();
                let byte_6 = chunk.get(5).unwrap_or(&0).clone();
                let byte_7 = chunk.get(6).unwrap_or(&0).clone();
                let byte_8 = chunk.get(7).unwrap_or(&0).clone();

                let mut as_u64 = match args.endianness {
                    Endianness::Big => {
                        u64::from_be_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            byte_6,
                            byte_7,
                            byte_8
                        ])
                    },
                    Endianness::Little => {
                        u64::from_le_bytes([
                            byte_1,
                            byte_2,
                            byte_3,
                            byte_4,
                            byte_5,
                            byte_6,
                            byte_7,
                            byte_8
                        ])
                    }
                };

                // Apparently NaN happens when all the bits of the exponent are 1, so technically
                // just zeroing the least significant bit of the exponent will remove the NaN issue
                // with the least loss.
                as_u64 &= 0b1_11111111110_1111111111111111111111111111111111111111111111111111;

                let result = f64::from_bits(as_u64);

                resampled_bytes.push(f32::from_sample(result));

            }
        }

    };

    resampled_bytes
}