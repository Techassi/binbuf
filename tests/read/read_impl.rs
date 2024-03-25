use binbuf::{
    read::{Read, Reader, Result},
    BigEndian, Endianness,
};

#[test]
fn test_readable_impl() {
    struct Data {
        inner: u16,
    }

    impl Read for Data {
        fn read<E: Endianness>(buf: &mut Reader) -> Result<Self> {
            let inner = u16::read::<E>(buf)?;
            Ok(Self { inner })
        }
    }

    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = Reader::new(b.as_slice());

    let data = Data::read::<BigEndian>(&mut buf).unwrap();
    assert_eq!(data.inner, 17752)
}
