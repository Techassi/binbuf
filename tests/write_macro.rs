#[cfg(feature = "macros")]
#[test]
fn test_write_macro() {
    use binbuf::prelude::*;

    let mut b = WriteBuffer::new();

    struct Data {
        v1: u8,
        v2: u16,
    }

    impl Writeable for Data {
        type Error = BufferError;

        fn write<E: Endianness>(&self, buf: &mut WriteBuffer) -> Result<usize, Self::Error> {
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
