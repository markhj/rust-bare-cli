use std::io;
use std::io::Write;

mod config;
mod console;

use crate::config::Config;
use crate::console::console::Console;
use crate::console::error::ConsoleError;
use crate::console::lineending::LineEnding;

fn main() {
    let mut console : Console = Console {
        config : Config {
            welcome: "Rust Bare CLI\nVersion 0.1.0",
            line_ending_char: ';',
            line_ending: LineEnding::Optional,
        },
        keep_open: true,
    };

    console.welcome();

    while console.running() {
        print!("> ");

        let _flush_result = io::stdout().flush();

        let mut input = String::new();
        let _input_result = io::stdin().read_line(&mut input);

        match console.next(input.trim().to_string()) {
            Ok(_t) => { },
            Err(e) => {
                match e {
                    ConsoleError::NoInput => { },
                    _ => println!("Error: {}", e),
                }
            }
        }
    }
}
