use crate::parsing::{parse_date, parse_datetime, parse_timestamp};
use chrono::{NaiveDate, NaiveTime};

#[test]
fn parsing_date() {
    let input = "2024-01-01";
    let (rest, parsed) = parse_date(input).unwrap();
    assert_eq!(parsed, NaiveDate::from_ymd_opt(2024, 1, 1).unwrap());
    assert!(rest.is_empty());
}

#[test]
#[should_panic(expected = "invalid date")]
fn parsing_date_invalid_date() {
    let input = "2024-01-32";
    parse_date(input).unwrap();
}

#[test]
fn parsing_timestamp() {
    let input = "18:00:22";
    let (rest, parsed) = parse_timestamp(input).unwrap();
    assert_eq!(parsed, NaiveTime::from_hms_opt(18, 0, 22).unwrap());
    assert!(rest.is_empty());
}

#[test]
#[should_panic(expected = "invalid timestamp")]
fn parsing_timestamp_invalid_timestamp() {
    let input = "18:00:66";
    parse_timestamp(input).unwrap();
}

#[test]
fn parse_date_and_time() {
    let input = "2024-01-01 18:00:22";
    let (rest, parsed) = parse_datetime(input).unwrap();
    assert_eq!(
        parsed,
        NaiveDate::from_ymd_opt(2024, 1, 1)
            .unwrap()
            .and_hms_opt(18, 0, 22)
            .unwrap()
    );
    assert!(rest.is_empty());
}
