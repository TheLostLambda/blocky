use blocky::Blocky;
use std::env;
use std::io;
use std::process;

const RESPONSE_FILE: &str = "blocky.toml";

fn main() {
    let args: Vec<_> = env::args().collect();
    let config = if args.len() > 1 {
        &args[1]
    } else {
        RESPONSE_FILE
    };
    let block = Blocky::new(config).unwrap_or_else(|err| {
        eprintln!(
            "An error occurred while parsing from {}. The error was: {}",
            RESPONSE_FILE, err
        );
        process::exit(1)
    });
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap_or_else(|err| {
            eprintln!("Failed to read a line from stdin with error: {}", err);
            process::exit(1)
        });
        let line = blocky::trim_leader(&line);
        let resp = block.respond(line).unwrap_or_else(|err| {
            eprintln!("{}", err);
            process::exit(1)
        });
        print!("{}", resp);
    }
}
