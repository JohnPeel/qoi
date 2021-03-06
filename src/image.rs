
use std::io;

use image::{ColorType, ImageDecoder, ImageEncoder, ImageError, ImageResult, error::{DecodingError, EncodingError, ImageFormatHint}};

use crate::{ColorSpace, DecoderError, EncoderError, QoiDecoder, QoiEncoder};

impl<R: io::Read> io::Read for QoiDecoder<R> {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        Ok(self.decode(buf)?)
    }
}

impl<'a, R: 'a + io::Read> ImageDecoder<'a> for QoiDecoder<R> {
    type Reader = Self;

    fn dimensions(&self) -> (u32, u32) {
        self.dimensions()
    }

    fn color_type(&self) -> ColorType {
        match self.channels() {
            3 => ColorType::Rgb8,
            4 => ColorType::Rgba8,
            _ => unreachable!()
        }
    }

    fn into_reader(self) -> ImageResult<Self::Reader> {
        Ok(self)
    }
}

impl<'a, W: 'a + io::Write> ImageEncoder for QoiEncoder<'a, W> {
    #[inline]
    fn write_image(
        mut self,
        buf: &[u8],
        width: u32,
        height: u32,
        color_type: ColorType,
    ) -> ImageResult<()> {
        // FIXME: How should I handle ColorSpace here? Currently it just assumes SRGB.
        match color_type {
            ColorType::Rgb8 | ColorType::Rgba8 => Ok(QoiEncoder::encode(&mut self, buf, width, height, color_type.bytes_per_pixel(), ColorSpace::Srgb)?),
            _ => Err(ImageError::IoError(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unsupported Color Type: {:?}.  Supported Color Types: RGB(8), RGBA(8).", color_type)
            )))
        }
    }
}

impl From<DecoderError> for ImageError {
    fn from(e: DecoderError) -> ImageError {
        ImageError::Decoding(DecodingError::new(ImageFormatHint::Name("QOI".to_string()), e))
    }
}

impl From<EncoderError> for ImageError {
    fn from(e: EncoderError) -> Self {
        ImageError::Encoding(EncodingError::new(ImageFormatHint::Name("QOI".to_string()), e))
    }
}
