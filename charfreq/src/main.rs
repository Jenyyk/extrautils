use std::collections::HashMap;
use std::fs;
use std::io::Read;

#[derive(Debug, Default)]
struct Config {
    group: bool,
    stdin: bool,
    recursive: bool,
}

const READ_SIZE: usize = 65536;

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

    // read data
    if config.stdin {
        let stdin = std::io::stdin();
        let mut frequencies: HashMap<char, usize> = HashMap::new();
        let mut reader = std::io::BufReader::new(stdin.lock());
        let mut read_buffer = [0u8; READ_SIZE];
        while let Ok(bytes_read) = reader.read(&mut read_buffer) {
            for &b in &read_buffer[..bytes_read] {
                if b != b'\n' {
                    *frequencies.entry(b as char).or_insert(0) += 1;
                }
            }
            if bytes_read < READ_SIZE {
                break;
            }
        }
        print_frequency(frequencies);
        std::process::exit(0);
    }

    let mut file_descriptors = Vec::new();
    for file_name in files {
        let file_metadata = fs::metadata(&file_name).unwrap();
        if config.recursive && file_metadata.is_dir() {
            get_files_recursive(&file_name, &mut file_descriptors);
        } else if file_metadata.is_file() {
            file_descriptors.push(file_name);
        }
    }

    #[cfg(any(debug_assertions, test))]
    {
        println!("file_descriptors: {:?}", file_descriptors);
    }

    // do the hustle
    let mut frequencies: HashMap<char, usize> = HashMap::new();
    for file_descriptor in &file_descriptors {
        let file = fs::File::open(file_descriptor).unwrap();
        let mut reader = std::io::BufReader::new(file);
        let mut read_buffer = [0u8; READ_SIZE];
        while let Ok(bytes_read) = reader.read(&mut read_buffer) {
            for &b in &read_buffer[..bytes_read] {
                if b != b'\n' {
                    *frequencies.entry(b as char).or_insert(0) += 1;
                }
            }
            if bytes_read < READ_SIZE {
                break;
            }
        }
        if !config.group {
            println!("{}:", file_descriptor);
            print_frequency(std::mem::take(&mut frequencies));
        }
    }
    if config.group {
        print_frequency(frequencies);
    }
}

fn print_frequency(frequencies: HashMap<char, usize>) {
    let mut sorted: Vec<(char, usize)> = frequencies.into_iter().collect::<Vec<_>>();
    sorted.sort_unstable_by_key(|kv| kv.1);
    for (c, count) in sorted.into_iter().rev() {
        // escape the newline char
        if c == '\n' {
            println!(" \\n - {}", count);
            continue;
        }
        println!("  {} - {}", c, count);
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
        "--group" => config.group = true,
        "--recursive" => config.recursive = true,
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
            'g' => config.group = true,
            'r' => config.recursive = true,
            _ => panic!("unknown flag: {}", c),
        }
    }
}
