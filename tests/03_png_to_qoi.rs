
use image::ImageEncoder;
use qoi::{self, QoiEncoder};

const INITIAL: &[u8] = include_bytes!("./image.png");
const EXPECTED: &[u8] = include_bytes!("./image.qoi");

fn compare_bytes(l: &[u8], r: &[u8]) {
    assert_eq!(l.len(), r.len());
    for i in 12..l.len() {
        if l[i] != r[i] {
            panic!("Byte {} (0x{:X}) doesn't match: 0x{:X} != 0x{:X}", i, i, l[i], r[i]);
        }
    }
}

#[test]
fn png_to_qoi() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    env_logger::init();

    let image = image::load_from_memory(INITIAL)?;
    
    let mut encoded = vec![];
    QoiEncoder::new(&mut encoded)
        .write_image(&image.into_rgba8(), 382, 480, image::ColorType::Rgba8)?;

    compare_bytes(&encoded, EXPECTED);

    Ok(())
}
