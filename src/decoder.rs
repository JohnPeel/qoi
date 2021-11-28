
use std::io;

use byteorder::{BigEndian, ReadBytesExt};

use crate::{QoiReader, DecoderError, consts::*};

pub struct QoiDecoder<R> {
    pub(crate) reader: R,

    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) channels: u8,
    pub(crate) colorspace: u8
}

impl<R: io::Read> QoiDecoder<R> {
    pub fn new(reader: R) -> io::Result<Self> {
        let mut decoder = QoiDecoder {
            reader,

            width: 0,
            height: 0,
            channels: 0,
            colorspace: 0
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
            self.colorspace = self.reader.read_u8()?;
            Ok(())
        } else {
            Err(DecoderError::SignatureInvalid.into())
        }
    }

    pub fn into_reader(self) -> QoiReader<R> {
        QoiReader::new(self.reader, self.width, self.height, self.channels)
    }
}

#[cfg(feature = "image")]
impl<'a, R: 'a + io::Read> image::ImageDecoder<'a> for QoiDecoder<R> {
    type Reader = QoiReader<R>;

    fn dimensions(&self) -> (u32, u32) {
        (self.width as u32, self.height as u32)
    }

    fn color_type(&self) -> image::ColorType {
        match self.channels {
            3 => image::ColorType::Rgb8,
            4 => image::ColorType::Rgba8,
            _ => unreachable!() // FIXME: Add error message here, or in read_metadata?
        }
    }

    fn into_reader(self) -> image::ImageResult<Self::Reader> {
        Ok(QoiDecoder::into_reader(self))
    }
}
