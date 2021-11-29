
use qoi::{self, DecoderError, QoiDecoder};

mod common;
use common::compare_bytes;

const INITIAL: &[u8] = include_bytes!("./image.qoi");
const EXPECTED: &[u8] = include_bytes!("./image.raw");

#[test]
fn qoi_to_raw() -> Result<(), DecoderError> {
    env_logger::init();

    let mut decoder = QoiDecoder::new(INITIAL)?;
    let (width, height) = decoder.dimensions();
    let channels = decoder.channels();
    let mut decoded = vec![0u8; width as usize * height as usize * channels as usize];
    decoder.decode(&mut decoded)?;

    compare_bytes(&decoded, EXPECTED);

    Ok(())
}
