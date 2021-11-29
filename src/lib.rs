#![cfg_attr(not(feature = "std"), no_std)]

#[cfg(not(feature = "std"))]
extern crate alloc;

mod consts;
mod error;
mod color_space;
mod chunk;
mod decoder;
mod encoder;

#[cfg(all(feature = "image", feature = "std"))]
mod image;

#[cfg(not(feature = "std"))]
pub mod io;

pub use color_space::ColorSpace;
use chunk::*;
pub use error::{DecoderError, EncoderError};
pub use decoder::QoiDecoder;
pub use encoder::QoiEncoder;
