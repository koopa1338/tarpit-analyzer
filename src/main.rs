use chrono::{Duration, NaiveDateTime};
use std::{
    fs::File,
    io::{BufRead, BufReader},
    net::Ipv4Addr,
    path::PathBuf,
    time::Duration as StdDuration,
};

enum Action {
    Connect,
    Disconnect,
    Unknown,
}

impl From<String> for Action {
    fn from(action: String) -> Self {
        match action.as_str() {
            "connected" => Action::Connect,
            "disconnected" => Action::Disconnect,
            _ => Action::Unknown,
        }
    }
}

struct TarpitLogEntry {
    timestamp: NaiveDateTime,
    ip: Ipv4Addr,
    duration: Duration,
    action: Action,
}

fn parse_logfile(path: PathBuf) -> Vec<TarpitLogEntry> {
    let file = BufReader::new(File::open(path).unwrap());
    let mut buffer = Vec::new();
    for line in file.lines() {
        let mut items = line
            .unwrap()
            .split(",")
            .map(|piece| piece.to_owned())
            .collect::<Vec<String>>();
        let timestamp =
            NaiveDateTime::parse_from_str(&items.pop().unwrap(), "%Y-%m-%d %H:%M:%S").unwrap();
        let ip = items.pop().unwrap().parse().unwrap();
        let duration = Duration::from_std(StdDuration::from_millis(
            items.pop().unwrap().parse().unwrap(),
        ))
        .unwrap();
        let action = items.pop().unwrap().into();
        buffer.push(TarpitLogEntry {
            timestamp,
            ip,
            duration,
            action,
        });
    }
    buffer
}

fn print_stats(parsed_file: Vec<TarpitLogEntry>) {
    let mut count_connected = 0;
    let mut max_time_spend: Duration = Duration::zero();
    for entry in parsed_file.iter() {
        match entry.action {
            Action::Connect => count_connected += 1,
            _ => {}
        }
        if max_time_spend.is_zero() {
            max_time_spend = Duration::max(max_time_spend, entry.duration);
        } else {
            max_time_spend = entry.duration;
        }
    }
    println!("Connections: {}", count_connected);
    println!("Max time spend: {}", max_time_spend);
}

fn main() {
    let file = PathBuf::from("./tarpit.log");
    let parsed_file = parse_logfile(file);
    print_stats(parsed_file);
}
