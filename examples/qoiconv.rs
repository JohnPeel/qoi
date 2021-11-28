
use std::{fs::File, io::BufReader, path::PathBuf};

use clap::{Parser, ValueHint};
use image::{ImageDecoder, ColorType, GenericImageView};
use qoi::{QoiDecoder, QoiEncoder};

#[derive(Parser)]
#[clap(name = "qoiconv", author = "John Peel <john@dgby.org>")]
struct Opts {
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    input: PathBuf,
    #[clap(parse(from_os_str), value_hint = ValueHint::FilePath)]
    output: PathBuf
}

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    env_logger::init();

    let opts: Opts = Opts::parse();

    if !opts.input.exists() {
        return Err("INPUT file not found.".into());
    }

    match (opts.input.extension(), opts.output.extension()) {
        (Some(ext), _) if ext == "qoi" => {
            let decoder = QoiDecoder::new(BufReader::new(File::open(opts.input)?))?;
            let (width, height) = decoder.dimensions();
            let color_type = decoder.color_type();
            let mut buf: Vec<u8> = vec![0; decoder.total_bytes() as usize];
            decoder.read_image(&mut buf)?;

            image::save_buffer(opts.output, &buf, width, height, color_type)?;
        },
        (_, Some(ext)) if ext == "qoi" => {
            let dynamic_image = image::open(opts.input)?;
            let (width, height) = dynamic_image.dimensions();
            let color_type = dynamic_image.color();

            let mut output = std::fs::File::create(opts.output)?;
            let encoder = QoiEncoder::new(&mut output);

            match color_type {
                ColorType::Rgb8 => {
                    let buf = dynamic_image.as_rgb8().unwrap();
                    encoder.encode(buf, width, height, 3)?;
                },
                ColorType::Rgba8 => {
                    let buf = dynamic_image.as_rgba8().unwrap();
                    encoder.encode(buf, width, height, 4)?;
                },
                color_type if color_type.bytes_per_pixel() == 3 && !color_type.has_alpha() => {
                    let buf = dynamic_image.to_rgb8();
                    encoder.encode(&buf, width, height, 3)?;
                }
                color_type if color_type.bytes_per_pixel() == 4 && color_type.has_alpha() => {
                    let buf = dynamic_image.to_rgba8();
                    encoder.encode(&buf, width, height, 4)?;
                },
                _ => return Err(format!("INPUT file has invalid color type {:?}.", color_type).into())
            }
        },
        _ => return Err("One of INPUT or OUTPUT must be a .qoi file.".into())
    }

    Ok(())
}
