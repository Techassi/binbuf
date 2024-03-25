use binbuf::write::Writer;

#[test]
fn test_write_span_basic() {
    let mut b = Writer::new();
    b.enter();

    {
        b.enter();

        b.push(69);
        b.push(88);

        assert_eq!(b.exit(), 2)
    }

    b.write([65, 77]);
    assert_eq!(b.exit(), 4);
}
