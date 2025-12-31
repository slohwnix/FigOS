use core::mem::{size_of, MaybeUninit};
use crate::drivers::keyboard::Keyboard;
use crate::system::apic::lapic_eoi;
use crate::system::time;

#[repr(C)]
pub struct InterruptStackFrame {
    pub instruction_pointer: u64,
    pub code_segment: u64,
    pub cpu_flags: u64,
    pub stack_pointer: u64,
    pub stack_segment: u64,
}

#[repr(C, packed(2))]
struct IdtDescriptor {
    size: u16,
    offset: u64,
}

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    ist: u8,
    flags: u8,
    offset_mid: u16,
    offset_high: u32,
    reserved: u32,
}

impl IdtEntry {
    pub fn new(handler: *const (), selector: u16, ist: u8) -> Self {
        let offset = handler as u64;
        Self {
            offset_low: offset as u16,
            selector,
            ist,
            flags: 0x8E,
            offset_mid: (offset >> 16) as u16,
            offset_high: (offset >> 32) as u32,
            reserved: 0,
        }
    }
}

#[repr(C, align(16))]
pub struct Idt([IdtEntry; 256]);

impl Idt {
    pub fn new() -> Self {
        let mut idt = Self([IdtEntry::new(generic_handler as *const (), 0x08, 0); 256]);

        idt.set_handler(3, breakpoint_handler as *const (), 0);
        idt.set_handler(8, double_fault_handler as *const (), 1);
        idt.set_handler(13, general_protection_fault_handler as *const (), 0);
        idt.set_handler(14, page_fault_handler as *const (), 0);

        idt.set_handler(32, timer_handler as *const (), 0);
        idt.set_handler(33, keyboard_handler as *const (), 0);

        idt.set_handler(255, spurious_handler as *const (), 0);

        idt
    }

    fn set_handler(&mut self, index: u8, handler: *const (), ist: u8) {
        self.0[index as usize] = IdtEntry::new(handler, 0x08, ist);
    }

    pub unsafe fn load(&'static self) {
        let ptr = self.0.as_ptr() as u64;
        let desc = IdtDescriptor {
            size: (size_of::<Self>() - 1) as u16,
            offset: ptr,
        };
        core::arch::asm!("lidt [{}]", in(reg) &desc, options(nostack, preserves_flags));
    }
}

pub extern "x86-interrupt" fn generic_handler(_frame: InterruptStackFrame) {
    unsafe { lapic_eoi(); }
}

pub extern "x86-interrupt" fn timer_handler(_frame: InterruptStackFrame) {
    time::tick();
    unsafe { lapic_eoi(); }
}

pub extern "x86-interrupt" fn keyboard_handler(_frame: InterruptStackFrame) {
    let scancode = unsafe { Keyboard::read_scancode() };
    Keyboard::handle_scancode(scancode);
    unsafe { lapic_eoi(); }
}

pub extern "x86-interrupt" fn spurious_handler(_frame: InterruptStackFrame) {}

pub extern "x86-interrupt" fn breakpoint_handler(_frame: InterruptStackFrame) {
    panic!("BREAKPOINT Exception");
}

pub extern "x86-interrupt" fn double_fault_handler(_frame: InterruptStackFrame, _err: u64) -> ! {
    panic!("DOUBLE FAULT Exception");
}

pub extern "x86-interrupt" fn general_protection_fault_handler(_frame: InterruptStackFrame, _err: u64) {
    panic!("GENERAL PROTECTION FAULT");
}

pub extern "x86-interrupt" fn page_fault_handler(_frame: InterruptStackFrame, _err: u64) {
    panic!("PAGE FAULT");
}
#[allow(dead_code)]
pub static mut IDT: MaybeUninit<Idt> = MaybeUninit::uninit();