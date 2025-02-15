use std::env;
use std::str::FromStr;

pub struct Args {
    parts: Vec<String>,
}

impl Args {
    pub fn from_argv() -> Self {
        let argv: Vec<_> = env::args().skip(1).collect();
        Args { parts: argv }
    }

    pub fn from_line(line: &str) -> Self {
        let parts = line.trim().split(' ').map(String::from).collect();
        Args { parts }
    }

    pub fn len(&self) -> usize {
        self.parts.len()
    }

    pub fn is_quit(&self) -> bool {
        self.cmd() == "quit"
    }

    pub fn cmd(&self) -> &str {
        &self.parts[0]
    }

    pub fn as_str(&self, what: &str, arg: usize) -> Result<&str, String> {
        self.parts
            .get(arg)
            .ok_or_else(|| format!("missing {what}"))
            .map(std::string::String::as_str)
    }

    pub fn parts(&self) -> Vec<&str> {
        self.parts.iter().map(String::as_str).collect()
    }

    pub fn join_from(&self, what: &str, start: usize) -> Result<String, String> {
        Ok(self
            .parts
            .get(start..)
            .ok_or_else(|| format!("missing {what}"))?
            .join(" "))
    }

    pub fn parse<T: FromStr>(&self, what: &str, arg: usize) -> Result<T, String> {
        self.parts
            .get(arg)
            .ok_or_else(|| format!("missing {what}"))?
            .parse::<T>()
            .map_err(|_| format!("invalid {what}"))
    }
}
