#![allow(unused_variables)]
use chrono::{Duration, NaiveDateTime};
use nom::IResult;
use std::net::Ipv4Addr;

pub enum Action {
    Connect,
    Disconnect,
    Unknown,
}

pub struct TarpitLogEntry {
    pub timestamp: NaiveDateTime,
    pub ip: Ipv4Addr,
    pub duration: Duration,
    pub action: Action,
}

pub fn parse_time_stamp(input: &str) -> IResult<&str, NaiveDateTime, ()> {
    unimplemented!();
}

pub fn parse_ipv4(input: &str) -> IResult<&str, Ipv4Addr, ()> {
    unimplemented!();
}

pub fn parse_action(input: &str) -> IResult<&str, Action, ()> {
    unimplemented!();
}

pub fn parse_duration(input: &str) -> IResult<&str, Duration, ()> {
    unimplemented!();
}

pub fn parse_log_entry(input: &str) -> TarpitLogEntry {
    let (input, action) = parse_action(input).unwrap();
    let (input, timestamp) = parse_time_stamp(input).unwrap();
    let (input, ip) = parse_ipv4(input).unwrap();
    let (_, duration) = parse_duration(input).unwrap();
    TarpitLogEntry {
        timestamp,
        ip,
        duration,
        action,
    }
}
