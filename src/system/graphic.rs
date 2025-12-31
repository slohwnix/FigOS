#[allow(dead_code)]

use crate::drivers::uefi_fb::Framebuffer as UefiFb;
use crate::drivers::gpu_fb::Framebuffer as GpuFb;

pub enum Backend {
    Uefi(UefiFb),
    Gpu(GpuFb),
}

pub trait GraphicBackend {
    fn width(&self) -> usize;
    fn height(&self) -> usize;
    fn pitch(&self) -> usize;
    fn addr(&self) -> *mut u32;
    fn draw_pixel(&self, x: usize, y: usize, color: u32);
    fn clear(&self, color: u32);
    fn swap_buffers(&self);
    fn swap_rect(&self, x: usize, y: usize, w: usize, h: usize);
    fn set_virtual_res(&mut self, width: usize, height: usize);
    fn draw_char(&self, c: char, x: usize, y: usize, color: u32);
    fn scroll(&self, lines: usize, char_height: usize, bg_color: Option<u32>);
}

impl GraphicBackend for Backend {
    fn width(&self) -> usize {
        match self {
            Backend::Uefi(fb) => fb.width,
            Backend::Gpu(fb) => fb.width,
        }
    }

    fn height(&self) -> usize {
        match self {
            Backend::Uefi(fb) => fb.height,
            Backend::Gpu(fb) => fb.height,
        }
    }

    fn pitch(&self) -> usize {
        match self {
            Backend::Uefi(fb) => fb.pitch,
            Backend::Gpu(fb) => fb.pitch,
        }
    }

    fn addr(&self) -> *mut u32 {
        match self {
            Backend::Uefi(fb) => fb.addr,
            Backend::Gpu(fb) => fb.fb_addr,
        }
    }

    fn draw_pixel(&self, x: usize, y: usize, color: u32) {
        match self {
            Backend::Uefi(fb) => {
                if x < fb.width && y < fb.height {
                    unsafe { *fb.addr.add(y * fb.pitch + x) = color; }
                }
            }
            Backend::Gpu(fb) => fb.draw_pixel(x, y, color),
        }
    }

    fn draw_char(&self, c: char, x: usize, y: usize, color: u32) {
        match self {
            Backend::Uefi(fb) => {
                let temp_fb = GpuFb {
                    fb_addr: fb.addr,
                    back_buffer: None,
                    width: fb.width,
                    height: fb.height,
                    pitch: fb.pitch,
                };
                temp_fb.draw_char(c, x, y, color);
            }
            Backend::Gpu(fb) => fb.draw_char(c, x, y, color),
        }
    }

    fn scroll(&self, lines: usize, char_height: usize, bg_color: Option<u32>) {
        match self {
            Backend::Uefi(fb) => {
                let temp_fb = GpuFb {
                    fb_addr: fb.addr,
                    back_buffer: None,
                    width: fb.width,
                    height: fb.height,
                    pitch: fb.pitch,
                };
                temp_fb.scroll(lines, char_height, bg_color);
            }
            Backend::Gpu(fb) => fb.scroll(lines, char_height, bg_color),
        }
    }

    fn clear(&self, color: u32) {
        match self {
            Backend::Uefi(fb) => {
                for i in 0..(fb.pitch * fb.height) {
                    unsafe { *fb.addr.add(i) = color; }
                }
            }
            Backend::Gpu(fb) => fb.clear(color),
        }
    }

    fn swap_buffers(&self) {
        if let Backend::Gpu(fb) = self {
            fb.swap_buffers();
        }
    }

    fn swap_rect(&self, x: usize, y: usize, w: usize, h: usize) {
        if let Backend::Gpu(fb) = self {
            fb.swap_rect(x, y, w, h);
        }
    }
    
    fn set_virtual_res(&mut self, width: usize, height: usize) {
        match self {
            Backend::Uefi(fb) => { fb.width = width; fb.height = height; }
            Backend::Gpu(fb) => { fb.width = width; fb.height = height; }
        }
    }
}