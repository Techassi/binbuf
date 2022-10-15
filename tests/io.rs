use binum::{BigEndian, ReadExt, WriteExt};
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
            assert_eq!(n, vec![17752, 16717]);
        }
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_vec_write_single() {
    let mut writer: Vec<u8> = Vec::new();
    match writer.write_into::<u16, BigEndian>(17752) {
        Ok(n) => {
            assert_eq!(n, 2);
            assert_eq!(writer.len(), 2);
            assert_eq!(writer, vec![69, 88]);
        }
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_vec_write_multi() {
    let mut writer: Vec<u8> = Vec::new();
    match writer.write_multi::<u16, BigEndian>(vec![17752, 16717]) {
        Ok(n) => {
            assert_eq!(n, 4);
            assert_eq!(writer.len(), 4);
            assert_eq!(writer, vec![69, 88, 65, 77]);
        }
        Err(err) => panic!("{}", err),
    }
}
