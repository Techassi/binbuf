use binbuf::prelude::*;

#[test]
fn test_writeable_impl() {
    struct Data {
        inner: u16,
    }

    impl<'a> Writeable<'a> for Data {
        type Error = BufferError;

        fn write<E: Endianness>(&self, buf: &mut impl ToWriteBuffer) -> Result<usize, Self::Error> {
            self.inner.write::<E>(buf)
        }
    }

    let d = Data { inner: 17752 };

    let mut b = WriteBuffer::new();
    match d.write::<BigEndian>(&mut b) {
        Ok(n) => assert_eq!(n, 2),
        Err(err) => panic!("{}", err),
    };

    assert_eq!(b.bytes(), &[69, 88])
}
