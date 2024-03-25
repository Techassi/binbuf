use binbuf::{
    write::{Result, Write, Writer},
    BigEndian, Endianness,
};

#[test]
fn test_writeable_impl() {
    struct Data {
        inner: u16,
    }

    impl Write for Data {
        fn write<E: Endianness>(&self, buf: &mut Writer) -> Result {
            self.inner.write::<E>(buf)
        }
    }

    let d = Data { inner: 17752 };

    let mut b = Writer::new();
    match d.write::<BigEndian>(&mut b) {
        Ok(n) => {
            assert_eq!(n, 2);
            assert_eq!(b.bytes(), &[69, 88]);
        }
        Err(err) => panic!("{}", err),
    };
}
