use core::mem::size_of;

#[repr(C, packed(2))] 
struct GdtDescriptor {
    size: u16,
    offset: u64,
}

#[repr(C, align(16))]
pub struct Gdt {
    entries: [u64; 3],
}

impl Gdt {
    pub fn new() -> Self {
        let mut entries = [0; 3];
        entries[0] = 0;
        entries[1] = 0x00af9a000000ffff;
        entries[2] = 0x00cf92000000ffff;
        Self { entries }
    }

    pub unsafe fn load(&'static self) {
        static mut GDT_DESC: GdtDescriptor = GdtDescriptor { size: 0, offset: 0 };
        
        GDT_DESC = GdtDescriptor {
            size: (size_of::<[u64; 3]>() - 1) as u16,
            offset: self.entries.as_ptr() as u64,
        };

        core::arch::asm!(
            "lgdt [{desc}]",
            "push 0x08",
            "lea {tmp}, [2f + rip]",
            "push {tmp}",
            "retfq",
            "2:",
            "mov ax, 0x10",
            "mov ds, ax",
            "mov es, ax",
            "mov ss, ax",
            "mov fs, ax",
            "mov gs, ax",
            desc = in(reg) &GDT_DESC,
            tmp = out(reg) _,
        );
    }
}