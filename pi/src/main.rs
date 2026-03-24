#[derive(Default)]
struct Config {
    only_decimals: bool,
    decimals: usize,
}

fn main() -> Result<(), String> {
    let pi = include_str!("../pi.txt");

    let mut config = Config {
        decimals: 2,
        ..Default::default()
    };
    let mut args = std::env::args();
    let _ = args.next();
    for arg in args {
        match arg.as_str() {
            "-d" | "--decimals" => config.only_decimals = true,
            _ => config.decimals = arg.parse().unwrap_or(2),
        }
    }

    if config.decimals > 1000001 {
        return Err(String::from("Too many decimals requested. Sorry."));
    }

    if config.only_decimals {
        println!("{}", &pi[2..config.decimals + 2]);
    } else {
        println!("{}", &pi[..config.decimals + 2]);
    }

    Ok(())
}
