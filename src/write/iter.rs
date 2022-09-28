use crate::{
    error::BinaryErrorVariant,
    write::{
        try_write_i16, try_write_i32, try_write_i64, try_write_u16, try_write_u32, try_write_u64,
    },
    BinaryError, BinaryWriteResult, Endianness, U16_OCTETS, U32_OCTETS, U64_OCTETS,
};

/// Write a vector of unsigned integers (`u16`) into the buffer `buf`. This
/// function returns an error when the provided `buf` slice is too short
/// (minimum length is `values.len()` times [`U16_OCTETS`]). This function
/// does **NOT** write partial data. It checks if `buf` is big enough to fit
/// all provided `values`. This function is especially usefull when dealing
/// with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 4];
/// let d = vec![17752, 16717];
/// let n = binum::write::try_write_iter_u16(d, &mut b, binum::Endianness::Big).unwrap();
///
/// assert_eq!(n, 4);
/// assert_eq!(b, vec![69, 88, 65, 77]);
/// ```
pub fn try_write_iter_u16(
    values: Vec<u16>,
    buf: &mut [u8],
    endianness: Endianness,
) -> BinaryWriteResult {
    if values.len() * U16_OCTETS > buf.len() {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U16));
    }

    let mut bytes_written = 0;

    for i in 0..values.len() {
        match try_write_u16(values[i], &mut buf[i * U16_OCTETS..], endianness) {
            Ok(n) => bytes_written += n,
            Err(err) => return Err(err),
        };
    }

    Ok(bytes_written)
}

/// Write a vector of signed integers (`i16`) into the buffer `buf`. This
/// function returns an error when the provided `buf` slice is too short
/// (minimum length is `values.len()` times [`U16_OCTETS`]). This function
/// does **NOT** write partial data. It checks if `buf` is big enough to fit
/// all provided `values`. This function is especially usefull when dealing
/// with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 4];
/// let d = vec![17752, 16717];
/// let n = binum::write::try_write_iter_i16(d, &mut b, binum::Endianness::Big).unwrap();
///
/// assert_eq!(n, 4);
/// assert_eq!(b, vec![69, 88, 65, 77]);
/// ```
pub fn try_write_iter_i16(
    values: Vec<i16>,
    buf: &mut [u8],
    endianness: Endianness,
) -> BinaryWriteResult {
    if values.len() * U16_OCTETS > buf.len() {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U16));
    }

    let mut bytes_written = 0;

    for i in 0..values.len() {
        match try_write_i16(values[i], &mut buf[i * U16_OCTETS..], endianness) {
            Ok(n) => bytes_written += n,
            Err(err) => return Err(err),
        };
    }

    Ok(bytes_written)
}

/// Write a vector of unsigned integers (`u16`) into the buffer `buf`. This
/// function panics if the provided `buf` slice is too short (minimum length
/// is `values.len()` times [`U16_OCTETS`]). This function does **NOT** write
/// partial data. It checks if `buf` is big enough to fit all provided
/// `values`. This function is especially usefull when dealing with multiple
/// integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 4];
/// let d = vec![17752, 16717];
/// let n = binum::write::write_iter_u16(d, &mut b, binum::Endianness::Big);
///
/// assert_eq!(n, 4);
/// assert_eq!(b, vec![69, 88, 65, 77]);
/// ```
pub fn write_iter_u16(values: Vec<u16>, buf: &mut [u8], endianness: Endianness) -> usize {
    match try_write_iter_u16(values, buf, endianness) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    }
}

/// Write a vector of signed integers (`i16`) into the buffer `buf`. This
/// function panics if the provided `buf` slice is too short (minimum length
/// is `values.len()` times [`U16_OCTETS`]). This function does **NOT** write
/// partial data. It checks if `buf` is big enough to fit all provided
/// `values`. This function is especially usefull when dealing with multiple
/// integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 4];
/// let d = vec![17752, 16717];
/// let n = binum::write::write_iter_i16(d, &mut b, binum::Endianness::Big);
///
/// assert_eq!(n, 4);
/// assert_eq!(b, vec![69, 88, 65, 77]);
/// ```
pub fn write_iter_i16(values: Vec<i16>, buf: &mut [u8], endianness: Endianness) -> usize {
    match try_write_iter_i16(values, buf, endianness) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    }
}

/// Write a vector of unsigned integers (`u32`) into the buffer `buf`. This
/// function returns an error when the provided `buf` slice is too short
/// (minimum length is `values.len()` times [`U32_OCTETS`]). This function
/// does **NOT** write partial data. It checks if `buf` is big enough to fit
/// all provided `values`. This function is especially usefull when dealing
/// with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 8];
/// let d = vec![1163411789, 1347175713];
/// let n = binum::write::try_write_iter_u32(d, &mut b, binum::Endianness::Big).unwrap();
///
/// assert_eq!(n, 8);
/// assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
/// ```
pub fn try_write_iter_u32(
    values: Vec<u32>,
    buf: &mut [u8],
    endianness: Endianness,
) -> BinaryWriteResult {
    if values.len() * U32_OCTETS > buf.len() {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U32));
    }

    let mut bytes_written = 0;

    for i in 0..values.len() {
        match try_write_u32(values[i], &mut buf[i * U32_OCTETS..], endianness) {
            Ok(n) => bytes_written += n,
            Err(err) => return Err(err),
        };
    }

    Ok(bytes_written)
}

/// Write a vector of unsigned integers (`u32`) into the buffer `buf`. This
/// function returns an error when the provided `buf` slice is too short
/// (minimum length is `values.len()` times [`U32_OCTETS`]). This function
/// does **NOT** write partial data. It checks if `buf` is big enough to fit
/// all provided `values`. This function is especially usefull when dealing
/// with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 8];
/// let d = vec![1163411789, 1347175713];
/// let n = binum::write::try_write_iter_i32(d, &mut b, binum::Endianness::Big).unwrap();
///
/// assert_eq!(n, 8);
/// assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
/// ```
pub fn try_write_iter_i32(
    values: Vec<i32>,
    buf: &mut [u8],
    endianness: Endianness,
) -> BinaryWriteResult {
    if values.len() * U32_OCTETS > buf.len() {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U32));
    }

    let mut bytes_written = 0;

    for i in 0..values.len() {
        match try_write_i32(values[i], &mut buf[i * U32_OCTETS..], endianness) {
            Ok(n) => bytes_written += n,
            Err(err) => return Err(err),
        };
    }

    Ok(bytes_written)
}

/// Write a vector of unsigned integers (`u32`) into the buffer `buf`. This
/// function panics if the provided `buf` slice is too short (minimum length
/// is `values.len()` times [`U32_OCTETS`]). This function does **NOT** write
/// partial data. It checks if `buf` is big enough to fit all provided
/// `values`. This function is especially usefull when dealing with multiple
/// integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 8];
/// let d = vec![17752, 16717];
/// let n = binum::write::write_iter_u32(d, &mut b, binum::Endianness::Big);
///
/// assert_eq!(n, 8);
/// assert_eq!(b, vec![0, 0, 69, 88, 0, 0, 65, 77]);
/// ```
pub fn write_iter_u32(values: Vec<u32>, buf: &mut [u8], endianness: Endianness) -> usize {
    match try_write_iter_u32(values, buf, endianness) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    }
}

/// Write a vector of signed integers (`i32`) into the buffer `buf`. This
/// function panics if the provided `buf` slice is too short (minimum length
/// is `values.len()` times [`U32_OCTETS`]). This function does **NOT** write
/// partial data. It checks if `buf` is big enough to fit all provided
/// `values`. This function is especially usefull when dealing with multiple
/// integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 8];
/// let d = vec![17752, 16717];
/// let n = binum::write::write_iter_i32(d, &mut b, binum::Endianness::Big);
///
/// assert_eq!(n, 8);
/// assert_eq!(b, vec![0, 0, 69, 88, 0, 0, 65, 77]);
/// ```
pub fn write_iter_i32(values: Vec<i32>, buf: &mut [u8], endianness: Endianness) -> usize {
    match try_write_iter_i32(values, buf, endianness) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    }
}

/// Write a vector of unsigned integers (`u64`) into the buffer `buf`. This
/// function returns an error when the provided `buf` slice is too short
/// (minimum length is `values.len()` times [`U64_OCTETS`]). This function
/// does **NOT** write partial data. It checks if `buf` is big enough to fit
/// all provided `values`. This function is especially usefull when dealing
/// with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 8];
/// let d = vec![4996815586883028257];
/// let n = binum::write::try_write_iter_u64(d, &mut b, binum::Endianness::Big).unwrap();
///
/// assert_eq!(n, 8);
/// assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
/// ```
pub fn try_write_iter_u64(
    values: Vec<u64>,
    buf: &mut [u8],
    endianness: Endianness,
) -> BinaryWriteResult {
    if values.len() * U64_OCTETS > buf.len() {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U64));
    }

    let mut bytes_written = 0;

    for i in 0..values.len() {
        match try_write_u64(values[i], &mut buf[i * U64_OCTETS..], endianness) {
            Ok(n) => bytes_written += n,
            Err(err) => return Err(err),
        };
    }

    Ok(bytes_written)
}

/// Write a vector of signed integers (`i64`) into the buffer `buf`. This
/// function returns an error when the provided `buf` slice is too short
/// (minimum length is `values.len()` times [`U64_OCTETS`]). This function
/// does **NOT** write partial data. It checks if `buf` is big enough to fit
/// all provided `values`. This function is especially usefull when dealing
/// with multiple integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 8];
/// let d = vec![4996815586883028257];
/// let n = binum::write::try_write_iter_i64(d, &mut b, binum::Endianness::Big).unwrap();
///
/// assert_eq!(n, 8);
/// assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
/// ```
pub fn try_write_iter_i64(
    values: Vec<i64>,
    buf: &mut [u8],
    endianness: Endianness,
) -> BinaryWriteResult {
    if values.len() * U64_OCTETS > buf.len() {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U64));
    }

    let mut bytes_written = 0;

    for i in 0..values.len() {
        match try_write_i64(values[i], &mut buf[i * U64_OCTETS..], endianness) {
            Ok(n) => bytes_written += n,
            Err(err) => return Err(err),
        };
    }

    Ok(bytes_written)
}

/// Write a vector of unsigned integers (`u64`) into the buffer `buf`. This
/// function panics if the provided `buf` slice is too short (minimum length
/// is `values.len()` times [`U64_OCTETS`]). This function does **NOT** write
/// partial data. It checks if `buf` is big enough to fit all provided
/// `values`. This function is especially usefull when dealing with multiple
/// integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 8];
/// let d = vec![4996815586883028257];
/// let n = binum::write::write_iter_u64(d, &mut b, binum::Endianness::Big);
///
/// assert_eq!(n, 8);
/// assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
/// ```
pub fn write_iter_u64(values: Vec<u64>, buf: &mut [u8], endianness: Endianness) -> usize {
    match try_write_iter_u64(values, buf, endianness) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    }
}

/// Write a vector of signed integers (`i64`) into the buffer `buf`. This
/// function panics if the provided `buf` slice is too short (minimum length
/// is `values.len()` times [`U64_OCTETS`]). This function does **NOT** write
/// partial data. It checks if `buf` is big enough to fit all provided
/// `values`. This function is especially usefull when dealing with multiple
/// integers of the same kind in a row.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 8];
/// let d = vec![4996815586883028257];
/// let n = binum::write::write_iter_i64(d, &mut b, binum::Endianness::Big);
///
/// assert_eq!(n, 8);
/// assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
/// ```
pub fn write_iter_i64(values: Vec<i64>, buf: &mut [u8], endianness: Endianness) -> usize {
    match try_write_iter_i64(values, buf, endianness) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    }
}
