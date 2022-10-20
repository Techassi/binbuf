use binum::prelude::*;

#[test]
fn test_read_u16() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];

    match binum::BigEndian::read::<u16>(&b) {
        Ok(n) => assert_eq!(n, 17752),
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_read_u16_generic() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];

    match binum::read::<u16, BigEndian>(&b) {
        Ok(n) => assert_eq!(n, 17752),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_multi_u16() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];

    match binum::BigEndian::read_multi::<u16>(&b, 4) {
        Ok(n) => assert_eq!(n, vec![17752, 16717, 20556, 17697]),
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_read_multi_u16_generic() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];

    match binum::read_multi::<u16, BigEndian>(&b, 4) {
        Ok(n) => assert_eq!(n, vec![17752, 16717, 20556, 17697]),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_u32() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];

    match binum::BigEndian::read::<u32>(&b) {
        Ok(n) => assert_eq!(n, 1163411789),
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_read_u32_generic() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];

    match binum::read::<u32, BigEndian>(&b) {
        Ok(n) => assert_eq!(n, 1163411789),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_multi_u32() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];

    match binum::BigEndian::read_multi::<u32>(&b, 2) {
        Ok(n) => assert_eq!(n, vec![1163411789, 1347175713]),
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_read_multi_u32_generic() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];

    match binum::read_multi::<u32, BigEndian>(&b, 2) {
        Ok(n) => assert_eq!(n, vec![1163411789, 1347175713]),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_u64() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];

    match binum::BigEndian::read::<u64>(&b) {
        Ok(n) => assert_eq!(n, 4996815586883028257),
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_read_u64_generic() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];

    match binum::read::<u64, BigEndian>(&b) {
        Ok(n) => assert_eq!(n, 4996815586883028257),
        Err(err) => panic!("{}", err),
    }
}

#[test]
fn test_read_multi_u64() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];

    match binum::BigEndian::read_multi::<u64>(&b, 1) {
        Ok(n) => assert_eq!(n, vec![4996815586883028257]),
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_read_multi_u64_generic() {
    let b = vec![69, 88, 65, 77, 80, 76, 69, 33];

    match binum::read_multi::<u64, BigEndian>(&b, 1) {
        Ok(n) => assert_eq!(n, vec![4996815586883028257]),
        Err(err) => panic!("{}", err),
    }
}
