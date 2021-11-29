#![allow(unused)]

#[cfg(feature = "std")]
use image::{GenericImageView, ImageEncoder};

use qoi::{self, ColorSpace, QoiEncoder};

mod common;
use common::compare_bytes;

const INITIAL: &[u8] = include_bytes!("./image.png");
const EXPECTED: &[u8] = include_bytes!("./image.qoi");

#[cfg(feature = "std")]
#[test]
fn png_to_qoi() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    env_logger::init();

    let image = image::load_from_memory(INITIAL)?;
    let (width, height) = image.dimensions();
    
    let mut encoded = vec![];
    QoiEncoder::new(&mut encoded)
        .encode(&image.into_rgba8(), width, height, 4, ColorSpace::Srgb)?;

    compare_bytes(&encoded, EXPECTED);

    Ok(())
}

#[cfg(not(feature = "std"))]
#[test]
#[ignore = "The png tests require std."]
fn png_to_qoi() {}
