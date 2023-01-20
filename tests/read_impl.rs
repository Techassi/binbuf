use binbuf::prelude::*;

#[test]
fn test_readable_impl() {
    struct Data {
        inner: u16,
    }

    impl Readable for Data {
        type Error = BufferError;

        fn read<E: Endianness>(buf: &mut impl ToReadBuffer) -> Result<Self, Self::Error> {
            let inner = u16::read::<E>(buf)?;

            Ok(Self { inner })
        }
    }

    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = ReadBuffer::new(b.as_slice());

    let data = Data::read::<BigEndian>(&mut buf).unwrap();
    assert_eq!(data.inner, 17752)
}

#[cfg(feature = "derive")]
#[test]
fn test_readable_impl_derive_simple() {
    #[derive(Read)]
    struct Data {
        inner: u16,
    }

    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = ReadBuffer::new(b.as_slice());

    let data = Data::read::<BigEndian>(&mut buf).unwrap();
    assert_eq!(data.inner, 17752)
}

#[cfg(feature = "derive")]
#[test]
fn test_readable_impl_derive_more_fields() {
    use std::net::Ipv4Addr;

    #[derive(Read)]
    struct Data {
        v1: u16,
        v2: u16,
        v3: Ipv4Addr,
    }

    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = ReadBuffer::new(b.as_slice());

    let data = Data::read::<BigEndian>(&mut buf).unwrap();
    assert_eq!(data.v1, 17752);
    assert_eq!(data.v2, 16717);
    assert_eq!(data.v3, Ipv4Addr::new(80, 76, 69, 33));
}
