pub mod time;
pub mod console;
pub mod gdt;
pub mod idt;
pub mod memory;
pub mod apic;
pub mod panic;
pub mod graphic;

use console::Console;
use core::fmt;
use spin::Mutex;

pub static mut GLOBAL_CONSOLE: Option<Console> = None;

struct KeyQueue {
    buffer: [char; 128],
    write_ptr: usize,
    read_ptr: usize,
}

static KEYBOARD_QUEUE: Mutex<KeyQueue> = Mutex::new(KeyQueue {
    buffer: ['\0'; 128],
    write_ptr: 0,
    read_ptr: 0,
});

pub fn push_key(c: char) {
    let mut queue = KEYBOARD_QUEUE.lock();
    let idx = queue.write_ptr % 128;
    queue.buffer[idx] = c;
    queue.write_ptr += 1;
}

pub fn pop_key() -> Option<char> {
    let mut queue = KEYBOARD_QUEUE.lock();
    if queue.read_ptr >= queue.write_ptr {
        return None;
    }
    let c = queue.buffer[queue.read_ptr % 128];
    queue.read_ptr += 1;
    Some(c)
}

pub fn print_fmt(args: fmt::Arguments) {
    use core::fmt::Write;
    unsafe {
        if let Some(ref mut c) = GLOBAL_CONSOLE {
            let _ = c.write_fmt(args);
        }
    }
}

pub fn get_status_color(status: &str) -> u32 {
    match status {
        "OK" => 0x00FF00,
        "WARN" | "INFO" => 0xFFFF00,
        "ERROR" => 0xFF0000,
        _ => 0xFFFFFF,
    }
}

impl fmt::Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::system::print_fmt(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! clear_screen {
    ($color:expr) => {
        if let Some(ref mut c) = unsafe { $crate::system::GLOBAL_CONSOLE.as_mut() } {
            c.clear($color);
        }
    };
}

#[macro_export]
macro_rules! log {
    ($status:expr, $($arg:tt)*) => {
        unsafe {
            if let Some(ref mut c) = $crate::system::GLOBAL_CONSOLE {
                let s: &str = $status;
                let color = $crate::system::get_status_color(s);
                c.set_color(color);
                $crate::print!("[ {} ] ", s);
                c.set_color(0xFFFFFF);
                $crate::print!($($arg)*);
                $crate::print!("\n");
            }
        }
    };
}