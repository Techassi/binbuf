use crate::{
    error::BinaryErrorVariant, BinaryError, BinaryWriteResult, Endianness, U16_OCTETS, U32_OCTETS,
    U64_OCTETS,
};

/// Write `value` as an unsigned integer (`u16`) into `buf`. If the buffer is
/// too short an error is returned, otherwise the number of bytes written to
/// `buf` is returned.
///
/// ### Example
///
/// ```
/// let mut v: Vec<u8> = vec![0; 8];
/// let n = binum::try_write_u16(69, &mut v, binum::Endianness::Big).unwrap();
/// assert_eq!(n, 2);
/// ```
pub fn try_write_u16(value: u16, buf: &mut [u8], endianness: Endianness) -> BinaryWriteResult {
    if buf.len() < U16_OCTETS {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U16));
    }

    match endianness {
        Endianness::Big => {
            buf[0] = (value >> 8) as u8;
            buf[1] = value as u8;
        }
        Endianness::Little => {
            buf[0] = value as u8;
            buf[1] = (value >> 8) as u8;
        }
    }

    Ok(U16_OCTETS)
}

/// Write `value` as an unsigned integer (`u16`) into `buf`. If the buffer is
/// too short this function panics, otherwise the number of bytes written to
/// `buf` is returned.
///
/// ### Example
///
/// ```
/// let mut v: Vec<u8> = vec![0; 8];
/// let n = binum::write_u16(69, &mut v, binum::Endianness::Big);
/// assert_eq!(n, 2);
/// ```
pub fn write_u16(value: u16, buf: &mut [u8], endianness: Endianness) -> usize {
    match try_write_u16(value, buf, endianness) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    }
}

/// Write `value` as an unsigned integer (`u32`) into `buf`. If the buffer is
/// too short an error is returned, otherwise the number of bytes written to
/// `buf` is returned.
///
/// ### Example
///
/// ```
/// let mut v: Vec<u8> = vec![0; 8];
/// let n = binum::try_write_u32(69, &mut v, binum::Endianness::Big).unwrap();
/// assert_eq!(n, 4);
/// ```
pub fn try_write_u32(value: u32, buf: &mut [u8], endianness: Endianness) -> BinaryWriteResult {
    if buf.len() < U32_OCTETS {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U32));
    }

    match endianness {
        Endianness::Big => {
            buf[0] = (value >> 24) as u8;
            buf[1] = (value >> 16) as u8;
            buf[2] = (value >> 8) as u8;
            buf[3] = value as u8;
        }
        Endianness::Little => {
            buf[0] = value as u8;
            buf[1] = (value >> 8) as u8;
            buf[2] = (value >> 16) as u8;
            buf[3] = (value >> 24) as u8;
        }
    }

    Ok(U32_OCTETS)
}

/// Write `value` as an unsigned integer (`u32`) into `buf`. If the buffer is
/// too short this function panics, otherwise the number of bytes written to
/// `buf` is returned.
///
/// ### Example
///
/// ```
/// let mut v: Vec<u8> = vec![0; 8];
/// let n = binum::write_u32(69, &mut v, binum::Endianness::Big);
/// assert_eq!(n, 4);
/// ```
pub fn write_u32(value: u32, buf: &mut [u8], endianness: Endianness) -> usize {
    match try_write_u32(value, buf, endianness) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    }
}

/// Write `value` as an unsigned integer (`u64`) into `buf`. If the buffer is
/// too short an error is returned, otherwise the number of bytes written to
/// `buf` is returned.
///
/// ### Example
///
/// ```
/// let mut v: Vec<u8> = vec![0; 8];
/// let n = binum::try_write_u64(69, &mut v, binum::Endianness::Big).unwrap();
/// assert_eq!(n, 8);
/// ```
pub fn try_write_u64(value: u64, buf: &mut [u8], endianness: Endianness) -> BinaryWriteResult {
    if buf.len() < U64_OCTETS {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U64));
    }

    match endianness {
        Endianness::Big => {
            buf[0] = (value >> 56) as u8;
            buf[1] = (value >> 48) as u8;
            buf[2] = (value >> 40) as u8;
            buf[3] = (value >> 32) as u8;
            buf[4] = (value >> 24) as u8;
            buf[5] = (value >> 16) as u8;
            buf[6] = (value >> 8) as u8;
            buf[7] = value as u8;
        }
        Endianness::Little => {
            buf[0] = value as u8;
            buf[1] = (value >> 8) as u8;
            buf[2] = (value >> 16) as u8;
            buf[3] = (value >> 24) as u8;
            buf[4] = (value >> 32) as u8;
            buf[5] = (value >> 40) as u8;
            buf[6] = (value >> 48) as u8;
            buf[7] = (value >> 56) as u8;
        }
    }

    Ok(U64_OCTETS)
}

/// Write `value` as an unsigned integer (`u64`) into `buf`. If the buffer is
/// too short this function panics, otherwise the number of bytes written to
/// `buf` is returned.
///
/// ### Example
///
/// ```
/// let mut v: Vec<u8> = vec![0; 8];
/// let n = binum::write_u64(69, &mut v, binum::Endianness::Big);
/// assert_eq!(n, 8);
/// ```
pub fn write_u64(value: u64, buf: &mut [u8], endianness: Endianness) -> usize {
    match try_write_u64(value, buf, endianness) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    }
}
