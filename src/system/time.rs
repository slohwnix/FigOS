use x86_64::instructions::port::Port;
use core::sync::atomic::{AtomicU64, Ordering};

static TICKS: AtomicU64 = AtomicU64::new(0);
pub static mut ACTUAL_HZ: u64 = 100;

pub fn init() {
    let divisor: u16 = 11931; 
    let mut cmd_port = Port::new(0x43);
    let mut data_port = Port::new(0x40);

    unsafe {
        cmd_port.write(0x36u8);
        data_port.write((divisor & 0xFF) as u8);
        data_port.write((divisor >> 8) as u8);
    }
}

fn get_rtc_seconds() -> u8 {
    let mut addr_port = Port::<u8>::new(0x70);
    let mut data_port = Port::<u8>::new(0x71);
    unsafe {
        addr_port.write(0x00);
        data_port.read()
    }
}

pub fn calibrate_uefi() {
    let s1 = get_rtc_seconds();
    while get_rtc_seconds() == s1 {}
    
    let start_ticks = get_ticks();
    let s2 = get_rtc_seconds();
    while get_rtc_seconds() == s2 {}
    let end_ticks = get_ticks();

    unsafe {
        ACTUAL_HZ = end_ticks.wrapping_sub(start_ticks);
        if ACTUAL_HZ < 10 { ACTUAL_HZ = 100; }
    }
}

pub fn tick() {
    TICKS.fetch_add(1, Ordering::Relaxed);
}

pub fn get_ticks() -> u64 {
    TICKS.load(Ordering::Relaxed)
}

pub fn sleep(seconds: u64) {
    let start = get_ticks();
    let hz = unsafe { ACTUAL_HZ };
    let wait_ticks = seconds * hz;
    
    while (get_ticks().wrapping_sub(start)) < wait_ticks {
        unsafe {
            core::arch::asm!("sti", "hlt");
        }
    }
}

pub unsafe fn get_rtc_time() -> (u8, u8, u8) {
    fn read_cmos(reg: u8) -> u8 {
        unsafe {
            core::arch::asm!("out 0x70, al", in("al") reg);
            let mut val: u8;
            core::arch::asm!("in al, 0x71", out("al") val);
            val
        }
    }

    let mut s = read_cmos(0x00);
    let mut m = read_cmos(0x02);
    let mut h = read_cmos(0x04);

    s = (s & 0x0F) + ((s / 16) * 10);
    m = (m & 0x0F) + ((m / 16) * 10);
    h = (h & 0x0F) + ((h / 16) * 10);

    (h, m, s)
}