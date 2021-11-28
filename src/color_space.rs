
use crate::consts::QoiConsts;

pub enum ColorSpace {
    Srgb,
    SrgbLinearAlpha,
    Linear,

    Custom(bool, bool, bool, bool),
    Unknown(u8)
}

impl From<u8> for ColorSpace {
    fn from(byte: u8) -> Self {
        match byte {
            QoiConsts::SRGB => ColorSpace::Srgb,
            QoiConsts::SRGB_LINEAR_ALPHA => ColorSpace::SrgbLinearAlpha,
            QoiConsts::LINEAR => ColorSpace::Linear,
            byte if byte & 0xF0 != 0 => ColorSpace::Unknown(byte),
            _ => {
                ColorSpace::Custom(
                    byte & QoiConsts::LINEAR_R != 0,
                    byte & QoiConsts::LINEAR_G != 0,
                    byte & QoiConsts::LINEAR_B != 0,
                    byte & QoiConsts::LINEAR_A != 0
                )
            }
        }
    }
}

impl From<ColorSpace> for u8 {
    fn from(color_space: ColorSpace) -> u8 {
        match color_space {
            ColorSpace::Srgb => QoiConsts::SRGB,
            ColorSpace::SrgbLinearAlpha => QoiConsts::SRGB_LINEAR_ALPHA,
            ColorSpace::Linear => QoiConsts::LINEAR,
            ColorSpace::Custom(r, g, b, a) => {
                (if r { QoiConsts::LINEAR_R } else { 0 })
                | (if g { QoiConsts::LINEAR_G } else { 0 })
                | (if b { QoiConsts::LINEAR_B } else { 0 })
                | (if a { QoiConsts::LINEAR_A } else { 0 })
            },
            ColorSpace::Unknown(byte) => byte,
        }
    }
}
