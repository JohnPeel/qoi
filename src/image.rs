
use std::io;

use crate::{ColorSpace, DecoderError, QoiDecoder, QoiEncoder};

impl<R: io::Read> io::Read for QoiDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        self.decode(buf)
    }
}

impl<'a, R: 'a + io::Read> image::ImageDecoder<'a> for QoiDecoder<R> {
    type Reader = Self;

    fn dimensions(&self) -> (u32, u32) {
        self.dimensions()
    }

    fn color_type(&self) -> image::ColorType {
        match self.channels() {
            3 => image::ColorType::Rgb8,
            4 => image::ColorType::Rgba8,
            _ => unreachable!()
        }
    }

    fn into_reader(self) -> image::ImageResult<Self::Reader> {
        Ok(self)
    }
}

impl<'a, W: io::Write + 'a> image::ImageEncoder for QoiEncoder<'a, W> {
    #[inline]
    fn write_image(
        self,
        buf: &[u8],
        width: u32,
        height: u32,
        color_type: image::ColorType,
    ) -> image::ImageResult<()> {
        // FIXME: How should I handle ColorSpace here? Currently it just assumes SRGB.
        match color_type {
            image::ColorType::Rgb8 | image::ColorType::Rgba8 => Ok(QoiEncoder::encode(self, buf, width, height, color_type.bytes_per_pixel(), ColorSpace::Srgb)?),
            _ => Err(image::ImageError::IoError(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unsupported Color Type: {:?}.  Supported Color Types: RGB(8), RGBA(8).", color_type)
            )))
        }
    }
}

impl From<DecoderError> for image::ImageError {
    fn from(e: DecoderError) -> image::ImageError {
        image::ImageError::Decoding(image::error::DecodingError::new(image::error::ImageFormatHint::Name("QOI".to_string()), e))
    }
}
