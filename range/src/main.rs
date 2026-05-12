use std::io::Write;

fn main() {
    let mut args = std::env::args();
    let _program = args.next();
    let (start, end, step): (usize, usize, usize) = match (args.next(), args.next(), args.next()) {
        (Some(start), Some(end), Some(step)) => (
            start.parse().unwrap(),
            end.parse().unwrap(),
            step.parse().unwrap(),
        ),
        (Some(start), Some(end), None) => (start.parse().unwrap(), end.parse().unwrap(), 1),
        (Some(end), None, _) => (0, end.parse().unwrap(), 1),
        (None, _, _) => {
            print_usage();
            std::process::exit(0);
        }
    };

    let stdout = std::io::stdout().lock();
    let mut writer = std::io::BufWriter::new(stdout);
    const MAX_NUM_LEN: usize = 39;
    let mut buf = [b'\n'; MAX_NUM_LEN];
    let mut i = 0;
    let mut skip_str_len = 1usize;
    loop {
        let mut curr_mut = start + i * step;
        if curr_mut >= end {
            break;
        }

        let mut str_len = 0usize;
        loop {
            let digit_u8 = usize_to_char_as_u8(curr_mut % 10);
            let indexed = &mut buf[MAX_NUM_LEN - str_len - 2];
            if *indexed == digit_u8 {
                str_len = skip_str_len;
                break;
            } else {
                *indexed = digit_u8;
            }
            str_len += 1;
            curr_mut /= 10;
            if curr_mut == 0 {
                skip_str_len = str_len;
                break;
            }
        }
        writer.write_all(&buf[MAX_NUM_LEN - str_len - 1..]).unwrap();
        i += 1;
    }
    writer.flush().unwrap();
}

/// panics if the argument isnt a single digit
#[inline]
fn usize_to_char_as_u8(num: usize) -> u8 {
    match num {
        0 => b'0',
        1 => b'1',
        2 => b'2',
        3 => b'3',
        4 => b'4',
        5 => b'5',
        6 => b'6',
        7 => b'7',
        8 => b'8',
        9 => b'9',
        _ => unreachable!(),
    }
}

fn print_usage() {
    println!("range - Prints a range for shell script for loops");
    println!("Usage:");
    println!("  range 5  --  prints a range from 0 (inclusive) to 5 (exclusive)");
    println!("  range 13 37  --  prints a range from 13 (inclusive) to 37 (exclusive)");
    println!("  range 4 10 2 -- prints a range from 4 to 10, incrementing by 2");
}
