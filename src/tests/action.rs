use crate::{parsing::parse_action, Action};
use rstest::rstest;

#[rstest]
#[case("connected", Action::Connect)]
#[case("disconnected", Action::Disconnect)]
fn parsing_action(#[case] input: &str, #[case] expected: Action) {
    let (rest, parsed) = parse_action(input).unwrap();
    assert_eq!(parsed, expected);
    assert!(rest.is_empty());
}
