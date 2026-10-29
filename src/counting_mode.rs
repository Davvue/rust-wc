use std::str::FromStr;

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum CountingMode {
    Characters,
    Lines
}

pub struct CountingModeInfo {
    name: &'static str,
    description: &'static str,
    aliases: &'static [&'static str],
    count: fn(&str) -> usize,
}

impl CountingModeInfo {
    pub fn get_aliases(&self) -> &'static [&'static str] {
        self.aliases
    }

    pub fn get_description(&self) -> &'static str {
        self.description
    }
}

impl CountingMode {
    pub const ALL: [CountingMode; 2] = [
        CountingMode::Characters,
        CountingMode::Lines
    ];

    pub fn info(self) -> CountingModeInfo {
        match self {
            CountingMode::Characters => CountingModeInfo {
                name: "Characters",
                description: "Count characters",
                aliases: &["-c"],
                count: |content| content.chars().count()
            },
            CountingMode::Lines => CountingModeInfo {
                name: "Lines",
                description: "Count lines",
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
        f.write_str(self.info().name)
    }
}