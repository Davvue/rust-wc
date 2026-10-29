use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CountingMode {
    Characters,
    Lines
}

struct CountingModeInfo {
    descriptor: &'static str,
    aliases: &'static [&'static str],
    count: fn(&str) -> usize,
}

impl CountingMode {
    pub const ALL: [CountingMode; 2] = [
        CountingMode::Characters,
        CountingMode::Lines
    ];

    fn info(self) -> CountingModeInfo {
        match self {
            CountingMode::Characters => CountingModeInfo {
                descriptor: "Characters",
                aliases: &["-c"],
                count: |content| content.chars().count()
            },
            CountingMode::Lines => CountingModeInfo {
                descriptor: "Lines",
                aliases: &["-l"],
                count: |content| content.lines().count()
            }
        }
    }

    pub fn count(self, content: &str) -> usize {
        (self.info().count)(content)
    }
}

impl FromStr for CountingMode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        CountingMode::ALL
            .into_iter()
            .find(|cm| cm.info().aliases.contains(&s))
            .ok_or_else(|| format!("unknown counting mode '{s}'"))
    }
}

impl std::fmt::Display for CountingMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.info().descriptor)
    }
}

pub fn get_counting_mode(option: &str) -> Result<CountingMode, String> {
    match option {
        "-c" => Ok(CountingMode::Characters),
        "-l" => Ok(CountingMode::Lines),
        _ => Err(format!("unsupported counting mode {}", option))
    }
}