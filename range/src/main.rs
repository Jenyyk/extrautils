unsafe extern "C" {
    pub unsafe fn printf(fmt: *const u8, ...) -> i32;
}

use std::{io::Write};
const FMT: &str = "%d\n";
const FMT_PTR: *const u8 = FMT.as_ptr();

fn main() {
    let mut args = std::env::args();
    let _program = args.next();
    let (start, end): (usize, usize) = match (args.next(), args.next()) {
        (Some(start), Some(end)) => (start.parse().unwrap(), end.parse().unwrap()),
        (Some(end), None) => (0, end.parse().unwrap()),
        (None, _) => {
            print_usage();
            std::process::exit(0);
        }
    };

    let mut stdout = std::io::stdout().lock();
    for i in start..end {
        unsafe {
            printf(FMT_PTR, i as i32);
        }
    }
    stdout.flush().unwrap();
}

fn print_usage() {
    println!("range - Prints a range for shell script for loops");
    println!("Usage:");
    println!("  range 5  --  prints a range from 0 (inclusive) to 5 (exclusive)");
    println!("  range 13 37  --  prints a range from 13 (inclusive) to 37 (exclusive)");
}
