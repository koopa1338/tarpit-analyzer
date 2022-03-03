#![allow(unused_variables)]
use chrono::{Duration, NaiveDateTime};
use nom::IResult;
use std::net::Ipv4Addr;

pub enum Action {
    Connect,
    Disconnect,
    Unknown,
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

