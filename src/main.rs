mod counting_mode;

use std::fs::read_to_string;
use std::path::Path;
use std::env;
use std::process::ExitCode;
use crate::counting_mode::{CountingMode};

fn help(program: &str) -> String {
    let mut result: String = format!("Usage: {program} [OPTIONS] <file>\n");
    result += "\tOptions:\n";

    for mode in CountingMode::ALL {
        let info = mode.info();
        result += &format!("\t\t{} {}\n", info.aliases().join(", "), info.description());
    }

    result
}

fn main() -> ExitCode {
    let args: Vec<String> = env::args().collect();
    let program = args.get(0).map(String::as_str).unwrap_or("rust-wc");

    let (file, counting_mode) = match args.get(1..).unwrap_or(&[]) {
        [h] if (h == "--help" || h == "-h") => {
            println!("{}", help(program));
            return ExitCode::SUCCESS;
        },
        [file] if !file.starts_with('-') => (file, CountingMode::DEFAULT),
        [options, file] if !file.starts_with('-') => match options.parse() {
            Ok(value) => (file, value),
            Err(why) => {
                eprintln!("{}", why);
                eprintln!("{}", help(program));
                return ExitCode::FAILURE;
            }
        },
        _ => {
            eprintln!("Invalid input args");
            eprintln!("{}", help(program));
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

    let count = counting_mode.count(&content);

    println!("{}", count);
    ExitCode::SUCCESS
}
