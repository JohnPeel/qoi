
use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use crate::{ColorSpace, DecoderError, chunk::{QoiChunk, ReadChunk}, consts::*};

pub struct QoiDecoder<R> {
    reader: R,

    width: u32,
    height: u32,
    channels: u8,
    colorspace: u8,

    chunk_count: usize,
    chunks_read: usize,

    run: usize,
    pixel: [u8; 4],
    index: [[u8; 4]; QoiConsts::INDEX_SIZE]
}

impl<R: io::Read> QoiDecoder<R> {
    pub fn new(reader: R) -> io::Result<Self> {
        let mut decoder = QoiDecoder {
            reader,

            width: 0,
            height: 0,
            channels: 0,
            colorspace: 0,

            chunk_count: 0,
            chunks_read: 0,

            run: 0,
            pixel: [0, 0, 0, 255],
            index: [[0; 4]; QoiConsts::INDEX_SIZE],
        };
        decoder.read_metadata()?;
        Ok(decoder)
    }

    fn read_metadata(&mut self) -> io::Result<()> {
        let mut signature = [0; 4];
        self.reader.read_exact(&mut signature)?;

        if &signature == QoiConsts::MAGIC {
            self.width = self.reader.read_u32::<BigEndian>()?;
            self.height = self.reader.read_u32::<BigEndian>()?;
            self.channels = self.reader.read_u8()?;
            if self.channels > 4 || self.channels < 3 {
                return Err(DecoderError::InvalidChannelCount(self.channels).into());
            }
            self.colorspace = self.reader.read_u8()?;
            self.chunk_count = self.width as usize * self.height as usize;
            Ok(())
        } else {
            Err(DecoderError::InvalidSignature(signature).into())
        }
    }

    pub fn dimensions(&self) -> (u32, u32) {
        (self.width, self.height)
    }

    pub fn channels(&self) -> u8 {
        self.channels
    }

    pub fn color_space(&self) -> ColorSpace {
        self.colorspace.into()
    }

    pub fn decode(&mut self, buf: &mut [u8]) -> io::Result<usize> {
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
            if padding != *QoiConsts::PADDING {
                return Err(DecoderError::InvalidPadding(padding).into());
            }
        }

        Ok(read)
    }
}
