pub const FONT: &[u8] = include_bytes!("font.psf");

pub const PSF1_MAGIC: [u8; 2] = [0x36, 0x04];
pub const PSF2_MAGIC: [u8; 4] = [0x72, 0xb5, 0x4a, 0x86];

#[repr(C)]
pub struct Psf1Header {
    pub magic: [u8; 2],
    pub mode: u8,
    pub char_size: u8,
}

#[repr(C)]
pub struct Psf2Header {
    pub magic: [u8; 4],
    pub version: u32,
    pub header_size: u32,
    pub flags: u32,
    pub length: u32,
    pub char_size: u32,
    pub height: u32,
    pub width: u32,
}