use std::net::{Ipv4Addr, Ipv6Addr};

use binbuf::{read::*, BigEndian};

mod read_buffer;
mod read_derive_enum;
mod read_derive_struct;
mod read_impl;
mod read_multi;

#[test]
fn test_read_u8() {
    let data = &[69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = ReadBuffer::new(data);

    match u8::read::<BigEndian>(&mut buf) {
        Ok(n) => assert_eq!(n, 69),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_u16() {
    let data = &[69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = ReadBuffer::new(data);

    match u16::read::<BigEndian>(&mut buf) {
        Ok(n) => assert_eq!(n, 17752),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_u32() {
    let b = &[69, 88, 65, 77, 80, 76, 69, 33];
    let mut b = ReadBuffer::new(b);

    match u32::read::<BigEndian>(&mut b) {
        Ok(n) => assert_eq!(n, 1163411789),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_u64() {
    let b = &[69, 88, 65, 77, 80, 76, 69, 33];
    let mut b = ReadBuffer::new(b);

    match u64::read::<BigEndian>(&mut b) {
        Ok(n) => assert_eq!(n, 4996815586883028257),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_u128() {
    let b = &[
        69, 88, 65, 77, 80, 76, 69, 33, 69, 88, 65, 77, 80, 76, 69, 33,
    ];
    let mut b = ReadBuffer::new(b);

    match u128::read::<BigEndian>(&mut b) {
        Ok(n) => assert_eq!(n, 92174978314754016623629927450611041569),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_char_string() {
    let b = &[8, 69, 88, 65, 77, 80, 76, 69, 33];
    let mut b = ReadBuffer::new(b);

    match b.read_char_string(None) {
        Ok(n) => assert_eq!(n, &[69, 88, 65, 77, 80, 76, 69, 33]),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_ipv4addr() {
    let b = &[127, 0, 0, 1];
    let mut b = ReadBuffer::new(b);

    match Ipv4Addr::read::<BigEndian>(&mut b) {
        Ok(ip) => assert_eq!(ip, Ipv4Addr::new(127, 0, 0, 1)),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_ipv6addr() {
    let b = &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1];
    let mut b = ReadBuffer::new(b);

    match Ipv6Addr::read::<BigEndian>(&mut b) {
        Ok(ip) => assert_eq!(ip, Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1)),
        Err(err) => panic!("{}", err),
    }
}
