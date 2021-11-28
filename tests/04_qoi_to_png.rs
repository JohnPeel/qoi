
use image::{ImageDecoder, png::{CompressionType, FilterType, PngEncoder}};
use qoi::{self, QoiDecoder};

const INITIAL: &[u8] = include_bytes!("./image.qoi");
const EXPECTED: &[u8] = include_bytes!("./image.png");

fn compare_bytes(l: &[u8], r: &[u8]) {
    assert_eq!(l.len(), r.len());
    for i in 0..l.len() {
        if l[i] != r[i] {
            panic!("Byte {} (0x{:X}) doesn't match: 0x{:X} != 0x{:X}", i, i, l[i], r[i]);
        }
    }
}

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
