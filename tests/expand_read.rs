use binum;
use binum::macros::Readable;

#[test]
fn test_expand_read() {
    #[derive(Readable)]
    pub struct Target {
        pub foo: u16,
        pub bar: u16,
    }

    let d: Vec<u8> = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let t = match Target::try_read_from(&d, binum::Endianness::Big) {
        Ok(t) => t,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(t.foo, 17752);
    assert_eq!(t.bar, 16717);
}
