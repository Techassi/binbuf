use std::net::{Ipv4Addr, Ipv6Addr};

use binbuf::{
    write::{WriteError, Writer},
    BigEndian, Write,
};

mod write_buffer;
mod write_derive;
mod write_impl;
mod write_macro;
mod write_multi;
mod write_span;

#[test]
fn write_u8() {
    let mut writer = Writer::new();

    let n = 69u8.write::<BigEndian>(&mut writer).unwrap();

    assert_eq!(n, 1);
    assert_eq!(writer.bytes(), &[69]);
}

#[test]
fn write_u16() {
    let mut writer = Writer::new();

    let n = 17752u16.write::<BigEndian>(&mut writer).unwrap();

    assert_eq!(n, 2);
    assert_eq!(writer.bytes(), &[69, 88]);
}

#[test]
fn write_u32() {
    let mut writer = Writer::new();

    let n = 1163411789u32.write::<BigEndian>(&mut writer).unwrap();

    assert_eq!(n, 4);
    assert_eq!(writer.bytes(), &[69, 88, 65, 77]);
}

#[test]
fn write_u64() {
    let mut writer = Writer::new();

    let n = 4996815586883028257u64
        .write::<BigEndian>(&mut writer)
        .unwrap();

    assert_eq!(n, 8);
    assert_eq!(writer.bytes(), &[69, 88, 65, 77, 80, 76, 69, 33]);
}

#[test]
fn write_char_string() {
    let mut writer = Writer::new();

    let n = writer
        .write_char_string(&[69, 88, 65, 77, 80, 76, 69, 33], None)
        .unwrap();

    assert_eq!(writer.bytes(), &[8, 69, 88, 65, 77, 80, 76, 69, 33]);
    assert_eq!(n, 9);
}

#[test]
fn write_char_string_max_len() {
    let mut writer = Writer::new();

    let err = writer
        .write_char_string(&[69, 88, 65, 77, 80, 76, 69, 33], Some(3))
        .unwrap_err();

    assert_eq!(err, WriteError::MaxLengthOverflow);
}

#[test]
fn write_ipv4addr() {
    let mut writer = Writer::new();
    let addr = Ipv4Addr::new(127, 0, 0, 1);

    let n = addr.write::<BigEndian>(&mut writer).unwrap();

    assert_eq!(n, 4);
    assert_eq!(writer.bytes(), &[127, 0, 0, 1])
}

#[test]
fn write_ipv6addr() {
    let mut writer = Writer::new();
    let addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 1);

    let n = addr.write::<BigEndian>(&mut writer).unwrap();

    assert_eq!(n, 16);
    assert_eq!(
        writer.bytes(),
        &[0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1]
    )
}
