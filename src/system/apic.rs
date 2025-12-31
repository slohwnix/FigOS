use core::sync::atomic::{AtomicBool, Ordering};

pub struct LocalApic {
    base: usize,
}

static mut LAPIC: Option<LocalApic> = None;
static LAPIC_READY: AtomicBool = AtomicBool::new(false);

impl LocalApic {
    pub unsafe fn init() -> Self {
        let lapic = Self { base: 0xFEE00000 };
        lapic.write(0xF0, lapic.read(0xF0) | 0x100 | 0xFF);

        LAPIC_READY.store(true, Ordering::SeqCst);
        lapic
    }

    pub unsafe fn end_of_interrupt(&self) {
        self.write(0xB0, 0);
    }

    unsafe fn read(&self, reg: usize) -> u32 {
        core::ptr::read_volatile((self.base + reg) as *const u32)
    }

    unsafe fn write(&self, reg: usize, val: u32) {
        core::ptr::write_volatile((self.base + reg) as *mut u32, val);
    }
}

pub unsafe fn init_lapic() {
    let lapic = LocalApic::init();
    LAPIC = Some(lapic);
}

pub unsafe fn lapic_eoi() {
    if LAPIC_READY.load(Ordering::SeqCst) {
        if let Some(ref lapic) = LAPIC {
            lapic.end_of_interrupt();
        }
    }
}

pub struct IoApic {
    base: usize,
}

impl IoApic {
    pub unsafe fn init() -> Self {
        let ioapic = Self { base: 0xFEC00000 };
        ioapic.write_redirection(2, 32); 
        ioapic.write_redirection(1, 33);
        
        ioapic
    }

    unsafe fn write_redirection(&self, irq: u8, vector: u8) {
        let low_reg = 0x10 + (irq as u32) * 2;
        let high_reg = low_reg + 1;
        
        
        self.write(low_reg, vector as u32);
        self.write(high_reg, 0);
    }

    unsafe fn write(&self, reg: u32, val: u32) {
        let ioregsel = self.base as *mut u32;
        let iowin = (self.base + 0x10) as *mut u32;
        core::ptr::write_volatile(ioregsel, reg);
        core::ptr::write_volatile(iowin, val);
    }
}