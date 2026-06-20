use crate::args::{Args, Endianness, SampleFormat};
use dasp_sample::Sample;

pub fn process(args: &Args, bytes: &Vec<u8>) -> Vec<u8> {

    // Bitcast to chosen format and then normalize to f32, idk if even half of these are actual formats used in music but why not use them anyway.
    let mut resampled_bytes: Vec<f32> = match args.sample_format {

        SampleFormat::U8 => {
            bytes.iter().map(|&byte| { f32::from_sample(u8::from_be_bytes([byte])) }).collect()
        }

        SampleFormat::U16 => {

            bytes.chunks(2).map(|chunk| {

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

                f32::from_sample(result)

            }).collect()

        }

        SampleFormat::U24 => {
            bytes.chunks(3).map(|chunk| {
                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => {
                        u32::from_be_bytes([byte_1, byte_2, byte_3, 0])
                    },
                    Endianness::Little => {
                        u32::from_le_bytes([byte_1, byte_2, byte_3, 0])
                    }
                };

                f32::from_sample(result)

            }).collect()
        }

        SampleFormat::U32 => {
            bytes.chunks(4).map(|chunk| {
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

                f32::from_sample(result)
            }).collect()
        }

        SampleFormat::U40 => {

            bytes.chunks(5).map(|chunk| {
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

                f32::from_sample(result)
            }).collect()
        }

        SampleFormat::U48 => {
            bytes.chunks(6).map(|chunk| {
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

                f32::from_sample(result)
            }).collect()
        }

        SampleFormat::U56 => {
            bytes.chunks(7).map(|chunk| {
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

                f32::from_sample(result)
            }).collect()

        }

        SampleFormat::U64 => {
            bytes.chunks(8).map(|chunk| {
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

                f32::from_sample(result)
            }).collect()
        }

        SampleFormat::I8 => {
            bytes.iter().map(|&byte| { f32::from_sample(i8::from_be_bytes([byte])) }).collect()
        }

        SampleFormat::I16 => {
            bytes.chunks(2).map(|chunk| {
                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => i16::from_be_bytes([byte_1, byte_2]),
                    Endianness::Little => i16::from_le_bytes([byte_1, byte_2]),
                };

                f32::from_sample(result)
            }).collect()
        }

        SampleFormat::I24 => {
            bytes.chunks(3).map(|chunk| {
                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => i32::from_be_bytes([byte_1, byte_2, byte_3, 0]),
                    Endianness::Little => i32::from_le_bytes([byte_1, byte_2, byte_3, 0]),
                };

                f32::from_sample(result)
            }).collect()
        }

        SampleFormat::I32 => {
            bytes.chunks(4).map(|chunk| {
                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => i32::from_be_bytes([byte_1, byte_2, byte_3, byte_4]),
                    Endianness::Little => i32::from_le_bytes([byte_1, byte_2, byte_3, byte_4]),
                };

                f32::from_sample(result)
            }).collect()
        }

        SampleFormat::I40 => {
            bytes.chunks(5).map(|chunk| {
                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();
                let byte_5 = chunk.get(4).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => i64::from_be_bytes([
                        byte_1,
                        byte_2,
                        byte_3,
                        byte_4,
                        byte_5,
                        0,
                        0,
                        0,
                    ]),
                    Endianness::Little => i64::from_le_bytes([
                        byte_1,
                        byte_2,
                        byte_3,
                        byte_4,
                        byte_5,
                        0,
                        0,
                        0,
                    ]),
                };

                f32::from_sample(result)
            }).collect()
        }

        SampleFormat::I48 => {
            bytes.chunks(6).map(|chunk| {
                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();
                let byte_5 = chunk.get(4).unwrap_or(&0).clone();
                let byte_6 = chunk.get(5).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => i64::from_be_bytes([
                        byte_1,
                        byte_2,
                        byte_3,
                        byte_4,
                        byte_5,
                        byte_6,
                        0,
                        0,
                    ]),
                    Endianness::Little => i64::from_le_bytes([
                        byte_1,
                        byte_2,
                        byte_3,
                        byte_4,
                        byte_5,
                        byte_6,
                        0,
                        0,
                    ]),
                };

                f32::from_sample(result)
            }).collect()
        }

        SampleFormat::I56 => {
            bytes.chunks(7).map(|chunk| {
                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();
                let byte_5 = chunk.get(4).unwrap_or(&0).clone();
                let byte_6 = chunk.get(5).unwrap_or(&0).clone();
                let byte_7 = chunk.get(6).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => i64::from_be_bytes([
                        byte_1,
                        byte_2,
                        byte_3,
                        byte_4,
                        byte_5,
                        byte_6,
                        byte_7,
                        0,
                    ]),
                    Endianness::Little => i64::from_le_bytes([
                        byte_1,
                        byte_2,
                        byte_3,
                        byte_4,
                        byte_5,
                        byte_6,
                        byte_7,
                        0,
                    ]),
                };

                f32::from_sample(result)
            }).collect()
        }

        SampleFormat::I64 => {
            bytes.chunks(8).map(|chunk| {
                let byte_1 = chunk.get(0).unwrap_or(&0).clone();
                let byte_2 = chunk.get(1).unwrap_or(&0).clone();
                let byte_3 = chunk.get(2).unwrap_or(&0).clone();
                let byte_4 = chunk.get(3).unwrap_or(&0).clone();
                let byte_5 = chunk.get(4).unwrap_or(&0).clone();
                let byte_6 = chunk.get(5).unwrap_or(&0).clone();
                let byte_7 = chunk.get(6).unwrap_or(&0).clone();
                let byte_8 = chunk.get(7).unwrap_or(&0).clone();

                let result = match args.endianness {
                    Endianness::Big => i64::from_be_bytes([
                        byte_1,
                        byte_2,
                        byte_3,
                        byte_4,
                        byte_5,
                        byte_6,
                        byte_7,
                        byte_8,
                    ]),
                    Endianness::Little => i64::from_le_bytes([
                        byte_1,
                        byte_2,
                        byte_3,
                        byte_4,
                        byte_5,
                        byte_6,
                        byte_7,
                        byte_8,
                    ]),
                };

                f32::from_sample(result)
            }).collect()
        }

        SampleFormat::F32 => {
            bytes.chunks(4).map(|chunk| {
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

                f32::from_bits(as_u32)
            }).collect()
        }

        SampleFormat::F64 => {
            bytes.chunks(8).map(|chunk| {
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

                f32::from_sample(result)
            }).collect()
        }

    };

    // Pad the last frame so that there's enough values for all channels.
    let remainder_frames = resampled_bytes.len() % args.channels as usize;
    let padding = args.channels as usize - remainder_frames;

    resampled_bytes.resize(resampled_bytes.len() + padding, 0.0);

    resampled_bytes.iter().flat_map(|&f| f.to_le_bytes()).collect()
}