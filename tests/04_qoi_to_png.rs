#![allow(unused)]

#[cfg(feature = "std")]
use image::{ImageDecoder, png::{CompressionType, FilterType, PngEncoder}};

use qoi::{self, QoiDecoder};

mod common;
use common::compare_bytes;

const INITIAL: &[u8] = include_bytes!("./image.qoi");
const EXPECTED: &[u8] = include_bytes!("./image.png");

#[cfg(feature = "std")]
#[test]
#[ignore = "Cannot get image's png support to output the large pngs created by stb."]
fn qoi_to_png() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    env_logger::init();

    let decoder = QoiDecoder::new(INITIAL)?;
    let (width, height) = decoder.dimensions();
    let mut decoded = vec![0u8; (width * height * 4) as usize];
    decoder.read_image(&mut decoded)?;

    let mut encoded = vec![];
    let encoder = PngEncoder::new_with_quality(&mut encoded, CompressionType::Fast, FilterType::NoFilter);
    encoder.encode(&decoded, width, height, image::ColorType::Rgba8)?;

    compare_bytes(&encoded, EXPECTED);

    Ok(())
}

#[cfg(not(feature = "std"))]
#[test]
#[ignore = "The png tests require std."]
fn qoi_to_png() {}