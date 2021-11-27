
use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use crate::{QoiReader, DecoderError, consts::*};

pub struct QoiDecoder<R, const CHANNELS: usize> {
    pub(crate) reader: R,

    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) size: usize,
}

impl<R: io::Read, const CHANNELS: usize> QoiDecoder<R, CHANNELS> {
    pub fn new(reader: R) -> io::Result<Self> {
        let mut decoder = QoiDecoder {
            reader,

            width: 0,
            height: 0,
            size: 0,
        };
        decoder.read_metadata()?;
        Ok(decoder)
    }

    fn read_metadata(&mut self) -> io::Result<()> {
        let mut signature = [0; 4];
        self.reader.read_exact(&mut signature)?;

        if &signature == QoiConsts::MAGIC {
            self.width = self.reader.read_u16::<BigEndian>()? as usize;
            self.height = self.reader.read_u16::<BigEndian>()? as usize;
            self.size = self.reader.read_u32::<BigEndian>()? as usize;
            Ok(())
        } else {
            Err(DecoderError::SignatureInvalid.into())
        }
    }

    pub fn into_reader(self) -> QoiReader<R, CHANNELS> {
        QoiReader::new(self.reader, self.width, self.height, self.size)
    }
}

#[cfg(feature = "image")]
impl<'a, R: 'a + io::Read, const CHANNELS: usize> image::ImageDecoder<'a> for QoiDecoder<R, CHANNELS> {
    type Reader = QoiReader<R, CHANNELS>;

    fn dimensions(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    fn color_type(&self) -> image::ColorType {
        match CHANNELS {
            3 => image::ColorType::Rgb8,
            4 => image::ColorType::Rgba8,
            _ => panic!("CHANNELS must be 3 or 4.")
        }
    }

    fn into_reader(self) -> image::ImageResult<Self::Reader> {
        Ok(QoiDecoder::into_reader(self))
    }
}
