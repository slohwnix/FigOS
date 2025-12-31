use crate::system;

static mut SHIFT_PRESSED: bool = false;
static mut CAPS_LOCK: bool = false;
static mut ESCAPED: bool = false;

pub struct Keyboard;

impl Keyboard {
    pub fn handle_scancode(scancode: u8) {
        unsafe {
            
            if scancode == 0xE0 {
                ESCAPED = true;
                return;
            }

            
            
            if ESCAPED {
                ESCAPED = false;
                if scancode < 0x80 { 
                    match scancode {
                        0x48 => system::push_key('\x11'), 
                        0x50 => system::push_key('\x12'), 
                        _ => {}
                    }
                }
                return; 
            }

            
            match scancode {
                
                0x2A | 0x36 => SHIFT_PRESSED = true,
                0xAA | 0xB6 => SHIFT_PRESSED = false,
                0x3A => CAPS_LOCK = !CAPS_LOCK,
                
                
                0x0E => system::push_key('\x08'),
                0x1C => system::push_key('\n'),

                
                s if s < 0x80 => {
                    let keymap = include_bytes!("../assets/keymap.bin");
                    
                    let is_letter = (0x10..0x32).contains(&s);
                    let use_shift = if is_letter {
                        SHIFT_PRESSED ^ CAPS_LOCK
                    } else {
                        SHIFT_PRESSED
                    };

                    let offset = if use_shift { 256 } else { 0 };
                    
                    
                    if (s as usize + offset) < keymap.len() {
                        let ascii = keymap[s as usize + offset];
                        if ascii != 0 {
                            system::push_key(ascii as char);
                        }
                    }
                }
                
                _ => {}
            }
        }
    }

    pub unsafe fn read_scancode() -> u8 {
        let scancode: u8;
        core::arch::asm!("in al, dx", out("al") scancode, in("dx") 0x60u16);
        scancode
    }
}