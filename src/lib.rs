
mod consts;
mod error;
mod reader;
mod decoder;
mod encoder;

use error::DecoderError;
use reader::QoiReader;
pub use decoder::QoiDecoder;
pub use encoder::QoiEncoder;
