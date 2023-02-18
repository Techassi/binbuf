use std::collections::HashMap;

use binbuf::prelude::*;

#[test]
fn test_write_multi_u8() {
    let d = vec![69u8, 88u8];
    let mut b = WriteBuffer::new();

    match d.write::<BigEndian>(&mut b) {
        Ok(n) => {
            assert_eq!(n, 2);
            assert_eq!(b.bytes(), &[69, 88]);
        }
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_write_multi_u16() {
    let d = vec![17752u16, 16717u16];
    let mut b = WriteBuffer::new();

    match d.write::<BigEndian>(&mut b) {
        Ok(n) => {
            assert_eq!(n, 4);
            assert_eq!(b.bytes(), &[69, 88, 65, 77]);
        }
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_write_multi_u32() {
    let d = vec![1163411789u32, 1347175713u32];
    let mut b = WriteBuffer::new();

    match d.write::<BigEndian>(&mut b) {
        Ok(n) => {
            assert_eq!(n, 8);
            assert_eq!(b.bytes(), &[69, 88, 65, 77, 80, 76, 69, 33]);
        }
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_write_multi_u64() {
    let d = vec![4996815586883028257u64];
    let mut b = WriteBuffer::new();

    match d.write::<BigEndian>(&mut b) {
        Ok(n) => {
            assert_eq!(n, 8);
            assert_eq!(b.bytes(), &[69, 88, 65, 77, 80, 76, 69, 33]);
        }
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_write_multi_u128() {
    let d = vec![92174978314754016623629927450611041569u128];
    let mut b = WriteBuffer::new();

    match d.write::<BigEndian>(&mut b) {
        Ok(n) => {
            assert_eq!(n, 16);
            assert_eq!(
                b.bytes(),
                &[69, 88, 65, 77, 80, 76, 69, 33, 69, 88, 65, 77, 80, 76, 69, 33]
            );
        }
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_write_multi_hashmap() {
    let mut m: HashMap<&str, u8> = HashMap::new();
    m.insert("69", 69);
    m.insert("88", 88);

    let mut b = WriteBuffer::new();
    match m.write::<BigEndian>(&mut b) {
        Ok(n) => {
            assert_eq!(n, 2);
            assert_eq!(b.bytes(), &[69, 88]);
        }
        Err(err) => panic!("{}", err),
    }
}
