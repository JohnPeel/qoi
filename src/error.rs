
use std::{error, fmt, io};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum DecoderError {
    SignatureInvalid
}

impl fmt::Display for DecoderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecoderError::SignatureInvalid =>
                f.write_str("QOI signature not found"),
        }
    }
}

impl From<DecoderError> for io::Error {
    fn from(e: DecoderError) -> Self {
        io::Error::new(io::ErrorKind::InvalidData, e)
    }
}

#[cfg(feature = "image")]
impl From<DecoderError> for image::ImageError {
    fn from(e: DecoderError) -> image::ImageError {
        image::ImageError::Decoding(image::error::DecodingError::new(image::error::ImageFormatHint::Name("QOI".to_string()), e))
    }
}

impl error::Error for DecoderError {}
