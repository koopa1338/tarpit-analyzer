use chrono::{Duration, NaiveDateTime};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
    net::Ipv4Addr,
    path::PathBuf,
    str::FromStr,
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

impl FromStr for TarpitLogEntry {
    type Err = String;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut items = line
            .split(",")
            .map(|piece| piece.to_owned())
            .collect::<Vec<String>>();
        let timestamp =
            NaiveDateTime::parse_from_str(&items.pop().unwrap(), "%Y-%m-%d %H:%M:%S").unwrap();
        let ip: Ipv4Addr = items.pop().unwrap().parse().unwrap();
        let duration = Duration::from_std(StdDuration::from_millis(
            items.pop().unwrap().parse().unwrap(),
        ))
        .unwrap();
        let action = items.pop().unwrap().into();
        Ok(TarpitLogEntry {
            timestamp,
            ip,
            duration,
            action,
        })
    }
}

fn parse_logfile(path: PathBuf) -> Vec<TarpitLogEntry> {
    let file = BufReader::new(File::open(path).unwrap());
    let mut buffer: Vec<TarpitLogEntry> = Vec::new();
    for line in file.lines() {
        buffer.push(TarpitLogEntry::from_str(line.unwrap().as_str()).unwrap())
    }
    buffer
}

fn print_stats(parsed_file: Vec<TarpitLogEntry>) {
    let mut count_connected = 0;
    let mut max_time_spend: Duration = Duration::zero();
    let mut ip_set: HashSet<Ipv4Addr> = HashSet::new();
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
        ip_set.insert(entry.ip);
    }
    println!("Connections: {}", count_connected);
    println!("Max time spend: {}", max_time_spend);
    println!("Count of different IPs: {}", ip_set.len());
}

fn main() {
    let file = PathBuf::from("./tarpit.log");
    let parsed_file = parse_logfile(file);
    print_stats(parsed_file);
}
