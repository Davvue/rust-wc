use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args.get(0).map(String::as_str).unwrap_or("wc");

    println!("{}", program);

    println!("Hello, world!");
}
