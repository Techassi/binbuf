use binbuf::prelude::*;

#[test]
fn test_readable_impl() {
    struct Data {
        inner: u16,
    }

    impl Readable for Data {
        type Error = BufferError;

        fn read<E: Endianness>(buf: &mut ReadBuffer) -> Result<Self, Self::Error> {
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
fn test_readable_impl_derive_three_fields() {
    #[derive(Read)]
    struct Data {
        v1: u16,
        v2: u32,
        v3: u16,
    }

    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = ReadBuffer::new(b.as_slice());

    let data = Data::read::<BigEndian>(&mut buf).unwrap();
    assert_eq!(data.v1, 17752);
    assert_eq!(data.v2, 1095585868);
    assert_eq!(data.v3, 17697);
}

#[cfg(feature = "derive")]
#[test]
fn test_readable_impl_derive_ipaddr() {
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

#[cfg(feature = "derive")]
#[test]
#[allow(dead_code, unused_variables)]
#[should_panic(expected = "called `Result::unwrap()` on an `Err` value: BufTooShort")]
fn test_readable_impl_derive_overflow() {
    #[derive(Read)]
    struct Data {
        v1: u64,
        v2: u16,
    }

    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = ReadBuffer::new(b.as_slice());

    let data = Data::read::<BigEndian>(&mut buf).unwrap();
}

#[cfg(feature = "derive")]
#[test]
fn test_readable_impl_derive_nested() {
    #[derive(Read)]
    struct Data {
        nested: Nested,
    }

    #[derive(Read)]
    struct Nested {
        v1: u16,
        v2: u16,
    }

    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = ReadBuffer::new(b.as_slice());

    let data = Data::read::<BigEndian>(&mut buf).unwrap();

    assert_eq!(data.nested.v1, 17752);
    assert_eq!(data.nested.v2, 16717);
}
