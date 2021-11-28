
use image::{GenericImageView, ImageEncoder};
use qoi::{self, QoiEncoder};

mod common;
use common::compare_bytes;

const INITIAL: &[u8] = include_bytes!("./image.png");
const EXPECTED: &[u8] = include_bytes!("./image.qoi");

#[test]
fn png_to_qoi() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    env_logger::init();

    let image = image::load_from_memory(INITIAL)?;
    let (width, height) = image.dimensions();
    
    let mut encoded = vec![];
    QoiEncoder::new(&mut encoded)
        .write_image(&image.into_rgba8(), width, height, image::ColorType::Rgba8)?;

    compare_bytes(&encoded, EXPECTED);

    Ok(())
}
