
pub use read::ReadQoiChunk;
pub use write::WriteQoiChunk;

#[derive(Debug)]
pub enum QoiChunk {
    Index(u8),
    Run8(u8),
    Run16(u16),
    Diff8(u8, u8, u8),
    Diff16(u8, u8, u8),
    Diff24(u8, u8, u8, u8),
    Color(Option<u8>, Option<u8>, Option<u8>, Option<u8>)
}

mod read {
    #[cfg(feature = "std")]
    use std::io;
    #[cfg(not(feature = "std"))]
    use crate::io;

    use crate::{DecoderError, consts::QoiConsts};
    use super::QoiChunk;

    pub trait ReadQoiChunk {
        fn read_qoi_chunk(&mut self) -> Result<QoiChunk, DecoderError>;
    }
    impl<R: io::Read> ReadQoiChunk for R {
        #[inline]
        fn read_qoi_chunk(&mut self) -> Result<QoiChunk, DecoderError> {
            #[cfg(feature = "std")]
            use byteorder::ReadBytesExt;
            use QoiChunk::*;

            let first_byte = self.read_u8()?;

            let chunk = if first_byte & QoiConsts::MASK_2 == QoiConsts::INDEX {
                Index(first_byte ^ QoiConsts::INDEX)
            } else if first_byte & QoiConsts::MASK_3 == QoiConsts::RUN_8 {
                Run8(first_byte ^ QoiConsts::RUN_8)
            } else if first_byte & QoiConsts::MASK_3 == QoiConsts::RUN_16 {
                let (first_byte, second_byte) = (first_byte as u16, self.read_u8()? as u16);
                Run16((((first_byte ^ QoiConsts::RUN_16 as u16) << 8) | second_byte) + 32)
            } else if first_byte & QoiConsts::MASK_2 == QoiConsts::DIFF_8 {
                Diff8((first_byte >> 4) & 0x03, (first_byte >> 2) & 0x03, first_byte & 0x03)
            } else if first_byte & QoiConsts::MASK_3 == QoiConsts::DIFF_16 {
                let second_byte = self.read_u8()?;
                Diff16(first_byte & 0x1f, second_byte >> 4, second_byte & 0x0f)
            } else if first_byte & QoiConsts::MASK_4 == QoiConsts::DIFF_24 {
                let second_byte = self.read_u8()?;
                let third_byte = self.read_u8()?;
                Diff24(
                    ((first_byte & 0x0f) << 1) | (second_byte >> 7),
                    (second_byte & 0x7c) >> 2,
                    ((second_byte & 0x03) << 3) | ((third_byte & 0xe0) >> 5),
                    third_byte & 0x1f
                )
            } else if first_byte & QoiConsts::MASK_4 == QoiConsts::COLOR {
                Color(
                    if first_byte & QoiConsts::COLOR_R != 0 { Some(self.read_u8()?) } else { None },
                    if first_byte & QoiConsts::COLOR_G != 0 { Some(self.read_u8()?) } else { None },
                    if first_byte & QoiConsts::COLOR_B != 0 { Some(self.read_u8()?) } else { None },
                    if first_byte & QoiConsts::COLOR_A != 0 { Some(self.read_u8()?) } else { None },
                )
            } else {
                return Err(DecoderError::InvalidChunkStart(first_byte));
            };

            log::trace!("{:?}", chunk);
            Ok(chunk)
        }
    }
}

mod write {
    #[cfg(feature = "std")]
    use std::io;
    #[cfg(not(feature = "std"))]
    use crate::io;

    use crate::{consts::QoiConsts, error::EncoderError};
    use super::QoiChunk;

    pub trait WriteQoiChunk {
        fn write_qoi_chunk(&mut self, chunk: QoiChunk) -> Result<usize, EncoderError>;
    }

    impl<W: io::Write> WriteQoiChunk for W {
        #[inline]
        fn write_qoi_chunk(&mut self, chunk: QoiChunk) -> Result<usize, EncoderError> {
            #[cfg(feature = "std")]
            use byteorder::WriteBytesExt;
            use QoiChunk::*;

            let mut wrote = 1;        
            match chunk {
                Index(pos) => self.write_u8(QoiConsts::INDEX | pos)?,
                Run8(run) => self.write_u8(QoiConsts::RUN_8 | run)?,
                Run16(run) => {
                    let run = run.to_be_bytes();
                    self.write_u8(QoiConsts::RUN_16 | run[0])?;
                    self.write_u8(run[1])?;
                    wrote += 1;
                },
                Diff8(r, g, b) => self.write_u8(QoiConsts::DIFF_8 | (r << 4) | (g << 2) | b)?,
                Diff16(r, g, b) => {
                    self.write_u8(QoiConsts::DIFF_16 | r)?;
                    self.write_u8((g << 4) | b)?;
                    wrote += 1;
                },
                Diff24(r, g, b, a) => {
                    self.write_u8(QoiConsts::DIFF_24 | r >> 1)?;
                    self.write_u8((r << 7) | (g << 2) | (b >> 3))?;
                    self.write_u8((b << 5) | a)?;
                    wrote += 2;
                },
                Color(r, g, b, a) => {
                    self.write_u8(QoiConsts::COLOR
                        | if r.is_some() { QoiConsts::COLOR_R } else { 0 }
                        | if g.is_some() { QoiConsts::COLOR_G } else { 0 }
                        | if b.is_some() { QoiConsts::COLOR_B } else { 0 }
                        | if a.is_some() { QoiConsts::COLOR_A } else { 0 })?;
                    if let Some(r) = r { self.write_u8(r)?; wrote += 1; }
                    if let Some(g) = g { self.write_u8(g)?; wrote += 1; }
                    if let Some(b) = b { self.write_u8(b)?; wrote += 1; }
                    if let Some(a) = a { self.write_u8(a)?; wrote += 1; }
                },
            }

            log::trace!("{:?}", chunk);
            Ok(wrote)
        }
    }
}
