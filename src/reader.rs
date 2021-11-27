
use std::io;

use byteorder::ReadBytesExt;

use crate::consts::*;

pub struct QoiReader<R: io::Read, const CHANNELS: usize> {
    reader: R,

    #[allow(unused)]
    width: usize,
    #[allow(unused)]
    height: usize,
    size: usize,
    total_read: usize,

    run: usize,
    pixel: [u8; 4],
    index: [[u8; 4]; QoiConsts::INDEX_SIZE]
}

impl<R: io::Read, const CHANNELS: usize> QoiReader<R, CHANNELS> {
    pub(crate) fn new(reader: R, width: usize, height: usize, size: usize) -> Self {
        Self {
            reader,

            width,
            height,
            size,
            total_read: 0,

            run: 0,
            pixel: [0, 0, 0, 255],
            index: [[0; 4]; QoiConsts::INDEX_SIZE],
        }
    }

    pub fn decode(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let mut read = 0;
        for chunk in buf.chunks_exact_mut(CHANNELS) {
            if self.run > 0 {
                self.run -= 1;
            } else {
                let first_byte = self.reader.read_u8()?;
                self.total_read += 1;
                
                if first_byte & QoiConsts::MASK_2 == QoiConsts::INDEX {
                    self.pixel.copy_from_slice(&self.index[(first_byte ^ QoiConsts::INDEX) as usize]);
                } else if first_byte & QoiConsts::MASK_3 == QoiConsts::RUN_8 {
                    self.run = (first_byte & 0x1f) as usize;
                } else if first_byte & QoiConsts::MASK_3 == QoiConsts::RUN_16 {
                    let second_byte = self.reader.read_u8()?;
                    self.total_read += 1;
                    self.run = ((((first_byte & 0x1f) as usize) << 8) | second_byte as usize) + 32;
                } else if first_byte & QoiConsts::MASK_2 == QoiConsts::DIFF_8 {
                    self.pixel[0] = (self.pixel[0] as i8).wrapping_add((((first_byte >> 4) & 0x03) as i8) - 1) as u8;
                    self.pixel[1] = (self.pixel[1] as i8).wrapping_add((((first_byte >> 2) & 0x03) as i8) - 1) as u8;
                    self.pixel[2] = (self.pixel[2] as i8).wrapping_add(((first_byte & 0x03) as i8) - 1) as u8;
                } else if first_byte & QoiConsts::MASK_3 == QoiConsts::DIFF_16 {
                    let second_byte = self.reader.read_u8()?;
                    self.total_read += 1;
                    self.pixel[0] = (self.pixel[0] as i8).wrapping_add(((first_byte & 0x1f) as i8) - 15) as u8;
                    self.pixel[1] = (self.pixel[1] as i8).wrapping_add(((second_byte >> 4) as i8) - 7) as u8;
                    self.pixel[2] = (self.pixel[2] as i8).wrapping_add(((second_byte & 0x0f) as i8) - 7) as u8;
                } else if first_byte & QoiConsts::MASK_4 == QoiConsts::DIFF_24 {
                    let second_byte = self.reader.read_u8()?;
                    let third_byte = self.reader.read_u8()?;
                    self.total_read += 2;
                    self.pixel[0] = (self.pixel[0] as i8).wrapping_add(((((first_byte & 0x0f) << 1) | (second_byte >> 7)) as i8) - 15) as u8;
                    self.pixel[1] = (self.pixel[1] as i8).wrapping_add((((second_byte & 0x7c) >> 2) as i8) - 15) as u8;
                    self.pixel[2] = (self.pixel[2] as i8).wrapping_add(((((second_byte & 0x03) << 3) | ((third_byte & 0xe0) >> 5)) as i8) - 15) as u8;
                    self.pixel[3] = (self.pixel[3] as i8).wrapping_add(((third_byte & 0x1f) as i8) - 15) as u8;
                } else if first_byte & QoiConsts::MASK_4 == QoiConsts::COLOR {
                    if first_byte & QoiConsts::COLOR_R != 0 {
                        self.pixel[0] = self.reader.read_u8()?;
                        self.total_read += 1;
                    }
                    if first_byte & QoiConsts::COLOR_G != 0 {
                        self.pixel[1] = self.reader.read_u8()?;
                        self.total_read += 1;
                    }
                    if first_byte & QoiConsts::COLOR_B != 0 {
                        self.pixel[2] = self.reader.read_u8()?;
                        self.total_read += 1;
                    }
                    if first_byte & QoiConsts::COLOR_A != 0 {
                        self.pixel[3] = self.reader.read_u8()?;
                        self.total_read += 1;
                    }
                } else {
                    unreachable!()
                }

                self.index[QoiConsts::pixel_hash(&self.pixel)].copy_from_slice(&self.pixel);
            }

            chunk[..CHANNELS].copy_from_slice(&self.pixel[..CHANNELS]);
            read += CHANNELS;
        }

        if self.size - self.total_read == 4 {
            let padding = &mut [0; 4];
            self.reader.read_exact(padding)?;
            self.total_read += 4;
            assert_eq!(padding, QoiConsts::PADDING, "invalid padding: {:x?}", dbg!(&padding));
        }

        Ok(read)
    }
}

impl<R: io::Read, const CHANNELS: usize> io::Read for QoiReader<R, CHANNELS> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        QoiReader::decode(self, buf)
    }
}
