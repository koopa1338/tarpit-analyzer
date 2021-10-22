use std::{fs::File, io::{BufRead, BufReader}, path::PathBuf};

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
    timestamp: String,
    ip: String,
    duration: String,
    action: Action,
}

fn parse_logfile(path: PathBuf) -> Vec<TarpitLogEntry> {
    let file = BufReader::new(File::open(path).unwrap());
    let mut buffer = Vec::new();
    for line in file.lines() {
        let mut items = line.unwrap().split(",").map(|piece| {
            piece.to_owned()
        }).collect::<Vec<String>>();
        let timestamp = items.pop().unwrap().into();
        let ip = items.pop().unwrap().into();
        let duration = items.pop().unwrap().into();
        let action: Action = items.pop().unwrap().into();
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
}

fn main() {
    let file = PathBuf::from("./tarpit.log");
    let parsed_file = parse_logfile(file);
    print_stats(parsed_file);
}
