use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    unsafe {
        if let Some(ref mut c) = crate::system::GLOBAL_CONSOLE {
            crate::clear_screen!(0xFF0000); 

            c.cursor_x = 50;
            c.cursor_y = 50;
            c.set_color(0xFFFFFF);

            crate::print!("A problem has been detected and FigOS has been shut down to prevent damage\n");
            crate::print!("to your computer.\n\n");

            crate::print!("KERNEL_PANIC_CRITICAL_ERROR\n\n");

            crate::print!("REASON:\n");
            crate::print!("  {}\n\n", info.message());

            if let Some(location) = info.location() {
                crate::print!("LOCATION:\n");
                crate::print!("  File: {}\n", location.file());
                crate::print!("  Line: {}\n\n", location.line());
            }

            crate::print!("--------------------------------------------------------------------------\n");
            crate::print!("TECHNICAL INFORMATION:\n\n");
            
            
            let mut brand_string = [0u8; 48];
            
            for (i, &leaf) in [0x80000002u32, 0x80000003, 0x80000004].iter().enumerate() {
                let (eax, ebx, ecx, edx): (u32, u32, u32, u32);
                core::arch::asm!(
                    "push rbx",
                    "cpuid",
                    "mov {0:e}, ebx",
                    "pop rbx",
                    out(reg) ebx,
                    lateout("eax") eax,
                    lateout("ecx") ecx,
                    lateout("edx") edx,
                    in("eax") leaf,
                    options(nostack, nomem, preserves_flags)
                );

                let offset = i * 16;
                brand_string[offset..offset+4].copy_from_slice(&eax.to_le_bytes());
                brand_string[offset+4..offset+8].copy_from_slice(&ebx.to_le_bytes());
                brand_string[offset+8..offset+12].copy_from_slice(&ecx.to_le_bytes());
                brand_string[offset+12..offset+16].copy_from_slice(&edx.to_le_bytes());
            }

            crate::print!("PROCESSOR:\n  ");
            for &b in brand_string.iter() {
                if b != 0 { 
                    crate::print!("{}", b as char);
                }
            }
            crate::print!("\n\n");
            
            crate::print!("*** STOP: 0x0000001E (0xFFFFFFFFC0000005, 0xFFFFF80002A5D5A2)\n");
            crate::print!("--------------------------------------------------------------------------\n");
            crate::print!("The system has halted. Please restart your machine manually.\n");
        }
    }

    loop {
        unsafe { core::arch::asm!("hlt"); }
    }
}