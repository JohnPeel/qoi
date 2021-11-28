
use std::{error, fmt, io};

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum DecoderError {
    InvalidSignature([u8; 4]),
    InvalidChannelCount(u8),
    InvalidPadding([u8; 4]),
}

impl fmt::Display for DecoderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecoderError::InvalidSignature(signature) =>
                write!(f, "QOI header has invalid signature ({:?})", signature),
            DecoderError::InvalidChannelCount(count) =>
                write!(f, "QOI header has invalid channel count ({})", count),
            DecoderError::InvalidPadding(padding) =>
                write!(f, "QOI file has invalid padding ({:?})", padding)
        }
    }
}

impl From<DecoderError> for io::Error {
    fn from(e: DecoderError) -> Self {
        io::Error::new(io::ErrorKind::InvalidData, e)
    }
}

impl error::Error for DecoderError {}
