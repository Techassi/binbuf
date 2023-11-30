use binbuf::write::*;

#[test]
fn test_new_write_buffer() {
    let mut buf = WriteBuffer::new();
    buf.push(69);

    assert_eq!(buf.len(), 1);
    assert_eq!(buf.bytes(), &[69]);
}

#[test]
fn test_write_buffer_clear() {
    let mut buf = WriteBuffer::new();
    buf.push(69);

    assert_eq!(buf.len(), 1);
    assert_eq!(buf.bytes(), &[69]);

    buf.clear();

    assert_eq!(buf.len(), 0);
    assert_eq!(buf.bytes(), &[]);
}

#[test]
fn test_new_write_buffer_with() {
    let mut buf = WriteBuffer::new_with([69, 88]);
    buf.push(65);

    assert_eq!(buf.len(), 3);
    assert_eq!(buf.bytes(), &[69, 88, 65]);
}

#[test]
fn test_write_buffer_with_clear() {
    let mut buf = WriteBuffer::new_with([69, 88]);
    buf.push(65);

    assert_eq!(buf.len(), 3);
    assert_eq!(buf.bytes(), &[69, 88, 65]);

    buf.clear();

    assert_eq!(buf.len(), 0);
    assert_eq!(buf.bytes(), &[]);
}
