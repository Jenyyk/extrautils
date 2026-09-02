use std::fs;
use std::io::{BufReader, Read};

#[derive(Debug, Default)]
struct Config {
    group: bool,
    stdin: bool,
    recursive: bool,
    sort: bool,
}

const READ_SIZE: usize = 65536;
const UNICODE_CHAR_COUNT: usize = 1_114_112;

fn main() {
    let mut config = Config::default();

    let mut files = Vec::new();
    let mut args = std::env::args();
    // rid us of our own invocation
    let _ = args.next();

    for arg in args {
        let arg_str = arg.as_str();
        // parse options, flags, and files
        if arg_str.starts_with("--") {
            parse_option(arg_str, &mut config);
        } else if arg_str.starts_with('-') {
            parse_flags(arg_str, &mut config);
        } else {
            files.push(arg);
        }
    }

    #[cfg(any(debug_assertions, test))]
    {
        println!("config: {:?}", config);
        println!("files: {:?}", files);
    }

    let mut file_descriptors = Vec::new();
    let readers: Vec<Box<BufReader<dyn Read>>> = if config.stdin {
        let stdin = std::io::stdin();
        let reader = std::io::BufReader::new(stdin.lock());
        vec![Box::new(reader)]
    } else {
        for file_name in files.clone() {
            let file_metadata = fs::metadata(&file_name).unwrap_or_else(|_| {
                eprintln!("No such file or directory \"{}\"", file_name);
                std::process::exit(1)
            });
            if config.recursive && file_metadata.is_dir() {
                get_files_recursive(&file_name, &mut file_descriptors);
            } else if file_metadata.is_file() {
                file_descriptors.push(file_name);
            }
        }
        file_descriptors
            .iter()
            .map(|descriptor| {
                let file = fs::File::open(descriptor).unwrap();
                Box::new(std::io::BufReader::new(file)) as Box<BufReader<dyn Read>>
            })
            .collect()
    };

    // do the hustle
    let mut frequencies = vec![0; UNICODE_CHAR_COUNT];
    for (i, reader) in readers.into_iter().enumerate() {
        if count_from_bufreader(reader, &mut frequencies).is_err() {
            let bad_file = if config.stdin {
                "stdin"
            } else {
                &file_descriptors[i]
            };
            eprintln!("File {} contains invalid UTF-8.", bad_file);
            continue;
        };
        if !config.group && !config.stdin {
            println!("{}:", file_descriptors[i]);
            print_frequency(&frequencies, &config);
            frequencies = vec![0; UNICODE_CHAR_COUNT];
        }
    }
    if config.group || config.stdin {
        print_frequency(&frequencies, &config);
    }
}

fn count_from_bufreader<T: std::io::Read + ?Sized>(
    mut reader: Box<BufReader<T>>,
    frequencies: &mut [usize],
) -> Result<(), std::str::Utf8Error> {
    let mut read_buffer = [0u8; READ_SIZE];
    while let Ok(bytes_read) = reader.read(&mut read_buffer) {
        if bytes_read == 0 {
            break;
        }
        let str = std::str::from_utf8(&read_buffer[..bytes_read])?;
        for c in str.chars() {
            frequencies[c as usize] += 1;
        }
    }
    Ok(())
}

fn print_frequency(frequencies: &[usize], config: &Config) {
    if config.sort {
        print_frequency_sorted(frequencies);
    } else {
        print_frequency_however(frequencies);
    }
}

fn print_frequency_sorted(frequencies: &[usize]) {
    let mut pairs: Vec<(usize, usize)> = Vec::new();
    for (i, &count) in frequencies.iter().enumerate() {
        if count > 0 {
            pairs.push((i, count));
        }
    }
    pairs.sort_by_key(|pair| pair.1);
    for (i, count) in pairs.into_iter() {
        let c = i as u8 as char;
        // escape the newline char
        if c == '\n' {
            println!(" \\n - {}", count);
            continue;
        }
        println!("  {} - {}", c, count);
    }
}

fn print_frequency_however(frequencies: &[usize]) {
    for (i, &count) in frequencies.iter().enumerate() {
        if count > 0 {
            let c = i as u8 as char;
            // escape the newline char
            if c == '\n' {
                println!(" \\n - {}", count);
                continue;
            }
            println!("  {} - {}", c, count);
        }
    }
}

fn get_files_recursive(dir: &str, file_descriptors: &mut Vec<String>) {
    let entries = fs::read_dir(dir).unwrap();
    for entry in entries {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            get_files_recursive(&path.to_string_lossy(), file_descriptors);
        } else {
            file_descriptors.push(path.to_string_lossy().into_owned());
        }
    }
}

fn parse_option(option: &str, config: &mut Config) {
    match option {
        "--help" => println!("For help, check the manpage: man charfreq"),
        "--group" => config.group = true,
        "--recursive" => config.recursive = true,
        "--sort" => config.sort = true,
        _ => panic!("unknown option: {}", option),
    }
}

fn parse_flags(flag: &str, config: &mut Config) {
    if flag == "-" {
        config.stdin = true;
        return;
    }
    let mut chars = flag.chars();
    let _ = chars.next();
    for c in chars {
        match c {
            'h' => println!("For help, check the manpage: man charfreq"),
            'g' => config.group = true,
            'r' => config.recursive = true,
            's' => config.sort = true,
            _ => panic!("unknown flag: {}", c),
        }
    }
}
