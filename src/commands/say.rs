use crate::print;

pub fn execute(text: &[u8]) {
    print!("\n");
    for &b in text {
        print!("{}", b as char);
    }
}