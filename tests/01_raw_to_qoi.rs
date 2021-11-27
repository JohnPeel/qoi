
use image::ImageEncoder;
use qoi::{self, QoiEncoder};

const INITIAL: &[u8] = include_bytes!("./image.raw");
const EXPECTED: &[u8] = include_bytes!("./image.qoi");

fn compare_bytes(l: &[u8], r: &[u8]) {
    assert_eq!(l.len(), r.len());
    for i in 12..l.len() {
        if l[i] != r[i] {
            panic!("Byte {} (0x{:X}) doesn't match: {} != {}", i, i, l[i], r[i]);
        }
    }
}

#[test]
fn raw_to_qoi() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    env_logger::init();
    
    let mut encoded = vec![];
    QoiEncoder::<_, 4>::new(&mut encoded)
        .write_image(INITIAL, 382, 480, image::ColorType::Rgba8)?;

    compare_bytes(&encoded, EXPECTED);

    Ok(())
}
