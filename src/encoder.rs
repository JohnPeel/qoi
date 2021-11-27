
use std::io;

use byteorder::{BigEndian, WriteBytesExt};

use crate::consts::*;

pub struct QoiEncoder<'a, W: 'a, const CHANNELS: usize> {
    writer: &'a mut W
}

impl<'a, W: io::Write + 'a, const CHANNELS: usize> QoiEncoder<'a, W, CHANNELS> {
    pub fn new(writer: &'a mut W) -> Self {
        Self { writer }
    }

    pub fn encode(
        self,
        buf: &[u8],
        width: u32,
        height: u32
    ) -> io::Result<()> {
        self.writer.write_all(QoiConsts::MAGIC)?;
        self.writer.write_u16::<BigEndian>(width as u16)?;
        self.writer.write_u16::<BigEndian>(height as u16)?;

        let pixels = buf.chunks_exact(CHANNELS);
        let pixels_len = pixels.len();

        let mut buffer: Vec<u8> = Vec::with_capacity(width as usize * height as usize * 5);
        let mut pixel = [0, 0, 0, 255];
        let mut previous_pixel = pixel;
        let mut index = [[0; 4]; QoiConsts::INDEX_SIZE];
        let mut run: u16 = 0;
        let mut wrote: usize = 0;

        for (pixel_index, current) in pixels.enumerate() {
            pixel[..CHANNELS].copy_from_slice(current);

            if pixel == previous_pixel {
                run += 1;
            }

            if run > 0 && (run == 0x2020 || pixel != previous_pixel || pixel_index == pixels_len - 1) {
                if run < 33 {
                    run -= 1;
                    log::trace!("[{}] RUN_8: 0x{:02X} {}", pixel_index, QoiConsts::RUN_8 | (run as u8), run);
                    buffer.push(QoiConsts::RUN_8 | (run as u8));
                } else {
                    run -= 33;
                    log::trace!("[{}] RUN_16: 0x{:02X}{:02X} {}", pixel_index, QoiConsts::RUN_8 | (run as u8), run as u8, run);
                    buffer.push(QoiConsts::RUN_16 | ((run >> 8) as u8));
                    buffer.push(run as u8);
                }

                run = 0;
            }

            if pixel != previous_pixel {
                let index_pos = QoiConsts::pixel_hash(&pixel);

                if index[index_pos] == pixel {
                    log::trace!("[{}:{}] INDEX: 0x{:02X} {:02} {:?} ({:?})", wrote, pixel_index, QoiConsts::INDEX | index_pos as u8, index_pos, pixel, index[index_pos]);
                    buffer.push(QoiConsts::INDEX | (index_pos as u8));
                } else {
                    index[index_pos].copy_from_slice(&pixel);

                    let r = pixel[0] as i32 - previous_pixel[0] as i32;
                    let g = pixel[1] as i32 - previous_pixel[1] as i32;
                    let b = pixel[2] as i32 - previous_pixel[2] as i32;
                    let a = pixel[3] as i32 - previous_pixel[3] as i32;

                    match (r, g, b, a) {
                        (-1..=2, -1..=2, -1..=2, 0) => {
                            let (r, g, b) = ((r + 1) as u8, (g + 1) as u8, (b + 1) as u8);
                            log::trace!("[{}:{}] DIFF_8: 0x{:02X} 0x{:02x}{:02x}{:02x} {:02X?}", wrote, pixel_index, QoiConsts::DIFF_8 | (r << 4) | (g << 2) | b, r, g, b, pixel);
                            buffer.push(QoiConsts::DIFF_8 | (r << 4) | (g << 2) | b);
                        },
                        (-15..=16, -7..=8, -7..=8, 0) => {
                            let (r, g, b) = ((r + 15) as u8, (g + 7) as u8, (b + 7) as u8);
                            log::trace!("[{}:{}] DIFF_16: 0x{:02X}{:02X} {:02x} {:02x} {:02x} {:02X?}", wrote, pixel_index, QoiConsts::DIFF_16 | r, (g << 4) | b, r, g, b, pixel);
                            buffer.push(QoiConsts::DIFF_16 | r);
                            buffer.push((g << 4) | b);
                        },
                        (-15..=16, -15..=16, -15..=16, -15..=16) => {
                            let (r, g, b, a) = ((r + 15) as u8, (g + 15) as u8, (b + 15) as u8, (a + 15) as u8);
                            log::trace!("[{}:{}] DIFF_24: 0x{:02X}{:02X}{:02X} {:02x} {:02x} {:02x} {:02x} {:02X?}", wrote, pixel_index, QoiConsts::DIFF_24 | r >> 1, (r << 7) | (g << 2) | (b >> 3), (b << 5) | a, r, g, b, a, pixel);
                            buffer.push(QoiConsts::DIFF_24 | r >> 1);
                            buffer.push((r << 7) | (g << 2) | (b >> 3));
                            buffer.push((b << 5) | a);
                        }
                        _ => {
                            let tag = QoiConsts::COLOR
                                | if r != 0 { QoiConsts::COLOR_R } else { 0 }
                                | if g != 0 { QoiConsts::COLOR_G } else { 0 }
                                | if b != 0 { QoiConsts::COLOR_B } else { 0 }
                                | if a != 0 { QoiConsts::COLOR_A } else { 0 };
                            log::trace!("[{}:{}] COLOR: 0x{:02X} {:02X?}", wrote, pixel_index, tag, pixel);

                            buffer.push(tag);
                            if tag & QoiConsts::COLOR_R == QoiConsts::COLOR_R { buffer.push(pixel[0]); }
                            if tag & QoiConsts::COLOR_G == QoiConsts::COLOR_G { buffer.push(pixel[1]); }
                            if tag & QoiConsts::COLOR_B == QoiConsts::COLOR_B { buffer.push(pixel[2]); }
                            if tag & QoiConsts::COLOR_A == QoiConsts::COLOR_A { buffer.push(pixel[3]); }
                        }
                    }
                }

                wrote += CHANNELS;
                previous_pixel[..CHANNELS].copy_from_slice(&pixel);
            }
        }

        self.writer.write_u32::<BigEndian>(buffer.len() as u32)?;
        self.writer.write_all(&buffer)?;
        self.writer.write_all(QoiConsts::PADDING)?;

        Ok(())
    }
}

#[cfg(feature = "image")]
impl<'a, W: io::Write + 'a, const CHANNELS: usize> image::ImageEncoder for QoiEncoder<'a, W, CHANNELS> {
    fn write_image(
        self,
        buf: &[u8],
        width: u32,
        height: u32,
        color_type: image::ColorType,
    ) -> image::ImageResult<()> {
        match color_type {
            image::ColorType::Rgb8 | image::ColorType::Rgba8 if color_type.bytes_per_pixel() as usize == CHANNELS => Ok(QoiEncoder::encode(self, buf, width, height)?),
            image::ColorType::Rgb8 | image::ColorType::Rgba8 => Err(image::ImageError::IoError(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Wrong number of channels: {}.  Should be: {}.", CHANNELS, color_type.bytes_per_pixel())
            ))),
            _ => Err(image::ImageError::IoError(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unsupported Color Type: {:?}.  Supported Color Types: RGB(8), RGBA(8).", color_type)
            )))
        }
    }
}
