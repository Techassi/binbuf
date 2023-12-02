use binbuf::{BigEndian, ReadBuffer, ReadableMulti};

#[test]
fn test_read_multi_u8() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut b = ReadBuffer::new(b.as_slice());

    let [v1, v2] = match u8::read_multi::<BigEndian, 2>(&mut b) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(v1, 69);
    assert_eq!(v2, 88);
}

#[test]
fn test_read_multi_u16() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut b = ReadBuffer::new(b.as_slice());

    let [v1, v2] = match u16::read_multi::<BigEndian, 2>(&mut b) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(v1, 17752);
    assert_eq!(v2, 16717);
}

#[test]
fn test_read_multi_u32() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut b = ReadBuffer::new(b.as_slice());

    let [v1, v2] = match u32::read_multi::<BigEndian, 2>(&mut b) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(v1, 1163411789);
    assert_eq!(v2, 1347175713);
}

#[test]
fn test_read_multi_u64() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];
    let mut b = ReadBuffer::new(b.as_slice());

    let [v] = match u64::read_multi::<BigEndian, 1>(&mut b) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(v, 4996815586883028257);
}

#[test]
fn test_read_multi_u128() {
    let b = vec![
        69, 88, 65, 77, 80, 76, 69, 33, 69, 88, 65, 77, 80, 76, 69, 33,
    ];
    let mut b = ReadBuffer::new(b.as_slice());

    let [v] = match u128::read_multi::<BigEndian, 1>(&mut b) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    };

    assert_eq!(v, 92174978314754016623629927450611041569)
}
