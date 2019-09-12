use blocky::Blocky;
use std::io;
use std::process;

const RESPONSE_FILE: &str = "blocky.toml";

fn main() {
    let block = Blocky::new(RESPONSE_FILE).unwrap_or_else(|err| {
        println!(
            "An error occurred while parsing from {}. The error was: {}",
            RESPONSE_FILE, err
        );
        process::exit(1)
    });
    loop {
        let mut line = String::new();
        // Handle errors!
        io::stdin().read_line(&mut line).unwrap();
        // Separate this into a trim function
        let line: String = line.split("]: ").skip(1).collect();
        let resp = block.respond(line).unwrap_or_else(|err| {
            println!("{}", err);
            process::exit(1)
        });
        print!("{}", resp);
    }
}
