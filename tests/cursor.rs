use binum::{BigEndian, ReadExt};
use std::io::Cursor;

#[test]
fn test_cursor_read_single() {
    let mut reader = Cursor::new(vec![69, 88, 65, 77, 80, 76, 69, 33]);
    match reader.read_from::<u16, BigEndian>() {
        Ok(n) => assert_eq!(n, 17752),
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_cursor_read_multi() {
    let mut reader = Cursor::new(vec![69, 88, 65, 77, 80, 76, 69, 33]);
    match reader.read_multi::<u16, BigEndian>(2) {
        Ok(n) => {
            assert_eq!(n[0], 17752);
            assert_eq!(n[1], 16717);
        }
        Err(err) => panic!("{}", err),
    };
}
