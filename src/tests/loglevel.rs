use crate::{parse_log_level, LogLevel};
use rstest::rstest;

#[rstest]
#[case("TRACE", LogLevel::Trace)]
#[case("DEBUG", LogLevel::Debug)]
#[case("INFO", LogLevel::Info)]
#[case("WARN", LogLevel::Warn)]
#[case("ERROR", LogLevel::Error)]
fn parsing_log_level(#[case] input: &str, #[case] expected: LogLevel) {
    let (rest, parsed) = parse_log_level(input).unwrap();
    assert_eq!(parsed, expected);
    assert!(rest.is_empty());
}
