use crate::system::GLOBAL_CONSOLE;
use crate::system::graphic::{Backend, GraphicBackend};
use crate::drivers::gpu_fb::Framebuffer;
use crate::print;
use crate::MM_INSTANCE;

pub fn execute() {
    unsafe {
        if let Some(ref mut c) = GLOBAL_CONSOLE {
            let target_width = c.backend.width();
            let target_height = c.backend.height();
            let target_pitch = c.backend.pitch();
            let phys_addr = c.backend.addr();
            let mut back_buffer_slice: Option<&'static mut [u32]> = None;
            
            if let Some(mm_mutex) = MM_INSTANCE.get() {
                let mut mm = mm_mutex.lock();
                
                let bytes_needed = target_width * target_height * 4;
                let frames_needed = (bytes_needed + 4095) / 4096;

                if let Some(ptr) = mm.alloc_frames(frames_needed) {
                    back_buffer_slice = Some(core::slice::from_raw_parts_mut(
                        ptr as *mut u32,
                        target_width * target_height
                    ));
                    print!(" OK!");
                } else {
                    print!(" FAILED (Not enough memory)");
                    return;
                }
            }

            c.backend = Backend::Gpu(Framebuffer {
                fb_addr: phys_addr,
                back_buffer: back_buffer_slice,
                width: target_width,
                height: target_height,
                pitch: target_pitch,
            });
            
            c.cursor_x = 0;
            c.cursor_y = 0;
            c.clear(0x000000);
            
            c.set_color(0x00FF00);
            print!("GPU Backend Active");
            c.set_color(0xFFFFFF);
            
            c.backend.swap_buffers();
        }
    }
}