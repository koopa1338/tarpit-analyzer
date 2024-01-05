use chrono::NaiveDateTime;
use nom::character::complete::line_ending;
use nom::multi::separated_list0;
use nom::Finish;
use parsing::parse_log_line;
use std::net::SocketAddrV4;

mod parsing;
#[cfg(test)]
mod tests;

#[derive(Debug)]
pub struct TarpitLog {
    pub lines: Vec<TarpitLogEntry>,
}

#[derive(Debug)]
pub struct TarpitLogEntry {
    pub timestamp: NaiveDateTime,
    pub ip: SocketAddrV4,
    pub action: Action,
    pub log_level: LogLevel,
}

#[derive(Debug, PartialEq)]
pub enum Action {
    Connect,
    Disconnect,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

// TODO: error handling
pub fn parse_tarpit_log(input: &str) -> Result<TarpitLog, String> {
    separated_list0(line_ending, parse_log_line)(input)
        .finish()
        .map(|result| TarpitLog { lines: result.1 })
        .map_err(|e: nom::error::Error<&str>| e.to_string())
}
