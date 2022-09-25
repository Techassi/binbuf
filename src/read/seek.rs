use crate::{
    error::BinaryErrorVariant,
    read::{try_read_u16, try_read_u32, try_read_u64},
    BinaryError, BinaryReadResult, Endianness, U16_OCTETS, U32_OCTETS, U64_OCTETS,
};

/// Reads two octets at `offset` from the byte slice `data`as an unsigned
/// integer (`u16`) and advances the `offset` by [`U16_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `data`
/// vector the function returns an error.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let mut o = 0;
///
/// let n = binum::read::try_read_seek_u16(&d, &mut o, binum::Endianness::Big).unwrap();
/// assert_eq!(n, 17752);
/// assert_eq!(o, 2);
/// ```
pub fn try_read_seek_u16(
    data: &Vec<u8>,
    offset: &mut usize,
    endianness: Endianness,
) -> BinaryReadResult<u16> {
    if *offset + U16_OCTETS > data.len() {
        return Err(BinaryError::new("Offset overflow", BinaryErrorVariant::U16));
    }

    match try_read_u16(&data[*offset..], endianness) {
        Ok(n) => {
            *offset += U16_OCTETS;
            Ok(n)
        }
        Err(err) => Err(err),
    }
}

/// Reads two octets at `offset` from the byte slice `data`as a signed
/// integer (`i16`) and advances the `offset` by [`U16_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `data`
/// vector the function returns an error.
pub fn try_read_seek_i16(
    data: &Vec<u8>,
    offset: &mut usize,
    endianness: Endianness,
) -> BinaryReadResult<i16> {
    match try_read_seek_u16(data, offset, endianness) {
        Ok(n) => Ok(n as i16),
        Err(err) => Err(err),
    }
}

/// Reads two octets at `offset` from the byte slice `data`as an unsigned
/// integer (`u16`) and advances the `offset` by [`U16_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `data`
/// vector the function panics.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let mut o = 0;
///
/// let n = binum::read::read_seek_u16(&d, &mut o, binum::Endianness::Big);
/// assert_eq!(n, 17752);
/// assert_eq!(o, 2);
/// ```
pub fn read_seek_u16(data: &Vec<u8>, offset: &mut usize, endianness: Endianness) -> u16 {
    match try_read_seek_u16(data, offset, endianness) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    }
}

/// Reads two octets at `offset` from the byte slice `data`as a signed
/// integer (`i16`) and advances the `offset` by [`U16_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `data`
/// vector the function panics.
pub fn read_seek_i16(data: &Vec<u8>, offset: &mut usize, endianness: Endianness) -> i16 {
    read_seek_u16(data, offset, endianness) as i16
}

/// Reads four octets at `offset` from the byte slice `data`as an unsigned
/// integer (`u32`) and advances the `offset` by [`U32_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `data`
/// vector the function returns an error.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let mut o = 0;
///
/// let n = binum::read::try_read_seek_u32(&d, &mut o, binum::Endianness::Big).unwrap();
/// assert_eq!(n, 1163411789);
/// assert_eq!(o, 4);
/// ```
pub fn try_read_seek_u32(
    data: &Vec<u8>,
    offset: &mut usize,
    endianness: Endianness,
) -> BinaryReadResult<u32> {
    if *offset + U32_OCTETS > data.len() {
        return Err(BinaryError::new("Offset overflow", BinaryErrorVariant::U32));
    }

    match try_read_u32(&data[*offset..], endianness) {
        Ok(n) => {
            *offset += U32_OCTETS;
            Ok(n)
        }
        Err(err) => Err(err),
    }
}

/// Reads four octets at `offset` from the byte slice `data`as a signed
/// integer (`i32`) and advances the `offset` by [`U32_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `data`
/// vector the function returns an error.
pub fn try_read_seek_i32(
    data: &Vec<u8>,
    offset: &mut usize,
    endianness: Endianness,
) -> BinaryReadResult<i32> {
    match try_read_seek_u32(data, offset, endianness) {
        Ok(n) => Ok(n as i32),
        Err(err) => Err(err),
    }
}

/// Reads four octets at `offset` from the byte slice `data`as an unsigned
/// integer (`u32`) and advances the `offset` by [`U32_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `data`
/// vector the function panics.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let mut o = 0;
///
/// let n = binum::read::read_seek_u32(&d, &mut o, binum::Endianness::Big);
/// assert_eq!(n, 1163411789);
/// assert_eq!(o, 4);
/// ```
pub fn read_seek_u32(data: &Vec<u8>, offset: &mut usize, endianness: Endianness) -> u32 {
    match try_read_seek_u32(data, offset, endianness) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    }
}

/// Reads four octets at `offset` from the byte slice `data`as a signed
/// integer (`i32`) and advances the `offset` by [`U32_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `data`
/// vector the function panics.
pub fn read_seek_i32(data: &Vec<u8>, offset: &mut usize, endianness: Endianness) -> i32 {
    read_seek_u32(data, offset, endianness) as i32
}

/// Reads eight octets at `offset` from the byte slice `data`as an unsigned
/// integer (`u64`) and advances the `offset` by [`U64_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `data`
/// vector the function returns an error.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let mut o = 0;
///
/// let n = binum::read::try_read_seek_u64(&d, &mut o, binum::Endianness::Big).unwrap();
/// assert_eq!(n, 4996815586883028257);
/// assert_eq!(o, 8);
/// ```
pub fn try_read_seek_u64(
    data: &Vec<u8>,
    offset: &mut usize,
    endianness: Endianness,
) -> BinaryReadResult<u64> {
    if *offset + U64_OCTETS > data.len() {
        return Err(BinaryError::new("Offset overflow", BinaryErrorVariant::U64));
    }

    match try_read_u64(&data[*offset..], endianness) {
        Ok(n) => {
            *offset += U64_OCTETS;
            Ok(n)
        }
        Err(err) => Err(err),
    }
}

/// Reads eight octets at `offset` from the byte slice `data`as a signed
/// integer (`i64`) and advances the `offset` by [`U64_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `data`
/// vector the function returns an error.
pub fn try_read_seek_i64(
    data: &Vec<u8>,
    offset: &mut usize,
    endianness: Endianness,
) -> BinaryReadResult<i64> {
    match try_read_seek_u64(data, offset, endianness) {
        Ok(n) => Ok(n as i64),
        Err(err) => Err(err),
    }
}

/// Reads eight octets at `offset` from the byte slice `data`as an unsigned
/// integer (`u64`) and advances the `offset` by [`U64_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `data`
/// vector the function panics.
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let mut o = 0;
///
/// let n = binum::read::read_seek_u64(&d, &mut o, binum::Endianness::Big);
/// assert_eq!(n, 4996815586883028257);
/// assert_eq!(o, 8);
/// ```
pub fn read_seek_u64(data: &Vec<u8>, offset: &mut usize, endianness: Endianness) -> u64 {
    match try_read_seek_u64(data, offset, endianness) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    }
}

/// Reads eight octets at `offset` from the byte slice `data`as a signed
/// integer (`i64`) and advances the `offset` by [`U64_OCTETS`]. If the
/// advancement of the offset would overflow the length of the `data`
/// vector the function panics.
pub fn read_seek_i64(data: &Vec<u8>, offset: &mut usize, endianness: Endianness) -> i64 {
    match try_read_seek_u64(data, offset, endianness) {
        Ok(n) => n as i64,
        Err(err) => panic!("{}", err),
    }
}
