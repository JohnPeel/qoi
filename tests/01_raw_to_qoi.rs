
use image::ImageEncoder;
use qoi::{self, QoiEncoder};

mod common;
use common::compare_bytes;

const INITIAL: &[u8] = include_bytes!("./image.raw");
const EXPECTED: &[u8] = include_bytes!("./image.qoi");

#[test]
fn raw_to_qoi() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    env_logger::init();
    
    let mut encoded = vec![];
    QoiEncoder::new(&mut encoded)
        .write_image(INITIAL, 382, 480, image::ColorType::Rgba8)?;

    compare_bytes(&encoded, EXPECTED);

    Ok(())
}
