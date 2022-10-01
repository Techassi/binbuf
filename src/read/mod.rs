use crate::{
    error::BinaryErrorVariant, BinaryError, BinaryReadResult, Endianness, U16_OCTETS, U32_OCTETS,
    U64_OCTETS,
};

mod iter;
mod seek;

pub use iter::*;
pub use seek::*;

/// Read two octets from the byte slice `data` as an unsigned integer (`u16`).
/// This function returns an error when the provided `data` slice is too short
/// (minimum length is [`U16_OCTETS`]).
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let n = binum::read_u16(&d[0..2], binum::Endianness::Big).unwrap();
/// assert_eq!(n, 17752);
/// ```
pub fn read_u16(data: &[u8], endianness: Endianness) -> BinaryReadResult<u16> {
    if data.len() < U16_OCTETS {
        return Err(BinaryError::new(
            "Slice of bytes too short",
            BinaryErrorVariant::U16,
        ));
    }

    match endianness {
        Endianness::Big => {
            let v = ((data[0] as u16) << 8) + (data[1] as u16);
            Ok(v)
        }
        Endianness::Little => {
            let v = ((data[1] as u16) << 8) + (data[0] as u16);
            Ok(v)
        }
    }
}

/// Read two octets from the byte slice `data` as an signed integer (`i16`).
/// This function returns an error when the provided `data` slice is too short
/// (minimum length is [`U16_OCTETS`]). See [`read_u16`] for more
/// information.
pub fn read_i16(data: &[u8], endianness: Endianness) -> BinaryReadResult<i16> {
    match read_u16(data, endianness) {
        Ok(n) => Ok(n as i16),
        Err(err) => Err(err),
    }
}

/// Read four octets from the byte slice `data` as an unsigned integer (`u32`).
/// This function returns an error when the provided `data` slice is too short
/// (minimum length is [`U32_OCTETS`]).
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let n = binum::read_u32(&d[0..4], binum::Endianness::Big).unwrap();
/// assert_eq!(n, 1163411789);
/// ```
pub fn read_u32(data: &[u8], endianness: Endianness) -> BinaryReadResult<u32> {
    if data.len() < U32_OCTETS {
        return Err(BinaryError::new(
            "Slice of bytes too short",
            BinaryErrorVariant::U32,
        ));
    }

    match endianness {
        Endianness::Big => {
            let v = ((data[0] as u32) << 24)
                + ((data[1] as u32) << 16)
                + ((data[2] as u32) << 8)
                + (data[3] as u32);
            Ok(v)
        }
        Endianness::Little => {
            let v = ((data[3] as u32) << 24)
                + ((data[2] as u32) << 16)
                + ((data[1] as u32) << 8)
                + (data[0] as u32);
            Ok(v)
        }
    }
}

/// Read four octets from the byte slice `data` as an signed integer (`i32`).
/// This function returns an error when the provided `data` slice is too short
/// (minimum length is [`U32_OCTETS`]). See [`read_u32`] for more
/// information.
pub fn read_i32(data: &[u8], endianness: Endianness) -> BinaryReadResult<i32> {
    match read_u32(data, endianness) {
        Ok(n) => Ok(n as i32),
        Err(err) => Err(err),
    }
}

/// Read eight octets from the byte slice `data` as an unsigned integer (`u64`).
/// This function returns an error when the provided `data` slice is too short
/// (minimum length is [`U64_OCTETS`]).
///
/// ### Example
///
/// ```
/// let d = vec![69, 88, 65, 77, 80, 76, 69, 33];
/// let n = binum::read_u64(&d[0..], binum::Endianness::Big).unwrap();
/// assert_eq!(n, 4996815586883028257);
/// ```
pub fn read_u64(data: &[u8], endianness: Endianness) -> BinaryReadResult<u64> {
    if data.len() < U64_OCTETS {
        return Err(BinaryError::new(
            "Slice of bytes too short to read",
            BinaryErrorVariant::U64,
        ));
    }

    match endianness {
        Endianness::Big => {
            let v = ((data[0] as u64) << 56)
                + ((data[1] as u64) << 48)
                + ((data[2] as u64) << 40)
                + ((data[3] as u64) << 32)
                + ((data[4] as u64) << 24)
                + ((data[5] as u64) << 16)
                + ((data[6] as u64) << 8)
                + (data[7] as u64);
            Ok(v)
        }
        Endianness::Little => {
            let v = ((data[7] as u64) << 56)
                + ((data[6] as u64) << 48)
                + ((data[5] as u64) << 40)
                + ((data[4] as u64) << 32)
                + ((data[3] as u64) << 24)
                + ((data[2] as u64) << 16)
                + ((data[1] as u64) << 8)
                + (data[0] as u64);
            Ok(v)
        }
    }
}

/// Read eight octets from the byte slice `data` as an signed integer (`i64`).
/// This function returns an error when the provided `data` slice is too short
/// (minimum length is [`U64_OCTETS`]). See [`read_u64`] for more
/// information.
pub fn read_i64(data: &[u8], endianness: Endianness) -> BinaryReadResult<i64> {
    match read_u64(data, endianness) {
        Ok(n) => Ok(n as i64),
        Err(err) => Err(err),
    }
}
