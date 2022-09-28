use crate::{
    error::BinaryErrorVariant, BinaryError, BinaryWriteResult, Endianness, U16_OCTETS, U32_OCTETS,
    U64_OCTETS,
};

mod iter;
mod seek;

pub use iter::*;
pub use seek::*;

/// Write `value` as an unsigned integer (`u16`) into `buf`. If the buffer is
/// too short an error is returned, otherwise the number of bytes written to
/// `buf` is returned.
///
/// ### Example
///
/// ```
/// let mut v: Vec<u8> = vec![0; 8];
/// let n = binum::write::try_write_u16(69, &mut v, binum::Endianness::Big).unwrap();
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

/// Write `value` as a signed integer (`i16`) into `buf`. If the buffer is
/// too short an error is returned, otherwise the number of bytes written to
/// `buf` is returned. See [`try_write_u16`] for more information.
pub fn try_write_i16(value: i16, buf: &mut [u8], endianness: Endianness) -> BinaryWriteResult {
    if buf.len() < U16_OCTETS {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U16));
    }

    match try_write_u16(value as u16, buf, endianness) {
        Ok(n) => Ok(n),
        Err(err) => Err(err),
    }
}

/// Write `value` as an unsigned integer (`u16`) into `buf`. If the buffer is
/// too short this function panics, otherwise the number of bytes written to
/// `buf` is returned.
///
/// ### Example
///
/// ```
/// let mut v: Vec<u8> = vec![0; 8];
/// let n = binum::write::write_u16(69, &mut v, binum::Endianness::Big);
/// assert_eq!(n, 2);
/// ```
pub fn write_u16(value: u16, buf: &mut [u8], endianness: Endianness) -> usize {
    match try_write_u16(value, buf, endianness) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    }
}

/// Write `value` as a signed integer (`i16`) into `buf`. If the buffer is
/// too short this function panics, otherwise the number of bytes written to
/// `buf` is returned. See [`write_u16`] for more information.
pub fn write_i16(value: i16, buf: &mut [u8], endianness: Endianness) -> usize {
    match try_write_i16(value, buf, endianness) {
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
/// let n = binum::write::try_write_u32(69, &mut v, binum::Endianness::Big).unwrap();
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

/// Write `value` as a signed integer (`i32`) into `buf`. If the buffer is
/// too short an error is returned, otherwise the number of bytes written to
/// `buf` is returned. See [`try_write_u32`] for more information.
pub fn try_write_i32(value: i32, buf: &mut [u8], endianness: Endianness) -> BinaryWriteResult {
    if buf.len() < U32_OCTETS {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U32));
    }

    match try_write_u32(value as u32, buf, endianness) {
        Ok(n) => Ok(n),
        Err(err) => Err(err),
    }
}

/// Write `value` as an unsigned integer (`u32`) into `buf`. If the buffer is
/// too short this function panics, otherwise the number of bytes written to
/// `buf` is returned.
///
/// ### Example
///
/// ```
/// let mut v: Vec<u8> = vec![0; 8];
/// let n = binum::write::write_u32(69, &mut v, binum::Endianness::Big);
/// assert_eq!(n, 4);
/// ```
pub fn write_u32(value: u32, buf: &mut [u8], endianness: Endianness) -> usize {
    match try_write_u32(value, buf, endianness) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    }
}

/// Write `value` as a signed integer (`i32`) into `buf`. If the buffer is
/// too short this function panics, otherwise the number of bytes written to
/// `buf` is returned. See [`write_u32`] for more information.
pub fn write_i32(value: i32, buf: &mut [u8], endianness: Endianness) -> usize {
    match try_write_i32(value, buf, endianness) {
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
/// let n = binum::write::try_write_u64(69, &mut v, binum::Endianness::Big).unwrap();
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

/// Write `value` as a signed integer (`i64`) into `buf`. If the buffer is
/// too short an error is returned, otherwise the number of bytes written to
/// `buf` is returned. See [`try_write_u64`] for more information.
pub fn try_write_i64(value: i64, buf: &mut [u8], endianness: Endianness) -> BinaryWriteResult {
    if buf.len() < U64_OCTETS {
        return Err(BinaryError::new("Buf too short", BinaryErrorVariant::U64));
    }

    match try_write_u64(value as u64, buf, endianness) {
        Ok(n) => Ok(n),
        Err(err) => Err(err),
    }
}

/// Write `value` as an unsigned integer (`u64`) into `buf`. If the buffer is
/// too short this function panics, otherwise the number of bytes written to
/// `buf` is returned.
///
/// ### Example
///
/// ```
/// let mut v: Vec<u8> = vec![0; 8];
/// let n = binum::write::write_u64(69, &mut v, binum::Endianness::Big);
/// assert_eq!(n, 8);
/// ```
pub fn write_u64(value: u64, buf: &mut [u8], endianness: Endianness) -> usize {
    match try_write_u64(value, buf, endianness) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    }
}

/// Write `value` as a signed integer (`i64`) into `buf`. If the buffer is
/// too short this function panics, otherwise the number of bytes written to
/// `buf` is returned. See [`write_u64`] for more information.
pub fn write_i64(value: i64, buf: &mut [u8], endianness: Endianness) -> usize {
    match try_write_i64(value, buf, endianness) {
        Ok(n) => n,
        Err(err) => panic!("{}", err),
    }
}
