#[cfg(feature = "std")]
use std::io;
#[cfg(not(feature = "std"))]
use crate::io;

use byteorder::BigEndian;
#[cfg(feature = "std")]
use byteorder::ReadBytesExt;

use crate::{ColorSpace, DecoderError, QoiChunk, ReadQoiChunk, consts::*};

pub struct QoiDecoder<R> {
    reader: R,

    width: u32,
    height: u32,
    channels: u8,
    color_space: u8,

    chunk_count: usize,
    chunks_read: usize,

    run: usize,
    pixel: [u8; 4],
    index: [[u8; 4]; QoiConsts::INDEX_SIZE]
}

impl<R: io::Read> QoiDecoder<R> {
    pub fn new(mut reader: R) -> Result<Self, DecoderError> {
        let mut signature = [0; QoiConsts::MAGIC_LEN];
        reader.read_exact(&mut signature)?;
        if signature != QoiConsts::MAGIC {
            return Err(DecoderError::InvalidSignature(signature));
        }

        let width = reader.read_u32::<BigEndian>()?;
        let height = reader.read_u32::<BigEndian>()?;
        let channels = reader.read_u8()?;
        let color_space = reader.read_u8()?;

        if !(QoiConsts::CHANNELS_MIN..=QoiConsts::CHANNELS_MAX).contains(&channels) {
            return Err(DecoderError::InvalidChannelCount(channels));
        }
        
        let decoder = QoiDecoder {
            reader,

            width,
            height,
            channels,
            color_space,

            chunk_count: width as usize * height as usize,
            chunks_read: 0,

            run: 0,
            pixel: [0, 0, 0, 255],
            index: [[0; 4]; QoiConsts::INDEX_SIZE],
        };
        Ok(decoder)
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn channels(&self) -> u8 {
        self.channels
    }

    pub fn color_space(&self) -> ColorSpace {
        self.color_space.into()
    }

    pub fn decode(&mut self, buf: &mut [u8]) -> Result<usize, DecoderError> {
        let mut read = 0;
        for chunk in buf.chunks_exact_mut(self.channels as usize).take(self.chunk_count - self.chunks_read) {
            if self.run > 0 {
                self.run -= 1;
            } else {
                match self.reader.read_qoi_chunk()? {
                    QoiChunk::Index(pos) => self.pixel.copy_from_slice(&self.index[pos as usize]),
                    QoiChunk::Run8(run) => self.run = run as usize,
                    QoiChunk::Run16(run) => self.run = run as usize,
                    QoiChunk::Diff8(r, g, b) => {
                        self.pixel[0] = self.pixel[0].wrapping_add(r).wrapping_sub(2);
                        self.pixel[1] = self.pixel[1].wrapping_add(g).wrapping_sub(2);
                        self.pixel[2] = self.pixel[2].wrapping_add(b).wrapping_sub(2);
                    },
                    QoiChunk::Diff16(r, g, b) => {
                        self.pixel[0] = self.pixel[0].wrapping_add(r).wrapping_sub(16);
                        self.pixel[1] = self.pixel[1].wrapping_add(g).wrapping_sub(8);
                        self.pixel[2] = self.pixel[2].wrapping_add(b).wrapping_sub(8);
                    },
                    QoiChunk::Diff24(r, g, b, a) => {
                        self.pixel[0] = self.pixel[0].wrapping_add(r).wrapping_sub(16);
                        self.pixel[1] = self.pixel[1].wrapping_add(g).wrapping_sub(16);
                        self.pixel[2] = self.pixel[2].wrapping_add(b).wrapping_sub(16);
                        self.pixel[3] = self.pixel[3].wrapping_add(a).wrapping_sub(16);
                    },
                    QoiChunk::Color(r, g, b, a) => {
                        if let Some(r) = r { self.pixel[0] = r; }
                        if let Some(g) = g { self.pixel[1] = g; }
                        if let Some(b) = b { self.pixel[2] = b; }
                        if let Some(a) = a { self.pixel[3] = a; }
                    },
                }

                self.index[QoiConsts::pixel_hash(&self.pixel)].copy_from_slice(&self.pixel);
            }

            chunk[..self.channels as usize].copy_from_slice(&self.pixel[..self.channels as usize]);
            read += self.channels as usize;
            self.chunks_read += 1;
        }

        if self.chunks_read == self.chunk_count {
            let mut padding = [0; 4];
            self.reader.read_exact(&mut padding)?;
            self.chunks_read += 1;
            if padding != QoiConsts::PADDING {
                return Err(DecoderError::InvalidPadding(padding));
            }
        }

        Ok(read)
    }
}
