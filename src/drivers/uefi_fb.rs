use crate::assets::{FONT, PSF1_MAGIC, PSF2_MAGIC, Psf1Header, Psf2Header};

pub struct Framebuffer {
    pub addr: *mut u32,
    pub width: usize,
    pub height: usize,
    pub pitch: usize, 
}

impl Framebuffer {
    #[inline(always)]
    pub unsafe fn draw_pixel(&self, x: usize, y: usize, color: u32) {
        if x < self.width && y < self.height {
            *self.addr.add(y * self.pitch + x) = color;
        }
    }

    pub unsafe fn clear(&self, color: u32) {
        let total_pixels = self.pitch * self.height;
        for i in 0..total_pixels {
            *self.addr.add(i) = color;
        }
    }

    pub unsafe fn scroll(&self, lines: usize, char_height: usize, bg_color: Option<u32>) {
        let scroll_height = lines * char_height;
        if scroll_height == 0 || scroll_height >= self.height { return; }
        let keep_height = self.height - scroll_height;
        let pixels_to_move = keep_height * self.pitch;
        let src = self.addr.add(scroll_height * self.pitch);
        let dst = self.addr;
        core::ptr::copy(src, dst, pixels_to_move);
        let clear_start = dst.add(pixels_to_move);
        let pixels_to_clear = scroll_height * self.pitch;
        let color = bg_color.unwrap_or(0);
        for i in 0..pixels_to_clear {
            *clear_start.add(i) = color;
        }
    }

    fn get_glyph_index(c: char) -> usize {
        match c {
            'Ç' => 128, 'ü' => 129, 'é' => 130, 'â' => 131, 'ä' => 132, 'à' => 133,
            'å' => 134, 'ç' => 135, 'ê' => 136, 'ë' => 137, 'è' => 138, 'ï' => 139,
            'î' => 140, 'ì' => 141, 'Ä' => 142, 'Å' => 143, 'É' => 144, 'æ' => 145,
            'Æ' => 146, 'ô' => 147, 'ö' => 148, 'ò' => 149, 'û' => 150, 'ù' => 151,
            'ÿ' => 152, 'Ö' => 153, 'Ü' => 154, 'ø' => 155, '£' => 156, 'Ø' => 157,
            _ => {
                let code = c as usize;
                if code < 128 { code } else { 63 }
            }
        }
    }

    pub unsafe fn draw_char(&self, c: char, x: usize, y: usize, color: u32) {
        self.draw_char_ex(c, x, y, color, None);
    }

    pub unsafe fn draw_char_ex(&self, c: char, x: usize, y: usize, color: u32, bg: Option<u32>) {
        let glyph_index = Self::get_glyph_index(c);
        let bg_color = bg;

        if FONT.starts_with(&PSF1_MAGIC) {
            let header = &*(FONT.as_ptr() as *const Psf1Header);
            let glyph_size = header.char_size as usize;
            let glyph_ptr = FONT.as_ptr().add(4 + glyph_index * glyph_size);
            for row in 0..glyph_size {
                let mut byte = *glyph_ptr.add(row);
                for col in 0..8 {
                    let pixel_color = if (byte & 0x80) != 0 { color } else { 
                        match bg_color { Some(bg) => bg, None => return self.draw_pixel(x + col, y + row, color) }
                    };
                    if bg_color.is_some() || (byte & 0x80) != 0 {
                        self.draw_pixel(x + col, y + row, pixel_color);
                    }
                    byte <<= 1;
                }
            }
        } else if FONT.starts_with(&PSF2_MAGIC) {
            let header = &*(FONT.as_ptr() as *const Psf2Header);
            let glyph_size = header.char_size as usize;
            let glyph_ptr = FONT.as_ptr().add(header.header_size as usize + glyph_index * glyph_size);
            let bytes_per_line = (header.width as usize + 7) / 8;
            for row in 0..header.height as usize {
                for b in 0..bytes_per_line {
                    let mut byte = *glyph_ptr.add(row * bytes_per_line + b);
                    for col in 0..8 {
                        let px = b * 8 + col;
                        if px < header.width as usize {
                            let pixel_color = if (byte & 0x80) != 0 { color } else { 
                                match bg_color { Some(bg) => bg, None => { byte <<= 1; continue; } }
                            };
                            if bg_color.is_some() || (byte & 0x80) != 0 {
                                self.draw_pixel(x + px, y + row, pixel_color);
                            }
                            byte <<= 1;
                        }
                    }
                }
            }
        }
    }
}