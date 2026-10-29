#[derive(Debug)]
pub enum CountingMode {
    Characters,
    Lines
}

pub fn get_counting_mode(option: &str) -> Result<CountingMode, String> {
    match option {
        "-c" => Ok(CountingMode::Characters),
        "-l" => Ok(CountingMode::Lines),
        _ => Err(format!("unsupported counting mode {}", option))
    }
}