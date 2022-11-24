use binbuf::prelude::*;

#[test]
fn test_write_u8() {
    let mut b = WriteBuffer::new();

    match 69u8.write::<BigEndian>(&mut b) {
        Ok(n) => {
            assert_eq!(n, 1);
            assert_eq!(b.bytes(), &[69]);
        }
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_write_u16() {
    let mut b = WriteBuffer::new();

    match 17752u16.write::<BigEndian>(&mut b) {
        Ok(n) => {
            assert_eq!(n, 2);
            assert_eq!(b.bytes(), &[69, 88]);
        }
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_write_u32() {
    let mut b = WriteBuffer::new();

    match 1163411789u32.write::<BigEndian>(&mut b) {
        Ok(n) => {
            assert_eq!(n, 4);
            assert_eq!(b.bytes(), &[69, 88, 65, 77]);
        }
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_write_u64() {
    let mut b = WriteBuffer::new();

    match 4996815586883028257u64.write::<BigEndian>(&mut b) {
        Ok(n) => {
            assert_eq!(n, 8);
            assert_eq!(b.bytes(), &[69, 88, 65, 77, 80, 76, 69, 33]);
        }
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_write_char_string() {
    let mut b = WriteBuffer::new();

    match b.write_char_string(vec![69, 88, 65, 77, 80, 76, 69, 33].as_slice()) {
        Ok(n) => {
            assert_eq!(n, 9);
            assert_eq!(b.bytes(), &[8, 69, 88, 65, 77, 80, 76, 69, 33]);
            b.clear()
        }
        Err(_) => todo!(),
    }

    match b.write_char_string(String::from("EXAMPLE!").as_bytes()) {
        Ok(n) => {
            assert_eq!(n, 9);
            assert_eq!(b.bytes(), &[8, 69, 88, 65, 77, 80, 76, 69, 33])
        }
        Err(_) => todo!(),
    }
}
