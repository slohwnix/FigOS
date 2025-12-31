use crate::print;
use crate::system::GLOBAL_CONSOLE;
use crate::system::time;
use crate::MM_INSTANCE;
use crate::system::graphic::{GraphicBackend, Backend};

fn get_cpu_name() -> &'static str {
    use core::arch::x86_64::__cpuid;
    let res = unsafe { __cpuid(0x80000000) };
    if res.eax < 0x80000004 { return "Generic x86_64 CPU"; }
    static mut BRAND_STRING: [u8; 48] = [0; 48];
    unsafe {
        for i in 0..3 {
            let res = __cpuid(0x80000002 + i);
            let offset = i as usize * 16;
            BRAND_STRING[offset..offset+4].copy_from_slice(&res.eax.to_le_bytes());
            BRAND_STRING[offset+4..offset+8].copy_from_slice(&res.ebx.to_le_bytes());
            BRAND_STRING[offset+8..offset+12].copy_from_slice(&res.ecx.to_le_bytes());
            BRAND_STRING[offset+12..offset+16].copy_from_slice(&res.edx.to_le_bytes());
        }
        let mut end = 48;
        for i in 0..48 { if BRAND_STRING[i] == 0 { end = i; break; } }
        while end > 0 && BRAND_STRING[end - 1] == b' ' { end -= 1; }
        let mut start = 0;
        while start < end && BRAND_STRING[start] == b' ' { start += 1; }
        core::str::from_utf8_unchecked(&BRAND_STRING[start..end])
    }
}

pub fn execute() {
    unsafe {
        if let Some(ref mut c) = GLOBAL_CONSOLE {
            let width = c.backend.width();
            let height = c.backend.height();
            let cpu_model = get_cpu_name();
            let (h, m, s) = time::get_rtc_time();

            let backend_name = match c.backend {
                Backend::Uefi(_) => "UEFI GOP",
                Backend::Gpu(_)  => "Natif GPU",
            };

            let mut used_mb = 0;
            let mut total_mb = 0;
            if let Some(mm_mutex) = MM_INSTANCE.get() {
                let mm = mm_mutex.lock();
                used_mb = mm.get_used_memory_kb() / 1024;
                total_mb = mm.get_total_memory_kb() / 1024;
            }

            let yellow = 0xFFFF00;
            let white = 0xFFFFFF;

            c.set_color(yellow);
            print!("\n---   ");
            c.set_color(white);
            print!("FigOS");
            c.set_color(yellow);
            print!("   ---");

            print!("\n");
            c.set_color(yellow);
            print!("CPU:         ");
            c.set_color(white);
            print!("{}", cpu_model);

            print!("\n");
            c.set_color(yellow);
            print!("Backend:     "); 
            c.set_color(white);
            print!("{}", backend_name);

            print!("\n");
            c.set_color(yellow);
            print!("Resolution:  ");
            c.set_color(white);
            print!("{}x{}", width, height);

            print!("\n");
            c.set_color(yellow);
            print!("Memory:      ");
            c.set_color(white);
            print!("{}MB / {}MB", used_mb, total_mb);

            print!("\n");
            c.set_color(yellow);
            print!("Time:        ");
            c.set_color(white);
            print!("{:02}:{:02}:{:02}", h, m, s);

            print!("\n");
            c.set_color(yellow);
            print!("---");
            c.set_color(white);
            print!("------------");
            c.set_color(yellow);
            print!("---");
            c.set_color(white);
        }
    }
}