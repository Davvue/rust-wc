use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

mod counting_mode;
use crate::counting_mode::{CountingMode, get_counting_mode};

fn help(program: &str, printer: fn(out: &str)) {
    printer(format!("Usage: {} [OPTIONS] <file>", program).as_str());
    printer("\tOptions");
    printer("\t\t-l Count lines [default]");
    printer("\t\t-c Count characters");
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args.get(0).map(String::as_str).unwrap_or("wc");

    let (file, counting_mode) = match args.get(1..).unwrap_or(&[]) {
        [h] if (h == "--help" || h == "-h") => {
            help(program, |out| println!("{}", out));
            return;
        },
        [file] if !file.starts_with('-') => (file, CountingMode::Lines),
        [options, file] => (file, get_counting_mode(options).unwrap_or(CountingMode::Lines)),
        _ => {
            eprintln!("Invalid input args");
            help(program, |out| eprintln!("{}", out));
            return;
        }
    };

    println!("{} -> file: {}; counting_mode: {:?}", program, file, counting_mode);

    println!("Hello, world!");
}
