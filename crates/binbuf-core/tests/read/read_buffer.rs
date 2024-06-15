use binbuf::read::Reader;

#[test]
fn test_new_read_buffer() {
    let data = &[69, 88, 65, 77, 80, 76, 69, 33];
    let buf = Reader::new(data);

    assert_eq!(buf.len(), 8);
}

#[test]
fn test_read_buffer_reset() {
    let data = &[69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = Reader::new(data);

    assert_eq!(buf.len(), 8);

    buf.read_vec(4).unwrap();
    assert_eq!(buf.len(), 4);

    buf.reset();
    assert_eq!(buf.len(), 8);
}

#[test]
fn test_read_buffer_jump() {
    let data = &[69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = Reader::new(data);

    buf.read_vec(4).unwrap();
    assert_eq!(buf.offset(), 4);

    buf.jump_to(0).unwrap();
    assert_eq!(buf.offset(), 0);
    assert_eq!(buf.jumped(), true);

    let jumped = buf.jump_reset();
    assert_eq!(jumped, true);
    assert_eq!(buf.offset(), 4);
    assert_eq!(buf.jumped(), false);
}
