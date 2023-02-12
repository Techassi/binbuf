#[cfg(feature = "derive")]
#[test]
fn text_writeable_impl_derive_simple() {
    use binbuf::prelude::*;

    #[derive(Write)]
    struct Data {
        inner: u16,
    }

    let d = Data { inner: 17752 };
    let mut b = WriteBuffer::new();

    match d.write::<BigEndian>(&mut b) {
        Ok(n) => {
            assert_eq!(n, 2);
            assert_eq!(b.bytes(), &[69, 88]);
        }
        Err(err) => panic!("{}", err),
    };
}

#[cfg(feature = "derive")]
#[test]
fn text_writeable_impl_derive_more_fields() {
    use binbuf::prelude::*;
    use std::net::Ipv4Addr;

    #[derive(Write)]
    struct Data {
        v1: u16,
        v2: u16,
        v3: Ipv4Addr,
    }

    let d = Data {
        v1: 17752,
        v2: 16717,
        v3: Ipv4Addr::new(80, 76, 69, 33),
    };
    let mut b = WriteBuffer::new();

    match d.write::<BigEndian>(&mut b) {
        Ok(n) => {
            assert_eq!(n, 8);
            assert_eq!(b.bytes(), &[69, 88, 65, 77, 80, 76, 69, 33]);
        }
        Err(err) => panic!("{}", err),
    };
}

#[cfg(feature = "derive")]
#[test]
fn test_writeable_impl_derive_nested() {
    use binbuf::prelude::*;

    #[derive(Write)]
    struct Data {
        nested: Nested,
    }

    #[derive(Write)]
    struct Nested {
        v1: u16,
        v2: u16,
    }

    let d = Data {
        nested: Nested {
            v1: 17752,
            v2: 16717,
        },
    };

    let mut b = WriteBuffer::new();
    match d.write::<BigEndian>(&mut b) {
        Ok(n) => {
            assert_eq!(n, 4);
            assert_eq!(b.bytes(), &[69, 88, 65, 77]);
        }
        Err(err) => panic!("{}", err),
    }
}
