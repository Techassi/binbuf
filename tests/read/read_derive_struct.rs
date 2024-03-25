#[cfg(feature = "derive")]
#[test]
fn test_readable_derive_simple() {
    use binbuf::{read::Reader, BigEndian, Read};

    #[derive(Readable)]
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
fn test_readable_derive_three_fields() {
    use binbuf::{read::Reader, BigEndian, Read};

    #[derive(Readable)]
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
fn test_readable_derive_ipaddr() {
    use binbuf::{read::Reader, BigEndian, Read};
    use std::net::Ipv4Addr;

    #[derive(Readable)]
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
#[should_panic(expected = "called `Result::unwrap()` on an `Err` value: BufferTooShort")]
fn test_readable_derive_overflow() {
    use binbuf::{read::Reader, BigEndian, Read};

    #[derive(Readable)]
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
fn test_readable_derive_nested() {
    use binbuf::{read::Reader, BigEndian, Read};

    #[derive(Readable)]
    struct Data {
        nested: Nested,
    }

    #[derive(Readable)]
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

#[cfg(feature = "derive")]
#[test]
fn test_readable_derive_struct_attrs_error() {
    use binbuf::{
        read::{Error, Reader},
        BigEndian, Read,
    };

    #[derive(Readable, Debug, PartialEq)]
    #[binbuf(error = "ReadError")]
    struct Data {
        v1: u64,
        v2: u16,
    }

    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = ReadBuffer::new(b.as_slice());

    let result = Data::read::<BigEndian>(&mut buf);
    assert_eq!(result, Result::Err(ReadError::BufferTooShort));
}

#[cfg(feature = "derive")]
#[test]
fn test_readable_derive_struct_attrs_endianness() {
    use binbuf::{
        read::{Error, Reader},
        BigEndian, Read, SupportedEndianness,
    };

    #[derive(Readable, Debug, PartialEq)]
    #[binbuf(endianness = "little")]
    struct Data {
        v1: u16,
        v2: u16,
    }

    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut buf = ReadBuffer::new(b.as_slice());

    let result = Data::read_verify::<BigEndian>(&mut buf);
    assert_eq!(
        result,
        Result::Err(ReadError::UnsupportedEndianness(
            SupportedEndianness::LittleEndian
        ))
    );
}

#[cfg(feature = "derive")]
#[test]
fn test_readable_derive_struct_field_attrs() {
    use binbuf::{BigEndian, Read};

    #[derive(Readable, Debug, PartialEq)]
    struct Data {
        #[binbuf(skip_read = true)]
        v1: u16,
        v2: u16,
    }
}
