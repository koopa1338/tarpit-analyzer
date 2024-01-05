use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use nom::branch::alt;
use nom::bytes::complete::{tag, take_until};
use nom::character::complete::{char, i32, space1, u16, u32, u8};
use nom::combinator::map;
use nom::sequence::{delimited, separated_pair, tuple};
use nom::IResult;
use std::net::{Ipv4Addr, SocketAddrV4};

use crate::{Action, TarpitLogEntry, LogLevel};

pub(crate) fn parse_date(input: &str) -> IResult<&str, NaiveDate> {
    map(
        tuple((i32, char('-'), u32, char('-'), u32)),
        |(year, _, month, _, day)| NaiveDate::from_ymd_opt(year, month, day).expect("invalid date"),
    )(input)
}

pub(crate) fn parse_timestamp(input: &str) -> IResult<&str, NaiveTime> {
    map(
        tuple((u32, char(':'), u32, char(':'), u32)),
        |(hour, _, minute, _, second)| {
            NaiveTime::from_hms_opt(hour, minute, second).expect("invalid timestamp.")
        },
    )(input)
}

pub(crate) fn parse_datetime(input: &str) -> IResult<&str, NaiveDateTime> {
    map(
        separated_pair(parse_date, space1, parse_timestamp),
        |(date, time)| NaiveDateTime::new(date, time),
    )(input)
}

pub(crate) fn parse_log_level(input: &str) -> IResult<&str, LogLevel> {
    alt((
        map(tag("TRACE"), |_| LogLevel::Trace),
        map(tag("DEBUG"), |_| LogLevel::Debug),
        map(tag("INFO"), |_| LogLevel::Info),
        map(tag("WARN"), |_| LogLevel::Warn),
        map(tag("ERROR"), |_| LogLevel::Error),
    ))(input)
}

pub(crate) fn parse_ipv4(input: &str) -> IResult<&str, Ipv4Addr> {
    map(
        tuple((u8, char('.'), u8, char('.'), u8, char('.'), u8)),
        |(u1, _, u2, _, u3, _, u4)| Ipv4Addr::new(u1, u2, u3, u4),
    )(input)
}

pub(crate) fn parse_port(input: &str) -> IResult<&str, u16> {
    u16(input)
}

pub(crate) fn parse_socket_addr(input: &str) -> IResult<&str, SocketAddrV4> {
    map(
        delimited(
            tag("('"),
            separated_pair(parse_ipv4, tag("', "), parse_port),
            char(')'),
        ),
        |(addr, port)| SocketAddrV4::new(addr, port),
    )(input)
}

pub(crate) fn parse_action(input: &str) -> IResult<&str, Action> {
    alt((
        map(tag("disconnected"), |_| Action::Disconnect),
        map(tag("connected"), |_| Action::Connect),
    ))(input)
}

pub(crate) fn parse_log_line(input: &str) -> IResult<&str, TarpitLogEntry> {
    let (rest, (timestamp, log_level)) =
        separated_pair(parse_datetime, space1, parse_log_level)(input)?;
    let (rest, _) = take_until("(")(rest)?;
    map(
        separated_pair(parse_socket_addr, space1, parse_action),
        move |(ip, action)| TarpitLogEntry {
            timestamp,
            ip,
            action,
            log_level,
        },
    )(rest)
}
