mod counting_mode;

use std::fs::read_to_string;
use std::io::prelude::*;
use std::path::Path;
use std::env;
use std::process::ExitCode;
use crate::counting_mode::{CountingMode, get_counting_mode};

fn help(program: &str, printer: fn(out: &str)) {
    printer(format!("Usage: {} [OPTIONS] <file>", program).as_str());
    printer("\tOptions");
    printer("\t\t-l Count lines [default]");
    printer("\t\t-c Count characters");
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let program = args.get(0).map(String::as_str).unwrap_or("wc");

    let (file, counting_mode) = match args.get(1..).unwrap_or(&[]) {
        [h] if (h == "--help" || h == "-h") => {
            help(program, |out| println!("{}", out));
            return ExitCode::SUCCESS;
        },
        [file] if !file.starts_with('-') => (file, CountingMode::Lines),
        [options, file] => (file, get_counting_mode(options).unwrap_or(CountingMode::Lines)),
        _ => {
            eprintln!("Invalid input args");
            help(program, |out| eprintln!("{}", out));
            return ExitCode::FAILURE;
        }
    };

    let path = Path::new(file);
    let path_display = path.display();

    let content = match read_to_string(path) {
        Ok(con) => con,
        Err(why) => {
            eprintln!("Failed reading file at {}: {}", path_display, why);
            return ExitCode::FAILURE;
        }
    };

    let count = match counting_mode {
        CountingMode::Characters => content.chars().count(),
        CountingMode::Lines => content.lines().count(),
    };
    println!("{}", count);
    ExitCode::SUCCESS
}
