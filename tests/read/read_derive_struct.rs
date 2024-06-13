#[cfg(feature = "derive")]
#[test]
fn test_readable_derive_simple() {
    use binbuf::{BigEndian, Read, Reader};

    #[derive(Read)]
    struct Data {
        inner: u16,
    }

    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = Reader::new(b.as_slice());

    let data = Data::read::<BigEndian>(&mut buf).unwrap();
    assert_eq!(data.inner, 17752)
}

#[cfg(feature = "derive")]
#[test]
fn test_readable_derive_three_fields() {
    use binbuf::{BigEndian, Read, Reader};

    #[derive(Read)]
    struct Data {
        v1: u16,
        v2: u32,
        v3: u16,
    }

    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = Reader::new(b.as_slice());

    let data = Data::read::<BigEndian>(&mut buf).unwrap();
    assert_eq!(data.v1, 17752);
    assert_eq!(data.v2, 1095585868);
    assert_eq!(data.v3, 17697);
}

#[cfg(feature = "derive")]
#[test]
fn test_readable_derive_ipaddr() {
    use binbuf::{BigEndian, Read, Reader};
    use std::net::Ipv4Addr;

    #[derive(Read)]
    struct Data {
        v1: u16,
        v2: u16,
        v3: Ipv4Addr,
    }

    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = Reader::new(b.as_slice());

    let data = Data::read::<BigEndian>(&mut buf).unwrap();
    assert_eq!(data.v1, 17752);
    assert_eq!(data.v2, 16717);
    assert_eq!(data.v3, Ipv4Addr::new(80, 76, 69, 33));
}

#[cfg(feature = "derive")]
#[test]
#[allow(dead_code, unused_variables)]
#[should_panic(expected = "called `Result::unwrap()` on an `Err` value: BufferTooShort")]
fn test_readable_derive_overflow() {
    use binbuf::{BigEndian, Read, Reader};

    #[derive(Read)]
    struct Data {
        v1: u64,
        v2: u16,
    }

    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = Reader::new(b.as_slice());

    let data = Data::read::<BigEndian>(&mut buf).unwrap();
}

#[cfg(feature = "derive")]
#[test]
fn test_readable_derive_nested() {
    use binbuf::{BigEndian, Read, Reader};

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
    let mut buf = Reader::new(b.as_slice());

    let data = Data::read::<BigEndian>(&mut buf).unwrap();

    assert_eq!(data.nested.v1, 17752);
    assert_eq!(data.nested.v2, 16717);
}

#[cfg(feature = "derive")]
#[test]
fn test_readable_derive_struct_attrs_error() {
    use binbuf::{read::Error, BigEndian, Read, Reader};

    #[derive(Read, Debug, PartialEq)]
    #[binbuf(error = "ReadError")]
    struct Data {
        v1: u64,
        v2: u16,
    }

    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = Reader::new(b.as_slice());

    let err = Data::read::<BigEndian>(&mut buf).unwrap_err();
    assert_eq!(err, Error::BufferTooShort);
}

#[cfg(feature = "derive")]
#[test]
fn test_readable_derive_struct_field_attrs() {
    use binbuf::Read;

    #[derive(Read, Debug, PartialEq)]
    struct Data {
        #[binbuf(skip_read = true)]
        v1: u16,
        v2: u16,
    }
}
