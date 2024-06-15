#[cfg(feature = "macros")]
#[test]
fn test_write_macro() {
    use binbuf::{write::Result, BigEndian, Endianness, Write, Writer};
    use binbuf_macros::bytes_written;

    let mut b = Writer::new();

    struct Data {
        v1: u8,
        v2: u16,
    }

    impl Write for Data {
        fn write<E: Endianness>(&self, buf: &mut Writer) -> Result {
            let n = bytes_written! {
                self.v1.write::<E>(buf)?;
                self.v2.write::<E>(buf)?
            };
            Ok(n)
        }
    }

    let data = Data { v1: 42, v2: 69 };

    match data.write::<BigEndian>(&mut b) {
        Ok(n) => assert_eq!(n, 3),
        Err(err) => panic!("{}", err),
    };
}
