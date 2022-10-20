use binum::prelude::*;

#[test]
fn test_write_u16() {
    let mut b = vec![0; 8];

    match binum::BigEndian::write::<u16>(17752, &mut b) {
        Ok(n) => {
            assert_eq!(n, 2);
            assert_eq!(b, vec![69, 88, 0, 0, 0, 0, 0, 0]);
        }
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_write_u16_generic() {
    let mut b = vec![0; 8];

    match binum::write::<u16, BigEndian>(17752, &mut b) {
        Ok(n) => {
            assert_eq!(n, 2);
            assert_eq!(b, vec![69, 88, 0, 0, 0, 0, 0, 0]);
        }
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_write_multi_u16() {
    let mut b = vec![0; 8];

    match binum::BigEndian::write_multi::<u16>(vec![17752, 16717, 20556, 17697], &mut b) {
        Ok(n) => {
            assert_eq!(n, 8);
            assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
        }
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_write_multi_u16_generic() {
    let mut b = vec![0; 8];

    match binum::write_multi::<u16, BigEndian>(vec![17752, 16717, 20556, 17697], &mut b) {
        Ok(n) => {
            assert_eq!(n, 8);
            assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
        }
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_write_u32() {
    let mut b = vec![0; 8];

    match binum::BigEndian::write::<u32>(1163411789, &mut b) {
        Ok(n) => {
            assert_eq!(n, 4);
            assert_eq!(b, vec![69, 88, 65, 77, 0, 0, 0, 0]);
        }
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_write_u32_generic() {
    let mut b = vec![0; 8];

    match binum::write::<u32, BigEndian>(1163411789, &mut b) {
        Ok(n) => {
            assert_eq!(n, 4);
            assert_eq!(b, vec![69, 88, 65, 77, 0, 0, 0, 0]);
        }
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_write_multi_u32() {
    let mut b = vec![0; 8];

    match binum::BigEndian::write_multi::<u32>(vec![1163411789, 1347175713], &mut b) {
        Ok(n) => {
            assert_eq!(n, 8);
            assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
        }
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_write_multi_u32_generic() {
    let mut b = vec![0; 8];

    match binum::write_multi::<u32, BigEndian>(vec![1163411789, 1347175713], &mut b) {
        Ok(n) => {
            assert_eq!(n, 8);
            assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
        }
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_write_u64() {
    let mut b = vec![0; 8];

    match binum::BigEndian::write::<u64>(4996815586883028257, &mut b) {
        Ok(n) => {
            assert_eq!(n, 8);
            assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
        }
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_write_u64_generic() {
    let mut b = vec![0; 8];

    match binum::write::<u64, BigEndian>(4996815586883028257, &mut b) {
        Ok(n) => {
            assert_eq!(n, 8);
            assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
        }
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_write_multi_u64() {
    let mut b = vec![0; 8];

    match binum::BigEndian::write_multi::<u64>(vec![4996815586883028257], &mut b) {
        Ok(n) => {
            assert_eq!(n, 8);
            assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
        }
        Err(err) => panic!("{}", err),
    };
}

#[test]
fn test_write_multi_u64_generic() {
    let mut b = vec![0; 8];

    match binum::write_multi::<u64, BigEndian>(vec![4996815586883028257], &mut b) {
        Ok(n) => {
            assert_eq!(n, 8);
            assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
        }
        Err(err) => panic!("{}", err),
    };
}
