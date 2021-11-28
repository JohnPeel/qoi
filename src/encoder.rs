
use std::io;

use byteorder::{BigEndian, WriteBytesExt};

use crate::{ColorSpace, chunk::{QoiChunk, WriteChunk}, consts::*};

pub struct QoiEncoder<'a, W: 'a> {
    writer: &'a mut W
}

impl<'a, W: io::Write + 'a> QoiEncoder<'a, W> {
    pub fn new(writer: &'a mut W) -> Self {
        Self { writer }
    }

    pub fn encode(
        self,
        buf: &[u8],
        width: u32,
        height: u32,
        channels: u8,
        color_space: ColorSpace
    ) -> io::Result<()> {
        self.writer.write_all(QoiConsts::MAGIC)?;
        self.writer.write_u32::<BigEndian>(width)?;
        self.writer.write_u32::<BigEndian>(height)?;
        self.writer.write_u8(channels)?;
        self.writer.write_u8(color_space.into())?;

        let channels = channels as usize;
        let pixels = buf.chunks_exact(channels);
        let pixels_len = pixels.len();

        let mut pixel = [0, 0, 0, 255];
        let mut previous_pixel = pixel;
        let mut index = [[0; 4]; QoiConsts::INDEX_SIZE];
        let mut run: u16 = 0;

        for (pixel_index, current) in pixels.enumerate() {
            pixel[..channels].copy_from_slice(current);

            if pixel == previous_pixel {
                run += 1;
            }

            if run > 0 && (run == 0x2020 || pixel != previous_pixel || pixel_index == pixels_len - 1) {
                self.writer.write_qoi_chunk(
                    if run < 33 {
                        run -= 1;
                        QoiChunk::Run8(run as u8)
                    } else {
                        run -= 33;
                        QoiChunk::Run16(run)
                    }
                )?;

                run = 0;
            }

            if pixel != previous_pixel {
                let index_pos = QoiConsts::pixel_hash(&pixel);

                self.writer.write_qoi_chunk(
                    if index[index_pos] == pixel {
                        QoiChunk::Index(index_pos as u8)
                    } else {
                        index[index_pos].copy_from_slice(&pixel);

                        let r = pixel[0].wrapping_sub(previous_pixel[0]).wrapping_add(16);
                        let g = pixel[1].wrapping_sub(previous_pixel[1]).wrapping_add(16);
                        let b = pixel[2].wrapping_sub(previous_pixel[2]).wrapping_add(16);
                        let a = pixel[3].wrapping_sub(previous_pixel[3]).wrapping_add(16);

                        match (r, g, b, a) {
                            (14..=17, 14..=17, 14..=17, 16) => QoiChunk::Diff8(r - 14, g - 14, b - 14),
                            (0..=31, 8..=23, 8..=23, 16) => QoiChunk::Diff16(r, g - 8, b - 8),
                            (0..=31, 0..=31, 0..=31, 0..=31) => QoiChunk::Diff24(r, g, b, a),
                            _ => QoiChunk::Color(
                                if r != 16 { Some(pixel[0]) } else { None },
                                if g != 16 { Some(pixel[1]) } else { None },
                                if b != 16 { Some(pixel[2]) } else { None },
                                if a != 16 { Some(pixel[3]) } else { None }
                            )
                        }
                    }
                )?;

                previous_pixel[..channels].copy_from_slice(&pixel);
            }
        }

        self.writer.write_all(QoiConsts::PADDING)?;

        Ok(())
    }
}
