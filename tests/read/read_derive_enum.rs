#[cfg(feature = "derive")]
#[test]
fn test_readable_derive_simple() {
    use binbuf::prelude::*;

    #[derive(Debug, PartialEq, Read)]
    enum Code {
        Request,
        Reply,
    }

    let b = vec![0, 1];
    let mut buf = ReadBuffer::new(b.as_slice());

    match Code::read::<BigEndian>(&mut buf) {
        Ok(c) => assert_eq!(c, Code::Request),
        Err(err) => panic!("{}", err),
    }

    match Code::read::<BigEndian>(&mut buf) {
        Ok(c) => assert_eq!(c, Code::Reply),
        Err(err) => panic!("{}", err),
    }
}

#[cfg(feature = "derive")]
#[test]
fn test_readable_derive_invalid() {
    use binbuf::prelude::*;

    #[derive(Debug, PartialEq, Read)]
    enum Code {
        Request,
        Reply,
    }

    let b = vec![2];
    let mut buf = ReadBuffer::new(b.as_slice());

    match Code::read::<BigEndian>(&mut buf) {
        Ok(_) => panic!("Invalid data, this should not return Ok"),
        Err(err) => assert_eq!(err, BufferError::InvalidData),
    }
}
