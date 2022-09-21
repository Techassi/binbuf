use crate::{
    try_read_i16, try_read_i32, try_read_i64, try_read_u16, try_read_u32, try_read_u64,
    BinaryReadResult, Endianness, U16_OCTETS, U32_OCTETS, U64_OCTETS,
};

/// Read `n` unsigned integers (`u16`) from from the byte slice `data`. This
/// function returns an error when the provided `data` slice is too short
/// (minimum length is `n` times [`U16_OCTETS`]). This function is especially
/// usefull when dealing with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let v = binum::try_iter_u16(&d[0..4], 2, binum::Endianness::Big).unwrap();
///
/// assert_eq!(v[0], 17752);
/// assert_eq!(v[1], 16717);
/// assert_eq!(v.len(), 2);
/// ```
pub fn try_iter_u16(data: &[u8], n: usize, endianness: Endianness) -> BinaryReadResult<Vec<u16>> {
    let mut v = vec![0; n];

    for i in 0..n {
        let num = match try_read_u16(&data[i * U16_OCTETS..], endianness) {
            Ok(num) => num,
            Err(err) => return Err(err),
        };

        v[i] = num;
    }

    Ok(v)
}

/// Read `n` signed integers (`i16`) from from the byte slice `data`. This
/// function returns an error when the provided `data` slice is too short
/// (minimum length is `n` times [`U16_OCTETS`]). This function is especially
/// usefull when dealing with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let v = binum::try_iter_i16(&d[0..4], 2, binum::Endianness::Big).unwrap();
///
/// assert_eq!(v[0], 17752);
/// assert_eq!(v[1], 16717);
/// assert_eq!(v.len(), 2);
/// ```
pub fn try_iter_i16(data: &[u8], n: usize, endianness: Endianness) -> BinaryReadResult<Vec<i16>> {
    let mut v = vec![0; n];

    for i in 0..n {
        let num = match try_read_i16(&data[i * U16_OCTETS..], endianness) {
            Ok(num) => num,
            Err(err) => return Err(err),
        };

        v[i] = num;
    }

    Ok(v)
}

/// Read `n` unsigned integers (`u16`) from from the byte slice `data`. This
/// function panics when the provided `data` slice is too short (minimum length
/// is `n` times [`U16_OCTETS`]). This function is especially usefull when
/// dealing with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let v = binum::iter_u16(&d[0..4], 2, binum::Endianness::Big);
///
/// assert_eq!(v[0], 17752);
/// assert_eq!(v[1], 16717);
/// assert_eq!(v.len(), 2);
/// ```
pub fn iter_u16(data: &[u8], n: usize, endianness: Endianness) -> Vec<u16> {
    match try_iter_u16(data, n, endianness) {
        Ok(v) => v,
        Err(err) => panic!("{}", err),
    }
}

/// Read `n` signed integers (`i16`) from from the byte slice `data`. This
/// function panics when the provided `data` slice is too short (minimum length
/// is `n` times [`U16_OCTETS`]). This function is especially usefull when
/// dealing with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let v = binum::iter_i16(&d[0..4], 2, binum::Endianness::Big);
///
/// assert_eq!(v[0], 17752);
/// assert_eq!(v[1], 16717);
/// assert_eq!(v.len(), 2);
/// ```
pub fn iter_i16(data: &[u8], n: usize, endianness: Endianness) -> Vec<i16> {
    match try_iter_i16(data, n, endianness) {
        Ok(v) => v,
        Err(err) => panic!("{}", err),
    }
}

/// Read `n` unsigned integers (`u32`) from from the byte slice `data`. This
/// function returns an error when the provided `data` slice is too short
/// (minimum length is `n` times [`U32_OCTETS`]). This function is especially
/// usefull when dealing with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let v = binum::try_iter_u32(&d[0..], 2, binum::Endianness::Big).unwrap();
///
/// assert_eq!(v[0], 1163411789);
/// assert_eq!(v[1], 1347175713);
/// assert_eq!(v.len(), 2);
/// ```
pub fn try_iter_u32(data: &[u8], n: usize, endianness: Endianness) -> BinaryReadResult<Vec<u32>> {
    let mut v = vec![0; n];

    for i in 0..n {
        let num = match try_read_u32(&data[i * U32_OCTETS..], endianness) {
            Ok(num) => num,
            Err(err) => return Err(err),
        };

        v[i] = num;
    }

    Ok(v)
}

/// Read `n` signed integers (`i32`) from from the byte slice `data`. This
/// function returns an error when the provided `data` slice is too short
/// (minimum length is `n` times [`U32_OCTETS`]). This function is especially
/// usefull when dealing with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let v = binum::try_iter_i32(&d[0..], 2, binum::Endianness::Big).unwrap();
///
/// assert_eq!(v[0], 1163411789);
/// assert_eq!(v[1], 1347175713);
/// assert_eq!(v.len(), 2);
/// ```
pub fn try_iter_i32(data: &[u8], n: usize, endianness: Endianness) -> BinaryReadResult<Vec<i32>> {
    let mut v = vec![0; n];

    for i in 0..n {
        let num = match try_read_i32(&data[i * U32_OCTETS..], endianness) {
            Ok(num) => num,
            Err(err) => return Err(err),
        };

        v[i] = num;
    }

    Ok(v)
}

/// Read `n` unsigned integers (`u32`) from from the byte slice `data`. This
/// function panics when the provided `data` slice is too short (minimum length
/// is `n` times [`U32_OCTETS`]). This function is especially usefull when
/// dealing with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let v = binum::iter_u32(&d[0..], 2, binum::Endianness::Big);
///
/// assert_eq!(v[0], 1163411789);
/// assert_eq!(v[1], 1347175713);
/// assert_eq!(v.len(), 2);
/// ```
pub fn iter_u32(data: &[u8], n: usize, endianness: Endianness) -> Vec<u32> {
    match try_iter_u32(data, n, endianness) {
        Ok(v) => v,
        Err(err) => panic!("{}", err),
    }
}

/// Read `n` signed integers (`i32`) from from the byte slice `data`. This
/// function panics when the provided `data` slice is too short (minimum length
/// is `n` times [`U32_OCTETS`]). This function is especially usefull when
/// dealing with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let v = binum::iter_i32(&d[0..], 2, binum::Endianness::Big);
///
/// assert_eq!(v[0], 1163411789);
/// assert_eq!(v[1], 1347175713);
/// assert_eq!(v.len(), 2);
/// ```
pub fn iter_i32(data: &[u8], n: usize, endianness: Endianness) -> Vec<i32> {
    match try_iter_i32(data, n, endianness) {
        Ok(v) => v,
        Err(err) => panic!("{}", err),
    }
}

/// Read `n` unsigned integers (`u64`) from from the byte slice `data`. This
/// function returns an error when the provided `data` slice is too short
/// (minimum length is `n` times [`U64_OCTETS`]). This function is especially
/// usefull when dealing with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let v = binum::try_iter_u64(&d[0..], 1, binum::Endianness::Big).unwrap();
///
/// assert_eq!(v[0], 4996815586883028257);
/// assert_eq!(v.len(), 1);
/// ```
pub fn try_iter_u64(data: &[u8], n: usize, endianness: Endianness) -> BinaryReadResult<Vec<u64>> {
    let mut v = vec![0; n];

    for i in 0..n {
        let num = match try_read_u64(&data[i * U64_OCTETS..], endianness) {
            Ok(num) => num,
            Err(err) => return Err(err),
        };

        v[i] = num;
    }

    Ok(v)
}

/// Read `n` signed integers (`i64`) from from the byte slice `data`. This
/// function returns an error when the provided `data` slice is too short
/// (minimum length is `n` times [`U64_OCTETS`]). This function is especially
/// usefull when dealing with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let v = binum::try_iter_i64(&d[0..], 1, binum::Endianness::Big).unwrap();
///
/// assert_eq!(v[0], 4996815586883028257);
/// assert_eq!(v.len(), 1);
/// ```
pub fn try_iter_i64(data: &[u8], n: usize, endianness: Endianness) -> BinaryReadResult<Vec<i64>> {
    let mut v = vec![0; n];

    for i in 0..n {
        let num = match try_read_i64(&data[i * U64_OCTETS..], endianness) {
            Ok(num) => num,
            Err(err) => return Err(err),
        };

        v[i] = num;
    }

    Ok(v)
}

/// Read `n` unsigned integers (`u64`) from from the byte slice `data`. This
/// function panics when the provided `data` slice is too short (minimum length
/// is `n` times [`U64_OCTETS`]). This function is especially usefull when
/// dealing with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let v = binum::iter_u64(&d[0..], 1, binum::Endianness::Big);
///
/// assert_eq!(v[0], 4996815586883028257);
/// assert_eq!(v.len(), 1);
/// ```
pub fn iter_u64(data: &[u8], n: usize, endianness: Endianness) -> Vec<u64> {
    match try_iter_u64(data, n, endianness) {
        Ok(v) => v,
        Err(err) => panic!("{}", err),
    }
}

/// Read `n` signed integers (`i64`) from from the byte slice `data`. This
/// function panics when the provided `data` slice is too short (minimum length
/// is `n` times [`U64_OCTETS`]). This function is especially usefull when
/// dealing with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let v = binum::iter_i64(&d[0..], 1, binum::Endianness::Big);
///
/// assert_eq!(v[0], 4996815586883028257);
/// assert_eq!(v.len(), 1);
/// ```
pub fn iter_i64(data: &[u8], n: usize, endianness: Endianness) -> Vec<i64> {
    match try_iter_i64(data, n, endianness) {
        Ok(v) => v,
        Err(err) => panic!("{}", err),
    }
}
