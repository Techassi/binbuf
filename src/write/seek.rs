use crate::{
    error::{BinaryError, BinaryErrorVariant},
    write::{write_i16, write_i32, write_i64, write_u16, write_u32, write_u64},
    BinaryWriteResult, Endianness, U16_OCTETS, U32_OCTETS, U64_OCTETS,
};

/// Writes two octets at `offset` into the byte slice `buf` as an unsigned
/// integer (`u16`) and advances the `offset` by [`U16_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `buf`
/// vector the function returns an error.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 2];
/// let mut o = 0;
///
/// let n = binum::write_seek_u16(17752, &mut b, &mut o, binum::Endianness::Big).unwrap();
/// assert_eq!(n, 2);
/// assert_eq!(b, vec![69, 88]);
/// ```
pub fn write_seek_u16(
    value: u16,
    buf: &mut [u8],
    offset: &mut usize,
    endianness: Endianness,
) -> BinaryWriteResult {
    if *offset + U16_OCTETS > buf.len() {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U16));
    }

    match write_u16(value, &mut buf[*offset..], endianness) {
        Ok(n) => {
            *offset += n;
            Ok(n)
        }
        Err(err) => Err(err),
    }
}

/// Writes two octets at `offset` into the byte slice `buf` as signed
/// integer (`i16`) and advances the `offset` by [`U16_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `buf`
/// vector the function returns an error.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 2];
/// let mut o = 0;
///
/// let n = binum::write_seek_i16(17752, &mut b, &mut o, binum::Endianness::Big).unwrap();
/// assert_eq!(n, 2);
/// assert_eq!(b, vec![69, 88]);
/// ```
pub fn write_seek_i16(
    value: i16,
    buf: &mut [u8],
    offset: &mut usize,
    endianness: Endianness,
) -> BinaryWriteResult {
    if *offset + U16_OCTETS > buf.len() {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U16));
    }

    match write_i16(value, &mut buf[*offset..], endianness) {
        Ok(n) => {
            *offset += n;
            Ok(n)
        }
        Err(err) => Err(err),
    }
}

/// Writes two octets at `offset` into the byte slice `buf` as an unsigned
/// integer (`u32`) and advances the `offset` by [`U32_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `buf`
/// vector the function returns an error.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 4];
/// let mut o = 0;
///
/// let n = binum::write_seek_u32(1163411789, &mut b, &mut o, binum::Endianness::Big).unwrap();
/// assert_eq!(n, 4);
/// assert_eq!(b, vec![69, 88, 65, 77]);
/// ```
pub fn write_seek_u32(
    value: u32,
    buf: &mut [u8],
    offset: &mut usize,
    endianness: Endianness,
) -> BinaryWriteResult {
    if *offset + U32_OCTETS > buf.len() {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U32));
    }

    match write_u32(value, &mut buf[*offset..], endianness) {
        Ok(n) => {
            *offset += n;
            Ok(n)
        }
        Err(err) => Err(err),
    }
}

/// Writes two octets at `offset` into the byte slice `buf` as a signed
/// integer (`i32`) and advances the `offset` by [`U32_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `buf`
/// vector the function returns an error.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 4];
/// let mut o = 0;
///
/// let n = binum::write_seek_i32(1163411789, &mut b, &mut o, binum::Endianness::Big).unwrap();
/// assert_eq!(n, 4);
/// assert_eq!(b, vec![69, 88, 65, 77]);
/// ```
pub fn write_seek_i32(
    value: i32,
    buf: &mut [u8],
    offset: &mut usize,
    endianness: Endianness,
) -> BinaryWriteResult {
    if *offset + U32_OCTETS > buf.len() {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U32));
    }

    match write_i32(value, &mut buf[*offset..], endianness) {
        Ok(n) => {
            *offset += n;
            Ok(n)
        }
        Err(err) => Err(err),
    }
}

/// Writes two octets at `offset` into the byte slice `buf` as an unsigned
/// integer (`u64`) and advances the `offset` by [`U64_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `buf`
/// vector the function returns an error.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 8];
/// let mut o = 0;
///
/// let n = binum::write_seek_u64(4996815586883028257, &mut b, &mut o, binum::Endianness::Big).unwrap();
/// assert_eq!(n, 8);
/// assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
/// ```
pub fn write_seek_u64(
    value: u64,
    buf: &mut [u8],
    offset: &mut usize,
    endianness: Endianness,
) -> BinaryWriteResult {
    if *offset + U64_OCTETS > buf.len() {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U64));
    }

    match write_u64(value, &mut buf[*offset..], endianness) {
        Ok(n) => {
            *offset += n;
            Ok(n)
        }
        Err(err) => Err(err),
    }
}

/// Writes two octets at `offset` into the byte slice `buf` as a signed
/// integer (`i64`) and advances the `offset` by [`U64_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `buf`
/// vector the function returns an error.
///
/// ### Example
///
/// ```
/// let mut b = vec![0; 8];
/// let mut o = 0;
///
/// let n = binum::write_seek_i64(4996815586883028257, &mut b, &mut o, binum::Endianness::Big).unwrap();
/// assert_eq!(n, 8);
/// assert_eq!(b, vec![69, 88, 65, 77, 80, 76, 69, 33]);
/// ```
pub fn write_seek_i64(
    value: i64,
    buf: &mut [u8],
    offset: &mut usize,
    endianness: Endianness,
) -> BinaryWriteResult {
    if *offset + U64_OCTETS > buf.len() {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U64));
    }

    match write_i64(value, &mut buf[*offset..], endianness) {
        Ok(n) => {
            *offset += n;
            Ok(n)
        }
        Err(err) => Err(err),
    }
}
