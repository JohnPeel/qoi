
#[cfg(feature = "std")]
use std::io;

#[cfg(not(feature = "std"))]
use crate::io;

use core::fmt;

#[derive(Debug)]
#[non_exhaustive]
pub enum DecoderError {
    InvalidSignature([u8; 4]),
    InvalidChannelCount(u8),
    InvalidChunkStart(u8),
    InvalidPadding([u8; 4]),
    IoError(io::Error)
}

#[derive(Debug)]
#[non_exhaustive]
pub enum EncoderError {
    IoError(io::Error)
}

impl fmt::Display for DecoderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DecoderError::InvalidSignature(signature) =>
                write!(f, "QOI header has invalid signature ({:?})", signature),
            DecoderError::InvalidChannelCount(count) =>
                write!(f, "QOI header has invalid channel count ({})", count),
            DecoderError::InvalidChunkStart(start) =>
                write!(f, "QOI chunk has an invalid start ({:X})", start),
            DecoderError::InvalidPadding(padding) =>
                write!(f, "QOI file has invalid padding ({:?})", padding),
            
            DecoderError::IoError(e) => fmt::Display::fmt(e, f),

            #[allow(unreachable_patterns)]
            _ => unreachable!()
        }
    }
}

impl fmt::Display for EncoderError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            EncoderError::IoError(e) => fmt::Display::fmt(e, f),

            #[allow(unreachable_patterns)]
            _ => {
                // NOTE: Remove this if/when this has more thant IoError.
                let _ = f;
                unreachable!()
            }
        }
    }
}


impl From<io::Error> for DecoderError {
    fn from(e: io::Error) -> Self {
        DecoderError::IoError(e)
    }
}

impl From<io::Error> for EncoderError {
    fn from(e: io::Error) -> Self {
        EncoderError::IoError(e)
    }
}

#[cfg(feature = "std")]
impl From<DecoderError> for std::io::Error {
    fn from(e: DecoderError) -> Self {
        match e {
            DecoderError::IoError(e) => e,
            _ => std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        }
    }
}

#[cfg(feature = "std")]
impl From<EncoderError> for std::io::Error {
    fn from(e: EncoderError) -> Self {
        match e {
            EncoderError::IoError(e) => e,
            #[allow(unreachable_patterns)]
            _ => std::io::Error::new(std::io::ErrorKind::InvalidData, e)
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for DecoderError {}

#[cfg(feature = "std")]
impl std::error::Error for EncoderError {}
