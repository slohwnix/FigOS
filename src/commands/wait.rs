use crate::print;
use crate::system::time;

pub fn execute(args: &[u8]) {
    if args.is_empty() {
        print!("\nUsage: wait [secondes]");
        return;
    }

    let mut seconds: u64 = 0;
    for &b in args {
        if b >= b'0' && b <= b'9' {
            seconds = seconds * 10 + (b - b'0') as u64;
        } else {
            
            break; 
        }
    }

    if seconds > 0 {
        print!("\nWaiting for {} seconds...", seconds);
        time::sleep(seconds);
        print!("\nDone!");
    } else {
        print!("\nInvalid duration.");
    }
}