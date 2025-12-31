pub const FRAME_SIZE: usize = 4096;

pub struct MemoryManager {
    bitmap: *mut u8,
    total_frames: usize,
    usable_frames: usize,
    used_frames: usize,
}

unsafe impl Send for MemoryManager {}
unsafe impl Sync for MemoryManager {}

impl MemoryManager {
    pub unsafe fn new(bitmap_addr: *mut u8, max_phys_addr: usize) -> Self {
        let total_frames = (max_phys_addr + FRAME_SIZE - 1) / FRAME_SIZE;
        let bitmap_size = (total_frames + 7) / 8;

        for i in 0..bitmap_size {
            bitmap_addr.add(i).write(0xFF);
        }

        Self {
            bitmap: bitmap_addr,
            total_frames,
            usable_frames: 0,
            used_frames: 0,
        }
    }

    pub fn free_region(&mut self, phys_addr: usize, size: usize) {
        let start_frame = phys_addr / FRAME_SIZE;
        let frame_count = size / FRAME_SIZE;

        for i in 0..frame_count {
            let idx = start_frame + i;
            if idx < self.total_frames && self.is_used(idx) {
                self.set_used(idx, false);
                self.usable_frames += 1;
            }
        }
    }

    pub fn alloc_frames(&mut self, count: usize) -> Option<*mut u8> {
        let mut run = 0;
        let mut start = 0;

        for i in 0..self.total_frames {
            if !self.is_used(i) {
                if run == 0 { start = i; }
                run += 1;
                if run == count {
                    for j in 0..count {
                        self.set_used(start + j, true);
                    }
                    self.used_frames += count;
                    return Some((start * FRAME_SIZE) as *mut u8);
                }
            } else {
                run = 0;
            }
        }
        None
    }

    pub fn free_frame(&mut self, addr: *mut u8) {
        let idx = (addr as usize) / FRAME_SIZE;
        if idx < self.total_frames && self.is_used(idx) {
            self.set_used(idx, false);
            self.used_frames -= 1;
        }
    }

    pub fn get_total_memory_kb(&self) -> usize {
        self.usable_frames * 4
    }

    pub fn get_used_memory_kb(&self) -> usize {
        self.used_frames * 4
    }

    #[inline]
    fn is_used(&self, index: usize) -> bool {
        let byte = index / 8;
        let bit = index % 8;
        unsafe { (self.bitmap.add(byte).read() & (1 << bit)) != 0 }
    }

    #[inline]
    fn set_used(&mut self, index: usize, used: bool) {
        let byte = index / 8;
        let bit = index % 8;
        unsafe {
            let ptr = self.bitmap.add(byte);
            let val = ptr.read();
            if used {
                ptr.write(val | (1 << bit));
            } else {
                ptr.write(val & !(1 << bit));
            }
        }
    }
}