use crate::print;

pub fn execute() {
    
    print!("\n--- FigOS Help Menu ---");
    print!("\nhelp       : Show this message");
    print!("\nclear      : Clear the screen");
    print!("\nsay [text] : Repeat the text");
    print!("\npanic      : Force a kernel panic");
    print!("\nwait [s]   : Wait for [s] seconds");
    print!("\nfetch      : Show system information");
    print!("\ngpu        : Switch to gpu buffer (beta)");
    print!("\n");
}