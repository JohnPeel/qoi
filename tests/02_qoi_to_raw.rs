
use image::ImageDecoder;
use qoi::{self, QoiDecoder};

mod common;
use common::compare_bytes;

const INITIAL: &[u8] = include_bytes!("./image.qoi");
const EXPECTED: &[u8] = include_bytes!("./image.raw");

#[test]
fn qoi_to_raw() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    env_logger::init();

    let decoder = QoiDecoder::new(INITIAL)?;
    let mut decoded = vec![0u8; decoder.total_bytes() as usize];
    decoder.read_image(&mut decoded)?;

    compare_bytes(&decoded, EXPECTED);

    Ok(())
}
