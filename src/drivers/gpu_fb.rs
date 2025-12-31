#[allow(dead_code)]
use core::ptr;
use crate::assets::{FONT, PSF1_MAGIC, PSF2_MAGIC, Psf1Header, Psf2Header};

pub struct Framebuffer {
    pub fb_addr: *mut u32,
    pub back_buffer: Option<&'static mut [u32]>,
    pub width: usize,
    pub height: usize,
    pub pitch: usize,
}

impl Framebuffer {
    #[inline(always)]
    fn buffer_ptr(&self) -> *mut u32 {
        self.fb_addr
    }

    #[inline(always)]
    pub fn draw_pixel(&self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            unsafe {
                *self.fb_addr.add(y * self.pitch + x) = color;
            }
        }
    }

    pub fn clear(&self, color: u32) {
        let buf = self.fb_addr;
        let total = self.pitch * self.height;
        if total == 0 { return; }
        unsafe {
            for i in 0..total {
                *buf.add(i) = color;
            }
        }
    }

    pub fn swap_buffers(&self) {}
    pub fn swap_rect(&self, _x: usize, _y: usize, _w: usize, _h: usize) {}

    pub fn draw_char(&self, c: char, x: usize, y: usize, color: u32) {
        self.draw_char_ex(c, x, y, color, None);
    }

    pub fn draw_char_ex(&self, c: char, x: usize, y: usize, color: u32, bg: Option<u32>) {
        if x + 8 >= self.width || y + 16 >= self.height { return; }
        let glyph = c as usize;
        let buf = self.fb_addr;
        let bg_color = bg.unwrap_or(0);

        unsafe {
            if FONT.starts_with(&PSF1_MAGIC) {
                let header = &*(FONT.as_ptr() as *const Psf1Header);
                let glyph_size = header.char_size as usize;
                let glyph_ptr = FONT.as_ptr().add(4 + glyph * glyph_size);
                let mut row: [u32; 8] = [0; 8];

                for r in 0..glyph_size {
                    let byte = *glyph_ptr.add(r);
                    for i in 0..8 {
                        row[i] = if (byte << i) & 0x80 != 0 { color } else { bg_color };
                    }
                    let dst = buf.add((y + r) * self.pitch + x);
                    ptr::copy_nonoverlapping(row.as_ptr(), dst, 8);
                }
            }

            if FONT.starts_with(&PSF2_MAGIC) {
                let header = &*(FONT.as_ptr() as *const Psf2Header);
                let char_size = header.char_size as usize;
                let glyph_ptr = FONT.as_ptr().add(header.header_size as usize + glyph * char_size);
                let width = header.width as usize;
                let height = header.height as usize;
                let bytes_per_line = (width + 7) / 8;

                const MAX_WIDTH: usize = 64;
                if width > MAX_WIDTH { return; }
                let mut row: [u32; MAX_WIDTH] = [0; MAX_WIDTH];

                for r in 0..height {
                    for b in 0..bytes_per_line {
                        let byte = *glyph_ptr.add(r * bytes_per_line + b);
                        for i in 0..8 {
                            let px = b * 8 + i;
                            if px < width {
                                row[px] = if (byte << i) & 0x80 != 0 { color } else { bg_color };
                            }
                        }
                    }
                    let dst = buf.add((y + r) * self.pitch + x);
                    ptr::copy_nonoverlapping(row.as_ptr(), dst, width);
                }
            }
        }
    }

    pub fn scroll(&self, lines: usize, char_h: usize, bg: Option<u32>) {
        let px = lines * char_h;
        if px == 0 || px >= self.height { return; }
        let buf = self.fb_addr;
        let total = self.pitch * self.height;
        let keep = (self.height - px) * self.pitch;
        unsafe {
            ptr::copy(buf.add(px * self.pitch), buf, keep);
            let color = bg.unwrap_or(0);
            for i in keep..total {
                *buf.add(i) = color;
            }
        }
    }
}