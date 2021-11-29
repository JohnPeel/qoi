
use qoi::{self, ColorSpace, EncoderError, QoiEncoder};

mod common;
use common::compare_bytes;

const INITIAL: &[u8] = include_bytes!("./image.raw");
const EXPECTED: &[u8] = include_bytes!("./image.qoi");

#[test]
fn raw_to_qoi() -> Result<(), EncoderError> {
    env_logger::init();
    
    let mut encoded = vec![];
    QoiEncoder::new(&mut encoded)
        .encode(INITIAL, 382, 480, 4, ColorSpace::Srgb)?;

    compare_bytes(&encoded, EXPECTED);

    Ok(())
}
