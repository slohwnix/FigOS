use crate::system::graphic::{Backend, GraphicBackend};

pub struct Console {
    pub backend: Backend,
    pub cursor_x: usize,
    pub cursor_y: usize,
    pub color: u32,
    pub bg_color: u32,
    pub line_start_x: usize,
    pub ticks: u64,
    pub cursor_visible: bool,
}

impl Console {
    pub fn new(backend: Backend) -> Self {
        Self { 
            backend, 
            cursor_x: 20, 
            cursor_y: 20, 
            color: 0xFFFFFF, 
            bg_color: 0x000000,
            line_start_x: 20,
            ticks: 0,
            cursor_visible: true,
        }
    }

    pub fn set_color(&mut self, color: u32) {
        self.color = color;
    }

    pub fn set_colors(&mut self, fg: u32, bg: u32) {
        self.color = fg;
        self.bg_color = bg;
    }

    pub fn lock_prompt(&mut self) {
        self.line_start_x = self.cursor_x;
    }

    fn draw_cursor(&mut self, color: u32) {
        for dy in 17..19 {
            for dx in 0..8 {
                self.backend.draw_pixel(self.cursor_x + dx, self.cursor_y + dy, color);
            }
        }
    }

    pub fn update(&mut self) {
        self.ticks += 1;
        if self.ticks % 80 == 0 {
            self.cursor_visible = !self.cursor_visible;
            let color = if self.cursor_visible { self.color } else { self.bg_color };
            self.draw_cursor(color);
            self.backend.swap_rect(self.cursor_x, self.cursor_y + 17, 8, 2);
        }
    }
    
    fn internal_write_char(&mut self, c: char) {
        if c == '\n' {
            self.new_line_no_swap();
        } else {
            self.backend.draw_char(c, self.cursor_x, self.cursor_y, self.color, Some(self.bg_color));
            self.cursor_x += 9;
            if self.cursor_x + 9 >= self.backend.width() {
                self.new_line_no_swap();
            }
        }
    }

    pub fn write_char(&mut self, c: char) {
        self.draw_cursor(self.bg_color); 
        self.internal_write_char(c);
        self.cursor_visible = true;
        self.draw_cursor(self.color);
        self.backend.swap_rect(0, self.cursor_y, self.backend.width(), 20);
    }

    pub fn write_str(&mut self, s: &str) {
        let mut scrolled = false;
        let start_y = self.cursor_y;
        self.draw_cursor(self.bg_color);
        for c in s.chars() {
            if c == '\n' {
                self.new_line_no_swap();
                scrolled = true;
            } else {
                self.backend.draw_char(c, self.cursor_x, self.cursor_y, self.color, Some(self.bg_color));
                self.cursor_x += 9;
                if self.cursor_x + 9 >= self.backend.width() {
                    self.new_line_no_swap();
                    scrolled = true;
                }
            }
        }
        self.cursor_visible = true;
        self.draw_cursor(self.color);
        if scrolled {
            self.backend.swap_buffers();
        } else {
            self.backend.swap_rect(0, start_y, self.backend.width(), 20);
        }
    }

    pub fn backspace(&mut self) {
        if self.cursor_x > self.line_start_x {
            self.draw_cursor(self.bg_color);
            self.cursor_x -= 9;
            for y in 0..20 {
                for x in 0..9 {
                    self.backend.draw_pixel(self.cursor_x + x, self.cursor_y + y, self.bg_color);
                }
            }
            self.cursor_visible = true;
            self.draw_cursor(self.color);
            self.backend.swap_rect(self.cursor_x, self.cursor_y, 18, 20);
        }
    }

    pub fn clear_current_line(&mut self) {
        self.draw_cursor(self.bg_color);
        while self.cursor_x > self.line_start_x {
            self.cursor_x -= 9;
            for y in 0..20 {
                for x in 0..9 { self.backend.draw_pixel(self.cursor_x + x, self.cursor_y + y, self.bg_color); }
            }
        }
        self.cursor_visible = true;
        self.draw_cursor(self.color);
        self.backend.swap_rect(0, self.cursor_y, self.backend.width(), 20);
    }

    fn new_line_no_swap(&mut self) {
        self.cursor_x = 20;
        let line_height = 20;
        let fb_height = self.backend.height();
        if self.cursor_y + line_height >= fb_height {
            self.backend.scroll(1, line_height, Some(self.bg_color));
            self.cursor_y = fb_height - line_height;
        } else {
            self.cursor_y += line_height;
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.bg_color = color;
        self.backend.clear(color);
        self.cursor_x = 20;
        self.cursor_y = 20;
        self.cursor_visible = true;
        self.draw_cursor(self.color);
        self.backend.swap_buffers();
    }

    pub fn flush(&self) {
        self.backend.swap_buffers();
    }
}