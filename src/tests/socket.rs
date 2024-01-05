use crate::{parse_ipv4, parse_port, parse_socket_addr};
use std::net::{Ipv4Addr, SocketAddrV4};

#[test]
fn parsing_ipv4() {
    let input = "192.168.2.1";
    let (rest, parsed) = parse_ipv4(input).unwrap();
    assert_eq!(parsed, Ipv4Addr::new(192, 168, 2, 1));
    assert!(rest.is_empty());
}

#[test]
fn parsing_port() {
    let input = "2222";
    let (rest, parsed) = parse_port(input).unwrap();
    assert_eq!(parsed, 2222u16);
    assert!(rest.is_empty());
}

#[test]
fn parsing_socket_address() {
    let input = "('192.168.2.1', 2222)";
    let (rest, parsed) = parse_socket_addr(input).unwrap();
    assert_eq!(
        parsed,
        SocketAddrV4::new(Ipv4Addr::new(192, 168, 2, 1), 2222u16)
    );
    assert!(rest.is_empty());
}
