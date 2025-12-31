pub mod help;
pub mod say;
pub mod wait;
pub mod fetch; 
pub mod gpu; 

use crate::print;
use crate::system::GLOBAL_CONSOLE;

static mut COMMAND_BUFFER: [u8; 64] = [0; 64];
static mut BUFFER_IDX: usize = 0;

static mut HISTORY: [[u8; 64]; 10] = [[0; 64]; 10];
static mut HISTORY_COUNT: usize = 0;
static mut HISTORY_POS: i32 = -1;

pub fn handle_key(c: char) {
    unsafe {
        match c {
            '\x11' => navigate_history(-1),
            '\x12' => navigate_history(1),
            _ => {
                if BUFFER_IDX < 64 && c != '\n' {
                    COMMAND_BUFFER[BUFFER_IDX] = c as u8;
                    BUFFER_IDX += 1;
                }
            }
        }
    }
}

unsafe fn navigate_history(direction: i32) {
    if HISTORY_COUNT == 0 { return; }
    let new_pos = if HISTORY_POS == -1 && direction == -1 { 0 } else { HISTORY_POS - direction };
    if new_pos < 0 || new_pos >= HISTORY_COUNT as i32 { return; }
    HISTORY_POS = new_pos;
    if let Some(ref mut c) = GLOBAL_CONSOLE {
        c.clear_current_line();
        let cmd = &HISTORY[HISTORY_POS as usize];
        for i in 0..64 { COMMAND_BUFFER[i] = 0; }
        BUFFER_IDX = 0;
        for &byte in cmd.iter() {
            if byte == 0 { break; }
            c.write_char(byte as char);
            COMMAND_BUFFER[BUFFER_IDX] = byte;
            BUFFER_IDX += 1;
        }
    }
}

pub fn delete_last_char() {
    unsafe {
        if BUFFER_IDX > 0 {
            BUFFER_IDX -= 1;
            COMMAND_BUFFER[BUFFER_IDX] = 0;
        }
    }
}

pub fn process_command() {
    unsafe {
        if BUFFER_IDX == 0 { return; }
        let cmd_line = &COMMAND_BUFFER[..BUFFER_IDX];

        if HISTORY_COUNT == 0 || &HISTORY[0][..cmd_line.len()] != cmd_line {
            for i in (1..10).rev() { HISTORY[i] = HISTORY[i-1]; }
            HISTORY[0] = [0; 64];
            HISTORY[0][..cmd_line.len()].copy_from_slice(cmd_line);
            if HISTORY_COUNT < 10 { HISTORY_COUNT += 1; }
        }
        HISTORY_POS = -1;

        let mut split_idx = BUFFER_IDX;
        for (i, &b) in cmd_line.iter().enumerate() {
            if b == b' ' { split_idx = i; break; }
        }

        let cmd_name = &cmd_line[..split_idx];
        let args = if split_idx < BUFFER_IDX { &cmd_line[split_idx + 1..] } else { b"" };

        match cmd_name {
            b"help"  => help::execute(),
            b"fetch" => fetch::execute(), 
            b"gpu"   => gpu::execute(), 
            b"clear" => { if let Some(ref mut c) = GLOBAL_CONSOLE { c.clear(0x000000); } },
            b"wait"  => wait::execute(args),
            b"say"   => say::execute(args),
            _ => print!("\nUnknown command"),
        }

        BUFFER_IDX = 0;
        for i in 0..64 { COMMAND_BUFFER[i] = 0; }
    }
}