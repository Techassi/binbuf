use binbuf::{read::*, BigEndian, Endianness};

#[test]
fn test_readable_impl() {
    struct Data {
        inner: u16,
    }

    impl Readable for Data {
        type Error = Error;

        fn read<E: Endianness>(buf: &mut Buffer) -> Result<Self, Self::Error> {
            let inner = u16::read::<E>(buf)?;

            Ok(Self { inner })
        }
    }

    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = Buffer::new(b.as_slice());

    let data = Data::read::<BigEndian>(&mut buf).unwrap();
    assert_eq!(data.inner, 17752)
}
