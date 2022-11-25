use binbuf::prelude::*;

#[test]
fn test_read_u8() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut b = ReadBuffer::new(b.as_slice());

    match u8::read::<BigEndian>(&mut b) {
        Ok(n) => assert_eq!(n, 69),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_u16() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut b = ReadBuffer::new(b.as_slice());

    match u16::read::<BigEndian>(&mut b) {
        Ok(n) => assert_eq!(n, 17752),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_u32() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut b = ReadBuffer::new(b.as_slice());

    match u32::read::<BigEndian>(&mut b) {
        Ok(n) => assert_eq!(n, 1163411789),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_u64() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut b = ReadBuffer::new(b.as_slice());

    match u64::read::<BigEndian>(&mut b) {
        Ok(n) => assert_eq!(n, 4996815586883028257),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_u128() {
    let b = vec![
        69, 88, 65, 77, 80, 76, 69, 33, 69, 88, 65, 77, 80, 76, 69, 33,
    ];
    let mut b = ReadBuffer::new(b.as_slice());

    match u128::read::<BigEndian>(&mut b) {
        Ok(n) => assert_eq!(n, 92174978314754016623629927450611041569),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_char_string() {
    let b = vec![8, 69, 88, 65, 77, 80, 76, 69, 33];
    let mut b = ReadBuffer::new(b.as_slice());

    match b.read_char_string(None) {
        Ok(n) => assert_eq!(n, &[69, 88, 65, 77, 80, 76, 69, 33]),
        Err(err) => panic!("{}", err),
    }
}
