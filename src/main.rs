#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]

mod assets;
mod drivers;
mod system;
mod commands;

use uefi::prelude::*;
use spin::{Mutex, Once};

use crate::system::gdt::Gdt;
use crate::system::idt::Idt;
use crate::system::memory::MemoryManager;
use crate::system::console::Console;
use crate::system::apic::{init_lapic, IoApic};
use crate::system::time;
use crate::system::graphic::Backend;
use uefi::boot::MemoryType;
use uefi::mem::memory_map::MemoryMap;

static GDT_INSTANCE: Once<Gdt> = Once::new();
static IDT_INSTANCE: Once<Idt> = Once::new();
pub static MM_INSTANCE: Once<Mutex<MemoryManager>> = Once::new();

#[entry]
fn main() -> Status {
    uefi::helpers::init().unwrap();

    let gop_handle = uefi::boot::get_handle_for_protocol::<uefi::proto::console::gop::GraphicsOutput>()
        .expect("Failed to get GOP handle");
    let mut gop = uefi::boot::open_protocol_exclusive::<uefi::proto::console::gop::GraphicsOutput>(gop_handle)
        .expect("Failed to open GOP");

    let mut max_resolution = (0usize, 0usize);
    let mut best_mode = None;
    for mode in gop.modes() {
        let info = mode.info();
        let (w, h) = info.resolution();
        if w * h > max_resolution.0 * max_resolution.1 {
            max_resolution = (w, h);
            best_mode = Some(mode);
        }
    }
    if let Some(mode) = best_mode {
        let _ = gop.set_mode(&mode);
    }

    let mode_info = gop.current_mode_info();
    let (width, height) = mode_info.resolution();
    let stride = mode_info.stride();
    let fb_ptr = gop.frame_buffer().as_mut_ptr();

    unsafe {
        system::GLOBAL_CONSOLE = Some(Console::new(
            Backend::Uefi(crate::drivers::uefi_fb::Framebuffer {
                addr: fb_ptr as *mut u32,
                width,
                height,
                pitch: stride,
            })
        ));
        if let Some(ref mut console) = system::GLOBAL_CONSOLE {
            console.clear(0x000000);
        }
    }

    log!("OK", "FigOS Kernel booting");
    log!("INFO", "Max Resolution set : {}x{}", width, height);

    log!("INFO", "Initializing GDT...");
    let gdt = GDT_INSTANCE.call_once(|| Gdt::new());
    unsafe { gdt.load(); }
    log!("OK", "GDT loaded successfully");

    log!("INFO", "Initializing IDT...");
    let idt = IDT_INSTANCE.call_once(|| Idt::new());
    unsafe { idt.load(); }
    log!("OK", "IDT loaded successfully");

    log!("INFO", "Initializing LAPIC...");
    unsafe { init_lapic(); }
    log!("OK", "LAPIC initialized successfully");

    log!("INFO", "Initializing IO APIC...");
    let _ioapic = unsafe { IoApic::init() };
    log!("OK", "IO APIC initialized successfully");

    log!("OK", "Enabling interrupts...");
    unsafe { core::arch::asm!("sti"); }

    log!("INFO", "Initializing time subsystem...");
    time::init();
    time::calibrate_uefi();
    log!("OK", "Time subsystem ready");

    log!("OK", "Fetching memory map...");
    let memory_map = uefi::boot::memory_map(MemoryType::LOADER_DATA)
        .expect("Failed to get memory map");

    let mut max_addr = 0usize;
    for desc in memory_map.entries() {
        let end = (desc.phys_start as usize).saturating_add((desc.page_count as usize) * 4096);
        if end > max_addr {
            max_addr = end;
        }
    }

    let total_frames = (max_addr + 4096 - 1) / 4096;
    let bitmap_size = (total_frames + 7) / 8;

    let mut bitmap_addr: *mut u8 = core::ptr::null_mut();
    for desc in memory_map.entries() {
        if desc.ty == MemoryType::CONVENTIONAL && (desc.page_count as usize * 4096) >= bitmap_size {
            bitmap_addr = desc.phys_start as *mut u8;
            break;
        }
    }

    if bitmap_addr.is_null() {
        log!("ERROR", "Could not find a free spot for the memory bitmap!");
        panic!("Memory bitmap allocation failed");
    }

    log!("INFO", "Initializing Memory Manager...");
    let mut mm = unsafe { MemoryManager::new(bitmap_addr, max_addr) };

    for desc in memory_map.entries() {
        if desc.ty == MemoryType::CONVENTIONAL
            || desc.ty == MemoryType::BOOT_SERVICES_CODE
            || desc.ty == MemoryType::BOOT_SERVICES_DATA
        {
            let phys = desc.phys_start as usize;
            let size = (desc.page_count as usize) * 4096;
            mm.free_region(phys, size);
        }
    }

    let bitmap_frame_count = (bitmap_size + 4095) / 4096;
    for _ in 0..bitmap_frame_count {
        mm.alloc_frames(1);
    }

    unsafe {
        core::arch::asm!("cli");
        core::arch::asm!("out 0x21, al", in("al") 0xFFu8);
        core::arch::asm!("out 0xA1, al", in("al") 0xFFu8);
        let _ = uefi::boot::exit_boot_services(Some(MemoryType::LOADER_DATA));
        core::arch::asm!("sti");
    }

    MM_INSTANCE.call_once(|| Mutex::new(mm));
    log!("OK", "Memory Manager ready");

    log!("OK", "Kernel ready");
    log!("OK", "Keyboard subsystem ready");

    print!("> ");
    unsafe {
        if let Some(ref mut console) = system::GLOBAL_CONSOLE {
            console.lock_prompt();
        }
    }

    loop {
        unsafe {
            if let Some(ref mut console) = system::GLOBAL_CONSOLE {
                console.update();
            }
        }

        if let Some(c) = system::pop_key() {
            match c {
                '\n' => {
                    commands::process_command();
                    print!("\n> ");
                    unsafe {
                        if let Some(ref mut console) = system::GLOBAL_CONSOLE {
                            console.lock_prompt();
                        }
                    }
                }
                '\x08' => {
                    unsafe {
                        if let Some(ref mut console) = system::GLOBAL_CONSOLE {
                            console.backspace();
                            commands::delete_last_char();
                        }
                    }
                }
                _ => {
                    if (c >= ' ' && c <= '~') || (c as u8 >= 0x80) {
                        print!("{}", c);
                    }
                    commands::handle_key(c);
                }
            }
        }
        x86_64::instructions::hlt();
    }
}