use binbuf::{
    read::{Error, ReadStringOption},
    Reader,
};

#[test]
fn read_string_sized() {
    let b = &[8, 69, 88, 65, 77, 80, 76, 69, 33];
    let mut r = Reader::new(b);

    let s = r.read_string(ReadStringOption::sized(None)).unwrap();
    assert_eq!(*s, [69, 88, 65, 77, 80, 76, 69, 33]);
}

#[test]
fn read_string_sized_max_length() {
    let b = &[8, 69, 88, 65, 77, 80, 76, 69, 33];
    let mut r = Reader::new(b);

    let e = r.read_string(ReadStringOption::sized(Some(5))).unwrap_err();
    assert_eq!(e, Error::MaxLengthOverflow);
}
