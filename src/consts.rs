
pub struct QoiConsts;

impl QoiConsts {
    pub const MAGIC: [u8; Self::MAGIC_LEN] = *b"qoif";

    pub const INDEX: u8 = 0b00000000;
    pub const RUN_8: u8 = 0b01000000;
    pub const RUN_16: u8 = 0b01100000;
    pub const DIFF_8: u8 = 0b10000000;
    pub const DIFF_16: u8 = 0b11000000;
    pub const DIFF_24: u8 = 0b11100000;
    pub const COLOR: u8 = 0b11110000;

    pub const COLOR_R: u8 = 0b00001000;
    pub const COLOR_G: u8 = 0b00000100;
    pub const COLOR_B: u8 = 0b00000010;
    pub const COLOR_A: u8 = 0b00000001;

    pub const MASK_2: u8 = 0b11000000;
    pub const MASK_3: u8 = 0b11100000;
    pub const MASK_4: u8 = 0b11110000;

    pub const SRGB: u8 = 0b00000000;
    pub const SRGB_LINEAR_ALPHA: u8 = 0x00000001;
    pub const LINEAR: u8 = 0b00001111;
    pub const LINEAR_R: u8 = 0b00001000;
    pub const LINEAR_G: u8 = 0b00000100;
    pub const LINEAR_B: u8 = 0b00000010;
    pub const LINEAR_A: u8 = 0b00000001;

    pub const MAGIC_LEN: usize = 4;
    pub const INDEX_SIZE: usize = 64;
    pub const PADDING_LENGTH: usize = 4;
    pub const PADDING:[u8; Self::PADDING_LENGTH] = [0; Self::PADDING_LENGTH];

    pub const CHANNELS_MIN: u8 = 3;
    pub const CHANNELS_MAX: u8 = 4;

    #[inline(always)]
    pub fn pixel_hash(pixel: &[u8]) -> usize {
        pixel.iter().fold(0, |a, c| a ^ c) as usize % Self::INDEX_SIZE
    }
}
